use warp::Filter;
//use warp::path::path;
use warp::path;
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;


mod api;
mod state;
mod util;
mod auth;
mod error;

use crate::api::routes;
//use crate::api::api;
use crate::state::State;

fn main() {
    LoggerBuilder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let localhost = [127, 0, 0, 1];
    let port = 8080;
    let addr = (localhost, port);

    let state = State::new();
//    let api = api(&state);
    let routes = routes(&state);

    warp::serve(routes)
        .run(addr);
}