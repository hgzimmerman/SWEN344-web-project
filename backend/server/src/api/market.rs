use crate::state::State;
use warp::filters::BoxedFilter;
use warp::path;
use warp::Filter;
use warp::Reply;

use crate::auth::user_filter;
use crate::error::Error;
use crate::util;
use crate::util::json_body_filter;
use db::funds::Funds;
use db::pool::PooledConn;
use db::stock::NewStock;
use db::stock::Stock;
use diesel::result::QueryResult;
use uuid::Uuid;
use warp::Rejection;

use chrono::Utc;
use db::stock::NewStockTransaction;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockTransactionRequest {
    symbol: String,
    /// The sign bit indicates if it is a sale or a purchase;
    quantity: i32,
}

/// The Filter for the market API.
pub fn market_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    let transact = path!("transact")
        .and(warp::post2())
        .and(json_body_filter(10))
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(transact);

    let owned_stocks = warp::get2().and(user_filter(s)).and(s.db.clone()).and_then(
        |user_uuid: Uuid, conn: PooledConn| {
            Stock::get_stocks_belonging_to_user(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        },
    );

    let user_transactions_for_stock = warp::get2()
        .and(path!("transactions" / String)) // The string is a symbol
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|symbol: String, user_uuid: Uuid, conn: PooledConn| {
            let stock = Stock::get_stock_by_symbol(symbol, &conn).map_err(Error::from_reject)?;
            Stock::get_user_transactions_for_stock(user_uuid, stock.uuid, &conn)
                .map_err(Error::from_reject)
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
                .or_else(|error| {
                    use diesel::result::Error as DieselError;
                    // If the funds row for the user doesn't exist yet,
                    // create one for the user.
                    if let DieselError::NotFound = error {
                        Funds::create_funds_for_user(user_uuid, &conn)
                    } else {
                        Err(error)
                    }
                })
                .map_err(Error::from_reject)
                .map(|funds: Funds| funds.quantity) // maps it to just a f64
                .map(util::json)
        });

    let funds_api = path!("funds").and(balance.or(add_funds).or(withdraw_funds));

    let stock_api = path!("stock").and(owned_stocks.or(transact).or(user_transactions_for_stock));

    path!("market").and(stock_api.or(funds_api)).boxed()
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
fn withdraw_funds(
    quantity: f64,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    if quantity < 0.0 {
        // No negative numbers allowed.
        Error::BadRequest.reject_result()
    } else {
        let negative_quantity = 0.0 - quantity;
        // Check if withdrawing will give a negative balance.
        let new_quantity: f64 = Funds::funds(user_uuid, &conn)
            .map_err(Error::from)
            .and_then(|current_funds| {
                calculate_new_quantity_of_funds(current_funds.quantity, negative_quantity)
            })
            .map_err(Error::reject)?;

        // Withdraw the funds from the user's balance.
        Funds::set_funds(user_uuid, new_quantity, &conn)
            .map(|funds: Funds| funds.quantity) // maps it to just a f64
            .map(util::json)
            .map_err(Error::from_reject)
    }
}

/// Calculates the quantity of funds after a transaction
/// Prevents the new quantity from being negative.
///
/// # Arguments
/// * current - The current amount of funds in the account.
/// * transaction_amount - The (possibly negative) amount of funds to be transferred.
fn calculate_new_quantity_of_funds(current: f64, transaction_amount: f64) -> Result<f64, Error> {
    let new_quantity = current + transaction_amount;
    if new_quantity < 0.0 {
        Err(Error::BadRequest) // TODO - More specific error
    } else {
        Ok(new_quantity)
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
        let new_quantity: f64 = Funds::funds(user_uuid, &conn)
            .map_err(Error::from)
            .map(|current_funds| {
                current_funds.quantity + quantity
            })
            .map_err(Error::reject)?;

        Funds::set_funds(user_uuid, new_quantity, &conn)
            .map_err(Error::from_reject)
            .map(|funds: Funds| funds.quantity) // maps it to just a f64
            .map(util::json)
    }
}

/// Get the stock or create it if needed.
/// Get the current transactions for the user for this stock.
/// Check if the transactions would cause them to own negative number of stocks.
/// Check if the user has the funds to make the transaction.
/// Subtract the funds from the user.
/// Record the transaction.
///
/// # Arguments
/// * request - The request struct representing a transaction.
/// * user_uuid - The unique id of the user whose funds are being modified
/// * conn - the connection to the database.
fn transact(
    request: StockTransactionRequest,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
    let stock: QueryResult<Stock> = Stock::get_stock_by_symbol(request.symbol.clone(), &conn);

    use diesel::result::Error as DieselError;

    let stock = stock.or_else(|e| match e {
        DieselError::NotFound => {
            let new_stock = NewStock {
                symbol: request.symbol.clone(),
                stock_name: "VOID - This field is slated for removal".to_string(),
            };
            Stock::create_stock(new_stock, &conn).map_err(Error::from_reject)
        }
        e => Error::from(e).reject_result(),
    })?;

    let transactions = Stock::get_user_transactions_for_stock(user_uuid, stock.uuid, &conn)
        .map_err(Error::from_reject)?;
    let quantity = transactions.into_iter().fold(0, |acc, t| acc + t.quantity);

    // Users can't sell more than they have.
    if -request.quantity > quantity {
        Error::BadRequest.reject_result()?; // TODO find a better rejection message
    }

    // TODO, this should be gotten from the stock api.
    let current_price = 420.0;
    let transaction_value = current_price * request.quantity as f64;

//    // if it is a purchase, check if the user has enough funds.
//    if request.quantity > 0 {
//        let funds = Funds::funds(user_uuid, &conn).map_err(Error::from_reject)?;
//        if transaction_value > funds.quantity {
//            Error::BadRequest.reject_result()?;
//        }
//    }
    let funds = Funds::funds(user_uuid, &conn)
        .map_err(Error::from_reject)?;
    let new_quantity = calculate_new_quantity_of_funds(funds.quantity, transaction_value)
        .map_err(Error::reject)?;

    // Add or remove funds for the user
    Funds::set_funds(user_uuid, new_quantity, &conn).map_err(Error::from_reject)?;

    let new_stock_transaction = NewStockTransaction {
        user_uuid,
        stock_uuid: stock.uuid,
        quantity: request.quantity,
        price_of_stock_at_time_of_trading: current_price,
        record_time: Utc::now().naive_utc(),
    };

    // Record that the stock was purchased for the user
    Stock::create_transaction(new_stock_transaction, &conn)
        .map_err(Error::from_reject)
        .map(util::json)
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn reject_negative_balance() {
        let q = calculate_new_quantity_of_funds(100.0, -200.0);
        assert_eq!(q, Err(Error::BadRequest))
    }

   #[test]
    fn approve_positive_balance() {
       let q = calculate_new_quantity_of_funds(100.0, -50.0);
       assert_eq!(q, Ok(50.0))
   }

}