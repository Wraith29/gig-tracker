use std::io::{self};

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    Io(io::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sqlx(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}
