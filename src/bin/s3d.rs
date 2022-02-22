extern crate s3d;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));
    #[cfg(feature = "fuse")]
    {
        s3d::fuse::Fuse::start_fuse_mount().await?;
    }
    s3d::s3::server::serve().await?;
    Ok(())
}
