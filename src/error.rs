use std::io::{self};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Sqlx(sqlx::Error),
    Str(String),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match &value {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    return Error::Str(String::from("Unique Constraint Violated"));
                } else if db_err.is_foreign_key_violation() {
                    return Error::Str(String::from("Foreign Key Constraint Violated"));
                }

                Error::Sqlx(value)
            }
            _ => Error::Sqlx(value),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Str(value)
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::Io(error) => error.to_string(),
            Error::Sqlx(error) => error.to_string(),
            Error::Str(error) => error.to_owned(),
        }
    }
}
