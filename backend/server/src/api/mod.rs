
mod calendar;
mod market;
mod auth;

use warp::filters::BoxedFilter;
use warp::Reply;

use warp::path;
use warp::Filter;
use crate::state::State;

use self::calendar::calendar_api;
use crate::api::market::market_api;
use crate::api::auth::auth_api;

pub fn api(state: &State) -> BoxedFilter<(impl Reply,)> {

    path!("api")
        .and(
            market_api(state)
                .or(calendar_api(state))
                .or(auth_api(state))
        )
        .boxed()

}


/// A function that:
/// * Routes the API
/// * Handles file requests and redirections - NOT IMPLEMENTED
/// * Initializes logging
/// * Handles errors
/// * Handles CORS
pub fn routes(state: &State) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
//        .allow_origin("http://localhost:8081")
        .allow_headers(vec!["Access-Control-Allow-Origin", "content-type", "Authorization"])
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT","DELETE"]);

    api(state)
        .with(warp::log("routes"))
        .with(cors)
        .recover(crate::error::customize_error)
        .boxed()
}



#[cfg(test)]
mod integration_test {
    use super::*;
}