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
