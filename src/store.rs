use codegen_server_s3::{error::*, input::*, output::*, ByteStream};
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;

pub async fn get_object(mut i: GetObjectInput) -> Result<GetObjectOutput, GetObjectError> {
    warn!("-----------------------");
    warn!("------ S3D STORE ------");
    warn!("-----------------------");
    let fname = to_file_name(i.bucket(), i.key());
    let stream = ByteStream::from_path(Path::new(&fname))
        .await
        .map_err(|err| {
            error!("{}", err);
            GetObjectError::NoSuchKey(NoSuchKey::builder().build())
        })?;
    Ok(GetObjectOutput::builder().set_body(Some(stream)).build())
}

pub async fn put_object(mut i: PutObjectInput) -> anyhow::Result<PutObjectOutput> {
    warn!("-----------------------");
    warn!("------ S3D STORE ------");
    warn!("-----------------------");
    let fname = to_file_name(i.bucket(), i.key());
    let mut file = File::create(fname).await?;
    while let Some(v) = i.body.next().await {
        file.write_all(&v?).await?;
    }
    file.flush().await?;
    file.sync_all().await?;
    file.shutdown().await?;
    Ok(PutObjectOutput::builder().e_tag("s3d-etag-todo").build())
}

pub fn to_file_name(bucket: Option<&str>, key: Option<&str>) -> String {
    format!(
        ".s3d/{}/{}",
        bucket.unwrap(),
        urlencoding::encode(key.unwrap())
    )
}
