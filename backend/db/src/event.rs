use crate::schema;
use crate::schema::events;
use crate::schema::events::SqlType;
use crate::user::User;
use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Timelike;
use diesel::pg::Pg;
use diesel::query_dsl::QueryDsl;
use diesel::result::QueryResult;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::{Identifiable, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "events"]
pub struct Event {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct NewEvent {
    pub user_uuid: Uuid,
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime,
}

#[derive(Clone, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct EventChangeset {
    pub uuid: Uuid,
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime,
}

type All = diesel::dsl::Select<events::table, AllColumns>;

type AllColumns = (
    events::uuid,
    events::user_uuid,
    events::title,
    events::text,
    events::start_at,
    events::stop_at,
);

pub const ALL_COLUMNS: AllColumns = (
    events::uuid,
    events::user_uuid,
    events::title,
    events::text,
    events::start_at,
    events::stop_at,
);

pub type BoxedQuery<'a> = events::BoxedQuery<'a, Pg, SqlType>;

impl Event {
    pub(crate) fn all() -> All {
        events::table.select(ALL_COLUMNS)
    }
    pub(crate) fn user_events<'a>(user_uuid: Uuid) -> BoxedQuery<'a> {
        Self::all()
            .filter(events::user_uuid.eq(user_uuid))
            .into_boxed()
    }

    pub fn events(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid).load::<Event>(conn)
    }

    pub fn events_today(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        // TODO may want to make local at some point
        // yes, this doesn't take into account the timezone of the user :/
        let today_00_00 = chrono::Utc::now()
            .naive_utc()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let tomorrow_00_00 = today_00_00 + chrono::Duration::days(1);

        Self::user_events(user_uuid)
            .filter(
                events::dsl::start_at
                    .gt(today_00_00)
                    .and(events::dsl::start_at.lt(tomorrow_00_00)),
            ) // TODO impl OR events that end before tomorrow?
            .load::<Event>(conn)
    }

    /// Actually not a even month.
    /// This gives every event made from the beginning of this month, to five weeks after that.
    pub fn events_month(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        // TODO may want to make local at some point
        // yes, this doesn't take into account the timezone of the user :/
        let start_of_this_month = chrono::Utc::now()
            .naive_utc()
            .with_day0(0) // first day of the month
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let five_weeks = start_of_this_month + chrono::Duration::weeks(5);

        Self::user_events(user_uuid)
            .filter(
                events::start_at
                    .gt(start_of_this_month)
                    .and(events::start_at.lt(five_weeks)),
            )
            .load::<Event>(conn)
    }

    pub fn events_from_n_to_n(
        user_uuid: Uuid,
        start: NaiveDateTime,
        end: NaiveDateTime,
        conn: &PgConnection,
    ) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid)
            .filter(events::start_at.gt(start).and(events::start_at.lt(end)))
            .load::<Event>(conn)
    }

    pub fn get_event(uuid: Uuid, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::get_row(schema::events::table, uuid, conn)
    }

    pub fn create_event(new_event: NewEvent, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::create_row(schema::events::table, new_event, conn)
    }

    pub fn delete_event(uuid: Uuid, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::delete_row(schema::events::table, uuid, conn)
    }

    pub fn change_event(changeset: EventChangeset, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::update_row(schema::events::table, changeset, conn)
    }
}
