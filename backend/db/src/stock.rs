use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{
    Identifiable,
    Queryable,
    Insertable
};
use crate::schema::{
    stocks,
    stock_prices,
    stock_transactions
};


#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[table_name = "stocks"]
pub struct Stock {
    pub uuid: Uuid,
    pub symbol: String,
    pub stock_name: String
}

#[derive(Insertable, Debug)]
#[table_name = "stocks"]
pub struct NewStock {
    pub symbol: String,
    pub stock_name: String
}


#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[belongs_to(Stock, foreign_key = "stock_uuid")]
#[table_name = "stock_prices"]
pub struct StockPrice {
    pub uuid: Uuid,
    pub stock_uuid: Uuid,
    pub price: f64, // should be decimal, but fucccc databases, we ok with losses with this application
    pub record_time: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "stock_prices"]
pub struct NewStockPrice {
    pub stock_uuid: Uuid,
    pub price: f64, // should be decimal, but fucccc databases, we ok with losses with this application
    pub record_time: NaiveDateTime
}


#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[belongs_to(StockPrice, foreign_key = "price_uuid")]
#[belongs_to(user, foreign_key = "user_uuid")]
#[table_name = "stock_transactions"]
pub struct StockAcquisition {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub price_uuid: Uuid,
    pub quantity: i32 // Can you get non-integer quantities of stocks?
}


#[table_name = "stock_transactions"]
pub struct NewStockAcquisition {
    pub user_uuid: Uuid,
    pub price_uuid: Uuid,
    pub quantity: i32 // Can you get non-integer quantities of
}

