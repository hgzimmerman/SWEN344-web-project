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

pub fn calendar_api(state: &State) -> BoxedFilter<(impl Reply,)> {

    // TODO take query parameters for month and year
    let get_events = warp::get2()
        .map(|| {
            "UNIMPLEMENTED"
        });

    let events_today = warp::get2()
        .and(path!("today"))
        .map(|| {
            "UNIMPLEMENTED"
        });

    let events_month = warp::get2()
        .and(path!("month"))
        .and(state.db.clone())
        .map(|conn: PooledConn| {
//            Event::events_month(user_uuid, &conn)
//                .unwrap()
//                .apply(crate::util::json);
            "aoeuaoeu"
        });

    let create_event = warp::post2()
        .and(json_body_filter(50))
        .and(state.db.clone())
        .map(|e: NewEvent, conn: PooledConn| {
            Event::create_event(e, &conn)
                .unwrap()
                .apply(crate::util::json)
        });

    // TODO, do we want canceling events as well?
    let delete_event = warp::delete2()
        .and(json_body_filter(5))
        .map(|_:String| {
            "UNIMPLEMENTED"
        });

    let modify_event = warp::post2()
        .and(json_body_filter(50))
        .map(|_:String| {
            "UNIMPLEMENTED"
        });

    let events = path!("events")
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