//! Responsible for enumerating all the possible ways the server may encounter undesired states.
//!
//! It handles serializing these errors so that they can be consumed by the user of the api.
//!
//! Currently, this is tightly coupled to both Warp, Diesel, and Authorization.
//! It likely should be broken off into its own crate with the warp-related functions allowed as a feature.
use apply::Apply;
use authorization::AuthError;
use diesel::result::DatabaseErrorKind;
use log::error;
use serde::Serialize;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

/// Server-wide error variants.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Error {
    /// The database could not be reached, or otherwise is experiencing troubles running queries.
    DatabaseUnavailable,
    /// The database encountered an error while running a query.
    DatabaseError(String),
    /// If the server needs to talk to an external API to properly serve a request,
    /// and that server experiences an error, this is the error to represent that.
    DependentConnectionFailed {
        url: String,
    },
    /// The server encountered an unspecified error.
    InternalServerError(Option<String>),
    /// The requested entity could not be located.
    NotFound {
        type_name: String,
    },
    /// The request was bad, with a dynamic reason.
    BadRequest(String),
    /// An error in authorization
    AuthError(AuthError),
    /// The user does not have access to a particular resource.
    NotAuthorized {
        reason: &'static str,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: String = match self {
            Error::DatabaseUnavailable => {
                "Could not acquire a connection to the database, the connection pool may be occupied".to_string()
            }
            Error::DatabaseError(e) => e.to_string(),
            Error::BadRequest(s)=> s.to_string(),
            Error::InternalServerError(s) => {
                if let Some(e) = s {
                    e.clone()
                } else {
                    "Internal server error encountered".to_string()
                }
            },
            Error::DependentConnectionFailed{url} => format!("An internal request needed to serve the request failed. URL: {}",url),
            Error::NotFound { type_name } => {
                format!("The resource ({}) you requested could not be found", type_name)
            }
            Error::AuthError(auth_error) => match auth_error {
                AuthError::DeserializeError => "Something could not be deserialized".to_string(),
                AuthError::SerializeError => "Something could not be serialized".to_string(),
                AuthError::JwtDecodeError => "JWT could not be decoded".to_string(),
                AuthError::JwtEncodeError => "JWT could not be encoded".to_string(),
                AuthError::IllegalToken => "The provided token is invalid".to_string(),
                AuthError::ExpiredToken => {
                    "The provided token has expired, please reauthenticate to acquire a new one".to_string()
                }
                AuthError::MalformedToken => "The token was not formatted correctly".to_string(),
                AuthError::MissingToken => {
                    "The Api route was expecting a JWT token and none was provided. Try logging in.".to_string()
                }

            }
            Error::NotAuthorized { reason } => {
                format!("You are forbidden from accessing this resource. ({})", reason)
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
    let not_found = Error::NotFound {
        type_name: "route not found".to_string(),
    };
    let internal_server = Error::InternalServerError(None);

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
    write!(s, "{}", cause).map_err(|_| Error::InternalServerError(None).reject())?;

    let error_response = ErrorResponse { message: s };
    let json = warp::reply::json(&error_response);

    let code: StatusCode = cause.error_code();
    Ok(warp::reply::with_status(json, code))
}


impl Error {

    /// Get the error code correlated with the status code.
    fn error_code(&self) -> StatusCode {
        match *self {
            Error::DatabaseUnavailable => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DependentConnectionFailed { .. } => StatusCode::BAD_GATEWAY,
            Error::AuthError(ref auth_error) => {
                match *auth_error {
                    AuthError::IllegalToken => StatusCode::UNAUTHORIZED,
                    AuthError::ExpiredToken => StatusCode::UNAUTHORIZED,
                    AuthError::MalformedToken => StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
                    AuthError::MissingToken => StatusCode::UNAUTHORIZED,
                    AuthError::DeserializeError => StatusCode::INTERNAL_SERVER_ERROR,
                    AuthError::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
                    AuthError::JwtDecodeError => StatusCode::UNAUTHORIZED,
                    AuthError::JwtEncodeError => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            Error::NotAuthorized { .. } => StatusCode::FORBIDDEN, // Forbidden is for requests that will not served due to a lack of privileges, eg resource does not belong to a user.
        }
    }

    pub fn reject_result<T>(self) -> Result<T, Rejection> {
        Err(warp::reject::custom(self))
    }

    pub fn reject(self) -> Rejection {
        warp::reject::custom(self)
    }

    pub fn from_reject(error: diesel::result::Error) -> Rejection {
        Error::from(error).apply(Self::reject)
    }

    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Error::BadRequest(message.into())
    }
    /// Construct an internal error with a custom message.
    pub fn internal_server_error<T: Into<String>>(reason: T) -> Self {
        Error::InternalServerError(Some(reason.into()))
    }

    /// Construct a generic internal error.
    #[allow(dead_code)]
    pub fn internal_server_error_empty() -> Self {
        Error::InternalServerError(None)
    }

    /// Construct a not found error with the name of the type that could not be found.
    #[allow(dead_code)]
    pub fn not_found<T: Into<String>>(type_name: T) -> Self {
        Error::NotFound {type_name: type_name.into()}
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        use self::Error::*;
        use diesel::result::Error as DieselError;
        match error {
            DieselError::DatabaseError(e, _) => {
                let e = match e {
                    DatabaseErrorKind::ForeignKeyViolation => {
                        "A foreign key constraint was violated in the database"
                    }
                    DatabaseErrorKind::SerializationFailure => {
                        "Value failed to serialize in the database"
                    }
                    DatabaseErrorKind::UnableToSendCommand => {
                        "Database Protocol violation, possibly too many bound parameters"
                    }
                    DatabaseErrorKind::UniqueViolation => {
                        "A unique constraint was violated in the database"
                    }
                    DatabaseErrorKind::__Unknown => "An unknown error occurred in the database",
                }
                .to_string();
                DatabaseError(e)
            }
            DieselError::NotFound => NotFound {
                type_name: "not implemented".to_string(),
            },
            e => {
                error!("{}", e);
                InternalServerError(None)
            }
        }
    }
}

/// Error response template for when the errors are rewritten.
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
