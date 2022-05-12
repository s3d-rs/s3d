extern crate s3d;
use clap::{IntoApp, Parser};
use std::fmt::Debug;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    env_logger::init();
    CLI::parse().run().await
}

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3")]
#[clap(about = "S3 CLI tool for applications or services that need to access S3 buckets (with/out the s3d daemon)")]
#[clap(version = clap::crate_version!())]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// subcommand
    #[clap(subcommand)]
    cmd: Cmd,
}

impl CLI {
    pub async fn run(self) -> anyhow::Result<()> {
        log::debug!("{:?}", self);
        match self.cmd {
            Cmd::Api(cmd) => cmd.run().await,
            Cmd::List(cmd) => cmd.run().await,
            Cmd::Get(cmd) => cmd.run().await,
            Cmd::Put(cmd) => cmd.run().await,
            // Cmd::Tag(cmd) => cmd.run().await,
            // Cmd::Remote(cmd) => cmd.run().await,
            // Cmd::Status(cmd) => cmd.run().await,
            Cmd::Completion(cmd) => cmd.run(CLI::command()).await,
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Api(s3d::cli::api_cmd::ApiCmd),
    List(s3d::cli::list_cmd::ListCmd),
    Get(s3d::cli::get_cmd::GetCmd),
    Put(s3d::cli::put_cmd::PutCmd),
    // Tag(s3d::cli::tag_cmd::TagCmd),
    // Remote(s3d::cli::remote_cmd::RemoteCmd),
    // Status(s3d::cli::status_cmd::StatusCmd),
    Completion(s3d::cli::completion_cmd::CompletionCmd),
}

// #[clap(about = "Init sets up config and local store for the daemon")]
// #[clap(about = "Fetch metadata only from remote")]
// #[clap(about = "Pull changes from remote")]
// #[clap(about = "Push changes to remote")]
// #[clap(about = "Prune objects from local store")]
// #[clap(about = "Diff shows objects pending for pull/push")]
