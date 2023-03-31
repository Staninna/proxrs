use config::ConfigKey::*;
use hyper::{service::service_fn, Body, Request, Server};
use proxy::proxy;
use session::SessionStore;
use std::net::SocketAddr;
use tera::Tera;
use tower::make::Shared;
mod config;
mod error;
mod login;
mod proxy;
mod session;

#[tokio::main]
async fn main() {
    // Load configiuration settings
    let conf = config::config().await;

    // Initialize the sessions map
    let sessions = SessionStore::new();

    // Initialize the template engine
    let tera_path = conf.get(TeraTemplatesDir).await;
    let tera_path = format!("{}/**/*", tera_path);
    let tera = match Tera::new(&tera_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading Tera templates: {}", e);
            std::process::exit(1);
        }
    };

    // Create the hyper service
    let conf_clone = conf.clone();
    let service = Shared::new(service_fn(move |req: Request<Body>| {
        let sessions = sessions.clone();
        let conf = conf_clone.clone();

        proxy(req, conf, tera.clone(), sessions)
    }));

    // Define the server address
    let ip = conf.get(Ip).await.parse().unwrap();
    let port = conf.get(Port).await.parse().unwrap();
    let addr = SocketAddr::new(ip, port);

    // Create the server with graceful shutdown capabilities
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Shutting down...");

            // Perform any necessary cleanup here

            println!("Goodbye!");
            std::process::exit(0);
        });

    // Start the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
