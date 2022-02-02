// use crate::utils::{new_s3d_client, parse_bucket_and_key};

/// Status of the daemon and the data
#[derive(clap::Parser, Debug, Clone)]
pub struct StatusCmd {
    /// When empty checks the daemon status.
    /// Otherwise checks a bucket or object status (`bucket` or `bucket/key`)
    #[clap(name = "bucket[/key]", default_value = "")]
    bucket_and_key: String,
}

impl StatusCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        debug!("{:?}", self);

        // let s3 = new_s3d_client();
        // let (bucket, prefix) = parse_bucket_and_key(&self.bucket_and_key)?;

        error!("TODO - not yet implemented");

        Ok(())
    }
}
