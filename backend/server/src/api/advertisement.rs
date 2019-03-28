//! [Self adaptive instructions section](http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html)
use crate::{
    adaptive::{get_load, get_num_servers_up, should_serve_adds_bf, Load, NumServers},
    error::Error,
    state::State,
    util,
};
use chrono::Utc;
use db::health::{HealthRecord, NewHealthRecord};
use futures::future::Future;
use pool::PooledConn;
use warp::{
    filters::BoxedFilter,
    path, Filter, Reply,
};
use log::info;
use crate::error::err_to_rejection;
use crate::state::HttpsClient;

/// Api for serving the advertisement.
///
/// # Arguments
/// * state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn ad_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Ad Api");
    path("advertisement")
        .and(warp::get2())
        .and(state.https.clone())
        .and_then(|client: HttpsClient| {
            // Get the stats asynchronously as a precondition to serving the request.
            let servers = get_num_servers_up(&client).map_err(Error::reject);
            let load = get_load().map_err(Error::reject);
            servers.join(load)
        })
        .untuple_one() // converts `(NumServers, Load)` to `NumServers, Load`
        .and(state.db.clone())
        .map(determine_and_record_ad_serving)
        .and_then(err_to_rejection)
        .untuple_one() // converts `()` to ``
        .and(warp::fs::file(".static/ad/rit_ad.png")) // TODO, verify that this is correct
        .boxed()
}

/// Api for accessing health information related to serving the advertisement.
///
/// # Arguments
/// * state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn health_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Health Api");
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

/// Determines if the add should be served and records the result.
///
/// # Arguments
/// * available_servers - The number of servers that are available.
/// * load - The "load" currently on those servers.
/// * conn - The connection to the database.
///
/// # Note
/// It returns Ok(()) if the add should be served, and throws an 500 internal server error if it can't be sent.
fn determine_and_record_ad_serving(
    available_servers: NumServers,
    load: Load,
    conn: PooledConn,
) -> Result<(), Error> {
    let should_send_advertisement = should_serve_adds_bf(load, available_servers);

    let new_health_record = NewHealthRecord {
        available_servers: available_servers.0 as i32,
        load: load.0 as i32,
        did_serve: should_send_advertisement,
        time_recorded: Utc::now().naive_utc(),
    };

    HealthRecord::create(new_health_record, &conn).map_err(Error::from)?;

    if should_send_advertisement {
        Ok(())
    } else {
        Err(Error::internal_server_error("The server load was determined to be too high, and therefore the \"advertisement\" was not sent."))
    }
}
