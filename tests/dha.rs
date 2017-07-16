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

extern crate siostail;

use siostail::error::Error;
use siostail::{Endpoint, Token};
use std::error::Error as StdError;

// TODO: Obtain from environment, to ease use with Travis.
// NB: Generated in browser; subject to expiration.
const TOKEN: &str = "8f38cf52b8b1e583a80f6af92ec58d07f343d32e51ab96f9ae9d674692c9e51d";

struct Helper {}

impl Helper {
    fn endpoint() -> Endpoint {
        let token = Token(TOKEN.to_string());
        let timeout = 5;
        Endpoint::new(token, timeout).unwrap()
    }
}

#[test]
#[ignore] // Possibly too demanding on the server.
fn indicators() {
    let mut endpoint = Helper::endpoint();
    let res = endpoint.indicators();
    match res.err() {
        None => return (),
        // Server timeouts are OK:
        Some(Error::Timeout) => {
            println!("{}", Error::Timeout.description());
            return ();
        }
        Some(ref err) => {
            panic!("{:?}", err);
        }
    }
}

#[test]
fn indicator() {
    let mut endpoint = Helper::endpoint();
    let res = endpoint.indicator("2017-07-12T00:00:00+02:00", "2017-07-12T23:50:00+02:00");
    match res.err() {
        None => return (),
        // Server timeouts are OK:
        Some(Error::Timeout) => {
            println!("{}", Error::Timeout.description());
            return ();
        }
        Some(ref err) => {
            panic!("{:?}", err);
        }
    }
}
