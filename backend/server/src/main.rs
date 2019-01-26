use warp::Filter;
//use warp::path::path;
use warp::path;
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;


mod api;
mod state;
mod util;

use crate::api::api;
use crate::state::State;

fn main() {
    LoggerBuilder::new()
        .filter_level(LevelFilter::Info)
        .init();



    // GET /hello/warp => 200 OK with body "Hello, warp!"


    let localhost = [127, 0, 0, 1];
    let port = 8080;
    let addr = (localhost, port);

    let state = State{};
    let api = api(&state);

    warp::serve(api)
        .run(addr);
}