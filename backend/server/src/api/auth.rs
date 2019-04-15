//! Responsible for granting JWT tokens on login.
use crate::state::State;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Reply};

use crate::error::Error;
use apply::Apply;
use authorization::{JwtPayload, Secret};
use chrono::Duration;
use db::user::{NewUser, User};
use futures::Future;
//use hyper::{Chunk, Uri};
use log::info;
use pool::PooledConn;
use warp::{path, Filter};
//use crate::error::err_to_rejection;
use crate::server_auth::{jwt_filter, Subject};
use askama::Template;
use egg_mode::{KeyPair, Token};
use warp::Rejection;

/// Meaningless id for testing purposes
#[cfg(test)]
pub static TEST_CLIENT_ID: &str = "yeet";

/// Gets a basic JWT from the state for use in testing.
///
/// # Note
/// This JWT will not work with any twitter-related apis,
/// this is because the keys are empty.
#[cfg(test)]
pub fn get_jwt(state: &State) -> String {
    use std::borrow::Cow;
    let secret: Secret = warp::test::request().filter(&state.secret.clone()).unwrap();
    let conn: PooledConn = warp::test::request().filter(&state.db.clone()).unwrap();
    let token = egg_mode::Token::Access {
        consumer: KeyPair {
            key: Cow::from(""),
            secret: Cow::from(""),
        },
        access: KeyPair {
            key: Cow::from(""),
            secret: Cow::from(""),
        },
    };
    let id = String::from(TEST_CLIENT_ID);
    get_or_create_user(token, id, secret, conn).expect("Should get or create user")
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
        .and_then(move |con_token| {
            // If its compiled for production, redirect to the release URL, otherwise, localhost.
            let callback_link = if cfg!(feature = "production") {
                // This makes the assumption that nginx sits in front of the application, making port numbers irrelevant.
                "https://vm344c.se.rit.edu/api/auth/callback"
            } else {
                "http://localhost:8080/api/auth/callback" // This makes the assumption that the port is 8080
            };

            egg_mode::request_token(&con_token, callback_link).map_err(|e| {
                use log::error;
                error!("{}", e);
                Error::InternalServerError(Some("Getting key pair failed".to_string())).reject()
            })
        })
        .map(|key_pair| {
            let authentication_url = egg_mode::authenticate_url(&key_pair);
            let link = Link { authentication_url };
            warp::reply::json(&link)
        });

    let callback = path!("callback")
        .and(warp::get2())
        .and(state.twitter_con_token.clone())
        .and(state.twitter_request_token.clone())
        .and(warp::query::query())
        .and_then(
            |con_token: KeyPair, key_pair: KeyPair, q_params: TwitterCallbackQueryParams| {
                info!("{:?}", q_params); // TODO remove this info!() after tests indicate this works
                egg_mode::access_token((&con_token).clone(), &key_pair, q_params.oauth_verifier)
                    .map_err(|_| {
                        Error::InternalServerError(Some("could not get access token.".to_owned()))
                            .reject()
                    })
            },
        )
        .untuple_one()
        .and(state.secret.clone())
        .and(state.db.clone())
        .and_then(
            |token: Token,
             id: u64,
             _screen_name: String,
             secret: Secret,
             conn: PooledConn|
             -> Result<String, Rejection> {
                let jwt = get_or_create_user(token, format!("{}", id), secret, conn)
                    .map_err(Error::reject)?;
                login_template_render(jwt).apply(Ok)
            },
        )
        .with(warp::reply::with::header("content-type", "text/html"));

    // Refreshes the JWT
    let refresh = path!("refresh")
        .and(warp::post2())
        .and(jwt_filter(&state))
        .and(state.secret.clone())
        .and_then(|payload: JwtPayload<Subject>, secret: Secret| {
            let subject = payload.subject();
            let payload = JwtPayload::new(subject, chrono::Duration::weeks(5));
            payload
                .encode_jwt_string(&secret)
                .map_err(Error::AuthError)
                .map_err(warp::reject::custom)
                .map(|a| warp::reply::json(&a))
        });

    // TODO remove me, this is for testing only
    let test_redirect = path!("test_redirect.html")
        .map(|| {
            #[derive(Template)]
            #[template(path = "login.html")]
            struct LoginTemplate<'a> {
                jwt: &'a str,
                target_url: &'a str,
            }
            let login = LoginTemplate {
                jwt: "yeet",
                target_url: "/",
            };
            login.render().unwrap_or_else(|e| e.to_string())
        })
        .with(warp::reply::with::header("content-type", "text/html"));

    let subroutes = link.or(callback).or(test_redirect).or(refresh);

    let api_root = path!("auth");

    api_root.and(subroutes).boxed()
}

/// The JSON returned by the /api/auth/link route.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    authentication_url: String,
}

/// Query parameters for use in the twitter login callback.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwitterCallbackQueryParams {
    pub oauth_token: String,
    pub oauth_verifier: String,
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
        target_url: "/",
    };
    login.render().unwrap_or_else(|e| e.to_string())
}

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
    info!(
        "Resolved OAuth token to twitter_user_id: {}",
        twitter_user_id
    );
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
                twitter_token: twitter_token.into(),
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
