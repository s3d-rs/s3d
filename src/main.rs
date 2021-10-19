//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

pub mod cli;
pub mod client;
pub mod conf;
pub mod daemon;
pub mod gen;
pub mod proto;
pub mod resources;
pub mod router;
pub mod store;

// pub mod fuse;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate anyhow;

use crate::cli::CLI;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    match CLI::run().await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
