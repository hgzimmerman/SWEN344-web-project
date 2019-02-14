//! Represents the shared server resources that all requests may utilize.
use crate::{error::Error, server_auth::secret_filter};

use authorization::Secret;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;
use pool::{init_pool, Pool, PoolConfig, PooledConn, DATABASE_URL};
use warp::{filters::BoxedFilter, Filter, Rejection};

pub type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// State that is passed around to all of the api handlers.
/// It can be used to acquire connections to the database,
/// or to reference the key that signs the access tokens.
///
/// These entities are acquired by running a filter function that brings them
/// into the scope of the relevant api.
pub struct State {
    /// A pool of database connections.
    pub db: BoxedFilter<(PooledConn,)>,
    /// The secret key.
    pub secret: BoxedFilter<(Secret,)>,
    /// Https client
    pub https: BoxedFilter<(HttpsClient,)>,
}

/// Configuration object for creating the state.
///
/// If unspecified, it will default to a sane default.
#[derive(Debug, Default)]
pub struct StateConfig {
    pub secret: Option<Secret>,
    pub max_pool_size: Option<u32>,
}

impl State {
    /// Creates a new state.
    pub fn new(conf: StateConfig) -> Self {
        let secret = conf
            .secret
            .unwrap_or_else(|| Secret::new("yeetyeetyeetyeetyeet")); // TODO Make this random

        let pool_conf = PoolConfig {
            max_connections: conf.max_pool_size,
            ..Default::default()
        };

        let pool = init_pool(DATABASE_URL, pool_conf);
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
            https: http_filter(client),
        }
    }

    /// Creates a new state object from an existing object pool.
    /// This is useful if using fixtures.
    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> Self {
        use std::time::Duration;
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder()
            .keep_alive_timeout(Some(Duration::new(12, 0)))
            .build::<_, hyper::Body>(https);

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
            https: http_filter(client),
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
            .get()
            .map_err(|_| Error::DatabaseUnavailable.reject())
    }

    warp::any()
        .and_then(move || -> Result<PooledConn, Rejection> { get_conn_from_pool(&pool) })
        .boxed()
}
