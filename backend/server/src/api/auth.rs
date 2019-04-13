//! Responsible for granting JWT tokens on login.
use crate::state::State;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Reply};

use crate::error::Error;
use apply::Apply;
use authorization::{JwtPayload, Secret};
use chrono::Duration;
use db::user::{NewUser, User};
use futures::{Future};
//use hyper::{Chunk, Uri};
use log::info;
use pool::PooledConn;
use warp::{path, Filter};
//use crate::error::err_to_rejection;
use egg_mode::KeyPair;
use egg_mode::Token;
use askama::Template;
use crate::server_auth::Subject;
use std::convert::TryInto;
use warp::Rejection;
use std::borrow::Cow;


/// Meaningless id for testing purposes
pub static TEST_CLIENT_ID: &str = "yeet";

// TODO make #[cfg(test)]
pub fn get_jwt(state: &State) -> String {
//    use std::borrow::Cow;
    let secret: Secret = warp::test::request()
        .filter(&state.secret.clone())
        .unwrap();
    let conn: PooledConn = warp::test::request()
        .filter(&state.db.clone())
        .unwrap();
    let token = egg_mode::Token::Access {
        consumer: KeyPair { key: Cow::from(""), secret: Cow::from("") },
        access: KeyPair { key: Cow::from(""), secret: Cow::from("")}
    };
    let id = String::from(TEST_CLIENT_ID);
    get_or_create_user(token, id, secret, conn).expect("Should get or create user")
}

// TODO remove this and associated nonsense.
#[cfg(test)]
/// Get a jwt, with dummy data only for testing.
fn login_test_fn(state: &State) -> BoxedFilter<(impl Reply,)> {
    use crate::error::err_to_rejection;
    path!("login")
        .and(warp::post2())
        .map(|| {
            (
                egg_mode::Token::Access {
                    consumer: KeyPair { key: Cow::from(""), secret: Cow::from("") },
                    access: KeyPair { key: Cow::from(""), secret: Cow::from("")}
                },
                String::from("1")
            )
        })
        .untuple_one()
        .and(state.secret.clone())
        .and(state.db.clone())
        .map(get_or_create_user)
        .and_then(err_to_rejection)
        .boxed()
}

/// Empty implementation for test login function
#[cfg(not(test))]
fn login_test_fn(_state: &State) -> BoxedFilter<(impl Reply,)> {
    path!("login")
        .and(warp::post2())
        .and_then(|| -> Result<String, Rejection> {Err(warp::reject::not_found())})
        .boxed()
}

/// The authentication api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {

    info!("Attaching Auth api");

    let link = path!("link")
        .and(warp::get2())
        .and(state.twitter_con_token.clone())
        .and_then(|con_token| {
            egg_mode::request_token(&con_token, "https://vm344c.se.rit.edu/api/auth/callback")
                .map_err(|e| {
                    use log::error;
                    error!("{}", e);
                    Error::InternalServerError(Some("getting key pair failed".to_string())).reject()
                })
        })
        .map(|key_pair| {
            let authentication_url = egg_mode::authenticate_url(&key_pair);
            let link = Link {
                authentication_url
            };
            warp::reply::json(&link)
        });


    let callback = path!("callback")
        .and(warp::get2())
        .and(state.twitter_con_token.clone())
        .and(state.twitter_request_token.clone())
        .and(warp::query::query())
        .and_then(|con_token: KeyPair, key_pair: KeyPair, q_params: TwitterCallbackQueryParams| {
            use log::info;
            info!("{:?}", q_params); // TODO remove this info!() after tests indicate this works
            egg_mode::access_token((&con_token).clone(), &key_pair, q_params.oauth_verifier)
                .map_err(|_| Error::InternalServerError(Some("could not get access token.".to_owned())).reject())
        })
        .untuple_one()
        .and(state.secret.clone())
        .and(state.db.clone())
        .and_then(|token: Token, id: u64, _screen_name: String, secret: Secret, conn: PooledConn| -> Result<String, Rejection> {
            let jwt = get_or_create_user(token, format!("{}", id), secret, conn)
                .map_err(Error::reject)?;
            login_template_render(jwt ).apply(Ok)
        })
        .with(warp::reply::with::header("content-type","text/html"));


    // TODO remove me, this is for testing only
    let test_redirect = path!("test_redirect.html")
        .map(||{
            #[derive(Template)]
            #[template(path = "login.html")]
            struct LoginTemplate<'a> {
                jwt: &'a str,
                target_url: &'a str,
            }
            let login = LoginTemplate {
                jwt: "yeet",
                target_url: "/"
            };
            login.render()
                .unwrap_or_else(|e| e.to_string())
        })
        .with(warp::reply::with::header("content-type","text/html"));

    let subroutes = link
        .or(callback)
        .or(test_redirect)
        .or(login_test_fn(state));

    let api_root = path!("auth");

    api_root.and(subroutes).boxed()

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    authentication_url: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwitterCallbackQueryParams {
    oauth_token: String,
    oauth_verifier: String
}


/// Login by sending a small html page that inserts the JWT into localstorage
/// and then redirects to the main page.
///
/// # Note
/// The JWT is stored in window.localstorage under the key: 'jwt'
fn login_template_render(jwt: String) -> String {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate<'a> {
        jwt: &'a str,
        target_url: &'a str,
    }
    let login = LoginTemplate {
        jwt: &jwt,
        target_url: "/"
    };
    login.render()
        .unwrap_or_else(|e| e.to_string())
}


/// Shim for the get_user_id_from_facebook function.
/// The shim allows tests to always have the auth process succeed succeed.
//fn get_user_id(
//    request: LoginRequest,
//    client: HttpsClient,
//) -> impl Future<Item = String, Error = Error> {
//    // If this runs in a test environment, it will work without question.
//    // Otherwise, it will attempt to acquire the user_id from facebook.
//    use futures::future::Either;
//    info!("Getting user id from oauth provider");
//    let oauth_token = &request.oauth_token;
//    if cfg!(test) {
//        futures::future::ok::<String, Error>(TEST_CLIENT_ID.to_string()).apply(Either::A) // Automatic user login for testing
//    } else {
//        get_user_id_from_facebook(oauth_token, client).apply(Either::B) // Await the response
//    }
//}

/// Gets the user id from facebook
///
/// # Arguments
/// * oauth_token - The string representing the oauth access token granted from facebook.
/// * client - The https client used to make the request.
// TODO verify that this works.
//fn get_user_id_from_facebook(
//    oauth_token: &str,
//    client: HttpsClient,
//) -> impl Future<Item = String, Error = Error> {
//    info!("Making request to Facebook to get the user_id");
//    let uri: Uri = format!("https://graph.facebook.com/me?access_token={}", oauth_token)
//        .parse()
//        .unwrap();
//    client
//        .get(uri.clone())
//        .map_err(move |_| Error::DependentConnectionFailed {
//            url: uri.to_string(),
//        })
//        .and_then(|res| {
//            if res.status().is_client_error() {
//                Err(
//                    Error::not_authorized("Bad OAuth token"))
//            } else {
//                Ok(res)
//            }
//        })
//        .and_then(|res| {
//            res.into_body()
//                .concat2()
//                .map_err(|_| Error::internal_server_error_empty()) // Await the whole body
//        })
//        .map(|chunk: Chunk| -> String {
//            let v = chunk.to_vec();
//            String::from_utf8_lossy(&v).to_string()
//        })
//}

/// Gets a user, and if the user doesn't exist yet, create one and return it anyway.
///
/// # Arguments
/// login - The request containing the oauth token.
/// secret - The secret used for signing JWTs.
/// conn - The connection to the database.
fn get_or_create_user(
    twitter_token: Token,
    twitter_user_id: String,
    secret: Secret,
    conn: PooledConn,
) -> Result<String, Error> {
    // take token, go to platform, get client id.
    info!("Resolved OAuth token to twitter_user_id: {}", twitter_user_id);
    // search DB for user with client id.
    User::get_user_by_twitter_id(&twitter_user_id, &conn)
        .or_else(|_| {
            info!("Could not find user, creating new one");
            // If user does not exist, create one.
            let new_user = NewUser { twitter_user_id };
            User::create_user(new_user, &conn)
        })
        .map(|user| {
            let subject = Subject {
                user_uuid: user.uuid,
                twitter_token: twitter_token.try_into().expect("For purposes as used by this project, this conversion should be infallible")
            };
            JwtPayload::new(subject, Duration::weeks(5))
        })
        .map_err(Error::from)
        .and_then(|payload| {
            payload
                .encode_jwt_string(&secret)
                .map_err(Error::AuthError)
                .map(|a| a)
        })
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

//    #[test]
//    fn login_works() {
//        setup_warp(|_fixture: &UserFixture, pool: Pool| {
//            let secret = Secret::new("test");
//            let s = State::testing_init(pool, secret);
//            let filter = auth_api(&s);
//
//            let login = LoginRequest {
//                oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
//                    .to_string(),
//            };
//            let resp = warp::test::request()
//                .method("POST")
//                .path("/auth/login")
//                .json(&login)
//                .header("content-length", "300")
//                .reply(&filter);
//
//            assert_eq!(resp.status(), 200)
//        });
//    }

}
