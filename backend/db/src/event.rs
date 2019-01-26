use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{
    Identifiable,
    Queryable,
    Insertable
};
use crate::schema::events;


#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "events"]
pub struct Event {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub text: String,
    pub time_due: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct NewEvent {
    pub user_uuid: Uuid,
    pub text: String,
    pub time_due: NaiveDateTime
}