extern crate crest;
extern crate futures;
extern crate hyper;
extern crate serde_json;

use futures::stream::Concat2;
use futures::{Future, Stream};
use hyper::header::{self, Encoding, qitem};
use hyper::{Body, Response, StatusCode};
use serde_json::Value;
use std::ops::Deref;

mod error;
mod impls;

use error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Token(pub String);

pub struct Endpoint {
    server: crest::Endpoint,
    token: Token,
}

// TODO: Handle 401 responses.
// TODO: Add request timeouts.
impl Endpoint {
    pub fn new(token: Token) -> Result<Self> {
        let server = crest::Endpoint::new("https://api.esios.ree.es/")?;
        Ok(Endpoint { server, token })
    }

    // TODO: Create type for deserializing.
    pub fn indicators(&mut self) -> Result<Value> {
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
            req.into_future().and_then(|res| {
                Helper::status_ok(&res);
                Helper::get_concat_body(res)
            })
        };
        let res = Helper::run_and_get_json_value(&mut self.server, work);
        Ok(res)
    }

    pub fn run<T>(&mut self, work: T) -> Result<T::Item>
    where
        T: Future,
        Error: From<T::Error>,
        crest::Error: From<T::Error>,
    {
        let resp = self.server.run(work)?;
        Ok(resp)
    }
}

struct Helper {}

impl Helper {
    fn status_ok(res: &Response) {
        assert_eq!(res.status(), StatusCode::Ok);
    }

    fn get_concat_body(res: Response) -> Concat2<Body> {
        res.body().concat2()
    }

    fn to_json_value(data: &[u8]) -> Value {
        serde_json::from_slice(data).unwrap()
    }

    fn run_and_get_json_value<T>(endpoint: &mut crest::Endpoint, work: T) -> Value
    where
        T: Future,
        T::Item: Deref<Target = [u8]>,
        Error: From<T::Error>,
        crest::Error: From<T::Error>,
    {
        let res = endpoint.run(work).unwrap();
        Helper::to_json_value(&*res)
    }
}
