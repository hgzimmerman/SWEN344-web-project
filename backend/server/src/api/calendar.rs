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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewEventMessage {
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime,
}

impl NewEventMessage {
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
    let get_events = warp::get2()
        .and(path!("events"))
        .and(path::end())
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    // TODO take optional query parameters for month and year
    //    let get_events = warp::get2()
    //        .and(path!("events" / u32))
    //        .and(path::end())
    //        .and(user_filter(state))
    //        .and(state.db.clone())
    //        .and_then(|month_index: u32, user_uuid: Uuid, conn: PooledConn| -> Result<impl Reply, Rejection> {
    ////            Event::events(user_uuid, &conn)
    ////                .map_err(Error::from_reject)
    ////                .map(util::json)
    //            unimplemented!()
    //        });

    let events_today = warp::get2()
        .and(path!("events" / "today"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_today(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

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
        .and_then(|e: NewEventMessage, user_uuid: Uuid, conn: PooledConn| {
            let new_event = e.into_new_event(user_uuid);
            // check logical ordering of start and stop times
            if new_event.start_at > new_event.stop_at {
                Error::BadRequestStr("Request can't start after it has ended.").reject_result()
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
        get_events
            .or(create_event)
            .or(events_today)
            .or(events_month)
            .or(delete_event)
            .or(modify_event),
    );

    path!("calendar").and(events).boxed()
}

/// Deletes the event after checking that it belongs to the user.
fn delete_event(
    event_uuid: Uuid,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    Event::get_event(event_uuid, &conn)
        .map_err(Error::from)
        .and_then(|event: Event| {
            if event.user_uuid != user_uuid {
                Err(Error::BadRequest)
            } else {
                Event::delete_event(event_uuid, &conn).map_err(Error::from)
            }
        })
        .map_err(Error::reject)
        .map(util::json)
}

fn modify_event(
    changeset: EventChangeset,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    // check logical ordering of start and stop times
    if changeset.start_at > changeset.stop_at {
        Error::BadRequestStr("Request can't start after it has ended.").reject_result()
    } else {
        // Check if the user has authority to change the event.
        Event::get_event(changeset.uuid, &conn)
            .map_err(Error::from)
            .and_then(|event: Event| {
                if event.user_uuid != user_uuid {
                    Err(Error::BadRequest)
                } else {
                    Event::change_event(changeset, &conn).map_err(Error::from)
                }
            })
            .map_err(Error::reject)
            .map(util::json)
    }
}
