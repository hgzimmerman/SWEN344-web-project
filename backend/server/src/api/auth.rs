//! Responsible for granting JWT tokens on login.
use crate::state::State;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Reply};

use crate::{error::Error, state::HttpsClient, util};
use apply::Apply;
use authorization::{JwtPayload, Secret};
use chrono::Duration;
use db::user::{NewUser, User};
use futures::{stream::Stream, Future};
use hyper::{Chunk, Uri};
use log::info;
use pool::PooledConn;
use warp::{path, Filter};
use crate::error::err_to_rejection;

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
    info!("Attaching Auth api");
    let login = path!("login")
        .and(warp::post2())
        .and(util::json_body_filter(3))
        .and(state.https.clone())
        .and_then(|request: LoginRequest, client: HttpsClient| {
            get_user_id(request, client).map_err(Error::reject)
        })
        .and(state.secret.clone())
        .and(state.db.clone())
        .map(get_or_create_user)
        .and_then(err_to_rejection);


    path!("auth").and(login).boxed()
}

pub const TEST_CLIENT_ID: &str = "test client id";

/// Shim for the get_user_id_from_facebook function.
/// The shim allows tests to always have the auth process succeed succeed.
fn get_user_id(
    request: LoginRequest,
    client: HttpsClient,
) -> impl Future<Item = String, Error = Error> {
    // If this runs in a test environment, it will work without question.
    // Otherwise, it will attempt to acquire the user_id from facebook.
    use futures::future::Either;
    info!("Getting user id from oauth provider");
    let oauth_token = &request.oauth_token;
    if cfg!(test) {
        futures::future::ok::<String, Error>(TEST_CLIENT_ID.to_string()).apply(Either::A) // Automatic user login for testing
    } else {
        get_user_id_from_facebook(oauth_token, client).apply(Either::B) // Await the response
    }
}

/// Gets the user id from facebook
///
/// # Arguments
/// * oauth_token - The string representing the oauth access token granted from facebook.
/// * client - The https client used to make the request.
// TODO verify that this works.
fn get_user_id_from_facebook(
    oauth_token: &str,
    client: HttpsClient,
) -> impl Future<Item = String, Error = Error> {
    info!("Making request to Facebook to get the user_id");
    let uri: Uri = format!("https://graph.facebook.com/me?access_token={}", oauth_token)
        .parse()
        .unwrap();
    client
        .get(uri.clone())
        .map_err(move |_| Error::DependentConnectionFailed {
            url: uri.to_string(),
        })
        .and_then(|res| {
            if res.status().is_client_error() {
                Err(
                    Error::not_authorized("Bad OAuth token"))
            } else {
                Ok(res)
            }
        })
        .and_then(|res| {
            res.into_body()
                .concat2()
                .map_err(|_| Error::internal_server_error_empty()) // Await the whole body
        })
        .map(|chunk: Chunk| -> String {
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
fn get_or_create_user(
    client_id: String,
    secret: Secret,
    conn: PooledConn,
) -> Result<String, Error> {
    // take token, go to platform, get client id.
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
        .and_then(|payload| {
            payload
                .encode_jwt_string(&secret)
                .map_err(Error::AuthError)
                .map(|a| a)
        }) //dbg!(a)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{state::State, testing_fixtures::user::UserFixture};
    use pool::Pool;
    use testing_common::setup::setup_warp;
    use std::option::Option;

    use crate::testing_fixtures::util::{deserialize, deserialize_string};

    use authorization::{AUTHORIZATION_HEADER_KEY, BEARER};

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

}
