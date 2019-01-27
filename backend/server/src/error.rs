use super::*;
use std::{
    error::Error as StdError,
    fmt::{
        self,
        Display,
    },
};
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::Reply,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Error {
    DatabaseUnavailable,
    DatabaseError(Option<String>),
    InternalServerError,
    NotFound {
        type_name: String,
    },
    BadRequest,
    /// The used did not have privileges to access the given method.
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
    /// The user has been banned and therefore can't perform their desired action.
    UserBanned,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: String = match self {
            Error::DatabaseUnavailable => {
                "Could not acquire a connection to the database, the connection pool may be occupied".to_string()
            }
            Error::DatabaseError(e) => match e {
                Some(s) => s.clone(),
                None => "A problem occurred with the database".to_string(),
            },
            Error::IllegalToken => "The provided token is invalid".to_string(),
            Error::ExpiredToken => {
                "The provided token has expired, please reauthenticate to acquire a new one".to_string()
            }
            Error::MalformedToken => "The token was not formatted correctly".to_string(),
            Error::MissingToken => {
                "The Api route was expecting a JWT token and none was provided. Try logging in.".to_string()
            }
            Error::NotAuthorized { reason } => {
                format!("You are forbidden from accessing this resource. ({})", reason)
            }
            Error::UserBanned => "Your account has been banned".to_string(),
            Error::BadRequest => "Your request is malformed".to_string(),
            Error::InternalServerError => "Internal server error encountered".to_string(),
            Error::NotFound { type_name } => {
                format!("The resource ({})you requested could not be found", type_name)
            }
        };
        write!(f, "{}", description)
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        None
    }
}

/// Takes a rejection, which Warp would otherwise handle in its own way, and transform it into
/// an Ok(Reply) where the status is set to correspond to the provided error.
///
/// This only works if the Rejection is of the custom Error type. Any others will just fall through this unchanged.
///
/// This should be used at the top level of the exposed api.
pub fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    let mut resp = err.json();
    if err.is_not_found() {
        *resp.status_mut() = StatusCode::NOT_FOUND;
        return Ok(resp);
    }

    let cause = match err.find_cause::<Error>() {
        Some(ok) => ok,
        None => return Ok(resp),
    };
    match *cause {
        Error::DatabaseUnavailable => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        Error::DatabaseError(_) => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        Error::IllegalToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        Error::ExpiredToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        Error::MalformedToken => *resp.status_mut() = StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
        Error::NotAuthorized { .. } => *resp.status_mut() = StatusCode::FORBIDDEN, // Forbidden is for requests that will not served due to a lack of privileges
        Error::UserBanned => *resp.status_mut() = StatusCode::FORBIDDEN,
        Error::BadRequest => *resp.status_mut() = StatusCode::BAD_REQUEST,
        Error::NotFound { .. } => *resp.status_mut() = StatusCode::NOT_FOUND,
        Error::InternalServerError => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        Error::MissingToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
    }

    //        warn!("rewrote error response: {:?}", resp);
    Ok(resp)
}

impl Error {
    pub fn reject_result<T>(self) -> Result<T, Rejection> {
        Err(warp::reject::custom(self))
    }

    pub fn reject(self) -> Rejection {
        warp::reject::custom(self)
    }
}

