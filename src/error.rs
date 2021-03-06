use std::convert::From;
use r2d2;
use diesel;
use grpcio::{RpcStatus, RpcStatusCode};
use rocket::response::Responder;
use rocket::request::Request;
use rocket::response::Response;
use rocket::http::Status;
use serde_json::Error as JsonError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Database cannot be reached")]
    DatabaseUnreachable,
    #[fail(display = "Internal server error")]
    Internal,
    #[fail(display = "Invalid input")]
    InvalidInput,
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Resource already exists")]
    AlreadyExists,
    #[fail(display = "Database error: {}", inner)]
    Db { inner: diesel::result::Error },
}

impl From<r2d2::Error> for Error {
    fn from(_err: r2d2::Error) -> Error {
        Error::DatabaseUnreachable
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        match err {
            diesel::result::Error::NotFound => Error::NotFound,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _info,
            ) => Error::AlreadyExists,
            err => Error::Db { inner: err },
        }
    }
}

impl From<JsonError> for Error {
    fn from(_err: JsonError) -> Error {
        Error::Internal
    }
}

impl From<::uuid::ParseError> for Error {
    fn from(_err: ::uuid::ParseError) -> Error {
        Error::NotFound
    }
}

impl Error {
    pub fn as_grpc(&self) -> RpcStatus {
        use self::Error::*;
        match *self {
            DatabaseUnreachable => {
                RpcStatus::new(RpcStatusCode::Internal, Some(format!("{}", &self)))
            }
            InvalidInput => {
                RpcStatus::new(RpcStatusCode::InvalidArgument, Some(format!("{}", &self)))
            }
            AlreadyExists => {
                RpcStatus::new(RpcStatusCode::AlreadyExists, Some(format!("{}", &self)))
            }
            NotFound => RpcStatus::new(RpcStatusCode::NotFound, None),
            Db { inner: _ } | Internal => {
                RpcStatus::new(RpcStatusCode::Internal, Some(format!("{}", &self)))
            }
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        use self::Error::*;

        match self {
            InvalidInput | AlreadyExists => Err(Status::UnprocessableEntity),
            DatabaseUnreachable => Err(Status::ServiceUnavailable),
            NotFound => Err(Status::NotFound),
            Db { inner: _ } | Internal => Err(Status::InternalServerError),
        }
    }
}
