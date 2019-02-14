//! Responsible for hosting routes that deal with stock market data.
use crate::state::State;
use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{
    error::Error,
    server_auth::user_filter,
    util::{self, json_body_filter},
};
use db::stock::{NewStock, Stock};
use diesel::result::QueryResult;
use pool::PooledConn;
use uuid::Uuid;
use warp::Rejection;

use crate::state::HttpsClient;
use chrono::Utc;
use db::stock::{NewStockTransaction, UserStockResponse};
use futures::{
    future::{self, Future},
    stream::Stream,
};
use hyper::{Chunk, Uri};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        .and(s.https.clone())
        .and(json_body_filter(10))
        .and_then(|client: HttpsClient, request: StockTransactionRequest| {
            // Get the current price from a remote source
            get_current_price(&request.symbol, &client)
                .join(future::ok::<_, Error>(request)) // Join in the request, so it isn't lost.
                .map_err(Error::reject) // Handle errors.
        })
        .untuple_one()
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(transact); // Store the purchase/sale in the db

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

    // Get the User's stock transactions
    // Get the current prices for those transactions
    // Zip them together, and calculate the net profit/loss for each.
    let portfolio_performance = warp::get2()
        .and(path!("performance"))
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(
            |user_uuid: Uuid, conn: PooledConn| -> Result<Vec<UserStockResponse>, Rejection> {
                Stock::get_stocks_belonging_to_user(user_uuid, &conn).map_err(Error::from_reject)
            },
        )
        .and(s.https.clone())
        .and_then(|stocks: Vec<UserStockResponse>, client: HttpsClient| {
            let symbols: Vec<&str> = stocks.iter().map(|s| s.stock.symbol.as_str()).collect();
            get_current_prices(&symbols, &client)
                .map_err(Error::reject)
                .join(future::ok(stocks))
        })
        .untuple_one()
        .map(
            |prices: Vec<f64>, stocks: Vec<UserStockResponse>| -> Vec<(UserStockResponse, f64)> {
                prices
                    .into_iter()
                    .zip(stocks)
                    .map(|(price, stock): (f64, UserStockResponse)| {
                        let net: f64 = stock.transactions.iter().fold(0.0, |acc, transaction| {
                            acc + ((price - transaction.price_of_stock_at_time_of_trading)
                                * f64::from(transaction.quantity))
                        });
                        (stock, net)
                    })
                    .collect::<Vec<(UserStockResponse, f64)>>() // TODO make an actual type for this.
            },
        )
        .map(util::json);

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
/// * current_price - The current price of the stock, retrieved from an async https call.
/// * request - The request struct representing a transaction.
/// * user_uuid - The unique id of the user whose funds are being modified
/// * conn - the connection to the database.
fn transact(
    current_price: f64,
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
        let err = format!(
            "Can't sell more stocks than you have. Owned: {}, Transaction: {}",
            quantity, request.quantity
        );
        Error::bad_request(err).reject_result()?;
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

fn get_current_price(
    stock_symbol: &str,
    client: &HttpsClient,
) -> impl Future<Item = f64, Error = Error> {
    let uri: Uri = format!(
        "https://api.iextrading.com/1.0/stock/{}/price",
        stock_symbol
    )
    .parse()
    .unwrap();
    let uri_copy_1 = uri.clone();
    let uri_copy_2 = uri.clone();
    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| Error::connection_failed(uri_copy_1))
        .and_then(move |chunk: Chunk| -> Result<f64, Error> {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            body.parse::<f64>().map_err(move |_| -> Error {
                crate::error::Error::internal_server_error(format!(
                    "Could not parse body of dependent connection: {}, body: {}",
                    uri_copy_2.to_string(),
                    body
                ))
            })
        })
}

/// Get the current prices for a set of stocks.
fn get_current_prices(
    stock_symbols: &[&str],
    client: &HttpsClient,
) -> impl Future<Item = Vec<f64>, Error = Error> {
    let uri: Uri = format!(
        "https://api.iextrading.com/1.0/stock/market/batch?symbols={}&types=price",
        stock_symbols.join(",")
    )
    .parse()
    .unwrap();
    info!("Getting current prices for: {}", uri);

    // handle json in the form: {"AAPL":{"price":170.67},"FB":{"price":165.465}}
    #[derive(Serialize, Deserialize, Debug)]
    struct Price {
        price: f64,
    }

    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| Error::DependentConnectionFailed {
            url: uri.to_string(),
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            serde_json::from_str::<HashMap<String, Price>>(&body)
                .map(|r| {
                    info!("Get current prices: {:#?}", r);
                    r.values().map(|v| v.price).collect()
                })
                .map_err(|_| {
                    crate::error::Error::internal_server_error(
                        "Could not get current prices".to_string(),
                    )
                })
        })
}

#[cfg(test)]
mod integration {
    use super::*;
    use futures::future;
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use tokio;

    #[test]
    fn can_get_current_price() {
        // This test assumes that apple's stock price is above 1 dollar per share.
        // A fair assumption, but it may not always be true :/.
        tokio::run(future::lazy(|| {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder().build::<_, hyper::Body>(https);
            get_current_price("aapl", &client)
                .map(|price| assert!(price > 0.0, "Aapl should have a positive share price."))
                .map_err(|_| panic!("Could not get current price"))
        }));
    }

    #[test]
    fn can_get_multiple_current_price() {
        // This test assumes that apple's stock price is above 1 dollar per share.
        // A fair assumption, but it may not always be true :/.
        tokio::run(future::lazy(|| {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder().build::<_, hyper::Body>(https);
            get_current_prices(&["aapl", "fb"], &client)
                .map(|prices| assert!(prices.len() == 2))
                .map_err(|_| panic!("Could not get current prices"))
        }));
    }
}
