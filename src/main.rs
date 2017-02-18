#[macro_use] extern crate nickel;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use nickel::Nickel;
use serde::de::{Deserialize};
use serde::ser::{Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    name: String,
    version: String,
    location: String,
    status: String,
}

fn main() {
    let mut server = Nickel::new();
    let host_port = "127.0.0.1:6767";
    let status = Status { name : "Transport-map-rest-rust".to_string(), version : "0.0.1".to_string(),
        location : host_port.to_string(), status : "GREEN".to_string() };

    server.utilize(router! {
        get "**" => |_req, _res| {
            serde_json::to_string_pretty(&status).unwrap()
        }
    });

    server.listen(host_port);
}