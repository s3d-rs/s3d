extern crate s3d;
use clap::Parser;
use std::fmt::Debug;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    CLI::parse().run().await
}

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3c")]
#[clap(about = clap::crate_description!())]
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
            Cmd::Remote(cmd) => cmd.run().await,
            Cmd::Status(cmd) => cmd.run().await,
            Cmd::List(cmd) => cmd.run().await,
            Cmd::Get(cmd) => cmd.run().await,
            Cmd::Put(cmd) => cmd.run().await,
            Cmd::Tag(cmd) => cmd.run().await,
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Api(s3d::cli::api_cmd::ApiCmd),
    Remote(s3d::cli::remote_cmd::RemoteCmd),
    Status(s3d::cli::status_cmd::StatusCmd),
    List(s3d::cli::list_cmd::ListCmd),
    Get(s3d::cli::get_cmd::GetCmd),
    Put(s3d::cli::put_cmd::PutCmd),
    Tag(s3d::cli::tag_cmd::TagCmd),
}

// #[clap(about = "Init sets up config and local store for the daemon")]
// #[clap(about = "Fetch metadata only from remote")]
// #[clap(about = "Pull changes from remote")]
// #[clap(about = "Push changes to remote")]
// #[clap(about = "Prune objects from local store")]
// #[clap(about = "Diff shows objects pending for pull/push")]
