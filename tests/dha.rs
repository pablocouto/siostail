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
