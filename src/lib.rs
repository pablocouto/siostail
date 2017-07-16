#[macro_use]
extern crate serde_derive;

extern crate crest;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_timer;

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
}

struct Helper {}

impl Helper {
    fn status_ok(res: &Response) {
        assert_eq!(res.status(), StatusCode::Ok);
    }
}
