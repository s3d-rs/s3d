//! s3d writer example is running a simple client that writes to s3d
//! with options to repeat/concurrency/more. It simulates an edge device
//! such as a sensor or a camera that sends data to s3d.

use bytes::BytesMut;
use clap::Parser;
use codegen_client_s3::{Builder, Client, Config, ByteStream};
use hyper::Uri;
use std::str::FromStr;
// use aws_sdk_s3::{ByteStream, Credentials, Endpoint, Region};

#[macro_use]
extern crate log;

// #[macro_use]
// extern crate anyhow;

const S3D_ENDPOINT: &'static str = "http://localhost:33333";
const DEFAULT_PREFIX: &'static str = "s3d-example-writer/";

#[derive(Parser, Debug)]
#[clap(name = "writer-example")]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
struct CLI {
    #[clap(long, short, default_value = S3D_ENDPOINT)]
    endpoint: String,

    #[clap(long, short)]
    bucket: String,

    #[clap(long, short, default_value = DEFAULT_PREFIX)]
    prefix: String,

    #[clap(long, short, default_value = "3")]
    num_objects: usize,

    #[clap(long, short, default_value = "1024")]
    object_size: usize,

    #[clap(long, short, default_value = "1000")]
    delay_ms: u64,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // init default log levels - override with RUST_LOG env var
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("warn,s3d=info,writer=info"),
    );

    // parse command line arguments
    let cli = CLI::parse();
    debug!("{:?}", cli);
    let endpoint = cli.endpoint.as_str();
    let bucket = cli.bucket.as_str();
    let prefix = cli.prefix.as_str();

    // let s3c = aws_sdk_s3::Client::from_conf({
    //     let ep = Endpoint::immutable(Uri::from_str(endpoint).unwrap());
    //     let creds = Credentials::new("s3d", "s3d", None, None, "s3d");
    //     let region = Region::new("s3d");
    //     aws_sdk_s3::Config::builder()
    //         .endpoint_resolver(ep)
    //         .credentials_provider(creds)
    //         .region(region)
    //         .build()
    // });

    let raw_client = Builder::dyn_https()
        .middleware_fn(|r| r)
        .build();
    let config = Config::builder().build();
    let s3c = Client::with_config(raw_client, config);

    let _r = s3c.head_bucket().bucket(bucket).send().await?;
    info!("head_bucket: OK bucket {} exists", bucket);

    let date_prefix = chrono::Utc::now().format("%Y-%m-%d/%H-%M-%S").to_string();
    let sub_prefix = format!("{}{}/", prefix, date_prefix);

    for i in 1..cli.num_objects + 1 {
        let key = format!("{}object-{}", sub_prefix, i);
        info!(
            "put_object: bucket={} key={} size={}",
            bucket, key, cli.object_size
        );
        let mut buf = BytesMut::with_capacity(cli.object_size);
        buf.resize(cli.object_size, b'3');
        let body = ByteStream::from(buf.freeze());
        let r = s3c
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;
        info!("put_object: OK {:?}", r);
        tokio::time::sleep(std::time::Duration::from_millis(cli.delay_ms)).await;
    }

    info!("list_objects: bucket={} prefix={}", bucket, prefix);
    let r = s3c
        .list_objects()
        .bucket(bucket)
        .prefix(sub_prefix)
        .delimiter("/")
        .send()
        .await?;
    println!("list_objects: OK is_truncated={}", r.is_truncated());
    for it in r.contents().unwrap_or_default() {
        println!("- key: {}", it.key().unwrap_or_default());
    }
    for it in r.common_prefixes().unwrap_or_default() {
        println!("- prefix: {}", it.prefix().unwrap_or_default());
    }

    Ok(())
}
