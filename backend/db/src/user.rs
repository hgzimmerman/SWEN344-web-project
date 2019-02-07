use crate::schema::{self, users};
use diesel::{
    pg::PgConnection, query_dsl::QueryDsl, result::QueryResult, ExpressionMethods, Identifiable,
    Insertable, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

// TODO this may have a name that is acquired when the user first logs in by requesting the name from the oauth provider, but it isn't strictly necessary.

#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "users"]
pub struct User {
    pub uuid: Uuid,
    pub client_id: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub client_id: String,
}

impl User {
    pub fn create_user(user: NewUser, conn: &PgConnection) -> QueryResult<User> {
        crate::util::create_row(schema::users::table, user, conn)
    }

    pub fn get_user(uuid: Uuid, conn: &PgConnection) -> QueryResult<User> {
        crate::util::get_row(schema::users::table, uuid, conn)
    }

    pub fn get_user_by_client_id(client_id: &str, conn: &PgConnection) -> QueryResult<User> {
        users::table
            .filter(users::dsl::client_id.eq(client_id))
            .first::<User>(conn)
    }
}
