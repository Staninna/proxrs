mod config;
mod database;
mod error;
mod routes;

use crate::{config::*, database::*, error::*, routes::*};

use axum::{Router, Server};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

type Client = hyper::Client<HttpsConnector<HttpConnector>, Body>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the config
    let conf = err!(init::conf());

    // Initialize the database
    let db_file = err!(conf.get(DbFile));
    let db = err!(Db::new(db_file).await);

    // Create the client
    let client = hyper::Client::builder().build(HttpsConnector::new());

    // Define the server address
    let ip = err!(err!(conf.get(Ip)).parse::<std::net::IpAddr>());
    let port = err!(err!(conf.get(Port)).parse::<u16>());
    let addr = SocketAddr::new(ip, port);

    // Create the app
    let app = Router::new()
        .fallback(proxy)
        .with_state(client)
        .with_state(conf)
        .with_state(db);

    // Start the server
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
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
        eprintln!("server error: {}", e);
    }

    unreachable!()
}
