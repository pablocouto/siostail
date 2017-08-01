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
extern crate serde_derive;

extern crate chrono;
extern crate crest;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_timer;

use chrono::{Date, FixedOffset, TimeZone};
use futures::future;
use futures::{BoxFuture, Future, Stream};
use hyper::header::qitem;
use hyper::header::{self, Encoding};
use hyper::{Response, StatusCode};
use std::time::Duration;
use tokio_timer::{Timeout, Timer};

pub mod error;
pub mod esios;

pub use error::Error;

mod impls;

use error::Result;

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
    pub fn new(token: &str, timeout: u64) -> Result<Self> {
        let server = crest::Endpoint::new("https://api.esios.ree.es/")?;
        let token = Token(token.to_string());
        let timeout = Duration::from_millis(timeout);
        let config = Config { token, timeout };
        let endpoint = Endpoint { server, config };
        Ok(endpoint)
    }

    fn set_timeout<T>(&self, req: T) -> Timeout<future::FromErr<T, Error>>
    where
        T: Future,
        Error: From<T::Error>,
    {
        let timer = Timer::default();
        let req: future::FromErr<_, Error> = req.from_err();
        let req = timer.timeout(req, self.config.timeout.clone());
        req
    }

    fn create_request(
        &self,
        route: &str,
    ) -> Result<Timeout<future::FromErr<hyper::client::FutureResponse, Error>>> {
        let req = self.server
            .get(route)?
            .header(header::UserAgent::new("siostail/dev"))
            .header(header::Accept::json())
            .header(header::AcceptEncoding(vec![qitem(Encoding::Identity)]))
            .header(header::Authorization(self.config.token.clone()));
        let req = self.set_timeout(req.into_future());
        Ok(req)
    }

    fn prepare_response(res: hyper::Response) -> BoxFuture<hyper::Chunk, Error> {
        assert_status(&res, StatusCode::Ok);
        let body = res.body().concat2().from_err();
        body.boxed()
    }

    // TODO: Add stream timeout for the body.
    pub fn indicators(&mut self) -> Result<esios::Indicators> {
        let route = "indicators";
        let req = self.create_request(route)?.and_then(Self::prepare_response);
        let res = self.server.run(req)?;
        let data = serde_json::from_slice(&*res)?;
        Ok(data)
    }

    pub fn indicator(&mut self, start_date: &str, end_date: &str) -> Result<esios::Indicator> {
        let mut route = "indicators/1014".to_string();
        route += &format!("?start_date={}&end_date={}", start_date, end_date);
        let req = self.create_request(&route)?.and_then(
            Self::prepare_response,
        );
        let res = self.server.run(req)?;
        let data = serde_json::from_slice(&*res)?;
        Ok(data)
    }
}

pub fn day_range_rfc3339<T>(date: &Date<T>) -> (String, String)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    let cest = FixedOffset::east(2 * 3600);
    let start_time = date.and_hms(0, 0, 0).with_timezone(&cest).to_rfc3339();
    let end_time = date.and_hms(23, 0, 0).with_timezone(&cest).to_rfc3339();
    (start_time, end_time)
}

fn assert_status(res: &Response, status: StatusCode) {
    assert_eq!(res.status(), status);
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
