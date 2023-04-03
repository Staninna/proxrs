mod config;
mod db;
mod error;

use config::*;
use db::*;
use error::Error;
use hyper::{Body, Request, Response, Server};
use tower::{make::Shared, service_fn};
// use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
// use tower::{make::Shared, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the config
    let conf = err!(init::conf());

    // Initialize the database
    let db_file = err!(conf.get(DbFile));
    let _db = err!(Db::new(db_file).await);

    // Define the server address
    let ip = err!(err!(conf.get(Ip)).parse::<std::net::IpAddr>());
    let port = err!(err!(conf.get(Port)).parse::<u16>());
    let addr = SocketAddr::new(ip, port);

    // Create a hyper service
    let service = Shared::new(service_fn(move |_req: Request<Body>| handle()));

    // Create the server with graceful shutdown capabilities
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Shutting down...");

            // Any cleanup code here

            println!("Goodbye!");
            std::process::exit(0);
        });

    // Run the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    unreachable!();
}

async fn handle() -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello, World!")))
}
