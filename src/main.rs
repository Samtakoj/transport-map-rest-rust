#[macro_use] extern crate nickel;
#[macro_use] extern crate serde_json;

use nickel::Nickel;

use serde_json::{
    to_string_pretty,
};

fn main() {
    let mut server = Nickel::new();
    let host_port = "127.0.0.1:6767";

    let json_response = json!({
        "name" : "Transport-map-rest-rust",
        "version" : "0.0.1",
        "location" : host_port,
        "status" : "GREEN"
    });

    server.utilize(router! {
        get "**" => |_req, _res| {
            to_string_pretty(&json_response).unwrap()
        }
    });

    server.listen(host_port);
}