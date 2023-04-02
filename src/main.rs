mod config;
mod db;

use crate::{
    config::{init, options::ConfigOptions::*},
    db::db::Db,
};

#[tokio::main]
async fn main() {
    // Initialize the config
    let conf = match init::conf() {
        Ok(conf) => conf,
        Err(err) => panic!("{:?}", err),
    };

    // Initialize the database
    let db_file = conf.get(DbFile).unwrap();
    let db = match Db::new(db_file).await {
        Ok(db) => db,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", conf);
}
