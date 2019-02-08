//! This is a crate for wrapping common JWT functionality needed for securing information in a webapp.
//! It is flexible in that it can support arbitrary payload subjects.

use warp::{filters::BoxedFilter, Filter, Rejection};
use chrono::{Duration, NaiveDateTime};
use frank_jwt::{decode, encode, Algorithm};
use serde::{Deserialize, Serialize};
use serde_json::json;


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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    /// Creates a new token for the user that will expire after a specified time.
    ///
    /// # Arguments
    /// - subject The subject of the JWT, it holds the contents that should be trusted by the server on return trips.
    /// - lifetime How long the JWT will be valid for after its creation.
    pub fn new(subject: T, lifetime: Duration) -> Self {
        let now = chrono::Utc::now().naive_utc();

        JwtPayload {
            iat: now,
            sub: subject,
            exp: now + lifetime,
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

/// Brings the secret into scope.
/// The secret is used to create and verify JWTs.
pub fn secret_filter(secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any().map(move || secret.clone()).boxed()
}



#[cfg(test)]
mod test {
    use super::*;

    /// Tests if a jwt payload can be encoded and then decoded.
    #[test]
    fn encode_decode() {
        let payload = JwtPayload::new("hello_there".to_string(), Duration::weeks(2));
        let secret = Secret::new("secret");

        let encoded = payload.encode_jwt_string(&secret).unwrap();
        let decoded = JwtPayload::<String>::decode_jwt_string(&encoded, &secret).unwrap();

        assert_eq!(decoded, payload)
    }

    /// Tests if a jwt can be extracted from a bearer string.
    #[test]
    fn encode_extract() {
        let payload = JwtPayload::new("hello_there".to_string(), Duration::weeks(2));
        let secret = Secret::new("secret");
        let encoded = payload.encode_jwt_string(&secret).unwrap();
        let header_string = format!("{} {}", BEARER, encoded);

        let decoded = JwtPayload::<String>::extract_jwt(header_string, &secret).unwrap();
        assert_eq!(decoded, payload)
    }

}