pub mod get;
pub mod list;
pub mod put;
pub mod run;
pub mod tag;
pub mod status;

use clap::Parser;
use std::fmt::Debug;

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = clap::crate_name!())]
#[clap(about = clap::crate_description!())]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// subcommands
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Run(self::run::RunCmd),
    Status(self::status::StatusCmd),
    List(self::list::ListCmd),
    Get(self::get::GetCmd),
    Put(self::put::PutCmd),
    Tag(self::tag::TagCmd),
}

impl CLI {
    pub async fn run() -> anyhow::Result<()> {
        let cli = CLI::parse();
        debug!("{:?}", cli);
        match cli.cmd {
            Cmd::Run(cmd) => cmd.run().await,
            Cmd::Status(cmd) => cmd.run().await,
            Cmd::List(cmd) => cmd.run().await,
            Cmd::Get(cmd) => cmd.run().await,
            Cmd::Put(cmd) => cmd.run().await,
            Cmd::Tag(cmd) => cmd.run().await,
        }
    }
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
