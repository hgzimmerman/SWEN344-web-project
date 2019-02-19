//! All database queries directly related to calendar events are contained within this module.
use crate::{
    schema::{
        self,
        events::{self, SqlType},
    },
    user::User,
};
use chrono::{Datelike, NaiveDateTime, Timelike};
use diesel::{
    pg::Pg, query_dsl::QueryDsl, result::QueryResult, BoolExpressionMethods, ExpressionMethods,
    Identifiable, Insertable, PgConnection, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A struct that represents a row in the `events` table.
///
/// # Note
/// The times associated with `Events` are created using UTC.
#[derive(Clone, Debug, Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "events"]
pub struct Event {
    /// Unique identifier
    pub uuid: Uuid,
    /// The user to which the event belongs.
    pub user_uuid: Uuid,
    /// The title of the event.
    pub title: String,
    /// The body of the event.
    pub text: String,
    /// When the event starts.
    pub start_at: NaiveDateTime,
    /// When the event stops.
    pub stop_at: NaiveDateTime,
}

/// A struct that facilitates creation of a new row in the `events` table.
#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct NewEvent {
    /// The user to which the event belongs.
    pub user_uuid: Uuid,
    /// The title of the event.
    pub title: String,
    /// The body of the event.
    pub text: String,
    /// When the event starts.
    pub start_at: NaiveDateTime,
    /// When the event stops.
    pub stop_at: NaiveDateTime,
}

/// A changeset that facilitates altering a row in the `events` table.
#[derive(Clone, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct EventChangeset {
    /// Unique identifier
    pub uuid: Uuid,
    /// The title of the event.
    pub title: String,
    /// The body of the event.
    pub text: String,
    /// When the event starts.
    pub start_at: NaiveDateTime,
    /// When the event stops.
    pub stop_at: NaiveDateTime,
}

/// Limits the number to between 0 and 11.
#[derive(Clone, Copy, Debug)]
pub struct MonthIndex(u32);

impl MonthIndex {
    /// Converts a 0 indexed u32 to a month index.
    pub fn from_1_indexed_u32(value: u32) -> Option<Self> {
        if value == 0 || value > 12 {
            None
        } else {
            Some(MonthIndex(value - 1))
        }
    }

    /// Converts a 0 indexed u32 to a month index.
    pub fn from_0_indexed_u32(value: u32) -> Option<Self> {
        if  value > 11 {
            None
        } else {
            Some(MonthIndex(value))
        }
    }
}

/// Wrapper around year values.
/// It provides basic validation.
#[derive(Clone, Copy, Debug)]
pub struct Year(i32);
impl Year {
    /// Validates that the i32 is a valid year
    /// For lack of understanding of the chrono API, I'm limiting this to 10_000 bce/ce.
   pub fn from_i32(value: i32) -> Option<Self> {
      if value < -10_0000  || value > 10_000 {
          None
      } else {
          Some(Year(value))
      }
   }
}

/// A type representing all the columns in the events table.
type All = diesel::dsl::Select<events::table, AllColumns>;

/// All columns contained within the events table
type AllColumns = (
    events::uuid,
    events::user_uuid,
    events::title,
    events::text,
    events::start_at,
    events::stop_at,
);

/// All columns contained within the event's table.
pub const ALL_COLUMNS: AllColumns = (
    events::uuid,
    events::user_uuid,
    events::title,
    events::text,
    events::start_at,
    events::stop_at,
);

/// Abstract boxed query specific to the events table and Postgres.
pub type BoxedQuery<'a> = events::BoxedQuery<'a, Pg, SqlType>;

impl Event {
    /// Abstract select statement getting all columns in `events` table.
    pub(crate) fn all() -> All {
        events::table.select(ALL_COLUMNS)
    }
    /// Abstract query returning all events that belong to a user.
    pub(crate) fn user_events<'a>(user_uuid: Uuid) -> BoxedQuery<'a> {
        Self::all()
            .filter(events::user_uuid.eq(user_uuid))
            .into_boxed()
    }

    /// Allows the creation of many events at a time.
    pub fn import_events(new_events: Vec<NewEvent>, conn: &PgConnection) -> QueryResult<()> {
        new_events.chunks(20_000)
            .map(move |chunk| {
                diesel::insert_into(events::table)
                    .values(chunk)
                    .execute(conn)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    /// Returns every event that belongs to a given user.
    pub fn events(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid).load::<Event>(conn)
    }


    /// The month index is 0-indexed.
    /// So 0-11 are valid input values.
    pub fn events_for_any_month(month_index: MonthIndex, year: Year, user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        let start = chrono::Utc::now()
            .naive_utc()
            .with_year(year.0)
            .unwrap()
            .with_month0(month_index.0)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let end = start.with_month((month_index.0 + 1) % 13).unwrap();
        Event::events_from_n_to_n(user_uuid, start, end, conn)
    }

    /// All events that belong to a user that ocurr on the current date.
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
        // TODO This may want to explicitly make this exactly a month.
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

    /// All events occurring from a starting time to an end time, that belong to a user.
    fn events_from_n_to_n(
        user_uuid: Uuid,
        start: NaiveDateTime,
        end: NaiveDateTime,
        conn: &PgConnection,
    ) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid)
            .filter(events::start_at.gt(start).and(events::start_at.lt(end)))
            .load::<Event>(conn)
    }

    /// Gets a single event.
    pub fn get_event(uuid: Uuid, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::get_row(schema::events::table, uuid, conn)
    }

    /// Creates a new event.
    pub fn create_event(new_event: NewEvent, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::create_row(schema::events::table, new_event, conn)
    }

    /// Deletes an event.
    pub fn delete_event(uuid: Uuid, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::delete_row(schema::events::table, uuid, conn)
    }

    /// Alters an event.
    pub fn change_event(changeset: EventChangeset, conn: &PgConnection) -> QueryResult<Event> {
        crate::util::update_row(schema::events::table, changeset, conn)
    }
}
