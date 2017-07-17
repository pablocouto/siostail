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
use futures::future;
use futures::{Future, Stream};
use hyper::header::{self, Encoding, qitem};
use hyper::{Response, StatusCode};
use std::time::Duration;
use tokio_timer::Timer;

pub mod error;

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

#[allow(dead_code)]
#[derive(Deserialize)]
struct Indicator {
    name: String,
    description: String,
    id: u32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Indicators {
    indicators: Vec<Indicator>,
    meta: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct HourData {
    pub value: f32,
    datetime: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Esios {
    Indicator {
        id: u16,
        values_updated_at: String,
        values: Vec<HourData>,
    },
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

    // TODO: Add stream timeout for the body.
    pub fn indicators(&mut self) -> Result<Indicators> {
        let route = "indicators";
        let work = {
            let mut req = self.server.get(route)?;
            {
                let hs = req.headers_mut();
                hs.set(header::UserAgent::new("siostail/dev"));
                hs.set(header::Accept::json());
                hs.set(header::AcceptEncoding(vec![qitem(Encoding::Identity)]));
                hs.set(header::Authorization(self.token.clone()))
            }
            let req: future::FromErr<_, Error> = req.into_future().from_err();
            let timer = Timer::default();
            let req = timer.timeout(req, self.config.timeout.clone());
            req.and_then(|res| {
                Helper::status_ok(&res);
                let body = res.body().concat2().from_err();
                body
            })
        };
        let res = self.server.run(work)?;
        let value: Indicators = serde_json::from_slice(&*res)?;
        Ok(value)
    }

    pub fn indicator(&mut self, start_date: &str, end_date: &str) -> Result<Esios> {
        let mut route = "indicators/1014".to_string();
        route += &format!("?start_date={}&end_date={}", start_date, end_date);
        let work = {
            let mut req = self.server.get(&route)?;
            {
                let hs = req.headers_mut();
                hs.set(header::UserAgent::new("siostail/dev"));
                hs.set(header::Accept::json());
                hs.set(header::AcceptEncoding(vec![qitem(Encoding::Identity)]));
                hs.set(header::Authorization(self.token.clone()))
            }
            let req: future::FromErr<_, Error> = req.into_future().from_err();
            let timer = Timer::default();
            let req = timer.timeout(req, self.config.timeout.clone());
            req.and_then(|res| {
                Helper::status_ok(&res);
                let body = res.body().concat2().from_err();
                body
            })
        };
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

struct Helper {}

impl Helper {
    fn status_ok(res: &Response) {
        assert_eq!(res.status(), StatusCode::Ok);
    }
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
