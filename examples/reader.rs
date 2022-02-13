//! s3d reader example is running a simple client that reads from s3d
//! with options to repeat/concurrency/more. It simulates a process
//! running on an edge server (perhaps as a trigger to object upload)
//! that reads data from s3d and processes it (AI/ML/etc).

use clap::Parser;
use hyper::Uri;
use std::str::FromStr;

#[macro_use]
extern crate log;

// #[macro_use]
// extern crate anyhow;

const S3D_ENDPOINT: &'static str = "http://localhost:33333";

#[derive(Parser, Debug)]
#[clap(name = "reader-example")]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
struct CLI {
    #[clap(long, short, default_value = S3D_ENDPOINT)]
    endpoint: String,

    #[clap(long, short)]
    bucket: String,

    #[clap(long, short, default_value = "")]
    prefix: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // init default log levels - override with RUST_LOG env var
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("warn,s3d=info,reader=info"),
    );

    // parse command line arguments
    let cli = CLI::parse();
    debug!("{:?}", cli);
    let endpoint = cli.endpoint.as_str();
    let bucket = cli.bucket.as_str();
    let prefix = cli.prefix.as_str();

    let s3c = aws_sdk_s3::Client::from_conf({
        let ep = aws_sdk_s3::Endpoint::immutable(Uri::from_str(endpoint).unwrap());
        let creds = aws_sdk_s3::Credentials::new("s3d", "s3d", None, None, "s3d");
        let region = aws_sdk_s3::Region::new("s3d");
        aws_sdk_s3::Config::builder()
            .endpoint_resolver(ep)
            .credentials_provider(creds)
            .region(region)
            .build()
    });

    let _r = s3c.head_bucket().bucket(bucket).send().await?;
    info!("head_bucket: OK bucket {} exists", bucket);

    info!("list_objects: bucket={} prefix={}", bucket, prefix);
    let r = s3c
        .list_objects()
        .bucket(bucket)
        .prefix(prefix)
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

    let key = "s3d/src/main.rs";
    info!("get_object: bucket={} key={}", bucket, key);
    let r = s3c.get_object().bucket(bucket).key(key).send().await?;
    let _data = r.body.collect().await?;

    Ok(())
}
