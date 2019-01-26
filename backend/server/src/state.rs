
use warp::Filter;
use db::pool::{
    Pool,
    DATABASE_URL,
    init_pool
};
use warp::filters::BoxedFilter;
use db::pool::PooledConn;


pub struct State {
    pub db: BoxedFilter<(PooledConn,)>,
}

impl State {
    pub fn new() -> Self {
        let pool = init_pool(DATABASE_URL);

        State {
            db: db_filter(pool),
        }
    }
}
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    warp::any()
        .map(move || pool.clone().get().unwrap())
//        .and_then(|pool_2: Pool| pool_2.get().unwrap())
        .boxed()
}
