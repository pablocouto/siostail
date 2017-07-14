use crest;
use hyper;
use serde_json;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Crest(crest::error::Error),
    Hyper(hyper::Error),
    SerdeJson(serde_json::Error),
    Unknown,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Crest(ref error) => error.description(),
            Error::Hyper(ref error) => error.description(),
            Error::SerdeJson(ref error) => error.description(),
            Error::Unknown => "Unknown error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl From<crest::error::Error> for Error {
    fn from(error: crest::error::Error) -> Self {
        Error::Crest(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Hyper(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}
