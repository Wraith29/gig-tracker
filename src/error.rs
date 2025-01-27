use std::io::{self};

#[derive(Debug)]
pub enum Error {
    Sqlx(()),
    Io(()),
}

impl From<sqlx::Error> for Error {
    fn from(_value: sqlx::Error) -> Self {
        Error::Sqlx(())
    }
}

impl From<io::Error> for Error {
    fn from(_value: io::Error) -> Self {
        Error::Io(())
    }
}
