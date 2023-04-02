mod config;
mod db;

use crate::config::init;

#[tokio::main]
async fn main() {
    // Initialize the config
    let conf = match init::conf() {
        Ok(conf) => conf,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", conf);
}
