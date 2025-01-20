use std::io;

#[derive(Debug)]
pub enum GTError {
    Sqlx(sqlx::Error),
    Io(io::Error),
}

impl From<sqlx::Error> for GTError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<io::Error> for GTError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
