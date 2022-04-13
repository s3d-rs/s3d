use crate::config;
use aws_smithy_http::byte_stream::ByteStream;
use s3d_smithy_codegen_server_s3::error::InternalServerError;
use serde::Deserialize;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::{read_to_string, File};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

/// staticify uses Box::leak to make a struct with static lifetime.
/// This is useful for async flows that require structs to live throughout the flow,
/// where not releasing their memory is fine.
pub fn staticify<T>(x: T) -> &'static T {
    Box::leak(Box::new(x))
}

/// new_s3_client creates a new s3 client which defaults to connect to the local daemon.
pub async fn new_s3_client() -> aws_sdk_s3::Client {
    if config::S3_ENDPOINT.is_none() {
        let s3_config = aws_config::load_from_env().await;
        return aws_sdk_s3::Client::new(&s3_config);
    }

    aws_sdk_s3::Client::from_conf({
        let ep = aws_sdk_s3::Endpoint::immutable(
            hyper::Uri::from_str(config::S3_ENDPOINT.as_ref().unwrap()).unwrap(),
        );
        let creds = aws_sdk_s3::Credentials::new("s3d", "s3d", None, None, "s3d");
        let region = aws_sdk_s3::Region::new("s3d");
        let sleep_impl = aws_smithy_async::rt::sleep::default_async_sleep().unwrap();
        aws_sdk_s3::Config::builder()
            .sleep_impl(sleep_impl)
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

pub async fn read_file_as_stream(fname: &str) -> anyhow::Result<ByteStream> {
    Ok(ByteStream::from_path(Path::new(&fname)).await?)
}

pub async fn write_stream_to_file(fname: &str, stream: &mut ByteStream) -> anyhow::Result<u64> {
    let mut file = File::create(fname).await?;
    let num_bytes = pipe_stream(stream, &mut file).await?;
    file.flush().await?;
    file.sync_all().await?;
    file.shutdown().await?;
    Ok(num_bytes)
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

pub async fn pipe_stream_to_outfile_or_stdout<I, E>(
    input: &mut I,
    outfile: Option<&str>,
) -> anyhow::Result<u64>
where
    I: tokio_stream::Stream<Item = Result<bytes::Bytes, E>> + std::marker::Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    match outfile {
        Some(ref path) => {
            let mut file = tokio::fs::File::create(path).await?;
            pipe_stream(input, &mut file).await
        }
        None => {
            let mut out = tokio::io::stdout();
            pipe_stream(input, &mut out).await
        }
    }
}

pub async fn byte_stream_from_infile_or_stdin(infile: Option<&str>) -> anyhow::Result<ByteStream> {
    let file = match infile {
        Some(ref path) => tokio::fs::File::open(path).await?,
        None => tokio::fs::File::from_std(unsafe { std::fs::File::from_raw_fd(0) }),
    };
    let stream = ByteStream::read_from().file(file).build().await?;
    Ok(stream)
}

pub async fn _read_yaml_file<T>(path: &Path) -> anyhow::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    Ok(serde_yaml::from_str(&read_to_string(path).await?)?)
}

pub fn to_internal_err<F: ToString, T: From<InternalServerError>>(err: F) -> T {
    InternalServerError {
        message: err.to_string(),
    }
    .into()
}
