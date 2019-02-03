use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;

mod api;
mod auth;
mod error;
mod state;
#[cfg(test)]
mod testing_fixtures;
mod util;

use crate::api::routes;
use crate::state::State;

fn main() {
    LoggerBuilder::new().filter_level(LevelFilter::Info).init();

    let localhost = [127, 0, 0, 1];
    let port = 8080;
    let addr = (localhost, port);

    let state = State::new(None);

    let routes = routes(&state);

    warp::serve(routes).run(addr);
}
