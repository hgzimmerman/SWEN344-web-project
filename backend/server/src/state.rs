
use warp::Filter;
use db::pool::{
    Pool,
    DATABASE_URL,
    init_pool
};
use warp::filters::BoxedFilter;
use db::pool::PooledConn;
use crate::auth::secret_filter;
use crate::auth::Secret;


pub struct State {
    pub db: BoxedFilter<(PooledConn,)>,
    pub secret: BoxedFilter<(Secret,)>
}

impl State {
    pub fn new() -> Self {
        let pool = init_pool(DATABASE_URL);


        State {
            db: db_filter(pool),
            secret: secret_filter(Secret::new("yeetyeetyeetyeetyeet"))
        }
    }
}
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    warp::any()
        .map(move || pool.clone().get().unwrap())
//        .and_then(|pool_2: Pool| pool_2.get().unwrap())
        .boxed()
}
