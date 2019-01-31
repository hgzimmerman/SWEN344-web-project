use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{
    Identifiable,
    Queryable,
    Insertable
};
use crate::schema::{
    self,
    stocks,
    stock_prices,
    stock_transactions
};
use serde::{Serialize, Deserialize};
use diesel::pg::PgConnection;
use crate::util;
use diesel::result::QueryResult;
use diesel::query_dsl::RunQueryDsl;
use diesel::BelongingToDsl;
use diesel::query_dsl::QueryDsl;
use diesel::ExpressionMethods;
use diesel::associations::GroupedBy;
use crate::user::User;
use std::collections::BTreeMap;
use apply::Apply;

#[derive(Clone, Debug, Identifiable, Queryable, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[primary_key(uuid)]
#[table_name = "stocks"]
pub struct Stock {
    pub uuid: Uuid,
    pub symbol: String,
    pub stock_name: String
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "stocks"]
pub struct NewStock {
    pub symbol: String,
    pub stock_name: String
}


#[derive(Clone, Debug, Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[primary_key(uuid)]
#[belongs_to(Stock, foreign_key = "stock_uuid")]
#[table_name = "stock_prices"]
pub struct StockPrice {
    pub uuid: Uuid,
    pub stock_uuid: Uuid,
    pub price: f64, // should be decimal, but fucccc databases, we ok with losses with this application
    pub record_time: NaiveDateTime
}



#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "stock_prices"]
pub struct NewStockPrice {
    pub stock_uuid: Uuid,
    pub price: f64, // should be decimal, but fucccc databases, we ok with losses with this application
    pub record_time: NaiveDateTime
}


#[derive(Clone, Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[primary_key(uuid)]
#[belongs_to(StockPrice, foreign_key = "price_uuid")]
#[belongs_to(Stock, foreign_key = "stock_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "stock_transactions"]
pub struct StockTransaction {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub stock_uuid: Uuid,
    pub price_uuid: Uuid,
    pub quantity: i32 // Can you get non-integer quantities of stocks?
}


#[derive(Insertable, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[table_name = "stock_transactions"]
pub struct NewStockTransaction {
    pub user_uuid: Uuid,
    pub stock_uuid: Uuid,
    pub price_uuid: Uuid,
    pub quantity: i32 // Can you get non-integer quantities of
}



impl Stock {
    pub fn create_stock(new_stock: NewStock, conn: &PgConnection) -> QueryResult<Stock> {
        util::create_row(schema::stocks::table, new_stock, conn)
    }
    pub fn create_price(new_price: NewStockPrice, conn: &PgConnection) -> QueryResult<StockPrice> {
        util::create_row(schema::stock_prices::table, new_price, conn)
    }
    pub fn create_transaction(new_transaction: NewStockTransaction, conn: &PgConnection) -> QueryResult<StockTransaction> {
        util::create_row(schema::stock_transactions::dsl::stock_transactions, new_transaction, conn)
    }

    // TODO, move this to the server crate so it can have a reasonable quantity.
    pub fn create_transaction_safe(user_uuid: Uuid, new_transaction: NewStockTransaction, conn: &PgConnection) -> QueryResult<StockTransaction> {
        let t = Self::get_user_transactions_for_stock(user_uuid, new_transaction.stock_uuid, conn)?;
        let quantity = t.iter().fold(0, | acc, st| acc + st.quantity);
        if new_transaction.quantity > quantity {
            panic!() // can't have negative -> need a real error
        } else {
            Stock::create_transaction(new_transaction, conn)
        }
    }

    pub fn get_stock(stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<Stock> {
        util::get_row(schema::stocks::table, stock_uuid, conn)
    }
    pub fn get_stocks(conn: &PgConnection) -> QueryResult<Vec<Stock>> {
        schema::stocks::table
            .load(conn)
    }

    pub fn get_stocks_and_their_current_prices(conn: &PgConnection) -> QueryResult<Vec<(Stock, Option<StockPrice>)>> {
        let stocks = Self::get_stocks(conn)?;

        let prices = StockPrice::belonging_to(&stocks)
            .order_by(schema::stock_prices::dsl::record_time.desc()) // TODO verify that this is in order, but it should be.
            .distinct_on(schema::stock_prices::stock_uuid)
            .load::<StockPrice>(conn)?
            .grouped_by(&stocks);

        stocks
            .into_iter()
            .zip(prices)
            .map(|(stock, mut prices): (Stock, Vec<StockPrice>)| (stock, prices.pop()) ) // There should only be one element in here
            .collect::<Vec<_>>()
            .apply(Ok)
    }

    pub fn get_stock_belonging_to_users(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<UserStockResponse>> {

        schema::stock_transactions::table
            .filter(schema::stock_transactions::dsl::user_uuid.eq(user_uuid))
            .inner_join(schema::stock_prices::dsl::stock_prices.inner_join(schema::stocks::dsl::stocks))
            .load::<(StockTransaction, (StockPrice, Stock))>(conn)?
            .into_iter()
            .fold(BTreeMap::<Stock, Vec<Transaction>>::new(), |mut acc, (transaction, (stock_price, stock))| {
                let t = Transaction {
                    price: stock_price.price,
                    quantity: transaction.quantity,
                    time: stock_price.record_time
                };
                acc.entry(stock)
                    .and_modify(|x| x.push(t.clone()))
                    .or_insert_with(|| vec![t]);
                acc
            })
            .into_iter()
            .map(|(stock, transactions)| {
                UserStockResponse {
                    stock: stock.clone(),
                    transactions
                }
            })
            .collect::<Vec<_>>()
            .apply(Ok)
    }

    pub fn get_user_transactions_for_stock(user_uuid: Uuid, stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<StockTransaction>> {
        schema::stock_transactions::table
            .filter(schema::stock_transactions::stock_uuid.eq(stock_uuid))
            .load(conn)
    }

    pub fn get_stock_price_history(stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<Vec<StockPrice>> {
        schema::stock_prices::table
            .filter(schema::stock_prices::dsl::stock_uuid.eq(stock_uuid))
            .load::<StockPrice>(conn)
    }

    pub fn get_most_recent_price(stock_uuid: Uuid, conn: &PgConnection) -> QueryResult<StockPrice> {
        schema::stock_prices::table
            .filter(schema::stock_prices::dsl::stock_uuid.eq(stock_uuid))
            .order_by(schema::stock_prices::dsl::record_time.desc()) // TODO verify that this is in order, but it should be.
            .first(conn)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStockResponse {
    stock: Stock,
    transactions: Vec<Transaction>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    price: f64,
    quantity: i32,
    time: NaiveDateTime
}


impl UserStockResponse {
    pub fn quantity_stocks_owned(&self) -> i32 {
        self.transactions
            .iter()
            .fold(0, |acc, t| acc + t.quantity)
    }
}


pub struct TransactRequest {
    symbol: String,
    quantity: i32,
}
