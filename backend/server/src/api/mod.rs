
mod calendar;
mod market;

use warp::filters::BoxedFilter;
use warp::Reply;

use warp::path;
use warp::Filter;
use crate::state::State;

use self::calendar::calendar_api;
use crate::api::market::market_api;

pub fn api(state: &State) -> BoxedFilter<(impl Reply,)> {


    path!("api")
        .and(
            market_api(state).or(calendar_api(state))
        )
        .boxed()

}