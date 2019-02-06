use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::Filter;
use warp::path;
use crate::adaptive::{
    get_num_servers_up,
    get_load,
    should_serve_adds
};
use pool::PooledConn;
use crate::error::Error;
use db::health::NewHealthRecord;
use chrono::Utc;
use db::health::HealthRecord;
use futures::future::Future;
use warp::filters::fs::File;
use crate::util;


pub fn add_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    path("advertisement")
        .and(warp::get2())
        .and(warp::fs::file("/static/add/rit_add.png")) // TODO create a real file for this.
        .and(state.db.clone())
        .and_then(|file: File, conn: PooledConn| {
            serve_add(&conn)
                .map(|_| file)
                .map_err(|e| e.reject())
        })
        .boxed()
}

pub fn health_api(state: &State) -> BoxedFilter<(impl Reply,)> {

    let all_health = warp::get2()
        .and(state.db.clone())
        .and_then(|conn: PooledConn| {
            HealthRecord::get_all(&conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    let last_week_health = warp::get2()
        .and(path("week"))
        .and(state.db.clone())
        .and_then(|conn: PooledConn| {
            HealthRecord::get_last_7_days(&conn)
                .map_err(Error::from_reject)
                .map(util::json)
        });

    let health_api = path("health")
        .and(
            all_health
                .or(last_week_health)
        );


    health_api
        .boxed()
}

fn serve_add(conn: &PooledConn) -> Result<(), Error> {
    let available_servers = get_num_servers_up()
        .wait()
        .map_err(|_| Error::InternalServerError)?; // TODO better error messages.

    let load = get_load()
        .wait()
        .map_err(|_| Error::InternalServerError)?; // TODO better error messages.

    let should_send_advertisement = should_serve_adds(load, available_servers);

    let hr = NewHealthRecord {
        available_servers: available_servers as i32,
        load: load as i32,
        did_serve: should_send_advertisement,
        time_recorded: Utc::now().naive_utc()
    };

    HealthRecord::create(hr, conn).map_err(Error::from)?;

    if should_send_advertisement {
        Ok(())
    } else {
        Err(Error::InternalServerError) // TODO better error messages.
    }


}