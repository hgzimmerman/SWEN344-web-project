use uuid::Uuid;
use diesel::{
    Identifiable,
    Queryable,
    Insertable
};
use crate::schema::funds;

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