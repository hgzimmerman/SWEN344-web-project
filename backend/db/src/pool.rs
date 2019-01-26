

use diesel::{
    pg::PgConnection,
    r2d2::ConnectionManager,
    Connection,
};
use r2d2::{
    Pool as R2D2Pool,
    PooledConnection,
};

pub const DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the pool.
pub fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool")
}

pub fn create_single_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).expect("Database not available. maybe provided url is wrong, or database is down?")
}