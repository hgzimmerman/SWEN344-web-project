//! Represents the shared server resources that all requests may utilize.
use crate::{error::Error, server_auth::secret_filter};

use apply::Apply;
use authorization::Secret;
use egg_mode::KeyPair;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;
use pool::{init_pool, Pool, PoolConfig, PooledConn, DATABASE_URL};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::path::PathBuf;
use warp::{filters::BoxedFilter, Filter, Rejection};

/// Simplified type for representing a HttpClient.
pub type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// State that is passed around to all of the api handlers.
/// It can be used to acquire connections to the database,
/// or to reference the key that signs the access tokens.
///
/// These entities are acquired by running a filter function that brings them
/// into the scope of the relevant api.
pub struct State {
    /// A pool of database connections.
    db: BoxedFilter<(PooledConn,)>,
    /// The secret key.
    secret: BoxedFilter<(Secret,)>,
    /// Https client
    https: BoxedFilter<(HttpsClient,)>,
    /// Twitter connection token
    pub twitter_con_token: BoxedFilter<(KeyPair,)>,
    /// Twitter key pair for the auth token
    pub twitter_request_token: BoxedFilter<(KeyPair,)>,
    /// The path to the server directory.
    /// This allows file resources to have a common reference point when determining from where to serve assets.
    pub server_lib_root: PathBuf,
}

/// Configuration object for creating the state.
///
/// If unspecified, it will default to a sane default.
#[derive(Debug, Default)]
pub struct StateConfig {
    pub secret: Option<Secret>,
    pub max_pool_size: Option<u32>,
    pub server_lib_root: Option<PathBuf>,
}

impl State {
    /// Creates a new state.
    pub fn new(conf: StateConfig) -> Self {
        const RANDOM_KEY_LENGTH: usize = 200;
        let secret = conf.secret.unwrap_or_else(|| {
            // Generate a new random key if none is provided.
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(RANDOM_KEY_LENGTH)
                .collect::<String>()
                .apply(|s| Secret::new(&s))
        });

        let pool_conf = PoolConfig {
            max_connections: conf.max_pool_size,
            ..Default::default()
        };

        let pool = init_pool(DATABASE_URL, pool_conf);
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);

        let twitter_con_token = get_twitter_con_token();
        let twitter_request_token = get_twitter_request_token(&twitter_con_token);

        let root = conf.server_lib_root.unwrap_or_else(|| PathBuf::from("./"));

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
            https: http_filter(client),
            twitter_con_token: twitter_key_pair_filter(twitter_con_token),
            twitter_request_token: twitter_key_pair_filter(twitter_request_token),
            server_lib_root: root,
        }
    }

    /// Gets a pooled connection to the database.
    pub fn db(&self) -> BoxedFilter<(PooledConn,)> {
        self.db.clone()
    }

    /// Gets the secret used for authoring JWTs
    pub fn secret(&self) -> BoxedFilter<(Secret,)> {
        self.secret.clone()
    }

    pub fn https_client(&self) -> BoxedFilter<(HttpsClient,)> {
        self.https.clone()
    }

    /// Creates a new state object from an existing object pool.
    /// This is useful if using fixtures.
    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> Self {
        use std::time::Duration;
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder()
            .keep_alive_timeout(Some(Duration::new(12, 0)))
            .build::<_, Body>(https);

        let twitter_con_token = get_twitter_con_token();
        let twitter_request_token = get_twitter_request_token(&twitter_con_token);

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
            https: http_filter(client),
            twitter_con_token: twitter_key_pair_filter(twitter_con_token),
            twitter_request_token: twitter_key_pair_filter(twitter_request_token),
            server_lib_root: PathBuf::from("./"), // THIS makes the assumption that the tests are run from the backend/server dir.
        }
    }
}

/// Function that creates the HttpClient filter.
pub fn http_filter(client: HttpsClient) -> BoxedFilter<(HttpsClient,)> {
    warp::any().map(move || client.clone()).boxed()
}

/// Filter that exposes connections to the database to individual filter requests
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    fn get_conn_from_pool(pool: &Pool) -> Result<PooledConn, Rejection> {
        pool.clone()
            .get() // Will get the connection from the pool, or wait a specified time until one becomes available.
            .map_err(|_| Error::DatabaseUnavailable.reject())
    }

    warp::any()
        .and_then(move || -> Result<PooledConn, Rejection> { get_conn_from_pool(&pool) })
        .boxed()
}

/// Gets the connection key pair for the serer.
/// This represents the authenticity of the application
fn get_twitter_con_token() -> KeyPair {
    // TODO move getting these into a config object, or get them directly from the filesystem.
    // These definitely shouldn't be in source code, but I don't care,
    // I just want this to work right now. Also, this is a school project.
    const KEY: &str = "Pq2sA4Lfbovd4SLQhSQ6UPEVg";
    const SECRET: &str = "uK6U7Xqj2QThlm6H3y8dKSH3itZgpo9AVhR5or80X9umZc62ln";

    egg_mode::KeyPair::new(KEY, SECRET)
}

/// Gets the request token.
fn get_twitter_request_token(con_token: &KeyPair) -> KeyPair {
    const CALLBACK_URL: &str = "https://vm344c.se.rit.edu/api/auth/callback";
    tokio::runtime::current_thread::block_on_all(egg_mode::request_token(con_token, CALLBACK_URL))
        .expect("Couldn't authenticate to twitter")
}

pub fn twitter_key_pair_filter(twitter_key_pair: KeyPair) -> BoxedFilter<(KeyPair,)> {
    warp::any().map(move || twitter_key_pair.clone()).boxed()
}
