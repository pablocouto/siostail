extern crate siostail;

use siostail::{Endpoint, Token};

// TODO: Obtain from environment, to ease use with Travis.
// NB: Generated in browser; subject to expiration.
const TOKEN: &str = "8f38cf52b8b1e583a80f6af92ec58d07f343d32e51ab96f9ae9d674692c9e51d";

struct Helper {}

impl Helper {
    fn endpoint() -> Endpoint {
        let token = Token(TOKEN.to_string());
        Endpoint::new(token).unwrap()
    }
}

#[test]
#[ignore]                       // Too demanding on the server.
fn indicators() {
    let mut endpoint = Helper::endpoint();
    let res = endpoint.indicators();
    // TODO: Test by deserializing into tailored type.
    assert!(res.is_ok());
}
