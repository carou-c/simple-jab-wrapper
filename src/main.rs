#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod protocol;
mod jab_api;
mod server;

use server::JabServer;
use std::env;

fn main() {
    let port = env::var("JAB_SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9250);

    println!("Starting JAB Server...");
    let server = JabServer::new();
    server.run(port);
}
