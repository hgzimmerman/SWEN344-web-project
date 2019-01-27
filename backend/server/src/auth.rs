use warp::Rejection;
use warp::filters::BoxedFilter;
use crate::state::State;
use warp::Filter;

use crate::error::Error;
use uuid::Uuid;
use serde::Serialize;
use serde::Deserialize;
use serde_json::json;
use frank_jwt::{
    decode,
    encode,
    Algorithm,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    claim: Uuid
}

impl JwtPayload {

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

#[derive(Clone, Debug)]
pub struct Secret(String);

impl Secret {
    pub fn new(s: &str) -> Self {
        Secret(s.to_string())
    }
}

const BEARER: &'static str = "bearer ";
const AUTHORIZATION_HEADER_KEY: &'static str = "Authorization";


/// Removes the jwt from the bearer string, and decodes it to determine if it was signed properly.
fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<JwtPayload, Error> {
    let authorization_words: Vec<String> = bearer_string.split_whitespace().map(String::from).collect();

    if authorization_words.len() != 2 {
        return Err(Error::MalformedToken);
    }
    if authorization_words[0] != BEARER {
        return Err(Error::MalformedToken);
    }
    let jwt_str: &str = &authorization_words[1];

    JwtPayload::decode_jwt_string(jwt_str, secret).map_err(|_| Error::IllegalToken)
}

pub fn jwt_filter(s: &State) -> BoxedFilter<(JwtPayload,)> {
    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
        .or_else(|_: Rejection| Error::MalformedToken.reject_result())
        .and(s.secret.clone())
        .and_then(|bearer_string, secret| extract_jwt(bearer_string, &secret).map_err(Error::reject))
        .boxed()
}

/// Brings the secret into scope.
pub fn secret_filter(secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any()
        .map(move || secret.clone())
        .boxed()
}
