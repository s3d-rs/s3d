use crate::s3_server;

/// Run the daemon
#[derive(clap::Parser, Debug, Clone)]
pub struct RunCmd {}

impl RunCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        debug!("{:?}", self);
        s3_server::serve().await?;
        Ok(())
    }
}
