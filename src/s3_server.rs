use crate::write_queue::WriteQueue;
use s3d_smithy_codegen_server_s3::{
    // error::*,
    // model::*,
    input::*,
    operation_registry::*,
    output::*,
};

pub type Router = aws_smithy_http_server::Router<hyper::Body>;

pub type SMClient = aws_smithy_client::Client<
    aws_smithy_client::erase::DynConnector,
    aws_sdk_s3::middleware::DefaultMiddleware,
>;

pub async fn serve() -> anyhow::Result<()> {
    let s3_config = aws_config::load_from_env().await;
    let s3_client = Box::leak(Box::new(aws_sdk_s3::Client::new(&s3_config)));
    let sm_builder = aws_sdk_s3::client::Builder::dyn_https()
        .middleware(aws_sdk_s3::middleware::DefaultMiddleware::new());
    let sm_client = Box::leak(Box::new(sm_builder.build()));
    let write_queue = Box::leak(Box::new(WriteQueue { s3_client }));
    write_queue.start();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 33333));
    let router = build_router(sm_client, s3_client, write_queue);
    let server = hyper::Server::bind(&addr).serve(router.into_make_service());
    warn!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

pub fn build_router(
    sm_client: &'static SMClient,
    s3_client: &'static aws_sdk_s3::Client,
    write_queue: &'static WriteQueue,
) -> Router {
    let mut b = OperationRegistryBuilder::default();

    macro_rules! register_s3_gateway_op1 {
        ($op:ident) => {
            paste::paste! {
                b = b.[<$op:snake>](move |i: [<$op Input>]| async {
                    info!("[<$op:snake>]: {:?}", i);
                    type ServerInput = [<$op Input>];
                    type ServerOutput = [<$op Output>];
                    type ClientInput = aws_sdk_s3::input::[<$op Input>];
                    type ClientOutput = aws_sdk_s3::output::[<$op Output>];
                    // see https://github.com/awslabs/smithy-rs/issues/1099
                    let into_client = |i: ServerInput| -> ClientInput {
                        unsafe { std::mem::transmute::<ServerInput, ClientInput>(i) }
                    };
                    let into_server = |o: ClientOutput| -> ServerOutput {
                        unsafe { std::mem::transmute::<ClientOutput, ServerOutput>(o) }
                    };
                    let r = sm_client
                        .call(into_client(i).make_operation(s3_client.conf()).await.unwrap())
                        .await
                        .map(into_server)
                        .map_err(|err| {
                            todo!("unhandled error {:?}", err)
                            // match err {
                            //     SdkError::ServiceError { err, .. } => err,
                            //     _ => [<$op Error>]::unhandled(err),
                            // }
                        });
                    info!("[<$op:snake>]: {:?}", r);
                    r
                });
            }
        };
    }

    macro_rules! register_s3_gateway_op2 {
        ($op:ident) => {
            paste::paste! {
                b = b.[<$op:snake>](move |i: [<$op Input>]| async {
                    info!("[<$op:snake>]: {:?}", i);
                    type ServerInput = [<$op Input>];
                    type ServerOutput = [<$op Output>];
                    type ClientInput = aws_sdk_s3::input::[<$op Input>];
                    type ClientOutput = aws_sdk_s3::output::[<$op Output>];
                    // see https://github.com/awslabs/smithy-rs/issues/1099
                    let into_client = |i: ServerInput| -> ClientInput {
                        unsafe { std::mem::transmute::<ServerInput, ClientInput>(i) }
                    };
                    let into_server = |o: ClientOutput| -> ServerOutput {
                        unsafe { std::mem::transmute::<ClientOutput, ServerOutput>(o) }
                    };
                    let r = sm_client
                        .call(into_client(i).make_operation(s3_client.conf()).await.unwrap())
                        .await
                        .map(into_server)
                        .unwrap();
                    info!("[<$op:snake>]: {:?}", r);
                    r
                });
            }
        };
    }

    b = b.put_object(move |i: PutObjectInput| async {
        info!("put_object: {:?}", i);
        write_queue.put_object(i).await.unwrap()
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
        let into_client = |x: GetObjectInput| -> aws_sdk_s3::input::GetObjectInput {
            unsafe { std::mem::transmute::<GetObjectInput, aws_sdk_s3::input::GetObjectInput>(x) }
        };
        let into_server = |x: aws_sdk_s3::output::GetObjectOutput| -> GetObjectOutput {
            unsafe {
                std::mem::transmute::<aws_sdk_s3::output::GetObjectOutput, GetObjectOutput>(x)
            }
        };
        let r = sm_client
            .call(
                into_client(i2)
                    .make_operation(s3_client.conf())
                    .await
                    .unwrap(),
            )
            .await
            .map(into_server)
            .map_err(|err| todo!("unhandled error {:?}", err));
        info!("get_object: read from remote {:?}", r);
        r
    });

    // LIST OPS
    register_s3_gateway_op2!(ListBuckets);
    register_s3_gateway_op1!(ListObjects);
    register_s3_gateway_op1!(ListObjectsV2);
    register_s3_gateway_op2!(ListObjectVersions);
    // SIMPLE OBJECT OPS
    register_s3_gateway_op1!(HeadObject);
    register_s3_gateway_op1!(CopyObject);
    register_s3_gateway_op2!(DeleteObject);
    register_s3_gateway_op2!(DeleteObjects);
    register_s3_gateway_op2!(GetObjectTagging);
    register_s3_gateway_op2!(PutObjectTagging);
    register_s3_gateway_op2!(DeleteObjectTagging);
    // SIMPLE BUCKET OPS
    register_s3_gateway_op1!(HeadBucket);
    register_s3_gateway_op1!(CreateBucket);
    register_s3_gateway_op2!(DeleteBucket);
    register_s3_gateway_op2!(GetBucketTagging);
    register_s3_gateway_op2!(PutBucketTagging);
    register_s3_gateway_op2!(DeleteBucketTagging);
    // MULTIPART UPLOAD OPS
    register_s3_gateway_op2!(CreateMultipartUpload);
    register_s3_gateway_op2!(CompleteMultipartUpload);
    register_s3_gateway_op1!(AbortMultipartUpload);
    register_s3_gateway_op2!(ListMultipartUploads);
    register_s3_gateway_op2!(ListParts);
    register_s3_gateway_op2!(UploadPart);
    register_s3_gateway_op2!(UploadPartCopy);
    // ADVANCED OBJECT OPS
    register_s3_gateway_op1!(GetObjectAcl);
    register_s3_gateway_op1!(PutObjectAcl);
    register_s3_gateway_op2!(GetObjectLegalHold);
    register_s3_gateway_op2!(PutObjectLegalHold);
    register_s3_gateway_op2!(GetObjectLockConfiguration);
    register_s3_gateway_op2!(PutObjectLockConfiguration);
    register_s3_gateway_op2!(GetObjectRetention);
    register_s3_gateway_op2!(PutObjectRetention);
    register_s3_gateway_op2!(GetObjectTorrent);
    register_s3_gateway_op1!(RestoreObject);
    // ADVANCED BUCKET OPS
    register_s3_gateway_op2!(GetBucketAccelerateConfiguration);
    register_s3_gateway_op2!(GetBucketAcl);
    register_s3_gateway_op2!(GetBucketAnalyticsConfiguration);
    register_s3_gateway_op2!(GetBucketCors);
    register_s3_gateway_op2!(GetBucketEncryption);
    register_s3_gateway_op2!(GetBucketIntelligentTieringConfiguration);
    register_s3_gateway_op2!(GetBucketInventoryConfiguration);
    register_s3_gateway_op2!(GetBucketLifecycleConfiguration);
    register_s3_gateway_op2!(GetBucketLocation);
    register_s3_gateway_op2!(GetBucketLogging);
    register_s3_gateway_op2!(GetBucketMetricsConfiguration);
    register_s3_gateway_op2!(GetBucketNotificationConfiguration);
    register_s3_gateway_op2!(GetBucketOwnershipControls);
    register_s3_gateway_op2!(GetBucketPolicy);
    register_s3_gateway_op2!(GetBucketPolicyStatus);
    register_s3_gateway_op2!(GetBucketReplication);
    register_s3_gateway_op2!(GetBucketRequestPayment);
    register_s3_gateway_op2!(GetBucketVersioning);
    register_s3_gateway_op2!(GetBucketWebsite);
    register_s3_gateway_op2!(GetPublicAccessBlock);
    register_s3_gateway_op2!(PutBucketAccelerateConfiguration);
    register_s3_gateway_op2!(PutBucketAcl);
    register_s3_gateway_op2!(PutBucketAnalyticsConfiguration);
    register_s3_gateway_op2!(PutBucketCors);
    register_s3_gateway_op2!(PutBucketEncryption);
    register_s3_gateway_op2!(PutBucketIntelligentTieringConfiguration);
    register_s3_gateway_op2!(PutBucketInventoryConfiguration);
    register_s3_gateway_op2!(PutBucketLifecycleConfiguration);
    register_s3_gateway_op2!(PutBucketLogging);
    register_s3_gateway_op2!(PutBucketMetricsConfiguration);
    register_s3_gateway_op2!(PutBucketNotificationConfiguration);
    register_s3_gateway_op2!(PutBucketOwnershipControls);
    register_s3_gateway_op2!(PutBucketPolicy);
    register_s3_gateway_op2!(PutBucketReplication);
    register_s3_gateway_op2!(PutBucketRequestPayment);
    register_s3_gateway_op2!(PutBucketVersioning);
    register_s3_gateway_op2!(PutBucketWebsite);
    register_s3_gateway_op2!(PutPublicAccessBlock);
    register_s3_gateway_op2!(WriteGetObjectResponse);
    register_s3_gateway_op2!(DeleteBucketAnalyticsConfiguration);
    register_s3_gateway_op2!(DeleteBucketCors);
    register_s3_gateway_op2!(DeleteBucketEncryption);
    register_s3_gateway_op2!(DeleteBucketIntelligentTieringConfiguration);
    register_s3_gateway_op2!(DeleteBucketInventoryConfiguration);
    register_s3_gateway_op2!(DeleteBucketLifecycle);
    register_s3_gateway_op2!(DeleteBucketMetricsConfiguration);
    register_s3_gateway_op2!(DeleteBucketOwnershipControls);
    register_s3_gateway_op2!(DeleteBucketPolicy);
    register_s3_gateway_op2!(DeleteBucketReplication);
    register_s3_gateway_op2!(DeleteBucketWebsite);
    register_s3_gateway_op2!(DeletePublicAccessBlock);
    register_s3_gateway_op2!(ListBucketAnalyticsConfigurations);
    register_s3_gateway_op2!(ListBucketIntelligentTieringConfigurations);
    register_s3_gateway_op2!(ListBucketInventoryConfigurations);
    register_s3_gateway_op2!(ListBucketMetricsConfigurations);

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
