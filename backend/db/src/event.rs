use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{
    Identifiable,
    Queryable,
    Insertable,
    RunQueryDsl
};
use diesel::PgConnection;
use crate::schema::events;
use crate::schema;
use diesel::query_dsl::QueryDsl;
use diesel::ExpressionMethods;
use diesel::pg::Pg;
use crate::schema::events::SqlType;
use diesel::result::QueryResult;
use chrono::Timelike;
use chrono::Datelike;
use diesel::BoolExpressionMethods;

#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "events"]
pub struct Event {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub text: String,
    pub time_due: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct NewEvent {
    pub user_uuid: Uuid,
    pub text: String,
    pub time_due: NaiveDateTime
}



// TODO needed functions
// Modify Event

type All = diesel::dsl::Select<events::table, AllColumns>;

type AllColumns = (
    events::uuid,
    events::user_uuid,
    events::text,
    events::time_due
);

pub const ALL_COLUMNS: AllColumns = (
    events::uuid,
    events::user_uuid,
    events::text,
    events::time_due
);

pub type BoxedQuery<'a> = events::BoxedQuery<'a, Pg, SqlType>;

impl Event {

    pub (crate) fn all() -> All {
        events::table.select(ALL_COLUMNS)
    }
    pub (crate) fn user_events<'a>(user_uuid: Uuid) -> BoxedQuery<'a> {
        Self::all()
            .filter(events::user_uuid.eq(user_uuid))
            .into_boxed()
    }

    pub fn events(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid)
            .load::<Event>(conn)
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
                events::time_due.gt(today_00_00)
                    .and(events::time_due.lt(tomorrow_00_00)))
            .load::<Event>(conn)
    }

    /// Actually not a even month.
    /// This gives every event made from the beginning of this month, to five weeks after that.
    pub fn events_month(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        // TODO may want to make local at some point
        // yes, this doesn't take into account the timezone of the user :/
        let start_of_this_month = chrono::Utc::now()
            .naive_utc()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let five_weeks= start_of_this_month + chrono::Duration::weeks(5);


        Self::user_events(user_uuid)
            .filter(
                events::time_due.gt(start_of_this_month)
                    .and(events::time_due.lt(five_weeks)))
            .load::<Event>(conn)
    }

    pub fn events_from_n_to_n(user_uuid: Uuid, start: NaiveDateTime, end: NaiveDateTime, conn: &PgConnection) -> QueryResult<Vec<Event>> {
         Self::user_events(user_uuid)
            .filter(
                events::time_due.gt(start)
                    .and(events::time_due.lt(end)))
            .load::<Event>(conn)
    }

    pub fn create_event(new_event: NewEvent, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::create_row(schema::events::table, new_event, conn)
    }

    pub fn delete_event(uuid: Uuid, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::delete_row(schema::events::table, uuid, conn)
    }

}

