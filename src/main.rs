mod conf;
mod database;
mod error;
mod routes;
mod sess;

use crate::{conf::*, database::*, error::*, routes::*, sess::*};

use axum::{
    routing::{get, post},
    Router, Server,
};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

type Client = hyper::Client<HttpsConnector<HttpConnector>, Body>;

#[derive(Clone)]
pub struct AppState {
    sessions: Sessions,
    client: Client,
    conf: Config,
    db: Db,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the config
    let conf = check_err!(init::conf());

    // Initialize the database
    let db_file = check_err!(conf.get(DbFile));
    let db = check_err!(Db::new(db_file).await);

    // Initialize the sessions
    let sessions = Sessions::new();

    // Create the client
    let client = hyper::Client::builder().build(HttpsConnector::new());

    // Define the server address
    let ip = check_err!(check_err!(conf.get(Ip)).parse::<std::net::IpAddr>());
    let port = check_err!(check_err!(conf.get(Port)).parse::<u16>());
    let addr = SocketAddr::new(ip, port);

    // Get special routes
    let special_route = check_err!(conf.get(SpecialRoute));
    let login_route = special_route.to_owned() + "/login";
    let logout_route = special_route.to_owned() + "/logout";

    // Create the app
    let app = Router::new()
        // Add the special routes
        .route(&login_route, get(login_page))
        .route(&login_route, post(login_req))
        .route(&logout_route, post(|| async { "logout" })) // TODO: Implement logout post
        // Add proxy route
        .fallback(proxy)
        // Add the app state
        .with_state(AppState {
            sessions,
            client,
            conf,
            db,
        });

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
