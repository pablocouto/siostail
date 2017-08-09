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

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate crest;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_timer;

use chrono::{Date, FixedOffset, TimeZone};
use crest::ResponseBody;
use hyper::StatusCode;
use hyper::header::qitem;
use hyper::header::{self, Encoding};
use std::fmt;
use std::time::Duration;

pub mod esios;

mod error;
mod impls;

pub use error::*;

#[derive(Clone, Debug)]
struct Token(String);

struct Config {
    token: Token,
    timeout: Duration,
}

pub struct Endpoint {
    server: crest::Endpoint,
    config: Config,
}

// TODO: Use caching where appropriate.
// TODO: Handle 401 responses.
impl Endpoint {
    pub fn new(token: &str, timeout: Duration) -> Result<Self> {
        // ESIOS’s server doesn’t appear to honor `keep-alive`.
        // Turning off the flag (as far as I could gather) prevents
        // Hyper from polling on a (possibly) broken pipe.
        let keep_alive = false;
        let server = crest::Endpoint::new("https://api.esios.ree.es/", keep_alive)?;
        let token = Token(token.to_string());
        let config = Config { token, timeout };
        let endpoint = Endpoint { server, config };
        Ok(endpoint)
    }

    fn get(&self, route: &str) -> Result<ResponseBody> {
        let timeout = self.config.timeout;
        let response = self.server
            .get(route)?
            .header(header::UserAgent::new("siostail/dev"))
            .header(header::Accept::json())
            .header(header::AcceptEncoding(vec![qitem(Encoding::Identity)]))
            .header(header::Authorization(self.config.token.clone()))
            .timeout(timeout)
            .into_future();
        let body = response.assert_status(StatusCode::Ok).body();
        Ok(body)
    }

    pub fn indicators(&mut self) -> Result<esios::Indicators> {
        let route = "indicators";
        let get = self.get(route)?;
        let body = self.server.run(get).map_err(handle_crest_err)?;
        let data = serde_json::from_slice(&*body).chain_err(
            || "Failed to deserialize `indicators` data",
        )?;
        Ok(data)
    }

    pub fn indicator(&mut self, start_date: &str, end_date: &str) -> Result<esios::Indicator> {
        let mut route = "indicators/1014".to_string();
        route += &format!("?start_date={}&end_date={}", start_date, end_date);
        let get = self.get(&route)?;
        let body = self.server.run(get).map_err(handle_crest_err)?;
        let data = serde_json::from_slice(&*body).chain_err(
            || "Failed to deserialize `indicator` data",
        )?;
        Ok(data)
    }

    pub fn indicator_at<T>(&mut self, date: &Date<T>) -> Result<esios::Indicator>
    where
        T: TimeZone,
        T::Offset: fmt::Display,
    {
        let (range_start, range_end) = day_range_rfc3339(date);
        let data = self.indicator(&range_start, &range_end)?;
        Ok(data)
    }
}

fn day_range_rfc3339<T>(date: &Date<T>) -> (String, String)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    let cest = FixedOffset::east(2 * 3600);
    let start_time = date.and_hms(0, 0, 0).with_timezone(&cest).to_rfc3339();
    let end_time = date.and_hms(23, 0, 0).with_timezone(&cest).to_rfc3339();
    (start_time, end_time)
}

fn handle_crest_err(error: crest::Error) -> Error {
    match *error.kind() {
        ::crest::ErrorKind::Timeout => ErrorKind::Timeout.into(),
        _ => error.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_range_rfc3339_cest() {
        let cest = FixedOffset::east(3600 * 2);
        let date = cest.ymd(2014, 04, 01);
        let (start_time, end_time) = super::day_range_rfc3339(&date);
        assert_eq!(&start_time, "2014-04-01T00:00:00+02:00");
        assert_eq!(&end_time, "2014-04-01T23:00:00+02:00");
    }

    #[test]
    fn day_range_rfc3339_clt() {
        // The timezone was picked arbitrarily to be any but CET/CEST,
        // in order to test conformance with the API quirks¹ for such
        // cases.
        //
        // ¹An apparent lack of timezone conversion functionality.
        let clt = FixedOffset::west(3600 * 4);
        let date = clt.ymd(2014, 04, 01);
        let (start_time, end_time) = super::day_range_rfc3339(&date);
        assert_eq!(&start_time, "2014-04-01T06:00:00+02:00");
        assert_eq!(&end_time, "2014-04-02T05:00:00+02:00");
    }
}
