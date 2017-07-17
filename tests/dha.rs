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

extern crate chrono;
extern crate siostail;

use chrono::TimeZone;
use chrono::offset::Local;
use siostail::day_range_rfc3339;
use siostail::error::{Error, Result};
use siostail::{Endpoint, Esios, Token};
use std::error::Error as StdError;

// TODO: Obtain from environment, to ease use with Travis.
// NB: Generated in browser; subject to expiration.
const TOKEN: &str = "8f38cf52b8b1e583a80f6af92ec58d07f343d32e51ab96f9ae9d674692c9e51d";

struct Helper;

impl Helper {
    fn endpoint() -> Result<Endpoint> {
        let token = Token(TOKEN.to_string());
        let timeout = 5;
        Endpoint::new(token, timeout)
    }

    fn handle_err(error: Option<Error>) {
        match error {
            None => return (),
            // Server timeouts are OK.
            Some(Error::Timeout) => {
                println!("{}", Error::Timeout.description());
                return ();
            }
            Some(ref err) => {
                panic!("{:?}", err);
            }
        }
    }
}

#[test]
#[ignore] // Possibly too demanding on the server.
fn indicators() {
    let mut esios = Helper::endpoint().unwrap();
    let res = esios.indicators();
    Helper::handle_err(res.err());
}

#[test]
fn indicator() {
    let mut esios = Helper::endpoint().unwrap();
    let (start_time, end_time) = day_range_rfc3339(Local.ymd(2014, 04, 01));
    let res = esios.indicator(&start_time, &end_time);
    let res = res.map(|data| {
        let Esios::Indicator { values, .. } = data;
        // Externally known value:
        assert_eq!(values[0].value, 40.55);
    });
    Helper::handle_err(res.err());
}
