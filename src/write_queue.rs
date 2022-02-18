use crate::utils::pipe_stream;
use crate::fsnotify::consume_fs_events;
use aws_smithy_http::byte_stream::ByteStream;
use s3d_smithy_codegen_server_s3::{error::*, input::*, output::*};
use std::path::Path;
use tokio::{
    fs::{read_to_string, File},
    io::AsyncWriteExt,
};

const S3D_WRITE_QUEUE_DIR: &str = ".s3d/write_queue";

pub struct WriteQueue {
    pub s3_client: &'static aws_sdk_s3::Client,
}

impl WriteQueue {

    pub fn start(&'static self) {
        tokio::spawn(consume_fs_events(S3D_WRITE_QUEUE_DIR, self));
    }

    pub async fn push_file(&self, entry_name: &str) -> anyhow::Result<()> {
        let bucket_path_cow = urlencoding::decode(entry_name).unwrap();
        let bucket_path = bucket_path_cow.as_ref();
        info!("Write queue item: {:?}", bucket_path);
        let mut parts = bucket_path.splitn(2, '/');
        let bucket = parts.next().unwrap();
        let key = parts.next().unwrap();
        let fname = format!("{}/{}", S3D_WRITE_QUEUE_DIR, entry_name);
        let body = ByteStream::from_path(Path::new(&fname)).await?;
        self.s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;
        tokio::fs::remove_file(fname).await?;
        info!("Write queue item: {:?}", bucket_path);
        Ok(())
    }

    pub async fn put_object(
        &self,
        mut i: PutObjectInput,
    ) -> Result<PutObjectOutput, anyhow::Error> {
        let fname = self.to_file_name(i.bucket(), i.key());
        let mut file = File::create(fname).await?;
        let _num_bytes = pipe_stream(&mut i.body, &mut file).await?;
        file.flush().await?;
        file.sync_all().await?;
        file.shutdown().await?;
        Ok(PutObjectOutput::builder().e_tag("s3d-etag").build())
    }

    pub async fn get_object(
        &self,
        mut i: GetObjectInput,
    ) -> Result<GetObjectOutput, GetObjectError> {
        let fname = self.to_file_name(i.bucket(), i.key());
        let stream = ByteStream::from_path(Path::new(&fname))
            .await
            .map_err(|err| GetObjectError::NoSuchKey(NoSuchKey::builder().build()))?;
        Ok(GetObjectOutput::builder().set_body(Some(stream)).build())
    }

    pub async fn head_object(
        &self,
        mut i: HeadObjectInput,
    ) -> Result<HeadObjectOutput, HeadObjectError> {
        // let fname = self.to_file_name(i.bucket(), i.key()) + ".s3d-object-md.yaml";
        Ok(HeadObjectOutput::builder().build())
    }

    pub fn to_file_name(&self, bucket: &str, key: &str) -> String {
        format!(
            "{}/{}",
            S3D_WRITE_QUEUE_DIR,
            urlencoding::encode(&format!("{}/{}", bucket, key))
        )
    }
}
