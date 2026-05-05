#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod jab_wrapper {
    include!(concat!(env!("OUT_DIR"), "/jab_wrapper.rs"));
}

mod jab_api;
mod protocol;
mod server;
mod types;

use server::JabServer;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 9250;
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;

    println!("Starting JAB gRPC Server on {}...", addr);
    let server = JabServer::new();

    Server::builder()
        .accept_http1(false)
        .add_service(server.into_service())
        .serve(addr)
        .await?;

    Ok(())
}
