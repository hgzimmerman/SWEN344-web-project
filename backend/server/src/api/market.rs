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
use db::stock::Stock;
use diesel::result::QueryResult;
use db::stock::NewStock;

use serde::Serialize;
use serde::Deserialize;
use db::stock::NewStockTransaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockTransactionRequest {
    symbol: String,
    /// The sign bit indicates if it is a sale or a purchase;
    quantity: i32
}

/// The Filter for the market API.
pub fn market_api(s: &State) -> BoxedFilter<(impl Reply,)> {


    let transact = path!("transact")
        .and(warp::post2())
        .and(json_body_filter(10))
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(transact);

    let owned_stocks = warp::get2()
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Stock::get_stocks_belonging_to_user(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    let add_funds = path!("add")
        .and(warp::post2())
        .and(json_body_filter(10))
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(add_funds);

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
                .or(transact)
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
///
/// /// # Arguments
/// * quantity - non-negative number that represents the quantity of funds being 'withdrawn'.
/// * user_uuid - The unique id of the user whose funds are being modified
/// * conn - the connection to the database.
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

/// Adds funds to the user's account.
///
/// # Arguments
/// * quantity - non-negative number that represents the quantity of funds being added.
/// * user_uuid - The unique id of the user whose funds are being modified
/// * conn - the connection to the database.
fn add_funds(quantity: f64, user_uuid: Uuid, conn: PooledConn) -> Result<impl Reply, Rejection> {
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
}

fn transact(request: StockTransactionRequest, user_uuid: Uuid, conn: PooledConn) -> Result<impl Reply, Rejection> {
    let stock: QueryResult<Stock> = Stock::get_stock_by_symbol(request.symbol.clone(), &conn);

    use diesel::result::Error as DieselError;

    let stock = stock
        .or_else( | e | {
            match e {
                DieselError::NotFound => {
                    let new_stock = NewStock {
                        symbol: request.symbol.clone(),
                        stock_name: "VOID - This field is slated for removal".to_string()
                    };
                    Stock::create_stock(new_stock, &conn)
                        .map_err(Error::from)
                        .map_err(Error::reject)
                }
                e => Error::from(e).reject_result()
            }
        })?;

    let transactions = Stock::get_user_transactions_for_stock(user_uuid, stock.uuid, &conn)
        .map_err(Error::from)
        .map_err(Error::reject)?;
    let quantity = transactions.into_iter().fold(0, |acc, t| acc + t.quantity);

    // Users can't sell more than they have.
    if request.quantity > quantity {
        Error::BadRequest.reject_result()?; // TODO find a better rejection message
    }

//    let new_stock_transaction = NewStockTransaction {
//        user_uuid,
//        stock_uuid: stock.uuid,
//        price_uuid: (), // TODO remove this field
//        quantity: request.quantity
//    };
//
//    Stock::create_transaction(new_stock_transaction, &conn)
//        .map_err(Error::from)
//        .map_err(Error::reject)
//        .map(util::json)

    Ok(warp::reply())

}