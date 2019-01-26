use uuid::Uuid;
use diesel::{
    Identifiable,
    Queryable,
    Insertable
};
use crate::schema::users;
use serde::{Serialize, Deserialize};



// Flow for user login/creation?
// user logs into facebook and gets a oauth access_token.
// user attaches token to request to backend.
// backend makes request to:
// https://graph.facebook.com/me?fields=id&access_token=xxx
// gets the unique id.
// maybe this id is a uuid?
// If that is the case, then we don't generate a random uuid.
// Also, it may not make sense to store an oauth token.
// Either way, we key the user on this id.

// So we probably have an endpoint called log_in that takes an oauth token,
// and looks up the ID. If the account exists, then they get the account, otherwise it is created.
// As a consequence, do we need to hit facebook for the id on every request???
// That would be terrible.
// Do we grant a JWT or cookie instead?


// app      fb       api
//  |        |         |
//  | login->|         |
//  | <-oauth|         |
//  | ----oauth-login->|
//  |        | <-get_id|
//  |        | -- id ->|
//  |<-JWT------------ |
//  |        |         |
// app holds on to oauth and can periodically refresh it.
// If it expires, they just log into facebook again.
// When that second login takes place, the api or app checks if the user is the same


#[derive(Clone, Debug, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "users"]
pub struct User {
    uuid: Uuid,
    name: String,
    oauth: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct  NewUser {
    name: String,
    oauth: String
}
