use crate::state::State;
use warp::{filters::BoxedFilter, path, Reply};

use crate::util::json_body_filter;
use db::event::{Event, NewEvent};
use pool::PooledConn;
use warp::Filter;
//use apply::Apply;
use crate::{auth::user_filter, error::Error, util};
use chrono::NaiveDateTime;
use db::event::EventChangeset;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Rejection;

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

pub fn calendar_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    // TODO take optional query parameters for month and year
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
                Error::BadRequest.reject_result()
            } else {
                Event::create_event(new_event, &conn)
                    .map_err(Error::from_reject)
                    .map(util::json)
            }
        });

    // TODO, do we want canceling events as well?
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
                Err(Error::NotAuthorized {
                    reason: "User does not own event",
                })
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
        Error::BadRequest.reject_result() // TODO better error message.
    } else {
        // Check if the user has authority to change the event.
        Event::get_event(changeset.uuid, &conn)
            .map_err(Error::from)
            .and_then(|event: Event| {
                if event.user_uuid != user_uuid {
                    Err(Error::NotAuthorized {
                        reason: "User does not own event",
                    })
                } else {
                    Event::change_event(changeset, &conn).map_err(Error::from)
                }
            })
            .map_err(Error::reject)
            .map(util::json)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::auth::{JwtPayload, Secret};

    #[test]
    fn get_events() {
        let secret = Secret::new("TEST");
        let state = State::new(Some(secret.clone()));
        let filter = calendar_api(&state);
        let jwt = JwtPayload::new(Uuid::new_v4())
            .encode_jwt_string(&secret)
            .unwrap();

        assert!(warp::test::request()
            .method("GET")
            .path("/calendar/event/events")
            .header("Authorization", format!("bearer {}", jwt))
            .matches(&filter));
    }

    #[test]
    fn get_events_today() {
        let secret = Secret::new("TEST");
        let state = State::new(Some(secret.clone()));
        let filter = calendar_api(&state);
        let jwt = JwtPayload::new(Uuid::new_v4())
            .encode_jwt_string(&secret)
            .unwrap();

        assert!(warp::test::request()
            .method("GET")
            .path("/calendar/event/events/today")
            .header("Authorization", format!("bearer {}", jwt))
            .matches(&filter));
    }

    #[test]
    fn get_events_month() {
        let secret = Secret::new("TEST");
        let state = State::new(Some(secret.clone()));
        let filter = calendar_api(&state);
        let jwt = JwtPayload::new(Uuid::new_v4())
            .encode_jwt_string(&secret)
            .unwrap();

        assert!(warp::test::request()
            .method("GET")
            .path("/calendar/event/events/month")
            .header("Authorization", format!("bearer {}", jwt))
            .matches(&filter));
    }

}
