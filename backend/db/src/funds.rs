use uuid::Uuid;
use diesel::{
    Identifiable,
    Queryable,
    Insertable,
    PgConnection
};
use crate::schema::funds;
use diesel::result::QueryResult;

#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "funds"]
pub struct Funds {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub quantity: f64
}

#[derive(Insertable, Debug)]
#[table_name = "funds"]
pub struct NewFunds {
    pub user_uuid: Uuid,
    pub quantity: f64
}


pub fn add_funds(user_uuid: Uuid, quantity: f64, conn: &PgConnection) -> QueryResult<Funds> {
    unimplemented!()
}

pub fn remove_funds(user_uuid: Uuid, quantity: f64, conn: &PgConnection) -> QueryResult<Funds> {
    unimplemented!()
}

pub fn funds(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Funds> {
    unimplemented!()
}