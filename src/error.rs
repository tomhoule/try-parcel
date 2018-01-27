#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display="Database cannot be reached")]
    DatabaseUnreachable,
    #[fail(display="Invalid input")]
    InvalidInput,
    #[fail(display="Not found")]
    NotFound,
}