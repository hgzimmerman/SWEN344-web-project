use auth::{
    Secret,
    AuthError
};
use warp::filters::BoxedFilter;
use crate::state::State;
use serde::Serialize;
use serde::Deserialize;
use warp::Rejection;
use crate::error::Error;
use uuid::Uuid;
use warp::Filter;
use auth::JwtPayload;
use auth::AUTHORIZATION_HEADER_KEY;


/// This filter will attempt to extract the JWT bearer token from the header Authorization field.
/// It will then attempt to transform the JWT into a usable JwtPayload that can be used by the app.
pub fn jwt_filter<T>(s: &State) -> BoxedFilter<(JwtPayload<T>,)>
where
    for <'de> T: Serialize + Deserialize<'de> + Send
{
    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
        .or_else(|_: Rejection| {
            Error::AuthError(AuthError::NotAuthorized {
                reason: "token required",
            })
            .reject_result()
        })
        .and(s.secret.clone())
        .and_then(|bearer_string, secret| {
            JwtPayload::extract_jwt(bearer_string, &secret)
                .and_then(JwtPayload::validate_dates)
                .map_err(Error::AuthError)
                .map_err(Error::reject)
        })
        .boxed()
}

/// Brings the secret into scope.
/// The secret is used to create and verify JWTs.
pub fn secret_filter(secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any().map(move || secret.clone()).boxed()
}

/// If the user has a JWT, then the user has basic user privileges.
pub fn user_filter(s: &State) -> BoxedFilter<(Uuid,)> {
    warp::any().and(jwt_filter(s)).map(JwtPayload::subject).boxed()
}

#[allow(dead_code)]
/// Gets an Option<UserUuid> from the request.
/// Returns Some(user_uuid) if the user has a valid JWT, and None otherwise.
pub fn optional_user_filter(s: &State) -> BoxedFilter<(Option<Uuid>,)> {
    user_filter(s)
        .map(Some)
        .or(warp::any().map(|| None))
        .unify::<(Option<Uuid>,)>()
        .boxed()
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::state::StateConfig;
    use auth::BEARER;

    #[test]
    fn pass_filter() {
        let secret = Secret::new("yeet");
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None
        };
        let state = State::new(conf);
        let uuid = Uuid::new_v4();
        let jwt = JwtPayload::new(uuid);
        let jwt = jwt.encode_jwt_string(&secret).unwrap();

        let filter = jwt_filter(&state);

        assert!(
            warp::test::request()
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .matches(&filter)
        )
    }

    #[test]
    fn does_not_pass_filter() {
        let secret = Secret::new("yeet");
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None
        };

        let state = State::new(conf);
        let filter = jwt_filter(&state);
        assert!(
            !warp::test::request()
                .matches(&filter)
        )
    }

    #[test]
    fn extract_jwt() {
        let secret = Secret::new("yeet");
        let uuid = Uuid::new_v4();

        let jwt = JwtPayload::new(uuid);
        let jwt =  jwt.encode_jwt_string(&secret).expect("should encode jwt.");

        assert_eq!(uuid, JwtPayload::decode_jwt_string(&jwt, &secret).expect("should decode jwt").sub)
    }

}
