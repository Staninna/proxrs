use hyper::{service::service_fn, Body, Request, Response, Server};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::make::Shared;
mod auth;
mod config;
mod proxy;

// Handles incoming requests
async fn handle(
    req: Request<Body>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Handle the login route
        (_, "/proxy/login") => auth::login_handler(req, sessions).await,

        // proxy all other routes
        _ => proxy::proxy_handler(req, sessions).await,
    }
}

#[tokio::main]
async fn main() {
    // Load configiuration settings
    let conf = config::config();

    // Initialize the sessions map
    let sessions: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    // Create the hyper service
    let service = Shared::new(service_fn(move |req: Request<Body>| {
        let sessions = sessions.clone();
        handle(req, sessions)
    }));

    // Define the server address
    let addr = SocketAddr::from((conf.addr, conf.port));

    // Create the server with graceful shutdown capabilities
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Shutting down...");

            // Perform any necessary cleanup here
        });

    // Start the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    println!("Server stopped");
}
