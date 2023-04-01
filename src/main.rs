use config::ConfigKey::*;
use db::Db;
use hyper::{service::service_fn, Body, Request, Server};
use proxy::proxy;
use session::SessionStore;
use std::net::SocketAddr;
use tera::Tera;
use tower::make::Shared;
mod auth;
mod config;
mod db;
mod error;
mod proxy;
mod session;

#[tokio::main]
async fn main() {
    // Load configiuration settings
    let conf = config::config().await;

    // Initialize the sessions map
    let sessions = SessionStore::new();

    // Initialize the database connection pool
    let db_file = conf.get(DbFile).await;
    let db = Db::new(&db_file).await;

    // Initialize the template engine
    let template_dir = conf.get(TemplateDir).await;
    let tera = match Tera::new(&format!("{}/*.html", template_dir)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading templates: {}", e);
            std::process::exit(1);
        }
    };

    // Create the hyper service
    let db_clone = db.clone();
    let tera_clone = tera.clone();
    let conf_clone = conf.clone();
    let service = Shared::new(service_fn(move |req: Request<Body>| {
        let sessions = sessions.clone();
        let conf = conf_clone.clone();
        let tera = tera_clone.clone();
        let db = db_clone.clone();

        proxy(db, req, conf, tera, sessions)
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
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            println!("Goodbye!");
            std::process::exit(0);
        });

    // Start the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
