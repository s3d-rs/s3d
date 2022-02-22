extern crate s3d;

use clap::Parser;
use std::fmt::Debug;

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3c")]
#[clap(about = clap::crate_description!())]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// subcommand
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Status(s3d::cli::status::StatusCmd),
    List(s3d::cli::list::ListCmd),
    Get(s3d::cli::get::GetCmd),
    Put(s3d::cli::put::PutCmd),
    Tag(s3d::cli::tag::TagCmd),
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    let cli = CLI::parse();
    log::debug!("{:?}", cli);
    match cli.cmd {
        Cmd::Status(cmd) => cmd.run().await?,
        Cmd::List(cmd) => cmd.run().await?,
        Cmd::Get(cmd) => cmd.run().await?,
        Cmd::Put(cmd) => cmd.run().await?,
        Cmd::Tag(cmd) => cmd.run().await?,
    }
    Ok(())
    // match CLI::run().await {
    //     Ok(_) => Ok(()),
    //     Err(err) => {
    //         log::error!("{}", err);
    //         std::process::exit(1);
    //     }
    // }
}

// enum Cmd {
//     ...
//     Init(InitCmd),
//     Fetch(FetchCmd),
//     Pull(PullCmd),
//     Push(PushCmd),
//     Prune(PruneCmd),
//     Diff(DiffCmd),
// }

// match cli.cmd {
//     ...
//     Cmd::Init(cmd) => cmd.run(conf).await,
//     Cmd::Fetch(cmd) => cmd.run(conf).await,
//     Cmd::Pull(cmd) => cmd.run(conf).await,
//     Cmd::Push(cmd) => cmd.run(conf).await,
//     Cmd::Prune(cmd) => cmd.run(conf).await,
//     Cmd::Diff(cmd) => cmd.run(conf).await,
//     cmd => bail!("CLI command not yet implemented: {:?}", cmd),
// }

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Init sets up config and local store for the daemon")]
// struct InitCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Fetch metadata only from remote")]
// struct FetchCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Pull changes from remote")]
// struct PullCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Push changes to remote")]
// struct PushCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Prune objects from local store")]
// struct PruneCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Diff shows objects pending for pull/push")]
// struct DiffCmd {}
