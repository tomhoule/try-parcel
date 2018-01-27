use std::convert::From;
use r2d2;
use diesel;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display="Database cannot be reached")]
    DatabaseUnreachable,
    #[fail(display="Invalid input")]
    InvalidInput,
    #[fail(display="Not found")]
    NotFound,
    #[fail(display="Database error: {}", inner)]
    Db { inner: diesel::result::Error }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::DatabaseUnreachable
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        Error::Db { inner: err }
    }
}