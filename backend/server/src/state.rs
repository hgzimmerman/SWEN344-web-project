use crate::auth::secret_filter;
use crate::auth::Secret;
use crate::error::Error;
use db::pool::PooledConn;
use db::pool::{init_pool, Pool, DATABASE_URL};
use warp::filters::BoxedFilter;
use warp::Filter;

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

impl State {
    // TODO parameterize this to allow setting arbitrary keys for the state.
    pub fn new(secret: Option<Secret>) -> Self {
        let pool = init_pool(DATABASE_URL);

        let secret = secret.unwrap_or_else(|| Secret::new("yeetyeetyeetyeetyeet"));

        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
        }
    }

    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> Self {
        State {
            db: db_filter(pool),
            secret: secret_filter(secret),
        }
    }
}
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    warp::any()
        .and_then(move || {
            pool.clone()
                .get()
                .map_err(|_| Error::DatabaseUnavailable.reject())
        })
        .boxed()
}
