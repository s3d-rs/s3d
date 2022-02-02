use crate::utils::pipe_stream;
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
        tokio::spawn(self.worker());
    }

    pub async fn worker(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
            if let Err(err) = self.work().await {
                debug!("{}", err);
            }
        }
    }

    pub async fn work(&self) -> anyhow::Result<()> {
        debug!("Write queue worker running ...");
        let mut queue = tokio::fs::read_dir(S3D_WRITE_QUEUE_DIR).await?;
        while let Some(entry) = queue.next_entry().await? {
            let entry_name_os = entry.file_name();
            let entry_name = entry_name_os.to_str().unwrap();
            if let Err(err) = self.push_file(entry_name).await {
                warn!("{}", err);
            }
        }
        Ok(())
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
        self.s3_client.put_object().bucket(bucket).key(key).body(body).send().await?;
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
            .map_err(|err| {
                GetObjectError::NoSuchKey(NoSuchKey::builder().build())
            })?;
        Ok(GetObjectOutput::builder().set_body(Some(stream)).build())
    }

    pub async fn head_object(
        &self,
        mut i: HeadObjectInput,
    ) -> Result<HeadObjectOutput, HeadObjectError> {
        // let fname = self.to_file_name(i.bucket(), i.key()) + ".s3d-object-md.yaml";
        Ok(HeadObjectOutput::builder().build())
    }

    pub fn to_file_name(&self, bucket: Option<&str>, key: Option<&str>) -> String {
        format!(
            "{}/{}",
            S3D_WRITE_QUEUE_DIR,
            urlencoding::encode(&format!("{}/{}", bucket.unwrap(), key.unwrap()))
        )
    }
}
