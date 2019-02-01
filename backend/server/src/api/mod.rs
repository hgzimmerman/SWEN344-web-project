
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
    use crate::api::calendar::NewEventMessage;
    use db::event::Event;
    use db::event::EventChangeset;

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

    mod events {
        use super::*;

        #[test]
        fn create_event() {
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
                };

                let resp = warp::test::request()
                    .method("POST")
                    .path("/api/calendar/event")
                    .json(&request)
                    .header("content-length", "500")
                    .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                    .reply(&filter);

                assert_eq!(
                    resp.status(),
                    200
                );

                let event: Event = deserialize(resp);
                assert_eq!(
                    event.title,
                    request.title
                )
            });
        }


        #[test]
        fn get_events() {
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
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
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
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
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::weeks(1) + chrono::Duration::hours(1)
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
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
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
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::weeks(1) + chrono::Duration::hours(1)
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
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
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
                    stop_at: event.stop_at
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
            setup_warp(|fixture: &UserFixture, pool: Pool| {
                let secret = Secret::new("test");
                let s = State::testing_init(pool, secret);
                let filter = routes(&s);

                let jwt = get_jwt(filter.clone());

                // create an event first.
                let request = NewEventMessage {
                    title: "Do a thing".to_string(),
                    text: "".to_string(),
                    start_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
                    stop_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2)
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

}
