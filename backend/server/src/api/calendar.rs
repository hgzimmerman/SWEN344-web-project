//! Responsible for hosting everything related to calendar data.
use crate::state::State;
use warp::{filters::BoxedFilter, path, Reply};

use crate::{
    error::Error,
    server_auth::user_filter,
    util::{self, json_body_filter},
};
use apply::Apply;
use chrono::{DateTime, NaiveDateTime, Utc};
use db::event::{Event, EventChangeset, ImportExportEvent, MonthIndex, NewEvent, Year};
use log::info;
use pool::PooledConn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Filter, Rejection};

/// A request for creating a new calendar Event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewEventRequest {
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime,
}

impl NewEventRequest {
    /// Attach the user uuid acquired from the JWT to create a NewEvent that can be inserted into the DB.
    ///
    /// # Arguments
    /// * user_uuid - The UUID of the user to be combined with Self to create a NewEvent.
    fn into_new_event(self, user_uuid: Uuid) -> NewEvent {
        NewEvent {
            user_uuid,
            title: self.title,
            text: self.text,
            start_at: self.start_at,
            stop_at: self.stop_at,
        }
    }
}

/// Query parameters for /events
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimeBoundaries {
    start: DateTime<Utc>,
    stop: DateTime<Utc>,
}

/// Calendar api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn calendar_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Calendar Api");
    // Get all events in the NewEventRequest format.
    let export_events = warp::get2()
        .and(path!("events" / "export"))
        .and(path::end())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |user_uuid: Uuid,
             conn: PooledConn|
             -> Result<Vec<ImportExportEvent>, diesel::result::Error> {
                Event::export_events(user_uuid, &conn)
            },
        )
        .and_then(util::json_or_reject);

    let import_events = warp::post2()
        .and(path!("events" / "import"))
        .and(path::end())
        .and(warp::post2())
        .and(json_body_filter(350)) // you can import a bunch 'o events
        .and(user_filter(state))
        .and(state.db())
        .map(
            |events: Vec<ImportExportEvent>,
             user_uuid: Uuid,
             conn: PooledConn|
             -> Result<(), diesel::result::Error> {
                Event::import_events(events, user_uuid, &conn)
            },
        )
        .and_then(util::json_or_reject);

    // Events with time bounds
    let events = warp::get2()
        .and(path!("events"))
        .and(warp::query())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |tb: TimeBoundaries,
             user_uuid: Uuid,
             conn: PooledConn|
             -> Result<Vec<Event>, diesel::result::Error> {
                Event::events_from_n_to_n(
                    user_uuid,
                    tb.start.naive_utc(),
                    tb.stop.naive_utc(),
                    &conn,
                )
            },
        )
        .and_then(util::json_or_reject);

    // TODO deprecate
    let get_events_custom_month_and_year = warp::get2()
        .and(path!("events" / i32 / u32)) // "events", year, month
        .and(path::end())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |year: i32,
             month_index: u32,
             user_uuid: Uuid,
             conn: PooledConn|
             -> Result<Vec<Event>, Error> {
                let month_index = MonthIndex::from_1_indexed_u32(month_index).ok_or_else(|| {
                    Error::bad_request("Month index is out of bounds (needs 1-12)")
                })?;
                let year = Year::from_i32(year)
                    .ok_or_else(|| Error::bad_request("Year is not within reasonable bounds"))?;
                Event::events_for_any_month(month_index, year, user_uuid, &conn)
                    .map_err(Error::from)
            },
        )
        .and_then(util::json_or_reject);

    // TODO deprecate
    // Events Today
    let events_today = warp::get2()
        .and(path!("events" / "today"))
        .and(user_filter(state))
        .and(state.db())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_today(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    // TODO deprecate
    // This month
    let events_month = warp::get2()
        .and(path!("events" / "month"))
        .and(user_filter(state))
        .and(state.db())
        .map(
            |user_uuid: Uuid, conn: PooledConn| -> Result<Vec<Event>, diesel::result::Error> {
                Event::events_month(user_uuid, &conn)
            },
        )
        .and_then(util::json_or_reject);

    let create_event = warp::post2()
        .and(json_body_filter(50))
        .and(user_filter(state))
        .and(state.db())
        .map(|e: NewEventRequest, user_uuid: Uuid, conn: PooledConn| {
            let new_event = e.into_new_event(user_uuid);
            // check logical ordering of start and stop times
            if new_event.start_at > new_event.stop_at {
                Error::bad_request("Request can't start after it has ended.").apply(Err)
            } else {
                Event::create_event(new_event, &conn).map_err(Error::from)
            }
        })
        .and_then(util::json_or_reject);

    let delete_event = warp::delete2()
        .and(path!(Uuid))
        .and(user_filter(state))
        .and(state.db())
        .and_then(delete_event);

    let modify_event = warp::put2()
        .and(json_body_filter(50))
        .and(user_filter(state))
        .and(state.db())
        .and_then(modify_event);

    let events = path!("event").and(
        export_events
            .or(import_events)
            .or(events)
            .or(create_event)
            .or(events_today)
            .or(events_month)
            .or(get_events_custom_month_and_year)
            .or(delete_event)
            .or(modify_event),
    );

    path!("calendar").and(events).boxed()
}

/// Deletes the event after checking that it belongs to the user.
/// First, it gets the event from the database, then it checks if the event belongs to the user,
/// then it deletes the event.
///
/// # Arguments
/// * event_uuid - The uuid of the event to be deleted.
/// * user_uuid - The user's uuid.
/// Used to validate that the user owns the event being modified.
/// * conn - The connection to the database.
fn delete_event(
    event_uuid: Uuid,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    Event::get_event(event_uuid, &conn)
        .map_err(Error::from)
        .and_then(|event: Event| {
            if event.user_uuid != user_uuid {
                Err(Error::not_authorized("User UUIDs do not match"))
            } else {
                Event::delete_event(event_uuid, &conn).map_err(Error::from)
            }
        })
        .map_err(Error::reject)
        .map(util::json)
}

/// Modifies an existing event.
///
/// # Arguments
/// * changeset - The changeset used to modify the event.
/// * user_uuid - The user's uuid.
/// Used to validate that the user owns the event being modified.
/// * conn - The connection to the database.
fn modify_event(
    changeset: EventChangeset,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    // check logical ordering of start and stop times
    if changeset.start_at > changeset.stop_at {
        Error::bad_request("Request can't start after it has ended.").reject_result()
    } else {
        // Check if the user has authority to change the event.
        Event::get_event(changeset.uuid, &conn)
            .map_err(Error::from)
            .and_then(|event: Event| {
                if event.user_uuid != user_uuid {
                    Err(Error::not_authorized("User UUIDs do not match"))
                } else {
                    Event::change_event(changeset, &conn).map_err(Error::from)
                }
            })
            .map_err(Error::reject)
            .map(util::json)
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::{api::auth::get_jwt, testing_fixtures::user::UserFixture};
    use authorization::{Secret, AUTHORIZATION_HEADER_KEY, BEARER};
    use pool::Pool;
    use testing_common::setup::setup_warp;

    #[test]
    fn date_time_query_param_matches() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = calendar_api(&s);

            let jwt = get_jwt(&s);

            let times = TimeBoundaries {
                start: chrono::Utc::now(),
                stop: chrono::Utc::now(),
            };
            dbg!(serde_urlencoded::to_string(times));

            let resp = warp::test::request()
                .method("GET")
                .path("/calendar/event/events?start=2019-04-15T15%3A13%3A56.792584378Z&stop=2019-04-15T15%3A13%3A56.792588175Z")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            assert_eq!(resp.status(), 200);
        });
    }
}
