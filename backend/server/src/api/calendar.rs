use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::path;

use warp::Filter;
use crate::util::json_body_filter;
use db::event::Event;
use db::event::NewEvent;
use db::pool::PooledConn;
use apply::Apply;
use uuid::Uuid;
use crate::auth::user_filter;
use chrono::NaiveDateTime;
use serde::Serialize;
use serde::Deserialize;
use crate::util;
use crate::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewEventMessage {
    pub title: String,
    pub text: String,
    pub start_at: NaiveDateTime,
    pub stop_at: NaiveDateTime
}

impl NewEventMessage {
    fn to_new_event(self, user_uuid: Uuid) -> NewEvent {
        NewEvent {
            user_uuid,
            title: self.title,
            text: self.text,
            start_at: self.start_at,
            stop_at: self.stop_at
        }
    }
}



pub fn calendar_api(state: &State) -> BoxedFilter<(impl Reply,)> {

    // TODO take query parameters for month and year
    let get_events = warp::get2()
        .and(path!("events"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    let events_today = warp::get2()
        .and(path!("events" / "today"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_today(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    let events_month = warp::get2()
        .and(path!("events" / "month"))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Event::events_month(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    let create_event = warp::post2()
        .and(json_body_filter(50))
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|e: NewEventMessage, user_uuid: Uuid, conn: PooledConn| {
            let new_event = e.to_new_event(user_uuid);
            Event::create_event(new_event, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    // TODO, do we want canceling events as well?
    let delete_event = warp::delete2()
        .and(path!(Uuid))
        .and(state.db.clone())
        .and_then(| event_uuid: Uuid, conn: PooledConn| {
            Event::delete_event(event_uuid, &conn)
               .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    let modify_event = warp::post2()
        .and(json_body_filter(50))
        .map(|_:String| {
            "UNIMPLEMENTED"
        });

    let events = path!("event")
        .and(
            get_events
                .or(create_event)
                .or(events_today)
                .or(events_month)
                .or(delete_event)
                .or(modify_event)
        );

    path!("calendar")
        .and(
            events
        )
        .boxed()

}