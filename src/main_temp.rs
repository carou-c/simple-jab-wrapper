#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod protocol;
mod jab_api;
mod server;

use protocol::RpcMethod;

fn main() {
    let schema = schemars::schema_for!(RpcMethod);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
