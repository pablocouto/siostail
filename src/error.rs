// Copyright 2017 Pablo Couto

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// version 3 as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License version 3 for more details.

// You should have received a copy of the GNU Lesser General Public
// License version 3 along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use chrono;
use crest;
use hyper;
use serde_json;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::result;
use tokio_timer;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Crest(crest::error::Error),
    Chrono(chrono::ParseError),
    Hyper(hyper::Error),
    SerdeJson(serde_json::Error),
    TimeoutError(tokio_timer::TimerError),

    Timeout,
    Unknown,
}

// TODO: Consistency in names.
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Crest(ref error) => error.description(),
            Error::Chrono(ref error) => error.description(),
            Error::Hyper(ref error) => error.description(),
            Error::SerdeJson(ref error) => error.description(),
            Error::TimeoutError(ref error) => error.description(),

            Error::Timeout => "Operation timed out",
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

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Error::Chrono(error)
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

impl<T> From<tokio_timer::TimeoutError<T>> for Error {
    fn from(error: tokio_timer::TimeoutError<T>) -> Self {
        match error {
            tokio_timer::TimeoutError::Timer(_, error) => Error::TimeoutError(error),
            tokio_timer::TimeoutError::TimedOut(_) => Error::Timeout,
        }
    }
}
