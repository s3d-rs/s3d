// use crate::build_gen::generate_code_for_each_s3_op;
use crate::utils::to_internal_err;
use std::future::Future;
use std::pin::Pin;

/// Why we need this TraitFuture:
/// We can't use async_trait macro inside our macro so we use the same thing it does
/// which is this pin-box-dyn-future - see long explanation here:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

pub type SMClient = aws_smithy_client::Client<
    aws_smithy_client::erase::DynConnector,
    aws_sdk_s3::middleware::DefaultMiddleware,
>;

macro_rules! s3_op_trait {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](&self, i: s3d_smithy_codegen_server_s3::input::[<$op Input>])
                -> TraitFuture<
                    s3d_smithy_codegen_server_s3::output::[<$op Output>],
                    s3d_smithy_codegen_server_s3::error::[<$op Error>],
                >;
        }
    };
}

macro_rules! s3_op_impl {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](&self, i: s3d_smithy_codegen_server_s3::input::[<$op Input>]) ->
                TraitFuture<
                    s3d_smithy_codegen_server_s3::output::[<$op Output>],
                    s3d_smithy_codegen_server_s3::error::[<$op Error>],
                >
            {
                Box::pin(async move {
                    info!("{}: {:?}", stringify!([<$op:snake>]), i);
                    let to_client = crate::build_gen::[<conv_to_client_ $op:snake _input>];
                    let from_client = crate::build_gen::[<conv_from_client_ $op:snake _output>];
                    let r = self.sm_client
                        .call(to_client(i).make_operation(self.s3_client.conf()).await.unwrap())
                        .await
                        .map(from_client)
                        .map_err(to_internal_err);
                    info!("{}: {:?}", stringify!([<$op:snake>]), r);
                    r
                })
            }
        }
    };
}

pub trait S3Api {
    // LIST OPS
    s3_op_trait!(ListBuckets);
    s3_op_trait!(ListObjects);
    s3_op_trait!(ListObjectsV2);
    s3_op_trait!(ListObjectVersions);
    // SIMPLE OBJECT OPS
    s3_op_trait!(HeadObject);
    s3_op_trait!(GetObject);
    s3_op_trait!(PutObject);
    s3_op_trait!(CopyObject);
    s3_op_trait!(DeleteObject);
    s3_op_trait!(DeleteObjects);
    s3_op_trait!(GetObjectTagging);
    s3_op_trait!(PutObjectTagging);
    s3_op_trait!(DeleteObjectTagging);
    // SIMPLE BUCKET OPS
    s3_op_trait!(HeadBucket);
    s3_op_trait!(CreateBucket);
    s3_op_trait!(DeleteBucket);
    s3_op_trait!(GetBucketTagging);
    s3_op_trait!(PutBucketTagging);
    s3_op_trait!(DeleteBucketTagging);
    // MULTIPART UPLOAD OPS
    s3_op_trait!(CreateMultipartUpload);
    s3_op_trait!(CompleteMultipartUpload);
    s3_op_trait!(AbortMultipartUpload);
    s3_op_trait!(ListMultipartUploads);
    s3_op_trait!(ListParts);
    s3_op_trait!(UploadPart);
    s3_op_trait!(UploadPartCopy);
    // ADVANCED OBJECT OPS
    s3_op_trait!(GetObjectAcl);
    s3_op_trait!(PutObjectAcl);
    s3_op_trait!(GetObjectLegalHold);
    s3_op_trait!(PutObjectLegalHold);
    s3_op_trait!(GetObjectLockConfiguration);
    s3_op_trait!(PutObjectLockConfiguration);
    s3_op_trait!(GetObjectRetention);
    s3_op_trait!(PutObjectRetention);
    s3_op_trait!(GetObjectTorrent);
    s3_op_trait!(RestoreObject);
    // ADVANCED BUCKET OPS
    s3_op_trait!(GetBucketAccelerateConfiguration);
    s3_op_trait!(GetBucketAcl);
    s3_op_trait!(GetBucketAnalyticsConfiguration);
    s3_op_trait!(GetBucketCors);
    s3_op_trait!(GetBucketEncryption);
    s3_op_trait!(GetBucketIntelligentTieringConfiguration);
    s3_op_trait!(GetBucketInventoryConfiguration);
    s3_op_trait!(GetBucketLifecycleConfiguration);
    s3_op_trait!(GetBucketLocation);
    s3_op_trait!(GetBucketLogging);
    s3_op_trait!(GetBucketMetricsConfiguration);
    s3_op_trait!(GetBucketNotificationConfiguration);
    s3_op_trait!(GetBucketOwnershipControls);
    s3_op_trait!(GetBucketPolicy);
    s3_op_trait!(GetBucketPolicyStatus);
    s3_op_trait!(GetBucketReplication);
    s3_op_trait!(GetBucketRequestPayment);
    s3_op_trait!(GetBucketVersioning);
    s3_op_trait!(GetBucketWebsite);
    s3_op_trait!(GetPublicAccessBlock);
    s3_op_trait!(PutBucketAccelerateConfiguration);
    s3_op_trait!(PutBucketAcl);
    s3_op_trait!(PutBucketAnalyticsConfiguration);
    s3_op_trait!(PutBucketCors);
    s3_op_trait!(PutBucketEncryption);
    s3_op_trait!(PutBucketIntelligentTieringConfiguration);
    s3_op_trait!(PutBucketInventoryConfiguration);
    s3_op_trait!(PutBucketLifecycleConfiguration);
    s3_op_trait!(PutBucketLogging);
    s3_op_trait!(PutBucketMetricsConfiguration);
    s3_op_trait!(PutBucketNotificationConfiguration);
    s3_op_trait!(PutBucketOwnershipControls);
    s3_op_trait!(PutBucketPolicy);
    s3_op_trait!(PutBucketReplication);
    s3_op_trait!(PutBucketRequestPayment);
    s3_op_trait!(PutBucketVersioning);
    s3_op_trait!(PutBucketWebsite);
    s3_op_trait!(PutPublicAccessBlock);
    s3_op_trait!(WriteGetObjectResponse);
    s3_op_trait!(DeleteBucketAnalyticsConfiguration);
    s3_op_trait!(DeleteBucketCors);
    s3_op_trait!(DeleteBucketEncryption);
    s3_op_trait!(DeleteBucketIntelligentTieringConfiguration);
    s3_op_trait!(DeleteBucketInventoryConfiguration);
    s3_op_trait!(DeleteBucketLifecycle);
    s3_op_trait!(DeleteBucketMetricsConfiguration);
    s3_op_trait!(DeleteBucketOwnershipControls);
    s3_op_trait!(DeleteBucketPolicy);
    s3_op_trait!(DeleteBucketReplication);
    s3_op_trait!(DeleteBucketWebsite);
    s3_op_trait!(DeletePublicAccessBlock);
    s3_op_trait!(ListBucketAnalyticsConfigurations);
    s3_op_trait!(ListBucketIntelligentTieringConfigurations);
    s3_op_trait!(ListBucketInventoryConfigurations);
    s3_op_trait!(ListBucketMetricsConfigurations);
}

pub struct S3ApiClient {
    sm_client: &'static SMClient,
    s3_client: &'static aws_sdk_s3::Client,

}

impl S3Api for S3ApiClient {
    // LIST OPS
    s3_op_impl!(ListBuckets);
    s3_op_impl!(ListObjects);
    s3_op_impl!(ListObjectsV2);
    s3_op_impl!(ListObjectVersions);
    // SIMPLE OBJECT OPS
    s3_op_impl!(HeadObject);
    s3_op_impl!(GetObject);
    s3_op_impl!(PutObject);
    s3_op_impl!(CopyObject);
    s3_op_impl!(DeleteObject);
    s3_op_impl!(DeleteObjects);
    s3_op_impl!(GetObjectTagging);
    s3_op_impl!(PutObjectTagging);
    s3_op_impl!(DeleteObjectTagging);
    // SIMPLE BUCKET OPS
    s3_op_impl!(HeadBucket);
    s3_op_impl!(CreateBucket);
    s3_op_impl!(DeleteBucket);
    s3_op_impl!(GetBucketTagging);
    s3_op_impl!(PutBucketTagging);
    s3_op_impl!(DeleteBucketTagging);
    // MULTIPART UPLOAD OPS
    s3_op_impl!(CreateMultipartUpload);
    s3_op_impl!(CompleteMultipartUpload);
    s3_op_impl!(AbortMultipartUpload);
    s3_op_impl!(ListMultipartUploads);
    s3_op_impl!(ListParts);
    s3_op_impl!(UploadPart);
    s3_op_impl!(UploadPartCopy);
    // ADVANCED OBJECT OPS
    s3_op_impl!(GetObjectAcl);
    s3_op_impl!(PutObjectAcl);
    s3_op_impl!(GetObjectLegalHold);
    s3_op_impl!(PutObjectLegalHold);
    s3_op_impl!(GetObjectLockConfiguration);
    s3_op_impl!(PutObjectLockConfiguration);
    s3_op_impl!(GetObjectRetention);
    s3_op_impl!(PutObjectRetention);
    s3_op_impl!(GetObjectTorrent);
    s3_op_impl!(RestoreObject);
    // ADVANCED BUCKET OPS
    s3_op_impl!(GetBucketAccelerateConfiguration);
    s3_op_impl!(GetBucketAcl);
    s3_op_impl!(GetBucketAnalyticsConfiguration);
    s3_op_impl!(GetBucketCors);
    s3_op_impl!(GetBucketEncryption);
    s3_op_impl!(GetBucketIntelligentTieringConfiguration);
    s3_op_impl!(GetBucketInventoryConfiguration);
    s3_op_impl!(GetBucketLifecycleConfiguration);
    s3_op_impl!(GetBucketLocation);
    s3_op_impl!(GetBucketLogging);
    s3_op_impl!(GetBucketMetricsConfiguration);
    s3_op_impl!(GetBucketNotificationConfiguration);
    s3_op_impl!(GetBucketOwnershipControls);
    s3_op_impl!(GetBucketPolicy);
    s3_op_impl!(GetBucketPolicyStatus);
    s3_op_impl!(GetBucketReplication);
    s3_op_impl!(GetBucketRequestPayment);
    s3_op_impl!(GetBucketVersioning);
    s3_op_impl!(GetBucketWebsite);
    s3_op_impl!(GetPublicAccessBlock);
    s3_op_impl!(PutBucketAccelerateConfiguration);
    s3_op_impl!(PutBucketAcl);
    s3_op_impl!(PutBucketAnalyticsConfiguration);
    s3_op_impl!(PutBucketCors);
    s3_op_impl!(PutBucketEncryption);
    s3_op_impl!(PutBucketIntelligentTieringConfiguration);
    s3_op_impl!(PutBucketInventoryConfiguration);
    s3_op_impl!(PutBucketLifecycleConfiguration);
    s3_op_impl!(PutBucketLogging);
    s3_op_impl!(PutBucketMetricsConfiguration);
    s3_op_impl!(PutBucketNotificationConfiguration);
    s3_op_impl!(PutBucketOwnershipControls);
    s3_op_impl!(PutBucketPolicy);
    s3_op_impl!(PutBucketReplication);
    s3_op_impl!(PutBucketRequestPayment);
    s3_op_impl!(PutBucketVersioning);
    s3_op_impl!(PutBucketWebsite);
    s3_op_impl!(PutPublicAccessBlock);
    s3_op_impl!(WriteGetObjectResponse);
    s3_op_impl!(DeleteBucketAnalyticsConfiguration);
    s3_op_impl!(DeleteBucketCors);
    s3_op_impl!(DeleteBucketEncryption);
    s3_op_impl!(DeleteBucketIntelligentTieringConfiguration);
    s3_op_impl!(DeleteBucketInventoryConfiguration);
    s3_op_impl!(DeleteBucketLifecycle);
    s3_op_impl!(DeleteBucketMetricsConfiguration);
    s3_op_impl!(DeleteBucketOwnershipControls);
    s3_op_impl!(DeleteBucketPolicy);
    s3_op_impl!(DeleteBucketReplication);
    s3_op_impl!(DeleteBucketWebsite);
    s3_op_impl!(DeletePublicAccessBlock);
    s3_op_impl!(ListBucketAnalyticsConfigurations);
    s3_op_impl!(ListBucketIntelligentTieringConfigurations);
    s3_op_impl!(ListBucketInventoryConfigurations);
    s3_op_impl!(ListBucketMetricsConfigurations);
}
