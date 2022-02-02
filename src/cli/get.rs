use crate::utils::{new_s3d_client, parse_bucket_and_key, pipe_stream};

/// Get object data to stdout, and meta-data and tags to stderr
#[derive(clap::Parser, Debug, Clone)]
pub struct GetCmd {
    /// Get object from `bucket/key`
    #[clap(name = "bucket/key")]
    bucket_and_key: String,
}

impl GetCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        debug!("{:?}", self);

        let s3 = new_s3d_client();
        let (bucket, key) = parse_bucket_and_key(&self.bucket_and_key)?;

        let mut res = s3.get_object().bucket(bucket).key(key).send().await?;
        info!("{:#?}", res);

        let num_bytes = pipe_stream(&mut res.body, &mut tokio::io::stdout()).await?;
        info!("Received {} bytes", num_bytes);

        Ok(())
    }
}
