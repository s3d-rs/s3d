extern crate s3d;
use clap::Parser;
use std::fmt::Debug;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    Daemon::parse().run().await
}

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3d")]
#[clap(about = clap::crate_description!())]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct Daemon {}

impl Daemon {
    pub async fn run(self) -> anyhow::Result<()> {
        log::debug!("{:?}", self);
        #[cfg(feature = "fuse")]
        {
            s3d::fuse::Fuse::start_fuse_mount().await?;
        }
        s3d::s3::server::serve().await?;
        Ok(())
    }
}
