//! Crate that defines the http routes and the business logic.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

mod adaptive;
mod api;
mod config;
mod error;
mod server_auth;
mod state;
mod static_files;
#[cfg(test)]
mod testing_fixtures;
mod util;

pub use config::Config;

use crate::{
    api::routes,
    state::{State, StateConfig},
};
use log::info;

/// parses the command line arguments and starts the server.
pub fn start(config: Config) {

    info!("{:#?}", config);
    let localhost = [0, 0, 0, 0];
    let addr = (localhost, config.port);

    let state_config = StateConfig {
        secret: config.secret,
        max_pool_size: config.max_pool_size,
    };

    let state = State::new(state_config);
    let routes = routes(&state);

    if config.tls_enabled {
        warp::serve(routes)
            .tls("tls/cert.pem", "tls/key.rsa") // TODO, actually get these keys.
            .run(addr);
    } else {
        warp::serve(routes).run(addr);
    }
}

