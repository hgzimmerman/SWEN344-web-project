//! [Self adaptive instructions section](http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html)
use crate::{
    adaptive::{get_load, get_num_servers_up, should_serve_adds, Load, NumServers},
    error::Error,
    state::State,
    util,
};
use chrono::Utc;
use db::health::{HealthRecord, NewHealthRecord};
use futures::future::Future;
use pool::PooledConn;
use warp::{
    filters::{fs::File, BoxedFilter},
    path, Filter, Rejection, Reply,
};

/// Api for serving the advertisement.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn add_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    path("advertisement")
        .and(warp::get2())
        .and_then(|| {
            // Get the stats asynchronously as a precondition to serving the request.
            let servers = get_num_servers_up().map_err(Error::reject);
            let load = get_load().map_err(Error::reject);
            servers.join(load)
        })
        .untuple_one()
        .and(warp::fs::file(".static/add/rit_add.png")) // TODO, verify that this is correct
        .and(state.db.clone())
        .and_then(
            |servers: NumServers,
             load: Load,
             file: File,
             conn: PooledConn|
             -> Result<File, Rejection> {
                serve_add(servers, load, &conn)
                    .map(|_| file)
                    .map_err(|e| e.reject())
            },
        )
        .boxed()
}

/// Api for accessing health information related to serving the advertisement.
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

    path("health").and(all_health.or(last_week_health)).boxed()
}

fn serve_add(available_servers: NumServers, load: Load, conn: &PooledConn) -> Result<(), Error> {
    let should_send_advertisement = should_serve_adds(load, available_servers);

    let hr = NewHealthRecord {
        available_servers: available_servers.0 as i32,
        load: load.0 as i32,
        did_serve: should_send_advertisement,
        time_recorded: Utc::now().naive_utc(),
    };

    HealthRecord::create(hr, conn).map_err(Error::from)?;

    if should_send_advertisement {
        Ok(())
    } else {
        Err(Error::internal_server_error("The server load was determined to be too high, and therefore the \"advertisement\" was not sent "))
    }
}
