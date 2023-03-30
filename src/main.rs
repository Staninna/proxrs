use config::get_value;
use hashbrown::HashMap;
use hyper::{service::service_fn, Body, Request, Response, Server};
use session::Session;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::make::Shared;
mod auth;
mod config;
mod proxy;
mod session;

// Handles incoming requests
async fn handle(
    req: Request<Body>,
    conf: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
) -> Result<Response<Body>, hyper::Error> {
    let auth_path = get_value(&conf, "auth_path").await;

    match (req.method(), req.uri().path()) {
        // Handle the auth route
        (_, path) if path.starts_with(&auth_path) => auth::handler(req, conf, sessions).await,

        // proxy all other routes
        _ => proxy::proxy_handler(req, sessions).await,
    }
}

#[tokio::main]
async fn main() {
    // Load configiuration settings
    let conf = config::config();

    // Initialize the sessions map
    let sessions = Arc::new(Mutex::new(HashMap::new()));

    // Create the hyper service
    let conf_clone = conf.clone();
    let service = Shared::new(service_fn(move |req: Request<Body>| {
        let sessions = sessions.clone();
        let conf = conf_clone.clone();

        handle(req, conf, sessions)
    }));

    // Define the server address
    let addr = get_value(&conf, "address").await.parse().unwrap();
    let port = get_value(&conf, "port").await.parse().unwrap();
    let addr = SocketAddr::new(addr, port);

    // Create the server with graceful shutdown capabilities
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Shutting down...");

            // Perform any necessary cleanup here

            println!("Goodbye!");
        });

    // Start the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
