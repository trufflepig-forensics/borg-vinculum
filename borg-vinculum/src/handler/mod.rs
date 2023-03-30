//! All handlers of the servers are defined in this module

use std::fmt::{Display, Formatter};

use actix_toolbox::tb_middleware::actix_session;
use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use borgbackup::errors::ListError;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub mod api;
pub mod frontend;

/// The path parameter for an uuid
#[derive(Deserialize, IntoParams)]
pub struct PathUuid {
    uuid: Uuid,
}

/// The result that is used throughout the complete api.
pub type ApiResult<T> = Result<T, ApiError>;

/// The status code represents a unique identifier for an error.
///
/// Error codes in the range of 1000..2000 represent client errors
/// that could be handled by the client.
/// Error codes in the range of 2000..3000 represent server errors.
#[derive(Serialize_repr, ToSchema)]
#[repr(u16)]
pub(crate) enum ApiStatusCode {
    Unauthenticated = 1000,
    NotFound = 1001,
    InvalidContentType = 1002,
    InvalidJson = 1003,
    PayloadOverflow = 1004,

    LoginFailed = 1005,
    InvalidPassword = 1006,
    NameAlreadyExists = 1007,
    InvalidName = 1008,
    ListRepositoryError = 1009,
    RepositoryAlreadyExists = 1010,
    InvalidUuid = 1011,

    InternalServerError = 2000,
    DatabaseError = 2001,
    SessionError = 2002,
}

/// The Response that is returned in case of an error
///
/// For client errors the HTTP status code will be 400,
/// for server errors the 500 will be used.
#[derive(Serialize, ToSchema)]
pub(crate) struct ApiErrorResponse {
    #[schema(example = "Error message is here")]
    message: String,
    #[schema(example = 1000)]
    status_code: ApiStatusCode,
}

impl ApiErrorResponse {
    fn new(status_code: ApiStatusCode, message: String) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

/// This enum holds all possible error types that can occur in the API
#[derive(Debug)]
pub enum ApiError {
    /// The user is not allowed to access the resource
    Unauthenticated,
    /// The resource was not found
    NotFound,
    /// Invalid content type sent
    InvalidContentType,
    /// Json error
    InvalidJson(serde_json::Error),
    /// Payload overflow
    PayloadOverflow(String),

    /// Login was not successful. Can be caused by incorrect username / password
    LoginFailed,
    /// Invalid password (e.g. empty)
    InvalidPassword,
    /// There exists already an entity with the chosen name
    NameAlreadyExists,
    /// Invalid name specified
    InvalidName,
    /// Borg error while listing repositories
    ListRepositoryError(ListError),
    /// There exists already an entity with the chosen repository
    RepositoryAlreadyExists,
    /// An invalid uuid was specified
    InvalidUuid,

    /// Unknown error occurred
    InternalServerError,
    /// All errors that are thrown by the database
    DatabaseError(rorm::Error),
    /// An invalid hash is retrieved from the database
    InvalidHash(argon2::password_hash::Error),
    /// Error inserting into a session
    SessionInsert(actix_session::SessionInsertError),
    /// Error retrieving data from a session
    SessionGet(actix_session::SessionGetError),
    /// Session is in a corrupt state
    SessionCorrupt,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::LoginFailed => write!(f, "The login was not successful"),
            ApiError::DatabaseError(_) => write!(f, "Database error occurred"),
            ApiError::Unauthenticated => write!(f, "Unauthenticated"),
            ApiError::InvalidHash(_) => write!(f, "Internal server error"),
            ApiError::InternalServerError | ApiError::NotFound => {
                write!(f, "The resource was not found")
            }
            ApiError::InvalidContentType => write!(f, "Content type error"),
            ApiError::InvalidJson(err) => write!(f, "Json error: {err}"),
            ApiError::PayloadOverflow(err) => write!(f, "{err}"),
            ApiError::SessionInsert(_) | ApiError::SessionGet(_) => {
                write!(f, "Session error occurred")
            }
            ApiError::SessionCorrupt => write!(f, "Corrupt session"),
            ApiError::InvalidPassword => write!(f, "Invalid password"),
            ApiError::NameAlreadyExists => {
                write!(f, "There exists already an entity with that name")
            }
            ApiError::InvalidName => write!(f, "Invalid name specified"),
            ApiError::ListRepositoryError(err) => {
                write!(f, "Error while listing repository: {err}")
            }
            ApiError::RepositoryAlreadyExists => {
                write!(f, "There exists already an entity with that repository")
            }
            ApiError::InvalidUuid => write!(f, "Invalid uuid specified"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiError::SessionInsert(err) => {
                error!("Session insert error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::SessionGet(err) => {
                error!("Session get error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::Unauthenticated => {
                trace!("Unauthenticated");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::Unauthenticated,
                    self.to_string(),
                ))
            }
            ApiError::LoginFailed => {
                debug!("Login request failed");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::LoginFailed,
                    self.to_string(),
                ))
            }
            ApiError::DatabaseError(err) => {
                error!("Database error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::DatabaseError,
                    self.to_string(),
                ))
            }

            ApiError::InvalidHash(err) => {
                error!("Got invalid password hash from db: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::InternalServerError,
                    self.to_string(),
                ))
            }
            ApiError::InternalServerError => HttpResponse::InternalServerError().json(
                ApiErrorResponse::new(ApiStatusCode::InternalServerError, self.to_string()),
            ),
            ApiError::NotFound => {
                info!("Not found");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::NotFound,
                    self.to_string(),
                ))
            }
            ApiError::InvalidContentType => HttpResponse::BadRequest().json(ApiErrorResponse::new(
                ApiStatusCode::InvalidContentType,
                self.to_string(),
            )),
            ApiError::InvalidJson(err) => {
                debug!("Received invalid json: {err}");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::InvalidJson,
                    self.to_string(),
                ))
            }
            ApiError::PayloadOverflow(err) => {
                debug!("Payload overflow: {err}");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::PayloadOverflow,
                    self.to_string(),
                ))
            }
            ApiError::SessionCorrupt => {
                warn!("Corrupt session");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::InvalidPassword => {
                debug!("Invalid password specified");
                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::InvalidPassword,
                    self.to_string(),
                ))
            }
            ApiError::NameAlreadyExists => {
                debug!("Name already exists");
                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::NameAlreadyExists,
                    self.to_string(),
                ))
            }
            ApiError::InvalidName => {
                debug!("Invalid name specified");
                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::InvalidName,
                    self.to_string(),
                ))
            }
            ApiError::ListRepositoryError(err) => {
                info!("Error while listing repositories: {err}");
                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::ListRepositoryError,
                    self.to_string(),
                ))
            }
            ApiError::RepositoryAlreadyExists => {
                debug!("Repository already exists");
                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::RepositoryAlreadyExists,
                    self.to_string(),
                ))
            }
            ApiError::InvalidUuid => HttpResponse::BadRequest().json(ApiErrorResponse::new(
                ApiStatusCode::InvalidUuid,
                self.to_string(),
            )),
        }
    }
}

impl From<rorm::Error> for ApiError {
    fn from(value: rorm::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::InvalidHash(value)
    }
}

impl From<actix_session::SessionInsertError> for ApiError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        Self::SessionInsert(value)
    }
}

impl From<actix_session::SessionGetError> for ApiError {
    fn from(value: actix_session::SessionGetError) -> Self {
        Self::SessionGet(value)
    }
}

impl From<ListError> for ApiError {
    fn from(value: ListError) -> Self {
        Self::ListRepositoryError(value)
    }
}
