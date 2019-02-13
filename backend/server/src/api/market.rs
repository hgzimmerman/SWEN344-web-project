//! Responsible for hosting routes that deal with stock market data.
use crate::state::State;
use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{
    server_auth::user_filter,
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
use hyper::Client;
use hyper::Uri;
use futures::future::Future;
use hyper::Chunk;
use futures::stream::Stream;
use futures::future;
use hyper_tls::HttpsConnector;
use crate::state::HttpsClient;
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
            get_current_price(&request.symbol, &client) // Get the current price
                .join(future::ok::<_, Error>(request)) // Join in the request, so it isn't lost.
                .map_err(Error::reject) // Handle errors.
        })
        .untuple_one()
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
        .and(s.https.clone())
        .and(user_filter(s))
        .and(s.db.clone())
        .and_then(|client: HttpsClient, user_uuid: Uuid, conn: PooledConn| {
            let stocks = Stock::get_stocks_belonging_to_user(user_uuid, &conn)
                .map_err(Error::from_reject)?;

            stocks
                .into_iter()
                .map(|s: UserStockResponse| {
                    // TODO, it would be much faster to use our stock api to get all of the prices up front and then zip them.
                    // TODO calling wait() tends to mess up tests. So remove them.
                    match get_current_price(&s.stock.symbol, &client).wait() {
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
//    client: HttpsClient,
    current_price: f64,
    request: StockTransactionRequest,
    user_uuid: Uuid,
    conn: PooledConn,
) -> Result<impl Reply, Rejection> {
//    let current_price = get_current_price(&request.symbol, &client).map_err(Error::reject)?;
//    let current_price = 2.0; // TODO FIX

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

fn get_current_price(stock_symbol: &str, client: &HttpsClient) -> impl Future<Item = f64, Error = Error> {
    let uri: Uri = format!("https://api.iextrading.com/1.0/stock/{}/price", stock_symbol).parse().unwrap();
    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| {
            Error::DependentConnectionFailed {
                url: uri.to_string(),
            }
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            body.parse::<f64>()
                .map_err(|_| crate::error::Error::InternalServerError)
        })
}

// TODO, this doesn't support getting an infinite number of stocks, as there is a finite limit on the size of a url string.
// This will need to make multiple requests in that case.
fn get_current_prices(stock_symbols: &[&str], client: &HttpsClient) -> impl Future<Item = Vec<f64>, Error = Error> {
    let uri: Uri = format!("https://api.iextrading.com/1.0/stock/market/batch?symbols={}&types=price", stock_symbols.join(",")).parse().unwrap();

    // handle something like: {"AAPL":{"price":170.67},"FB":{"price":165.465}}
    #[derive(Serialize, Deserialize)]
    struct Price {
        price: f64
    }
    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| {
            Error::DependentConnectionFailed {
                url: uri.to_string(),
            }
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            serde_json::from_str::<HashMap<String, Price>>(&body)
                .map(|r| r.values().map(|v| v.price).collect())
                .map_err(|_| crate::error::Error::InternalServerError)

        })
}

#[cfg(test)]
mod integration {
    use super::*;
    use futures::future;
    use tokio;

    #[test]
    fn can_get_current_price() {
        // This test assumes that apple's stock price is above 1 dollar per share.
        // A fair assumption, but it may not always be true :/.
        tokio::run(future::lazy(|| {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder()
                .build::<_, hyper::Body>(https);
            get_current_price("aapl", &client)
                .map(|price| assert!(price > 0.0, "Aapl should have a positive share price."))
                .map_err(|_| panic!("Could not get current price") )
        }));
    }


    #[test]
    fn can_get_multiple_current_price() {
        // This test assumes that apple's stock price is above 1 dollar per share.
        // A fair assumption, but it may not always be true :/.
        tokio::run(future::lazy(|| {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder()
                .build::<_, hyper::Body>(https);
            get_current_prices(&["aapl", "fb"], &client)
                .map(|prices| assert!(prices.len() == 2))
                .map_err(|_| panic!("Could not get current price") )
        }));
    }
}

