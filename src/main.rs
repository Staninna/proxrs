mod config;
mod db;
mod error;

use config::*;
use db::*;
use error::Error;
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
    let _addr = SocketAddr::new(ip, port);

    // Evrything went well
    Ok(())
}
