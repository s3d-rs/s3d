use aws_smithy_http::byte_stream::ByteStream;
use serde::Deserialize;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::read_to_string;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

pub fn new_s3d_client() -> aws_sdk_s3::Client {
    aws_sdk_s3::Client::from_conf({
        let ep = aws_sdk_s3::Endpoint::immutable(
            hyper::Uri::from_str("http://localhost:33333").unwrap(),
        );
        let creds = aws_sdk_s3::Credentials::new("s3d", "s3d", None, None, "s3d");
        let region = aws_sdk_s3::Region::new("s3d");
        aws_sdk_s3::Config::builder()
            .endpoint_resolver(ep)
            .credentials_provider(creds)
            .region(region)
            .build()
    })
}

pub fn parse_bucket_and_key(s: &str) -> anyhow::Result<(String, String)> {
    let mut parts = s.splitn(2, '/');
    let bucket = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing bucket"))?;
    let key = parts.next().ok_or_else(|| anyhow::anyhow!("Missing key"))?;
    Ok((String::from(bucket), String::from(key)))
}

pub fn parse_bucket_and_prefix(s: &str) -> anyhow::Result<(String, String)> {
    let mut parts = s.splitn(2, '/');
    let bucket = parts.next().unwrap_or("");
    let key = parts.next().unwrap_or("");
    Ok((String::from(bucket), String::from(key)))
}

pub async fn pipe_stream<I, O, E>(input: &mut I, output: &mut O) -> anyhow::Result<u64>
where
    I: tokio_stream::Stream<Item = Result<bytes::Bytes, E>> + std::marker::Unpin,
    O: tokio::io::AsyncWrite + std::marker::Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut num_bytes: u64 = 0;
    while let Some(ref mut buf) = input.try_next().await? {
        num_bytes += buf.len() as u64;
        output.write_all_buf(buf).await?;
    }
    Ok(num_bytes)
}

pub async fn byte_stream_from_stdin() -> ByteStream {
    let stdin_file = tokio::fs::File::from_std(unsafe { std::fs::File::from_raw_fd(0) });
    ByteStream::from_file(stdin_file).await.unwrap()
}

pub async fn read_yaml_file<T>(path: &Path) -> anyhow::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    Ok(serde_yaml::from_str(&read_to_string(path).await?)?)
}
