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
use chrono::offset::FixedOffset;
use siostail::*;
use std::env;
use std::time::Duration;

struct Helper;

impl Helper {
    fn endpoint() -> Result<Endpoint> {
        let token = &env::var("ESIOS_TOKEN").chain_err(|| ErrorKind::NoAuth)?;
        let timeout = Duration::from_millis(5000);
        Endpoint::new(token, timeout)
    }
}

#[test]
#[ignore] // Possibly too demanding on the server.
fn indicators() {
    let mut esios = Helper::endpoint().unwrap();
    let res = esios.indicators();
    assert!(res.is_ok());
}

#[test]
fn indicator() {
    let mut esios = Helper::endpoint().unwrap();
    // Using a non CET/CEST timezone here also tests this library’s
    // capability to deal with the API limitations in this regard.
    let clt = FixedOffset::west(3600 * 4);
    let res = esios.indicator_at(&clt.ymd(2014, 04, 01));
    let res = res.map(|data| {
        let esios::IndicatorInner { values, .. } = data.indicator;
        // Externally known value:
        assert_eq!(values[0].value, 32.63);
    });
    assert!(res.is_ok());
}
