use diesel::{pg::PgConnection, r2d2::ConnectionManager, Connection};
use r2d2::{Pool as R2D2Pool, PooledConnection};
use std::time::Duration;
use apply::Apply;

pub const DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;


/// Corfiguration object for the pool.
#[derive(Default, Debug)]
pub struct PoolConfig {
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    /// Max lifetime in minutes.
    pub max_lifetime: Option<Duration>,
    pub timeout: Duration
}

/// Initializes the pool.
pub fn init_pool(db_url: &str, conf: PoolConfig) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let builder = r2d2::Pool::builder()
        .apply(|builder| {
            if let Some(max_size) = conf.max_connections {
               builder.max_size(max_size)
            } else {
                builder
            }
        })
        .apply(|builder| {
            builder.min_idle(conf.min_connections)
        })
        .apply(|builder| {
            if let Some(max_lifetime) = conf.max_lifetime {
                builder.max_lifetime(Some(max_lifetime))
            } else {
                builder
            }
        });

    builder.build(manager).expect("Could not initialize DB Pool")
}

pub fn create_single_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url)
        .expect("Database not available. maybe provided url is wrong, or database is down?")
}
