use crate::utils::{byte_stream_from_stdin, new_s3d_client, parse_bucket_and_key};

/// Put object from stdin
#[derive(clap::Parser, Debug, Clone)]
pub struct PutCmd {
    /// Put object in `bucket/key`
    #[clap(name = "bucket/key")]
    bucket_and_key: String,
}

impl PutCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        debug!("{:?}", self);

        let s3 = new_s3d_client();
        let (bucket, key) = parse_bucket_and_key(&self.bucket_and_key)?;

        let body = byte_stream_from_stdin().await;
        let res = s3
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;
        info!("{:#?}", res);

        Ok(())
    }
}
