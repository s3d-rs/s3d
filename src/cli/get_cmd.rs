use crate::utils::{new_s3_client, parse_bucket_and_key, pipe_stream_to_outfile_or_stdout};

/// Get object data to stdout, and meta-data and tags to stderr
#[derive(clap::Parser, Debug, Clone)]
pub struct GetCmd {
    /// Get object from `bucket/key`
    #[clap(name = "bucket/key")]
    bucket_and_key: String,

    /// Output file name, if not specified, stdout is used
    #[clap(name = "outfile")]
    outfile: Option<String>,
}

impl GetCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = new_s3_client().await;
        let (bucket, key) = parse_bucket_and_key(&self.bucket_and_key)?;
        let mut res = s3.get_object().bucket(bucket).key(key).send().await?;

        info!("{:#?}", res);

        let num_bytes = pipe_stream_to_outfile_or_stdout(&mut res.body, self.outfile.as_deref()).await?;

        info!("Received {} bytes", num_bytes);

        Ok(())
    }
}
