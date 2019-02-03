use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Rejection;

use crate::error::Error;
use chrono::Duration;
use chrono::NaiveDateTime;
use frank_jwt::{decode, encode, Algorithm};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

/// The payload section of the JWT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    /// Issue date
    pub iat: NaiveDateTime,
    /// Subject (the user being authenticated by this token)
    pub sub: Uuid,
    /// Expire date
    pub exp: NaiveDateTime,
}

impl JwtPayload {
    /// Creates a new token for the user that will expire in 4 weeks.
    pub fn new(user_uuid: Uuid) -> Self {
        let now = chrono::Utc::now().naive_utc();

        JwtPayload {
            iat: now,
            sub: user_uuid,
            exp: now + Duration::weeks(4), // token will expire in 4 weeks
        }
    }

    pub fn uuid(self) -> Uuid {
        self.sub
    }

    /// Validates if the token is expired or not.
    /// It also checks if the token was issued in the future, to further complicate the attack surface of someone creating forgeries.
    pub fn validate_dates(self) -> Result<Self, Error> {
        let now = chrono::Utc::now().naive_utc();
        if self.exp < now || self.iat > now {
            Err(Error::ExpiredToken)
        } else {
            Ok(self)
        }
    }

    /// Encodes the payload, producing a JWT in string form.
    pub fn encode_jwt_string(&self, secret: &Secret) -> Result<String, Error> {
        let header = json!({});
        use serde_json::Value;

        let secret: &String = &secret.0;

        let payload: Value = match serde_json::to_value(&self) {
            Ok(x) => x,
            Err(_) => return Err(Error::SerializeError),
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => return Ok(x),
            Err(_) => return Err(Error::JwtEncodeError),
        }
    }

    /// Decodes the JWT into its payload.
    /// If the signature doesn't match, then a decode error is thrown.
    pub fn decode_jwt_string(jwt_str: &str, secret: &Secret) -> Result<JwtPayload, Error> {
        let secret: &String = &secret.0;
        let (_header, payload) = match decode(&jwt_str.to_string(), secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(_) => return Err(Error::JwtDecodeError),
        };
        let jwt: JwtPayload = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(Error::DeserializeError),
        };
        Ok(jwt)
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

pub const BEARER: &'static str = "bearer";
pub const AUTHORIZATION_HEADER_KEY: &'static str = "Authorization";

/// Removes the jwt from the bearer string, and decodes it to determine if it was signed properly.
fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<JwtPayload, Error> {
    let authorization_words: Vec<String> =
        bearer_string.split_whitespace().map(String::from).collect();

    if authorization_words.len() != 2 {
        return Err(Error::MissingToken);
    }
    if authorization_words[0] != BEARER {
        return Err(Error::MalformedToken);
    }
    let jwt_str: &str = &authorization_words[1];

    JwtPayload::decode_jwt_string(jwt_str, secret).map_err(|_| Error::IllegalToken)
}

/// This filter will attempt to extract the JWT bearer token from the header Authorization field.
/// It will then attempt to transform the JWT into a usable JwtPayload that can be used by the app.
pub fn jwt_filter(s: &State) -> BoxedFilter<(JwtPayload,)> {
    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
        .or_else(|_: Rejection| {
            Error::NotAuthorized {
                reason: "token required",
            }
            .reject_result()
        })
        .and(s.secret.clone())
        .and_then(|bearer_string, secret| {
            extract_jwt(bearer_string, &secret)
                .and_then(JwtPayload::validate_dates)
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
    warp::any().and(jwt_filter(s)).map(JwtPayload::uuid).boxed()
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
