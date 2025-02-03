//! Errors raised within the `dr` ecosystem
use std::{env::VarError, error::Error as E, fmt::Display};
use url::ParseError;

/// Placeholder error for issues that for `dr` code
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error {
            message: String::from(value),
        }
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error { message: value }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error {
            message: String::from("An unspecified error occurred"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl E for Error {}
