use clap::App;
use clap::Arg;
use crate::auth::Secret;
use apply::Apply;

pub struct Config {
    /// The port to start the server on.
    pub port: u16,
    /// If set to true, TLS will be enabled
    pub tls_enabled: bool,
    /// Command line defined secret. If none is provided, then the secret will be randomly generated.
    pub secret: Option<Secret>,
    /// The maximum size of the connection pool.
    /// If left unspecified, it will be left to the pool's discretion (At the time of writing, it defaults to 10)
    pub max_pool_size: Option<u32>
}

impl Config {
    pub fn parse_command_line_arguments() -> Self {
        let matches = App::new("RIT SWEN 344 Server")
            .version("0.1.0")
            .author("Group 3")
            .about("Serves things")
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .help("The port to run the server on.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("tls")
                    .long("tls")
                    .help("Run with TLS enabled. By default, TLS is not enabled.")
            )
            .arg(
                Arg::with_name("secret")
                    .long("secret")
                    .value_name("SECRET STRING")
                    .help("Initializes the secret to this value. It should be a long random string. If a secret is not provided, one will be randomly generated.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("max_pool_size")
                    .long("max-pool-size")
                    .value_name("POOL SIZE")
                    .help("Number of connections the database pool supports. Defaults to 10.")
                    .takes_value(true)
            )
            .get_matches();

        let port: u16 = if let Some(port) = matches.value_of("port") {
            port.parse().expect("Port must be an integer")
        } else {
            8080
        };

        let tls_enabled = matches.is_present("tls");

        let secret = matches.value_of("secret").map(Secret::new);

        let max_pool_size: u32 = if let Some(size) = matches.value_of("max_pool_size") {
            size.parse().expect("Pool size must be an integer.")
        } else {
            10
        };
        let max_pool_size = max_pool_size.apply(Some);

        Config {
            port,
            tls_enabled,
            secret,
            max_pool_size
        }
    }
}