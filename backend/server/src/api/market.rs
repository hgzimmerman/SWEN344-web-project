use crate::state::State;
use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{
    auth::user_filter,
    error::Error,
    util::{self, json_body_filter},
};
use db::stock::{NewStock, Stock};
use diesel::result::QueryResult;
use pool::PooledConn;
use uuid::Uuid;
use warp::Rejection;

use chrono::Utc;
use db::stock::{NewStockTransaction, UserStockResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockTransactionRequest {
    /// The stock symbol.
    pub symbol: String,
    /// The sign bit indicates if it is a sale or a purchase;
    pub quantity: i32,
}

/// The Filter for the market API.
///
/// # Arguments
/// s - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
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

    let portfolio_performance = warp::get2()
        .and(path!("performance")) // The string is a symbol
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            let stocks = Stock::get_stocks_belonging_to_user(user_uuid, &conn)
                .map_err(Error::from_reject)?;

            stocks
                .into_iter()
                .map(|s: UserStockResponse| {
                    // TODO, it would be much faster to use our stock api to get all of the prices up front and then zip them.
                    match get_current_price(&s.stock.symbol) {
                        Ok(price) => {
                            let net = s.transactions.into_iter().fold(0.0, |acc, transaction| {
                                acc + ((price - transaction.price_of_stock_at_time_of_trading)
                                    * f64::from(transaction.quantity))
                            });
                            Ok((s.stock, net))
                        }
                        Err(e) => Err(e),
                    }
                })
                .collect::<Result<Vec<_>, Error>>()
                .map_err(Error::reject)
                .map(util::json)
        });

    let stock_api = path!("stock").and(
        owned_stocks
            .or(transact)
            .or(user_transactions_for_stock)
            .or(portfolio_performance),
    );

    path!("market").and(stock_api).boxed()
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
    // TODO, this should be gotten from the stock api.
    let current_price = get_current_price(&request.symbol).map_err(Error::reject)?;

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

fn get_current_price(_stock_symbol: &str) -> Result<f64, Error> {
    Ok(420.0)
}
