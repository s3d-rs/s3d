//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

// #![allow(unused)]

pub mod build_gen;
pub mod cli;
pub mod s3_server;
pub mod utils;
pub mod write_queue;
pub mod fsnotify;

#[macro_use]
extern crate log;

// #[macro_use]
extern crate clap;

// #[macro_use]
extern crate anyhow;

use crate::cli::CLI;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    match CLI::run().await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
