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

use chrono::{Date, TimeZone};
use futures::{Future, Stream};
use futures::{future, stream};
use hyper::header::qitem;
use hyper::header::{self, Encoding};
use hyper::{Headers, Response, StatusCode};
use std::time::Duration;
use tokio_timer::{Timeout, Timer};

pub mod error;
pub mod esios;

mod impls;

use error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Token(pub String);

struct EndpointConfig {
    timeout: Duration,
}

// TODO: Use Cow?
pub struct Endpoint {
    server: crest::Endpoint,
    token: Token,
    config: EndpointConfig,
}

// TODO: Use caching where appropriate.
// TODO: Handle 401 responses.
impl Endpoint {
    pub fn new(token: Token, timeout: u64) -> Result<Self> {
        let server = crest::Endpoint::new("https://api.esios.ree.es/")?;
        let config = EndpointConfig { timeout: Duration::from_secs(timeout) };
        Ok(Endpoint {
            server,
            token,
            config,
        })
    }

    fn set_basic_headers(&self, headers: &mut Headers) {
        headers.set(header::UserAgent::new("siostail/dev"));
        headers.set(header::Accept::json());
        headers.set(header::AcceptEncoding(vec![qitem(Encoding::Identity)]));
        headers.set(header::Authorization(self.token.clone()))
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

    // TODO: Use builder pattern for the request?
    fn create_request(
        &self,
        route: &str,
    ) -> Result<Timeout<future::FromErr<hyper::client::FutureResponse, Error>>> {
        let mut req = self.server.get(route)?;
        self.set_basic_headers(req.headers_mut());
        let req = self.set_timeout(req.into_future());
        Ok(req)
    }

    // TODO: Add stream timeout for the body.
    pub fn indicators(&mut self) -> Result<esios::Indicators> {
        let route = "indicators";
        let req = self.create_request(route)?;
        let work = req.and_then(|res| {
            assert_status(&res, StatusCode::Ok);
            let body = concat_body(res).from_err();
            body
        });
        let res = self.server.run(work)?;
        let data = serde_json::from_slice(&*res)?;
        Ok(data)
    }

    pub fn indicator(&mut self, start_date: &str, end_date: &str) -> Result<esios::Indicator> {
        let mut route = "indicators/1014".to_string();
        route += &format!("?start_date={}&end_date={}", start_date, end_date);
        let req = self.create_request(&route)?;
        let work = req.and_then(|res| {
            assert_status(&res, StatusCode::Ok);
            let body = concat_body(res).from_err();
            body
        });
        let res = self.server.run(work)?;
        let data = serde_json::from_slice(&*res)?;
        Ok(data)
    }
}

pub fn day_range_rfc3339<T>(date: Date<T>) -> (String, String)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    let start_time = date.and_hms(0, 0, 0).to_rfc3339();
    let end_time = date.and_hms(23, 0, 0).to_rfc3339();
    (start_time, end_time)
}

fn assert_status(res: &Response, status: StatusCode) {
    assert_eq!(res.status(), status);
}

fn concat_body(res: Response) -> stream::Concat2<hyper::Body> {
    res.body().concat2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_range_rfc3339() {
        let fixed_offset = chrono::offset::FixedOffset::east(3600 * 2);
        let date_local = fixed_offset.ymd(2014, 04, 01);
        let (start_time, end_time) = super::day_range_rfc3339(date_local);
        assert_eq!(&start_time, "2014-04-01T00:00:00+02:00");
        assert_eq!(&end_time, "2014-04-01T23:00:00+02:00");
    }
}
