use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use serde::{Serialize, Deserialize};

use crate::util;
use warp::path;
use warp::Filter;
use db::user::User;
use db::pool::PooledConn;
use db::user::NewUser;
use crate::auth::JwtPayload;
use crate::auth::Secret;
use crate::auth::user_filter;
use uuid::Uuid;
use log::info;
use crate::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    oauth_token: String
}

pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    let login = path!("login")
        .and(warp::post2())
        .and(util::json_body_filter(1))
        .and(state.secret.clone())
        .and(state.db.clone())
        .and_then(|login: Login, secret: Secret, conn: PooledConn| {
            info!("Got token! {}", login.oauth_token);
            // take token, go to platform, get client id.
            let client_id = get_client_id(&login.oauth_token);
            // search DB for user with client id.
            User::get_user_by_oauth(&client_id, &conn)
                .or_else(|_|{
                    info!("Could not find user, creating new one");
                     // If user does not exist, create one.
                    let new_user = NewUser {
                        name: "I need to remove this field".to_string(),
                        oauth: client_id
                    };
                    User::create_user(new_user, &conn)
                })
                .map(|user| JwtPayload::new(user.uuid))
                .map_err(Error::from)
                .and_then(|payload| payload.encode_jwt_string(&secret))
                .map_err(Error::reject)
        });

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


    path!("auth")
        .and(
           login.or(user)
        )
        .boxed()
}


// TODO actually implement this
/// Given an oauth token, return a client id.
fn get_client_id(oauth_token: &str) -> String {
    "YEEEET".to_string()
}