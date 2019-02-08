//use super::*;
use apply::Apply;
use serde::Serialize;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

//use log::info;
use log::error;

/// Server-wide error variants.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Error {
    /// The database could not be reached, or otherwise is experiencing troubles running queries.
    DatabaseUnavailable,
    /// The database encountered an error while running a query.
    DatabaseError(Option<String>),
    /// If the server needs to talk to an external API to properly serve a request,
    /// and that server experiences an error, this is the error to represent that.
    DependentConnectionFailed {
        url: String,
    },
    /// The server encountered an unspecified error.
    InternalServerError,
    /// The requested entity could not be located.
    NotFound {
        type_name: String,
    },
    /// The request was bad.
    BadRequest,
    /// The used did not have privileges to access the given method.
    /// This can also be used for users that don't have a token, but it is required.
    // TODO the above use is out of date
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
            Error::BadRequest => "Your request is malformed".to_string(),
            Error::InternalServerError => "Internal server error encountered".to_string(),
            Error::DependentConnectionFailed{url} => format!("An internal request needed to serve the request failed. URL: {}",url),
            Error::NotFound { type_name } => {
                format!("The resource ({})you requested could not be found", type_name)
            }
            Error::DeserializeError => "Something could not be deserialized".to_string(),
            Error::SerializeError => "Something could not be serialized".to_string(),
            Error::JwtDecodeError => "JWT could not be decoded".to_string(),
            Error::JwtEncodeError => "JWT could not be encoded".to_string(),
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
    let not_found = Error::NotFound {
        type_name: "route not found".to_string(),
    };
    let internal_server = Error::InternalServerError;

    let cause = match err.find_cause::<Error>() {
        Some(ok) => ok,
        None => {
            if err.is_not_found() {
                &not_found
            } else {
                &internal_server
            }
        }
    };

    use std::fmt::Write;
    let mut s: String = String::new();
    let _ = write!(s, "{}", cause);

    let error_response = ErrorResponse { message: s };
    let json = warp::reply::json(&error_response);

    let code = match *cause {
        Error::DatabaseUnavailable => StatusCode::INTERNAL_SERVER_ERROR,
        Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::IllegalToken => StatusCode::UNAUTHORIZED,
        Error::ExpiredToken => StatusCode::UNAUTHORIZED,
        Error::MalformedToken => StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
        Error::NotAuthorized { .. } => StatusCode::FORBIDDEN, // Forbidden is for requests that will not served due to a lack of privileges
        Error::BadRequest => StatusCode::BAD_REQUEST,
        Error::NotFound { .. } => StatusCode::NOT_FOUND,
        Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        Error::DependentConnectionFailed { .. } => StatusCode::BAD_GATEWAY,
        Error::MissingToken => StatusCode::UNAUTHORIZED,
        Error::DeserializeError => StatusCode::INTERNAL_SERVER_ERROR,
        Error::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
        Error::JwtDecodeError => StatusCode::UNAUTHORIZED,
        Error::JwtEncodeError => StatusCode::INTERNAL_SERVER_ERROR,
    };

    Ok(warp::reply::with_status(json, code))
}

impl Error {
    pub fn reject_result<T>(self) -> Result<T, Rejection> {
        Err(warp::reject::custom(self))
    }

    pub fn reject(self) -> Rejection {
        warp::reject::custom(self)
    }

    pub fn from_reject(error: diesel::result::Error) -> Rejection {
        Error::from(error).apply(Self::reject)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        use self::Error::*;
        use diesel::result::Error as DieselError;
        match error {
            DieselError::DatabaseError(_, _) => DatabaseError(None), // todo flesh this one out a bit
            DieselError::NotFound => NotFound {
                type_name: "not implemented".to_string(),
            },
            e => {
                error!("{}", e);
                InternalServerError
            }
        }
    }
}

/// Error response template for when the errors are rewritten.
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
