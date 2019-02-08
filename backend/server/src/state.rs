//! Represents the shared server resources that all requests may utilize.
use crate::{
    error::Error,
    server_auth::secret_filter
};

use auth::{Secret};
use pool::{init_pool, Pool, PoolConfig, PooledConn, DATABASE_URL};
use warp::{filters::BoxedFilter, Filter, Rejection};

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
            .unwrap_or_else(|| Secret::new("yeetyeetyeetyeetyeet"));

        let pool_conf = PoolConfig {
            max_connections: conf.max_pool_size,
            ..Default::default()
        };
        let pool = init_pool(DATABASE_URL, pool_conf);

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
        }
    }

    /// Creates a new state object from an existing object pool.
    /// This is useful if using fixtures.
    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> Self {
        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
        }
    }
}

/// Filter that exposes connections to the database to individual filter requests
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    fn get_conn_from_pool(pool: &Pool) -> Result<PooledConn, Rejection> {
        pool.clone()
            .get()
            .map_err(|_| Error::DatabaseUnavailable.reject())
    }

    warp::any()
        .and_then(move || get_conn_from_pool(&pool))
        .boxed()
}
