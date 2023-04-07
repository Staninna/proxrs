mod config;
mod database;
mod error;
mod proxy;

use config::*;
use database::*;
use error::Error;
use hyper::{service::make_service_fn, Server};
use std::net::SocketAddr;

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
    let service = make_service_fn(|_conn| async {
        let svc = proxy::Proxy;

        Ok::<_, hyper::Error>(svc)
    });

    // Create the server with graceful shutdown capabilities
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
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
