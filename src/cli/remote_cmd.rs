// use crate::utils::{new_s3d_client, parse_bucket_and_key};

/// Manage S3 remotes
#[derive(clap::Parser, Debug, Clone)]
pub struct RemoteCmd {}

impl RemoteCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        error!("TODO - not yet implemented");
        Ok(())
    }
}
