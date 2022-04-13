use crate::config;
use crate::utils::{staticify, to_internal_err};
use crate::write_queue::WriteQueue;
use s3d_smithy_codegen_server_s3::{input::*, operation_registry::*};

pub type Router = aws_smithy_http_server::Router<hyper::Body>;

pub type SMClient = aws_smithy_client::Client<
    aws_smithy_client::erase::DynConnector,
    aws_sdk_s3::middleware::DefaultMiddleware,
>;

pub async fn serve() -> anyhow::Result<()> {
    let s3_config = aws_config::load_from_env().await;
    let s3_client = staticify(aws_sdk_s3::Client::new(&s3_config));
    let sleep_impl = aws_smithy_async::rt::sleep::default_async_sleep();
    let sm_builder = aws_sdk_s3::client::Builder::dyn_https()
        .sleep_impl(sleep_impl)
        .middleware(aws_sdk_s3::middleware::DefaultMiddleware::new());
    let sm_client = staticify(sm_builder.build());
    let write_queue = staticify(WriteQueue {
        s3_client,
        write_queue_dir: config::S3D_WRITE_QUEUE_DIR.to_string(),
    });
    write_queue.start();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 33333));
    let router = build_router(sm_client, s3_client, write_queue);
    let server = hyper::Server::bind(&addr).serve(router.into_make_service());
    info!("###################################");
    info!("Listening on http://{}", addr);
    info!("###################################");
    server.await?;
    Ok(())
}

pub fn build_router(
    sm_client: &'static SMClient,
    s3_client: &'static aws_sdk_s3::Client,
    write_queue: &'static WriteQueue,
) -> Router {
    let mut b = OperationRegistryBuilder::default();

    macro_rules! register_s3_gateway_op {
        ($op:ident) => {
            paste::paste! {
                b = b.[<$op:snake>](move |i: [<$op Input>]| async {
                    info!("{}: {:?}", stringify!([<$op:snake>]), i);
                    let to_client = crate::codegen_include::[<conv_to_client_ $op:snake _input>];
                    let from_client = crate::codegen_include::[<conv_from_client_ $op:snake _output>];
                    let r = sm_client
                        .call(to_client(i).make_operation(s3_client.conf()).await.unwrap())
                        .await
                        .map(from_client)
                        .map_err(to_internal_err);
                    info!("{}: {:?}", stringify!([<$op:snake>]), r);
                    r
                });
            }
        };
    }

    b = b.put_object(move |i: PutObjectInput| async {
        info!("put_object: {:?}", i);
        write_queue.put_object(i).await
    });

    b = b.get_object(move |i: GetObjectInput| async move {
        info!("get_object: {:?}", i);
        let i1 = i.to_owned();
        let i2 = i.to_owned();
        let qres = write_queue.get_object(i1).await;
        if qres.is_ok() {
            return qres;
        }
        info!("get_object: read from remote");
        let to_client = crate::codegen_include::conv_to_client_get_object_input;
        let from_client = crate::codegen_include::conv_from_client_get_object_output;
        let r = sm_client
            .call(
                to_client(i2)
                    .make_operation(s3_client.conf())
                    .await
                    .unwrap(),
            )
            .await
            .map(from_client)
            .map_err(to_internal_err);
        info!("get_object: read from remote {:?}", r);
        r
    });

    // LIST OPS
    register_s3_gateway_op!(ListBuckets);
    register_s3_gateway_op!(ListObjects);
    register_s3_gateway_op!(ListObjectsV2);
    register_s3_gateway_op!(ListObjectVersions);
    // SIMPLE OBJECT OPS
    register_s3_gateway_op!(HeadObject);
    register_s3_gateway_op!(CopyObject);
    register_s3_gateway_op!(DeleteObject);
    register_s3_gateway_op!(DeleteObjects);
    register_s3_gateway_op!(GetObjectTagging);
    register_s3_gateway_op!(PutObjectTagging);
    register_s3_gateway_op!(DeleteObjectTagging);
    // SIMPLE BUCKET OPS
    register_s3_gateway_op!(HeadBucket);
    register_s3_gateway_op!(CreateBucket);
    register_s3_gateway_op!(DeleteBucket);
    register_s3_gateway_op!(GetBucketTagging);
    register_s3_gateway_op!(PutBucketTagging);
    register_s3_gateway_op!(DeleteBucketTagging);
    // MULTIPART UPLOAD OPS
    register_s3_gateway_op!(CreateMultipartUpload);
    register_s3_gateway_op!(CompleteMultipartUpload);
    register_s3_gateway_op!(AbortMultipartUpload);
    register_s3_gateway_op!(ListMultipartUploads);
    register_s3_gateway_op!(ListParts);
    register_s3_gateway_op!(UploadPart);
    register_s3_gateway_op!(UploadPartCopy);
    // ADVANCED OBJECT OPS
    register_s3_gateway_op!(GetObjectAcl);
    register_s3_gateway_op!(PutObjectAcl);
    register_s3_gateway_op!(GetObjectLegalHold);
    register_s3_gateway_op!(PutObjectLegalHold);
    register_s3_gateway_op!(GetObjectLockConfiguration);
    register_s3_gateway_op!(PutObjectLockConfiguration);
    register_s3_gateway_op!(GetObjectRetention);
    register_s3_gateway_op!(PutObjectRetention);
    register_s3_gateway_op!(GetObjectTorrent);
    register_s3_gateway_op!(RestoreObject);
    // ADVANCED BUCKET OPS
    register_s3_gateway_op!(GetBucketAccelerateConfiguration);
    register_s3_gateway_op!(GetBucketAcl);
    register_s3_gateway_op!(GetBucketAnalyticsConfiguration);
    register_s3_gateway_op!(GetBucketCors);
    register_s3_gateway_op!(GetBucketEncryption);
    register_s3_gateway_op!(GetBucketIntelligentTieringConfiguration);
    register_s3_gateway_op!(GetBucketInventoryConfiguration);
    register_s3_gateway_op!(GetBucketLifecycleConfiguration);
    register_s3_gateway_op!(GetBucketLocation);
    register_s3_gateway_op!(GetBucketLogging);
    register_s3_gateway_op!(GetBucketMetricsConfiguration);
    register_s3_gateway_op!(GetBucketNotificationConfiguration);
    register_s3_gateway_op!(GetBucketOwnershipControls);
    register_s3_gateway_op!(GetBucketPolicy);
    register_s3_gateway_op!(GetBucketPolicyStatus);
    register_s3_gateway_op!(GetBucketReplication);
    register_s3_gateway_op!(GetBucketRequestPayment);
    register_s3_gateway_op!(GetBucketVersioning);
    register_s3_gateway_op!(GetBucketWebsite);
    register_s3_gateway_op!(GetPublicAccessBlock);
    register_s3_gateway_op!(PutBucketAccelerateConfiguration);
    register_s3_gateway_op!(PutBucketAcl);
    register_s3_gateway_op!(PutBucketAnalyticsConfiguration);
    register_s3_gateway_op!(PutBucketCors);
    register_s3_gateway_op!(PutBucketEncryption);
    register_s3_gateway_op!(PutBucketIntelligentTieringConfiguration);
    register_s3_gateway_op!(PutBucketInventoryConfiguration);
    register_s3_gateway_op!(PutBucketLifecycleConfiguration);
    register_s3_gateway_op!(PutBucketLogging);
    register_s3_gateway_op!(PutBucketMetricsConfiguration);
    register_s3_gateway_op!(PutBucketNotificationConfiguration);
    register_s3_gateway_op!(PutBucketOwnershipControls);
    register_s3_gateway_op!(PutBucketPolicy);
    register_s3_gateway_op!(PutBucketReplication);
    register_s3_gateway_op!(PutBucketRequestPayment);
    register_s3_gateway_op!(PutBucketVersioning);
    register_s3_gateway_op!(PutBucketWebsite);
    register_s3_gateway_op!(PutPublicAccessBlock);
    register_s3_gateway_op!(WriteGetObjectResponse);
    register_s3_gateway_op!(DeleteBucketAnalyticsConfiguration);
    register_s3_gateway_op!(DeleteBucketCors);
    register_s3_gateway_op!(DeleteBucketEncryption);
    register_s3_gateway_op!(DeleteBucketIntelligentTieringConfiguration);
    register_s3_gateway_op!(DeleteBucketInventoryConfiguration);
    register_s3_gateway_op!(DeleteBucketLifecycle);
    register_s3_gateway_op!(DeleteBucketMetricsConfiguration);
    register_s3_gateway_op!(DeleteBucketOwnershipControls);
    register_s3_gateway_op!(DeleteBucketPolicy);
    register_s3_gateway_op!(DeleteBucketReplication);
    register_s3_gateway_op!(DeleteBucketWebsite);
    register_s3_gateway_op!(DeletePublicAccessBlock);
    register_s3_gateway_op!(ListBucketAnalyticsConfigurations);
    register_s3_gateway_op!(ListBucketIntelligentTieringConfigurations);
    register_s3_gateway_op!(ListBucketInventoryConfigurations);
    register_s3_gateway_op!(ListBucketMetricsConfigurations);

    let ops = b.build().unwrap();

    {
        #[rustfmt::skip]
        let _: &OperationRegistry<
            hyper::Body,
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, ()> = &ops;
    }

    let router = Router::from(ops);
    router
}
