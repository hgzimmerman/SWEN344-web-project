
mod calendar;
mod market;
pub (crate) mod auth;

use warp::filters::BoxedFilter;
use warp::Reply;

use warp::path;
use warp::Filter;
use crate::state::State;

use self::calendar::calendar_api;
use crate::api::market::market_api;
use crate::api::auth::auth_api;

pub fn api(state: &State) -> BoxedFilter<(impl Reply,)> {

    path!("api")
        .and(
            market_api(state)
                .or(calendar_api(state))
                .or(auth_api(state))
        )
        .boxed()

}


/// A function that:
/// * Routes the API
/// * Handles file requests and redirections - NOT IMPLEMENTED
/// * Initializes logging
/// * Handles errors
/// * Handles CORS
pub fn routes(state: &State) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
//        .allow_origin("http://localhost:8081")
        .allow_headers(vec!["Access-Control-Allow-Origin", "content-type", "Authorization"])
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT","DELETE"]);

    api(state)
        .with(warp::log("routes"))
        .with(cors)
        .recover(crate::error::customize_error)
        .boxed()
}



#[cfg(test)]
mod integration_test {
    use super::*;
    use testing_common::fixture::Fixture;
    use testing_common::setup::setup_warp;
    use crate::testing_fixtures::user::UserFixture;
    use db::pool::Pool;
    use crate::state::State;

    use crate::testing_fixtures::util::deserialize_string;
    use crate::testing_fixtures::util::deserialize;
    use crate::auth::BEARER;
    use crate::auth::AUTHORIZATION_HEADER_KEY;
    use crate::auth::Secret;
    use crate::api::auth::Login;
    use db::user::User;

    /// Convenience function for requesting the JWT.
    /// In the testing environment, the login function will always work.
    fn get_jwt(filter: BoxedFilter<(impl Reply + 'static,)>) -> String {
            let login = Login {
                oauth_token: "Test Garbage because we don't want to have the tests depend on FB".to_string()
            };

            let resp = warp::test::request()
                .method("POST")
                .path("/api/auth/login")
                .json(&login)
                .header("content-length", "300")
                .reply(&filter);

            let jwt = deserialize_string(resp);
        jwt
    }

    #[test]
    fn login_works() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = routes(&s);

            let login = auth::Login {
                oauth_token: "Test Garbage because we don't want to have the tests depend on FB".to_string()
            };
            let resp = warp::test::request()
                .method("POST")
                .path("/api/auth/login")
                .json(&login)
                .header("content-length", "300")
                .reply(&filter);

            assert_eq!(
                resp.status(),
                200
            )
        });
    }

    #[test]
    fn user_works() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = routes(&s);

            let jwt = get_jwt(filter.clone());

            let resp = warp::test::request()
                .method("GET")
                .path("/api/auth/user")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            assert_eq!(
                resp.status(),
                200
            );

            let user: User = deserialize(resp);
            assert_eq!(
                user,
                fixture.user
            )
        });
    }



}
