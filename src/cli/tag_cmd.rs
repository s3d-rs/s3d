use crate::utils::{new_s3_client, parse_bucket_and_key};

/// Get or set tags for bucket or object
#[derive(clap::Parser, Debug, Clone)]
pub struct TagCmd {
    /// Set tags for `bucket` or `bucket/key`
    #[clap(name = "bucket[/key]")]
    bucket_and_key: String,

    /// Tag `name=value`. Can be used multiple times.
    #[clap(long, short, multiple_occurrences(true))]
    tag: Option<Vec<String>>,

    /// Reset previous tags instead of appending
    #[clap(long, short)]
    reset: bool,
}

impl TagCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = new_s3_client().await;
        let (bucket, key) = parse_bucket_and_key(&self.bucket_and_key)?;
        let tagging = aws_sdk_s3::model::Tagging::builder()
            .set_tag_set(self.tag.clone().map(|v| {
                v.iter()
                    .map(|t| {
                        let mut parts = t.splitn(2, '=');
                        let k = parts.next().map(String::from);
                        let v = parts.next().map(String::from);
                        aws_sdk_s3::model::Tag::builder()
                            .set_key(k)
                            .set_value(v)
                            .build()
                    })
                    .collect::<Vec<_>>()
            }))
            .build();

        if tagging.tag_set().is_none() {
            if key.is_empty() {
                let res = s3.get_bucket_tagging().bucket(bucket).send().await?;
                info!("{:#?}", res);
            } else {
                let res = s3
                    .get_object_tagging()
                    .bucket(bucket)
                    .key(key)
                    .send()
                    .await?;
                info!("{:#?}", res);
            }
        } else {
            if key.is_empty() {
                let res = s3
                    .put_bucket_tagging()
                    .bucket(bucket)
                    .tagging(tagging)
                    .send()
                    .await?;
                info!("{:#?}", res);
            } else {
                let res = s3
                    .put_object_tagging()
                    .bucket(bucket)
                    .key(key)
                    .tagging(tagging)
                    .send()
                    .await?;
                info!("{:#?}", res);
            }
        }

        Ok(())
    }
}
