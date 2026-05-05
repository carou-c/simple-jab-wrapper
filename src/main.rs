#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod jab_api;
mod protocol;
mod server;

use protocol::RpcMethod;
use server::JabServer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--schema" {
        print_schema();
        return;
    }

    let port = 9250;

    println!("Starting JAB Server...");
    let server = JabServer::new();
    server.run(port);
}

fn print_schema() {
    let schema = schemars::schema_for!(RpcMethod);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
