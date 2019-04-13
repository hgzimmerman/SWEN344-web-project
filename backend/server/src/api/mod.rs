//! The api defines all of the routes that are supported for the server.
mod advertisement;
pub(crate) mod auth;
mod calendar;
mod market;
mod user;
mod twitter_proxy;

use warp::{filters::BoxedFilter, Reply};

use warp::{path, Filter};

use self::calendar::calendar_api;
use crate::{
    api::{
        advertisement::{ad_api, health_api},
        auth::auth_api,
        market::market_api,
        user::user_api
    },
    static_files::{static_files_handler, FileConfig},
    state::State,
    api::twitter_proxy::twitter_proxy_api
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
                .or(ad_api(state))
                .or(health_api(state))
                .or(user_api(state))
                .or(twitter_proxy_api(state)),
        )
        .boxed()
}

/// A filter that is responsible for configuring everything that can be served.
///
/// # Notes
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
    use testing_common::setup::setup_warp;

    use crate::{
        api::{calendar::NewEventRequest},
        testing_fixtures::util::{deserialize, deserialize_string},
    };
    use db::{
        event::{Event, EventChangeset},
        user::User,
    };

    use crate::api::auth::get_jwt;
    use authorization::{Secret, AUTHORIZATION_HEADER_KEY, BEARER};

//    #[test]
//    fn test_login_works() {
//        setup_warp(|_fixture: &UserFixture, pool: Pool| {
//            let secret = Secret::new("test");
//            let s = State::testing_init(pool, secret);
//            let filter = routes(&s);
//
//            let login = LoginRequest {
//                oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
//                    .to_string(),
//            };
//            let resp = warp::test::request()
//                .method("POST")
//                .path("/api/auth/login")
//                .json(&login)
//                .header("content-length", "300")
//                .reply(&filter);
//
//            assert_eq!(resp.status(), 200)
//        });
//    }

    #[test]
    fn user_works() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let secret = Secret::new("test");
            let s = State::testing_init(pool, secret);
            let filter = routes(&s);

            let jwt = get_jwt(&s);

            let resp = warp::test::request()
                .method("GET")
                .path("/api/user")
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
        use chrono::NaiveDateTime;

        #[test]
        fn create_event() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(&s);

                let request = NewEventRequest {
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

                let jwt = get_jwt(&s);


                // create an event first.
                let request = NewEventRequest {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let start = chrono::Utc::now().naive_utc();
                let stop = start + chrono::Duration::hours(4);

                use serde::Serialize;
                #[derive(Serialize)]
                struct TimeBounds {
                    start: NaiveDateTime,
                    stop: NaiveDateTime
                }

                let tb = TimeBounds {
                    start,
                    stop
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
                    .path(&format!("/api/calendar/event/events?{}", serde_urlencoded::to_string(tb).unwrap()) )
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(resp.status(), 200);

                let events: Vec<Event> = deserialize(resp);
                assert_eq!(events.len(), 1);
                assert_eq!(&events[0].title, "Do a thing");
                assert_eq!(&events[0].text, "");
            });
        }

        // TODO This test is dumb because it will fail or pass depending on when the test is run.
        // TODO delete it and its associated backing api
        #[test]
        fn get_events_today() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(&s);

                // create an event first.
                let request = NewEventRequest {
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
                let request = NewEventRequest {
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

                let jwt = get_jwt(&s);

                // create an event first.
                let request = NewEventRequest {
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
                let request = NewEventRequest {
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

                let jwt = get_jwt(&s);

                // create an event first.
                let request = NewEventRequest {
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

                let jwt = get_jwt(&s);

                // create an event first.
                let request = NewEventRequest {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
                };

                let start = chrono::Utc::now().naive_utc();
                let stop = start + chrono::Duration::hours(4);


                use serde::Serialize;
                #[derive(Serialize)]
                struct TimeBounds {
                    start: NaiveDateTime,
                    stop: NaiveDateTime
                }

                let tb = TimeBounds {
                    start,
                    stop
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
                    .path(&format!("/api/calendar/event/events?{}", serde_urlencoded::to_string(tb).unwrap()) )
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

                let jwt = get_jwt(&s);

                let request = StockTransactionRequest {
                    symbol: "AAPL".to_string(),
                    quantity: 1,
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/market/stock/transact")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                dbg!(&resp.body());
                assert_eq!(resp.status(), 200);
            });
        }

        #[test]
        fn owned_stocks() {
            setup_warp(|_fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(&s);

                let request = StockTransactionRequest {
                    symbol: "AAPL".to_string(),
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
