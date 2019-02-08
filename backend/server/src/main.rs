use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;

mod adaptive;
mod api;
mod server_auth;
mod config;
mod error;
mod state;
mod static_files;
#[cfg(test)]
mod testing_fixtures;
mod util;

use crate::{
    api::routes,
    config::Config,
    state::{State, StateConfig},
};

fn main() {
    LoggerBuilder::new().filter_level(LevelFilter::Info).init();

    let config = Config::parse_command_line_arguments();

    let localhost = [127, 0, 0, 1];
    let addr = (localhost, config.port);

    let state_config = StateConfig {
        secret: config.secret,
        max_pool_size: config.max_pool_size,
    };

    let state = State::new(state_config);

    let routes = routes(&state);

    warp::serve(routes).run(addr);
}
