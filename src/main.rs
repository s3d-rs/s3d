//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

use clap::{AppSettings, Clap};
use std::error::Error;

type AnyError = Box<dyn Error + Send + Sync>;
type ResultOrAnyErr<T> = Result<T, AnyError>;

#[derive(Clap, Debug)]
#[clap(about = "s3d is an S3 daemon for the Edge written in Rust.")]
#[clap(setting = AppSettings::ColoredHelp)]
struct CLI {
    /// Sets a custom config file
    #[clap(short, long)]
    config: Option<String>,

    /// Verbosity level, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Cli command
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Clap, Debug)]
enum Cmd {
    Run(RunCmd),
}

#[derive(Clap, Debug)]
struct RunCmd {
    #[clap(short)]
    debug: bool,
}

#[tokio::main]
pub async fn main() -> ResultOrAnyErr<()> {
    let cli: CLI = CLI::parse();

    if cli.verbose > 0 {
        println!("[VERBOSE] CLI options: {:?}", cli);
    }

    match cli.cmd {
        Cmd::Run(c) => Ok(run().await?),
    }
}

async fn run() -> ResultOrAnyErr<()> {
    println!("run: start ...");
    for i in 1..10 {
        println!("run: {}", i);
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    println!("run: done.");

    Ok(())
}
