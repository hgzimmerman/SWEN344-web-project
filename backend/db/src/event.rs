//! All database queries directly related to calendar events are contained within this module.
use crate::{
    schema::{
        self,
        events::{self, SqlType},
    },
    user::User,
};
use apply::Apply;
use chrono::{NaiveDateTime};
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

/// A type used for importing and exporting events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportExportEvent {
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
        if value > 11 {
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
        if value < -10_0000 || value > 10_000 {
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

    fn delete_events_for_user(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(events::table.filter(events::user_uuid.eq(user_uuid))).execute(conn)
    }

    /// Allows the creation of many events at a time.
    pub fn import_events(
        import_events: Vec<ImportExportEvent>,
        user_uuid: Uuid,
        conn: &PgConnection,
    ) -> QueryResult<()> {
        // Requirements call for deduplication to be performed by just deleting the every event for the user.
        Event::delete_events_for_user(user_uuid, conn)?;

        let new_events: Vec<NewEvent> = import_events
            .into_iter()
            .map(|event| NewEvent {
                user_uuid,
                title: event.title,
                text: event.text,
                start_at: event.start_at,
                stop_at: event.stop_at,
            })
            .collect();

        new_events
            .chunks(20_000)
            .map(move |chunk| {
                diesel::insert_into(events::table)
                    .values(chunk)
                    .execute(conn)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    /// Returns every event that belongs to a given user, without user information.
    pub fn export_events(
        user_uuid: Uuid,
        conn: &PgConnection,
    ) -> QueryResult<Vec<ImportExportEvent>> {
        Self::user_events(user_uuid)
            .load::<Event>(conn)?
            .into_iter()
            .map(|e| ImportExportEvent {
                title: e.title,
                text: e.text,
                start_at: e.start_at,
                stop_at: e.stop_at,
            })
            .collect::<Vec<_>>()
            .apply(Ok)
    }

    /// Returns every event that belongs to a given user.
    pub fn events(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<Event>> {
        Self::user_events(user_uuid).load::<Event>(conn)
    }

    /// All events occurring from a starting time to an end time, that belong to a user.
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
