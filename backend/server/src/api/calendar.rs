//! Responsible for hosting everything related to calendar data.
use crate::state::State;
use warp::{filters::BoxedFilter, path, Reply};

use crate::{
    error::Error,
    server_auth::user_filter,
    util::{self, json_body_filter},
};
use chrono::NaiveDateTime;
use db::event::{Event, EventChangeset, NewEvent};
use pool::PooledConn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Filter, Rejection};
use log::info;
use db::event::MonthIndex;
use db::event::Year;

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

/// Calendar api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn calendar_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Calendar Api");
    // Get all events in the NewEventRequest format.
    let export_events = warp::get2()
        .and(path!("events"))
        .and(path::end())
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events(user_uuid, &conn)
                .map(|events| {
                    events
                        .into_iter()
                        .map(|event| {
                            NewEventRequest {
                                title: event.title,
                                text: event.text,
                                start_at: event.start_at,
                                stop_at: event.stop_at
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(Error::from_reject)
                .map(util::json)
        });

    let import_events = warp::post2()
        .and(path!("events/import"))
        .and(path::end())
        .and(json_body_filter(350)) // you can import a bunch 'o events
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|events: Vec<NewEventRequest>, user_uuid: Uuid, conn: PooledConn| {
            let events = events.into_iter().map(|e| e.into_new_event(user_uuid)).collect::<Vec<_>>();
            Event::import_events(events, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    // TODO take optional query parameters for month and year
        let get_events_custom_month_and_year = warp::get2()
            .and(path!("events" / i32 / u32))
            .and(path::end())
            .and(user_filter(state))
            .and(state.db.clone())
            .and_then(|year: i32, month_index: u32, user_uuid: Uuid, conn: PooledConn| {
                let month_index = MonthIndex::from_1_indexed_u32(month_index)
                    .ok_or_else(|| Error::bad_request("Month index is out of bounds (needs 1-12)").reject())?;
                let year = Year::from_i32(year)
                    .ok_or_else(|| Error::bad_request("Year is not within reasonable bounds").reject())?;
                Event::events_for_any_month(month_index, year, user_uuid, &conn)
                    .map_err(Error::from_reject)
                    .map(util::json)
            });

    // Events Today
    let events_today = warp::get2()
        .and(path!("events" / "today"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_today(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    // This month
    let events_month = warp::get2()
        .and(path!("events" / "month"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_month(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    let create_event = warp::post2()
        .and(json_body_filter(50))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|e: NewEventRequest, user_uuid: Uuid, conn: PooledConn| {
            let new_event = e.into_new_event(user_uuid);
            // check logical ordering of start and stop times
            if new_event.start_at > new_event.stop_at {
                Error::bad_request("Request can't start after it has ended.").reject_result()
            } else {
                Event::create_event(new_event, &conn)
                    .map_err(Error::from_reject)
                    .map(util::json)
            }
        });

    let delete_event = warp::delete2()
        .and(path!(Uuid))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(delete_event);

    let modify_event = warp::put2()
        .and(json_body_filter(50))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(modify_event);

    let events = path!("event").and(
        export_events
            .or(import_events)
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
