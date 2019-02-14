//! All database queries directly related to stocks and their transactions are contained within this module.
use crate::{
    schema::{self, stock_transactions, stocks},
    user::User,
    util,
};
use apply::Apply;
use chrono::NaiveDateTime;
use diesel::{
    pg::PgConnection,
    query_dsl::{QueryDsl, RunQueryDsl},
    result::QueryResult,
    BoolExpressionMethods, ExpressionMethods, Identifiable, Insertable, Queryable,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Struct to represent a row in the `stocks` table.
#[derive(
    Clone,
    Debug,
    Identifiable,
    Queryable,
    Serialize,
    Deserialize,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Hash,
)]
#[primary_key(uuid)]
#[table_name = "stocks"]
pub struct Stock {
    /// Unique identifier
    pub uuid: Uuid,
    /// Stock symbol [Ticker symbol](https://en.wikipedia.org/wiki/Ticker_symbol)
    pub symbol: String,
    /// The name of the company the stock is associated with.
    pub stock_name: String,
}

/// A struct used for creating new rows in the `stocks` table.
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "stocks"]
pub struct NewStock {
    /// Stock symbol [Ticker symbol](https://en.wikipedia.org/wiki/Ticker_symbol)
    pub symbol: String,
    /// The name of the company the stock is associated with.
    pub stock_name: String,
}


/// Struct to represent a row in the `stock_transactions` table.
/// A stock transaction records how much of a given stock a user purchases or sells at discrete times and prices.
#[derive(
    Clone,
    Copy,
    Debug,
    Identifiable,
    Queryable,
    Associations,
    Serialize,
    Deserialize,
    PartialOrd,
    PartialEq,
)]
#[primary_key(uuid)]
#[belongs_to(Stock, foreign_key = "stock_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "stock_transactions"]
pub struct StockTransaction {
    /// Unique identifier.
    pub uuid: Uuid,
    /// Uuid of the user the transaction is associated with.
    pub user_uuid: Uuid,
    /// The uuid of the stock the transaction is associated with.
    pub stock_uuid: Uuid,
    /// The quantity of stocks being exchanged.
    /// Negative indicates a sale, positive indicates a purchase.
    pub quantity: i32, // Can you get non-integer quantities of stocks?
    /// The price of the stock at the time of the transaction.
    pub price_of_stock_at_time_of_trading: f64,
    /// The time at which the trade occurred.
    pub record_time: NaiveDateTime,
}

/// A struct used for creating new rows in the `stock_transactions` table.
#[derive(Insertable, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Copy, Clone)]
#[table_name = "stock_transactions"]
pub struct NewStockTransaction {
    /// Uuid of the user the transaction is associated with.
    pub user_uuid: Uuid,
    /// The uuid of the stock the transaction is associated with.
    pub stock_uuid: Uuid,
    /// The quantity of stocks being exchanged.
    pub quantity: i32, // Can you get non-integer quantities of
    /// The price of the stock at the time of the transaction.
    pub price_of_stock_at_time_of_trading: f64,
    /// The time at which the trade occurred.
    pub record_time: NaiveDateTime,
}

impl Stock {
    /// Creates a stock.
    pub fn create_stock(new_stock: NewStock, conn: &PgConnection) -> QueryResult<Stock> {
        util::create_row(schema::stocks::table, new_stock, conn)
    }

    /// Creates a transaction.
    pub fn create_transaction(
        new_transaction: NewStockTransaction,
        conn: &PgConnection,
    ) -> QueryResult<StockTransaction> {
        util::create_row(
            schema::stock_transactions::dsl::stock_transactions,
            new_transaction,
            conn,
        )
    }

    // TODO this needs some work... or it could be removed, as I think a higher level of abstraction deals with this... double check.
    /// Creates a transaction
    pub fn create_transaction_safe(
        user_uuid: Uuid,
        new_transaction: NewStockTransaction,
        conn: &PgConnection,
    ) -> QueryResult<StockTransaction> {
        let t = Self::get_user_transactions_for_stock(user_uuid, new_transaction.stock_uuid, conn)?;
        let quantity = t.iter().fold(0, |acc, st| acc + st.quantity);
        if new_transaction.quantity > quantity {
            panic!("can't have negative -> need a real error")
        } else {
            Stock::create_transaction(new_transaction, conn)
        }
    }

    /// Gets a given stock.
    pub fn get_stock(stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<Stock> {
        util::get_row(schema::stocks::table, stock_uuid, conn)
    }
    /// Gets all stocks currently indexed by the database.
    pub fn get_stocks(conn: &PgConnection) -> QueryResult<Vec<Stock>> {
        schema::stocks::table.load(conn)
    }

    /// Gets a stock currently in the database via its ticker symbol.
    pub fn get_stock_by_symbol(stock_symbol: String, conn: &PgConnection) -> QueryResult<Stock> {
        schema::stocks::table
            .filter(schema::stocks::dsl::symbol.eq(stock_symbol))
            .first(conn)
    }

    /// Get the stocks and associated transactions that belong to a given user.
    pub fn get_stocks_belonging_to_user(
        user_uuid: Uuid,
        conn: &PgConnection,
    ) -> QueryResult<Vec<UserStockResponse>> {
        schema::stock_transactions::table
            .filter(schema::stock_transactions::dsl::user_uuid.eq(user_uuid))
            .inner_join(schema::stocks::dsl::stocks)
            .load::<(StockTransaction, Stock)>(conn)?
            .into_iter()
            .fold(
                BTreeMap::<Stock, Vec<StockTransaction>>::new(),
                |mut acc, (transaction, stock)| {
                    acc.entry(stock)
                        .and_modify(|x| x.push(transaction.clone()))
                        .or_insert_with(|| vec![transaction]);
                    acc
                },
            )
            .into_iter()
            .map(|(stock, transactions)| UserStockResponse {
                stock: stock.clone(),
                transactions,
            })
            .collect::<Vec<_>>()
            .apply(Ok)
    }

    /// Get the transactions for a given stock for a given user.
    pub fn get_user_transactions_for_stock(
        user_uuid: Uuid,
        stock_uuid: Uuid,
        conn: &PgConnection,
    ) -> QueryResult<Vec<StockTransaction>> {
        schema::stock_transactions::table
            .filter(
                schema::stock_transactions::stock_uuid
                    .eq(stock_uuid)
                    .and(schema::stock_transactions::user_uuid.eq(user_uuid)),
            )
            .load(conn)
    }
}

/// A response struct containing a stock row, as well as its associated transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStockResponse {
    /// Stock row.
    pub stock: Stock,
    /// Associated transactions for the stock.
    pub transactions: Vec<StockTransaction>,
}

impl UserStockResponse {
    /// Gets the net quantity of shares the user has acquired by summing the transactions.
    pub fn quantity_stocks_owned(&self) -> i32 {
        self.transactions.iter().fold(0, |acc, t| acc + t.quantity)
    }
}
