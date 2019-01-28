use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::path;
use warp::Filter;

use crate::util::json_body_filter;
use crate::auth::user_filter;
use db::pool::PooledConn;
use uuid::Uuid;
use db::funds::Funds;
use crate::error::Error;
use crate::util;
use warp::Rejection;

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
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|quantity: f64, user_uuid: Uuid, conn: PooledConn| {
            if quantity < 0.0 {
                // No negative numbers allowed
                Error::BadRequest.reject_result()
            } else {
                Funds::transact_funds(user_uuid, quantity, &conn)
                    .map_err(Error::from)
                    .map_err(Error::reject)
                    .map(|funds: Funds| funds.quantity) // maps it to just a f64
                    .map(util::json)
            }
        });

    let withdraw_funds = path!("withdraw")
        .and(warp::post2())
        .and(json_body_filter(10))
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(withdraw_funds);


    let balance = path!("balance")
        .and(warp::get2())
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Funds::funds(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(|funds: Funds| funds.quantity) // maps it to just a f64
                .map(util::json)
        });

    let net_profit = path!("profit")
        .and(warp::get2())
        .map(|| {
            "UNIMPLEMENTED"
        });

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

/// Withdraws funds from a user's Funds.
///
/// It will first check that the provided quantity is non-negative
/// It will then check that if by withdrawing funds, the user will not have a negative balance.
///
/// If those checks are OK, then the function will deduct the quantity from the users account.
fn withdraw_funds(quantity: f64, user_uuid: Uuid, conn: PooledConn) -> Result<impl Reply, Rejection> {
    if quantity < 0.0 {
        // No negative numbers allowed.
        Error::BadRequest.reject_result()
    } else {
        let negative_quantity = 0.0 - quantity;
        // Check if withdrawing will give a negative balance.
        let _ = Funds::funds(user_uuid, &conn)
            .map_err(Error::from)
            .and_then(|current_funds| {
                if current_funds.quantity + negative_quantity < 0.0 {
                    Err(Error::BadRequest)
                } else {
                    Ok(())
                }
            })
            .map_err(Error::reject)?;

        // Withdraw the funds from the user's balance.
        Funds::transact_funds(user_uuid, negative_quantity, &conn)
            .map(|funds: Funds| funds.quantity) // maps it to just a f64
            .map(util::json)
            .map_err(Error::from)
            .map_err(Error::reject)
    }
}