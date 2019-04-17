//! All database queries directly related to users are contained within this module.
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

/// A struct representing all the columns in the `users` table.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "users"]
pub struct User {
    /// The user's unique identifier within the application.
    pub uuid: Uuid,
    /// The user's unique identifier within facebook.
    pub twitter_user_id: String,
    /// Zip code that the user resides within
    pub zip_code: Option<String>,
}

/// Struct used to create new users.
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    /// The user's unique identifier within facebook.
    pub twitter_user_id: String,
}

impl User {
    /// Creates a user
    pub fn create_user(user: NewUser, conn: &PgConnection) -> QueryResult<User> {
        crate::util::create_row(schema::users::table, user, conn)
    }

    /// Gets a user using its unique identifier.
    pub fn get_user(uuid: Uuid, conn: &PgConnection) -> QueryResult<User> {
        crate::util::get_row(schema::users::table, uuid, conn)
    }

    /// Gets a user by the client id.
    pub fn get_user_by_twitter_id(client_id: &str, conn: &PgConnection) -> QueryResult<User> {
        users::table
            .filter(users::dsl::twitter_user_id.eq(client_id))
            .first::<User>(conn)
    }

    /// Sets the zip code for the user.
    pub fn set_zip_code(user_uuid: Uuid, zip: String, conn: &PgConnection) -> QueryResult<User> {
        diesel::update(users::table.find(user_uuid))
            .set(users::zip_code.eq(zip))
            .get_result(conn)
    }

    /// Gets the user's zip code.
    pub fn get_zip_code(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Option<String>> {
        users::table
            .find(user_uuid)
            .select(users::zip_code)
            .get_result(conn)
    }
}
