mod conf;
mod database;
mod error;
mod routes;
mod sess;
mod state;

use crate::{conf::*, database::*, error::*, routes::*, sess::*, state::*};

use axum::{
    routing::{get, post},
    Router, Server,
};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

type Client = hyper::Client<HttpsConnector<HttpConnector>, Body>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get the config
    let conf = check_err!(init::conf());

    // Initialize the app state
    let state = AppState::new(&conf).await;

    // Define the server address
    let ip = check_err!(check_err!(conf.get(Ip)).parse::<std::net::IpAddr>());
    let port = check_err!(check_err!(conf.get(Port)).parse::<u16>());
    let addr = SocketAddr::new(ip, port);

    // Get special routes
    let special_route = check_err!(conf.get(SpecialRoute));
    let login_route = special_route.to_owned() + "/login";
    let logout_route = special_route.to_owned() + "/logout";
    let admin_route = special_route.to_owned() + "/admin";

    // Create the app
    let app = Router::new()
        // Add the special routes
        .route(&login_route, get(login_page))
        .route(&login_route, post(login_req))
        .route(&logout_route, post(logout))
        .route(&admin_route, get(|| async { "admin" })) // TODO: Implement admin get
        .route(&admin_route, post(|| async { "admin" })) // TODO: Implement admin post
        // Add proxy route
        .fallback(proxy)
        // Add the app state
        .with_state(state);

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
