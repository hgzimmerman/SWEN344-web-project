//! Binary for Server.
use server::start;
use server::Config;
use log::LevelFilter;
use env_logger::Builder as LoggerBuilder;

/// Simple shell around starting the server.
fn main() {
    LoggerBuilder::new().filter_level(LevelFilter::Info).init();
    let config = Config::parse_command_line_arguments();
    start(config)
}
