#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serenity;
extern crate spin;
extern crate toml;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_error;

mod handlers;
mod error;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use std::env;
use std::error::Error;
use spin::Mutex;
use handlers::Handler;
use error;

#[derive(Deserialize)]
pub struct Config {
    token: String,
    log_channel: u64,
}

pub fn get_config() -> Result<Config, error::Error> {
    let home_dir = env::home_dir()?;
    let path = home_dir.join(".config").join("sudobot").join("config.toml");

    let mut f = File::open(path)?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;

    toml::from_str(&buf)
}

fn main() {
    match env_logger::init() {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to initialize env_logger. Reason: {}", e.cause().unwrap());
        },
    }

    let cfg = match get_config() {
        Ok(c) => c,
        Err(e) => {
            //Io error.
            error!("Error retrieving config from file: {}", e.description());
        },
    };
        
    let token = cfg.clone().token;
    
    let handler = Handler {
        cfg: cfg,
    };

    let mut client = Client::new(&token, handler);

    if let Err(e) = client.start() {
        error!("Client error: {:?}", e);
    }
}
