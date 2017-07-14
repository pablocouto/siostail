use hyper::header::Scheme;
use std::fmt;
use std::str::FromStr;

use error;
use {Result, Token};

impl Scheme for Token {
    fn scheme() -> Option<&'static str> {
        Some("Token")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "token=\"{}\"", &self.0)
    }
}

impl FromStr for Token {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Token(s.to_string()))
    }
}
