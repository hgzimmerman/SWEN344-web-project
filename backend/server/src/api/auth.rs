//! Responsible for granting JWT tokens on login.
use crate::state::State;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Reply};

use crate::{
    error::Error,
    util,
};
use auth::{JwtPayload, Secret};
use crate::server_auth::user_filter;
use db::user::{NewUser, User};
use log::info;
use pool::PooledConn;
use uuid::Uuid;
use warp::{path, Filter};
use chrono::Duration;
use warp::Rejection;
use hyper::Uri;
use futures::Future;
use hyper::Client;
use futures::stream::Stream;
use hyper::Chunk;
use apply::Apply;

/// A request to log in to the system.
/// This only requires the oauth_token, as the server can resolve other details from that.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub oauth_token: String,
}

/// The authentication api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    let login = path!("login")
        .and(warp::post2())
        .and(util::json_body_filter(3))
        .and(state.secret.clone())
        .and(state.db.clone())
        .and_then(get_or_create_user);

    // TODO maybe move this not under the auth/ route
    let user = path!("user")
        .and(user_filter(state))
        .and(state.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            User::get_user(user_uuid, &conn)
                .map_err(Error::from)
                .map_err(Error::reject)
                .map(util::json)
        });

    path!("auth").and(login.or(user)).boxed()
}

pub const TEST_CLIENT_ID: &str = "test client id";

/// Shim for the get_user_id_from_facebook function.
/// The shim allows tests to always have the auth process succeed succeed.
fn get_user_id(oauth_token: &str) -> Result<String, Error> {
    // If this runs in a test environment, it will work without question.
    // Otherwise, it will attempt to acquire the user_id from facebook.
    if cfg!(test) {
        TEST_CLIENT_ID.to_string().apply(Ok) // Allow user login for testing
    } else {
        get_user_id_from_facebook(oauth_token).wait() // Await the response
    }
}


/// Gets the user id from facebook
// TODO verify that this works.
fn get_user_id_from_facebook(oauth_token: &str) -> impl Future<Item = String, Error = Error> {
    let client = Client::new();
    let uri: Uri = format!("https://graph.facebook.com/me?access_token={}", oauth_token).parse().unwrap();

    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| Error::DependentConnectionFailed { // TODO, this should look at the response code and produce another error if the access token is invalid.
            url: uri.to_string(),
        })
        .map(|chunk: Chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
}


/// Gets a user, and if the user doesn't exist yet, create one and return it anyway.
///
/// # Arguments
/// login - The request containing the oauth token.
/// secret - The secret used for signing JWTs.
/// conn - The connection to the database.
fn get_or_create_user(login: LoginRequest, secret: Secret, conn: PooledConn)  -> Result<impl Reply, Rejection> {
    info!("Got token! {}", login.oauth_token); // TODO remove this in production
    // take token, go to platform, get client id.
    let client_id = get_user_id(&login.oauth_token)
        .map_err(Error::reject)?;
    info!("Resolved OAuth token to client_id: {}", client_id);
    // search DB for user with client id.
    User::get_user_by_client_id(&client_id, &conn)
        .or_else(|_| {
            info!("Could not find user, creating new one");
            // If user does not exist, create one.
            let new_user = NewUser { client_id };
            User::create_user(new_user, &conn)
        })
        .map(|user| JwtPayload::new(user.uuid, Duration::weeks(5)))
        .map_err(Error::from)
        .and_then(|payload| payload.encode_jwt_string(&secret).map_err(Error::AuthError))
        .map_err(Error::reject)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{state::State, testing_fixtures::user::UserFixture};
    use pool::Pool;
    use testing_common::setup::setup_warp;

    use crate::{
        testing_fixtures::util::{deserialize, deserialize_string},
    };

    use ::auth::{AUTHORIZATION_HEADER_KEY, BEARER};

    #[test]
    fn login_works() {
        setup_warp(|_fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = auth_api(&s);

            let login = LoginRequest {
                oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
                    .to_string(),
            };
            let resp = warp::test::request()
                .method("POST")
                .path("/auth/login")
                .json(&login)
                .header("content-length", "300")
                .reply(&filter);

            assert_eq!(resp.status(), 200)
        });
    }

    #[test]
    fn user_works() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = auth_api(&s);

            let login = LoginRequest {
                oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
                    .to_string(),
            };

            let resp = warp::test::request()
                .method("POST")
                .path("/auth/login")
                .json(&login)
                .header("content-length", "300")
                .reply(&filter);

            let jwt = deserialize_string(resp);

            let resp  = warp::test::request()
                .method("GET")
                .path("/auth/user")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            let user: User = deserialize(resp);
            assert_eq!(user, fixture.user)
        });
    }

}
