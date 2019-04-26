//! Responsible for hosting routes that deal with stock market data.
use crate::state::State;
use warp::{path, Filter, Reply};

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

use crate::{state::HttpsClient, util::json_or_reject};
use apply::Apply;
use chrono::Utc;
use db::stock::{NewStockTransaction, StockTransaction, UserStockResponse};
use diesel::result::Error as DieselError;
use futures::{
    future::{self, Either, Future},
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockAndPerfResponse {
    /// The stock.
    pub stock: UserStockResponse,
    /// Net loss or gain.
    pub performance: f64,
}

/// The Filter for the market API.
///
/// # Arguments
/// s - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn market_api(s: &State) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    info!("Attaching Market Api");
    let transact = path!("transact")
        .and(warp::post2())
        .and(s.https_client())
        .and(json_body_filter(10))
        .and_then(|client: HttpsClient, request: StockTransactionRequest| {
            // Get the current price from a remote source
            get_current_price(&request.symbol, &client)
                .join(future::ok::<_, Error>(request)) // Join in the request, so it isn't lost.
                .map_err(Error::reject) // Handle errors.
        })
        .untuple_one()
        .and(user_filter(s))
        .and(s.db())
        .map(transact) // Store the purchase/sale in the db
        .and_then(json_or_reject);

    let owned_stocks = warp::get2()
        .and(warp::path::end())
        .and(user_filter(s))
        .and(s.db())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            Stock::get_stocks_belonging_to_user(user_uuid, &conn)
                .map_err(Error::from_reject)
                .map(util::json)
        },
    );

    let user_transactions_for_stock = warp::get2()
        .and(path!("transactions" / String)) // The string is a symbol
        .and(user_filter(s))
        .and(s.db())
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
        .and(s.db())
        .and_then(
            |user_uuid: Uuid, conn: PooledConn| -> Result<Vec<UserStockResponse>, Rejection> {
                Stock::get_stocks_belonging_to_user(user_uuid, &conn).map_err(Error::from_reject)
            },
        )
        .and(s.https_client())
        .and_then(|stocks: Vec<UserStockResponse>, client: HttpsClient| {
            let symbols: Vec<&str> = stocks.iter().map(|s| s.stock.symbol.as_str()).collect();
            get_current_prices(&symbols, &client)
                .map_err(Error::reject)
                .join(future::ok(stocks))
        })
        .untuple_one()
        .map(
            |prices: Vec<f64>, stocks: Vec<UserStockResponse>| -> Vec<StockAndPerfResponse> {
                prices
                    .into_iter()
                    .zip(stocks)
                    .map(|(price, stock): (f64, UserStockResponse)| {
                        let net: f64 = stock.transactions.iter().fold(0.0, |acc, transaction| {
                            acc + ((price - transaction.price_of_stock_at_time_of_trading)
                                * f64::from(transaction.quantity))
                        });
                        StockAndPerfResponse {
                            stock,
                            performance: net,
                        }
                    })
                    .collect::<Vec<StockAndPerfResponse>>()
            },
        )
        .map(util::json);

    let stock_api = path!("stock").and(
        owned_stocks
            .or(transact)
            .or(user_transactions_for_stock)
            .or(portfolio_performance),
    );

    path!("market").and(stock_api)
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
) -> Result<StockTransaction, Error> {
    info!("Transacting stock {:?}, at current price: {}, for user: {}", request, current_price, user_uuid);
    let stock: QueryResult<Stock> = Stock::get_stock_by_symbol(request.symbol.clone(), &conn);

    let stock = stock.or_else(|e| match e {
        DieselError::NotFound => {
            let new_stock = NewStock {
                symbol: request.symbol.clone(),
                stock_name: "VOID - This field is slated for removal".to_string(),
            };
            Stock::create_stock(new_stock, &conn).map_err(Error::from)
        }
        e => Error::from(e).apply(Err),
    })?;

    let transactions = Stock::get_user_transactions_for_stock(user_uuid, stock.uuid, &conn)
        .map_err(Error::from)?;
    let quantity = transactions.into_iter().fold(0, |acc, t| acc + t.quantity);

    // Users can't sell more than they have.
    if -request.quantity > quantity {
        let err = format!(
            "Can't sell more stocks than you have. Owned: {}, Transaction: {}",
            quantity, request.quantity
        );
        Error::bad_request(err).apply(Err)?;
    }

    let new_stock_transaction = NewStockTransaction {
        user_uuid,
        stock_uuid: stock.uuid,
        quantity: request.quantity,
        price_of_stock_at_time_of_trading: current_price,
        record_time: Utc::now().naive_utc(),
    };

    // Record that the stock was purchased for the user
    Stock::create_transaction(new_stock_transaction, &conn).map_err(Error::from)
}

fn get_current_price(
    stock_symbol: &str,
    client: &HttpsClient,
) -> impl Future<Item = f64, Error = Error> {
    info!("Getting current price for stock: {}", stock_symbol);
    let stock_symbol_copy = stock_symbol.to_string();

    let uri = format!(
        "https://api.iextrading.com/1.0/stock/{}/price",
        stock_symbol
    )
    .parse::<Uri>()
    .map_err(|e| Error::bad_request(format!("{:?}", e)));

    match uri {
        Ok(uri) => {
            let uri_string = uri.to_string(); // create this here, so it can be moved into the closure.
            client
                .get(uri.clone())
                .and_then(|res| {
                    res.into_body().concat2() // Await the whole body
                })
                .map_err(move |_| {
                    Error::dependent_connection_failed(
                        uri.to_string(),
                        format!("Could not get current price for {}.", stock_symbol_copy),
                    )
                })
                .and_then(move |chunk: Chunk| -> Result<f64, Error> {
                    let v = chunk.to_vec();
                    let body = String::from_utf8_lossy(&v).to_string();
                    body.parse::<f64>().map_err(move |_| -> Error {
                        crate::error::Error::internal_server_error(format!(
                            "Could not parse body of dependent connection: {}, body: {}",
                            uri_string, body
                        ))
                    })
                })
                .apply(Either::A)
        }
        Err(e) => e.apply(futures::future::err).apply(Either::B),
    }
}

/// Get the current prices for a set of stocks.
fn get_current_prices(
    stock_symbols: &[&str],
    client: &HttpsClient,
) -> impl Future<Item = Vec<f64>, Error = Error> {
    info!("Getting current prices for multiple stocks: {:?}", stock_symbols);
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
        .map_err(move |_| {
            Error::dependent_connection_failed(uri.to_string(), "Could not get current stocks.")
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            serde_json::from_str::<HashMap<String, Price>>(&body)
                .map(|r| {
                    info!("Got current prices: {:#?}", r);
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
                .map(|prices| assert_eq!(prices.len(), 2))
                .map_err(|_| panic!("Could not get current prices"))
        }));
    }
}
