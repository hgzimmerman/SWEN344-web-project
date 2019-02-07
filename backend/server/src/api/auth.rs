use crate::state::State;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Reply};

use crate::{
    auth::{user_filter, JwtPayload, Secret},
    error::Error,
    util,
};
use db::user::{NewUser, User};
use log::info;
use pool::PooledConn;
use uuid::Uuid;
use warp::{path, Filter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub oauth_token: String,
}

pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    let login = path!("login")
        .and(warp::post2())
        .and(util::json_body_filter(3))
        .and(state.secret.clone())
        .and(state.db.clone())
        .and_then(|login: Login, secret: Secret, conn: PooledConn| {
            info!("Got token! {}", login.oauth_token); // TODO remove this in production
                                                       // take token, go to platform, get client id.
            let client_id = get_client_id(&login.oauth_token);
            info!("Resolved OAuth token to client_id: {}", client_id);
            // search DB for user with client id.
            User::get_user_by_client_id(&client_id, &conn)
                .or_else(|_| {
                    info!("Could not find user, creating new one");
                    // If user does not exist, create one.
                    let new_user = NewUser { client_id };
                    User::create_user(new_user, &conn)
                })
                .map(|user| JwtPayload::new(user.uuid))
                .map_err(Error::from)
                .and_then(|payload| payload.encode_jwt_string(&secret))
                .map_err(Error::reject)
        });

    //    #[cfg(test)]
    //    let login_unit_test = path!("login_unit_test")
    //        .and(warp::get2())
    //        .and(state.secret.clone())
    //        .map(|secret: Secret| {
    //            let payload = JwtPayload::new(Uuid::new_v4());
    //            payload.encode_jwt_string(&secret)
    //        });

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

// TODO actually implement this
/// This needs to contact facebook with the token, and get the unique client id.
/// Given an oauth token, return a client id.
fn get_client_id(_oauth_token: &str) -> String {
    if cfg!(test) {
        TEST_CLIENT_ID.to_string() // allow user login for testing
    } else {
        // TODO actually fetch the client id.
        "YEEEET".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{state::State, testing_fixtures::user::UserFixture};
    use pool::Pool;
    use testing_common::setup::setup_warp;

    use crate::{
        auth::{AUTHORIZATION_HEADER_KEY, BEARER},
        testing_fixtures::util::{deserialize, deserialize_string},
    };

    #[test]
    fn login_works() {
        setup_warp(|_fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = auth_api(&s);

            let login = Login {
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

            let login = Login {
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

            let resp = warp::test::request()
                .method("GET")
                .path("/auth/user")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            let user: User = deserialize(resp);
            assert_eq!(user, fixture.user)
        });
    }

}
