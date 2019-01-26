use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::path;
use warp::Filter;

use crate::util::json_body_filter;

pub fn market_api(s: &State) -> BoxedFilter<(impl Reply,)> {

    let buy = path!("buy")
        .and(warp::post2())
        .and(json_body_filter(10))
        .map(|s: String| {
            "UNIMPLEMENTED"
        });

    let sell = path!("sell")
        .and(warp::post2())
        .and(json_body_filter(10))
        .map(|s: String| {
            "UNIMPLEMENTED"
        });

    let owned_stocks = warp::get2()
        .map(|| {
            "UNIMPLEMENTED"
        });

    let add_funds = path!("add")
        .and(warp::post2())
        .and(json_body_filter(10))
        .map(|s: String| {
            "UNIMPLEMENTED"
        });

    let withdraw_funds = path!("withdraw")
        .and(warp::post2())
        .and(json_body_filter(10))
        .map(|s: String| {
            "UNIMPLEMENTED"
        });

    // balance
    let balance = path!("balance")
        .and(warp::get2())
        .map(|| {
            "UNIMPLEMENTED"
        });

    let net_profit = path!("profit")
        .and(warp::get2())
        .map(|| {
            "UNIMPLEMENTED"
        });

    // TODO consider moving this to a non-nested api node
    let funds_api = path!("funds")
        .and(
            balance
                .or(add_funds)
                .or(withdraw_funds)
        );

    let stock_api = path!("stock")
        .and(
             owned_stocks
                .or(buy)
                .or(sell)
        );


    path!("market")
        .and(
            stock_api
                .or(funds_api)
        )
        .boxed()

}