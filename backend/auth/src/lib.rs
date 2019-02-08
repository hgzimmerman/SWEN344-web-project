//use crate::state::State;
use warp::{filters::BoxedFilter, Filter, Rejection};

//use crate::error::Error;
use chrono::{Duration, NaiveDateTime};
use frank_jwt::{decode, encode, Algorithm};
use serde::{Deserialize, Serialize};
use serde_json::json;
//use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthError {
    NotAuthorized {
        reason: &'static str,
    },
    /// Used to indicate that the signature does not match the hashed contents + secret
    IllegalToken,
    /// The expired field in the token is in the past
    ExpiredToken,
    /// The request did not have a token.
    MissingToken,
    /// The JWT 'bearer schema' was not followed.
    MalformedToken,
    DeserializeError,
    SerializeError,
    JwtDecodeError,
    JwtEncodeError,
}

/// The payload section of the JWT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtPayload<T> {
    /// Issue date
    pub iat: NaiveDateTime,
    /// Subject (the user being authenticated by this token)
    pub sub: T,
    /// Expire date
    pub exp: NaiveDateTime,
}

impl <T> JwtPayload<T>
where
    for<'de> T: Serialize + Deserialize<'de> + Send
{
    /// Creates a new token for the user that will expire in 4 weeks.
    pub fn new(subject: T) -> Self {
        let now = chrono::Utc::now().naive_utc();

        JwtPayload {
            iat: now,
            sub: subject,
            exp: now + Duration::weeks(4), // token will expire in 4 weeks
        }
    }

    pub fn subject(self) -> T {
        self.sub
    }

    /// Validates if the token is expired or not.
    /// It also checks if the token was issued in the future, to further complicate the attack surface of someone creating forgeries.
    pub fn validate_dates(self) -> Result<Self, AuthError> {
        let now = chrono::Utc::now().naive_utc();
        if self.exp < now || self.iat > now {
            Err(AuthError::ExpiredToken)
        } else {
            Ok(self)
        }
    }

    /// Encodes the payload, producing a JWT in string form.
    pub fn encode_jwt_string(&self, secret: &Secret) -> Result<String, AuthError> {
        let header = json!({});
        use serde_json::Value;

        let secret: &String = &secret.0;

        let payload: Value = match serde_json::to_value(&self) {
            Ok(x) => x,
            Err(_) => return Err(AuthError::SerializeError),
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => Ok(x),
            Err(_) => Err(AuthError::JwtEncodeError),
        }
    }

    /// Decodes the JWT into its payload.
    /// If the signature doesn't match, then a decode error is thrown.
    pub fn decode_jwt_string(jwt_str: &str, secret: &Secret) -> Result<JwtPayload<T>, AuthError> {
        let secret: &String = &secret.0;
        let (_header, payload) = match decode(&jwt_str.to_string(), secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(_) => return Err(AuthError::JwtDecodeError),
        };
        let jwt: JwtPayload<T> = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(AuthError::DeserializeError),
        };
        Ok(jwt)
    }


    /// Removes the jwt from the bearer string, and decodes it to determine if it was signed properly.
    pub fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<JwtPayload<T>, AuthError> {
        let authorization_words: Vec<String> =
            bearer_string.split_whitespace().map(String::from).collect();

        if authorization_words.len() != 2 {
            return Err(AuthError::MissingToken);
        }
        if authorization_words[0] != BEARER {
            return Err(AuthError::MalformedToken);
        }
        let jwt_str: &str = &authorization_words[1];

        JwtPayload::decode_jwt_string(jwt_str, secret).map_err(|_| AuthError::IllegalToken)
    }

}

/// A string that acts like a key to validate JWT signatures.
#[derive(Clone, Debug)]
pub struct Secret(String);

impl Secret {
    pub fn new(s: &str) -> Self {
        Secret(s.to_string())
    }
}

pub const BEARER: &str = "bearer";
pub const AUTHORIZATION_HEADER_KEY: &str = "Authorization";


/// This filter will attempt to extract the JWT bearer token from the header Authorization field.
/// It will then attempt to transform the JWT into a usable JwtPayload that can be used by the app.
//pub fn jwt_filter<T>(s: BoxedFilter<(Secret,)>) -> BoxedFilter<(JwtPayload<T>,)>
//where
//    for <'de> T: Serialize + Deserialize<'de> + Send
//{
//    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
////        .or_else(|_: Rejection| {
////            AuthError::NotAuthorized {
////                reason: "token required",
////            }
////            .reject_result()
////        })
//        .and(s.clone())
//        .and_then(|bearer_string, secret| {
//            JwtPayload::extract_jwt(bearer_string, &secret)
//                .and_then(JwtPayload::validate_dates)
////                .map_err(AuthError::reject)
//        })
//        .boxed()
//}

/// Brings the secret into scope.
/// The secret is used to create and verify JWTs.
pub fn secret_filter(secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any().map(move || secret.clone()).boxed()
}

///// If the user has a JWT, then the user has basic user privileges.
//pub fn user_filter(s: &State) -> BoxedFilter<(Uuid,)> {
//    warp::any().and(jwt_filter(s)).map(JwtPayload::subject).boxed()
//}
//
//#[allow(dead_code)]
///// Gets an Option<UserUuid> from the request.
///// Returns Some(user_uuid) if the user has a valid JWT, and None otherwise.
//pub fn optional_user_filter(s: &State) -> BoxedFilter<(Option<Uuid>,)> {
//    user_filter(s)
//        .map(Some)
//        .or(warp::any().map(|| None))
//        .unify::<(Option<Uuid>,)>()
//        .boxed()
//}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::state::StateConfig;

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
