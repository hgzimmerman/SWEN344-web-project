//! The api defines all of the routes that are supported for the server.
mod advertisement;
pub(crate) mod auth;
mod calendar;
mod market;

use warp::{filters::BoxedFilter, Reply};

use crate::state::State;
use warp::{path, Filter};

use self::calendar::calendar_api;
use crate::{
    api::{
        advertisement::{add_api, health_api},
        auth::auth_api,
        market::market_api,
    },
    static_files::{static_files_handler, FileConfig},
};

/// The initial segment in the uri path.
pub const API_STRING: &str = "api";

/// The core of the exposed routes.
/// Anything that sits behind this filter accesses the DB in some way.
pub fn api(state: &State) -> BoxedFilter<(impl Reply,)> {
    path(API_STRING)
        .and(
            market_api(state)
                .or(calendar_api(state))
                .or(auth_api(state))
                .or(add_api(state))
                .or(health_api(state)),
        )
        .boxed()
}

/// A filter that is responsible for configuring everything that can be served.
///
/// It is responsible for:
/// * Routes the API
/// * Handles file requests and redirections
/// * Initializes warp logging
/// * converts errors
/// * Handles CORS
pub fn routes(state: &State) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
        //        .allow_origin("http://localhost:8081")
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "content-type",
            "Authorization",
        ])
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let file_config = FileConfig::default();

    api(state)
        .or(static_files_handler(file_config))
        .with(warp::log("routes"))
        .with(cors)
        .recover(crate::error::customize_error)
        .boxed()
}

#[cfg(test)]
mod integration_test {
    use super::*;
    use crate::{state::State, testing_fixtures::user::UserFixture};
    use pool::Pool;
    //    use testing_common::fixture::Fixture;
    use testing_common::setup::setup_warp;

    use crate::{
        api::{auth::LoginRequest, calendar::NewEventMessage},
        testing_fixtures::util::{deserialize, deserialize_string},
    };
//    use ::auth::{ AUTHORIZATION_HEADER_KEY, BEARER};
    use db::{
        event::{Event, EventChangeset},
        user::User,
    };

    use crate::testing_fixtures::util::get_jwt;
    use ::auth::Secret;
    use ::auth::AUTHORIZATION_HEADER_KEY;
    use ::auth::BEARER;


    #[test]
    fn test_login_works() {
        setup_warp(|_fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = routes(&s);

            let login = auth::LoginRequest {
                oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
                    .to_string(),
            };
            let resp = warp::test::request()
                .method("POST")
                .path("/api/auth/login")
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
            let filter = routes(&s);

            let jwt = get_jwt(filter.clone());

            let resp = warp::test::request()
                .method("GET")
                .path("/api/auth/user")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            assert_eq!(resp.status(), 200);

            let user: User = deserialize(resp);
            assert_eq!(user, fixture.user)
        });
    }

    mod events {
        use super::*;
        use chrono::Datelike;

        #[test]
        fn create_event() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let event: Event = deserialize(resp);
                assert_eq!(event.title, request.title)
            });
        }

        #[test]
        fn get_events() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let resp = warp::test::request()
                    .method("GET")
                    .path("/api/calendar/event/events")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let events: Vec<Event> = deserialize(resp);
                assert_eq!(events.len(), 1);
                assert_eq!(&events[0].title, "Do a thing");
                assert_eq!(&events[0].text, "");
            });
        }

        #[test]
        fn get_events_today() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                // create another event in a week.
                let request = NewEventMessage {
                    title: "Do a thing a week from now".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::weeks(1),
                    stop_at: chrono::Utc::now().naive_utc()
                        + chrono::Duration::weeks(1)
                        + chrono::Duration::hours(1),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let resp = warp::test::request()
                    .method("GET")
                    .path("/api/calendar/event/events/today")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let events: Vec<Event> = deserialize(resp);
                assert_eq!(events.len(), 1);
                assert_eq!(&events[0].title, "Do a thing");
                assert_eq!(&events[0].text, "");
            });
        }

        #[test]
        fn get_events_this_month() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc().with_day0(1).unwrap()
                        + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc().with_day0(1).unwrap()
                        + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                // create another event in a week.
                let request = NewEventMessage {
                    title: "Do a thing a week from now".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc().with_day0(1).unwrap()
                        + chrono::Duration::weeks(1),
                    stop_at: chrono::Utc::now().naive_utc().with_day0(1).unwrap()
                        + chrono::Duration::weeks(1)
                        + chrono::Duration::hours(1),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let resp = warp::test::request()
                    .method("GET")
                    .path("/api/calendar/event/events/month")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let events: Vec<Event> = deserialize(resp);
                assert_eq!(events.len(), 2);
                assert_eq!(&events[0].title, "Do a thing");
                assert_eq!(&events[1].title, "Do a thing a week from now");
            });
        }

        #[test]
        fn modify_event() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let event: Event = deserialize(resp);

                let request = EventChangeset {
                    uuid: event.uuid,
                    title: "Do another thing".to_string(),
                    text: "lol".to_string(),
                    start_at: event.start_at,
                    stop_at: event.stop_at,
                };

                let resp = warp::test::request()
                    .method("PUT")
                    .path("/api/calendar/event/events")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let event: Event = deserialize(resp);
                assert_eq!(&event.title, "Do another thing");
                assert_eq!(&event.text, "lol");
            });
        }

        #[test]
        fn delete_event() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let event: Event = deserialize(resp);

                let resp = warp::test::request()
                    .method("DELETE")
                    .path(&format!("/api/calendar/event/{}", event.uuid))
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let event: Event = deserialize(resp);
                assert_eq!(&event.title, "Do a thing");

                // verify it was deleted
                let resp = warp::test::request()
                    .method("GET")
                    .path("/api/calendar/event/events")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let events: Vec<Event> = deserialize(resp);
                assert_eq!(events.len(), 0);
            });
        }
    }

    mod market {
        use super::*;
        use crate::api::market::StockTransactionRequest;
        use db::stock::{Stock, StockTransaction, UserStockResponse};

        #[test]
        fn buy_stock() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                let request = StockTransactionRequest {
                    symbol: "APPL".to_string(),
                    quantity: 1,
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/market/stock/transact")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);
            });
        }

        #[test]
        fn owned_stocks() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                let request = StockTransactionRequest {
                    symbol: "APPL".to_string(),
                    quantity: 1,
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/market/stock/transact")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200, "could not buy stocks");

                let resp = warp::test::request()
                    .method("GET")
                    .path("/api/market/stock/")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200, "Could not find stocks for user");
                let r: Vec<UserStockResponse> = deserialize(resp);
                assert_eq!(1, r.len());
                assert_eq!(1, r[0].transactions.len());
                assert_eq!(1, r[0].transactions[0].quantity)
            });
        }
    }
}
