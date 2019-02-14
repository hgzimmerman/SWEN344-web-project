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
    pub uuid: Uuid,
    pub symbol: String,
    pub stock_name: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "stocks"]
pub struct NewStock {
    pub symbol: String,
    pub stock_name: String,
}

#[derive(
    Clone,
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
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub stock_uuid: Uuid,
    pub quantity: i32, // Can you get non-integer quantities of stocks?
    pub price_of_stock_at_time_of_trading: f64,
    pub record_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[table_name = "stock_transactions"]
pub struct NewStockTransaction {
    pub user_uuid: Uuid,
    pub stock_uuid: Uuid,
    pub quantity: i32, // Can you get non-integer quantities of
    pub price_of_stock_at_time_of_trading: f64,
    pub record_time: NaiveDateTime,
}

impl Stock {
    pub fn create_stock(new_stock: NewStock, conn: &PgConnection) -> QueryResult<Stock> {
        util::create_row(schema::stocks::table, new_stock, conn)
    }

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

    pub fn get_stock(stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<Stock> {
        util::get_row(schema::stocks::table, stock_uuid, conn)
    }
    pub fn get_stocks(conn: &PgConnection) -> QueryResult<Vec<Stock>> {
        schema::stocks::table.load(conn)
    }

    pub fn get_stock_by_symbol(stock_symbol: String, conn: &PgConnection) -> QueryResult<Stock> {
        schema::stocks::table
            .filter(schema::stocks::dsl::symbol.eq(stock_symbol))
            .first(conn)
    }

    //    pub fn get_stocks_and_their_current_prices(conn: &PgConnection) -> QueryResult<Vec<(Stock, Option<StockPrice>)>> {
    //        let stocks = Self::get_stocks(conn)?;
    //
    //        let prices = StockPrice::belonging_to(&stocks)
    //            .order_by(schema::stock_prices::dsl::record_time.desc()) // TODO verify that this is in order, but it should be.
    //            .distinct_on(schema::stock_prices::stock_uuid)
    //            .load::<StockPrice>(conn)?
    //            .grouped_by(&stocks);
    //
    //        stocks
    //            .into_iter()
    //            .zip(prices)
    //            .map(|(stock, mut prices): (Stock, Vec<StockPrice>)| (stock, prices.pop()) ) // There should only be one element in here
    //            .collect::<Vec<_>>()
    //            .apply(Ok)
    //    }

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStockResponse {
    pub stock: Stock,
    pub transactions: Vec<StockTransaction>,
}

impl UserStockResponse {
    pub fn quantity_stocks_owned(&self) -> i32 {
        self.transactions.iter().fold(0, |acc, t| acc + t.quantity)
    }
}
