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
    let conf = check!(init::conf());

    // Initialize the database
    let db_file = check!(conf.get(DbFile));
    let _db = check!(Db::new(db_file).await);

    // Define the server address
    let ip = check!(check!(conf.get(Ip)).parse::<std::net::IpAddr>());
    let port = check!(check!(conf.get(Port)).parse::<u16>());
    let _addr = SocketAddr::new(ip, port);

    // Evrything went well
    Ok(())
}
