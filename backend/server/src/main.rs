use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;

mod api;
mod auth;
mod error;
mod state;
#[cfg(test)]
mod testing_fixtures;
mod util;
mod static_files;
mod adaptive;
mod config;

use crate::api::routes;
use crate::state::State;
use crate::config::Config;
use crate::state::StateConfig;

fn main() {
    LoggerBuilder::new().filter_level(LevelFilter::Info).init();

    let config = Config::parse_command_line_arguments();

    let localhost = [127, 0, 0, 1];
    let addr = (localhost, config.port);

    let state_config = StateConfig {
        secret: config.secret,
        max_pool_size: config.max_pool_size
    };

    let state = State::new(state_config);

    let routes = routes(&state);

    warp::serve(routes).run(addr);
}



