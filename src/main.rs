mod config;
mod db;
mod error;

use crate::{
    config::{init, options::ConfigOptions::*},
    db::db::Db,
    error::Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the config
    let conf = init::conf()?;

    // Initialize the database
    let db_file = conf.get(DbFile)?;
    let db = Db::new(db_file).await?;

    println!("{:?}", conf);

    Ok(())
}
