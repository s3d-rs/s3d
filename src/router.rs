use crate::store;
use aws_smithy_types::date_time::{DateTime, Format};
use codegen_server_s3::{error::*, input::*, model::*, operation_registry::*, output::*};
use std::{future::Future, time::SystemTime};

pub type Router = aws_smithy_http_server::Router<hyper::Body>;

pub async fn serve() -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 33333));
    let router = build_router();
    let server = hyper::Server::bind(&addr).serve(router.into_make_service());
    info!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

pub fn build_router() -> Router {
    let ops = OperationRegistryBuilder::default()
        .get_object(|i: GetObjectInput| async move {
            info!("get_object: {:?}", i);
            store::get_object(i).await
        })
        .put_object(|i: PutObjectInput| async move {
            info!("put_object: {:?}", i);
            store::put_object(i).await.unwrap()
        })
        .get_object_tagging(|i: GetObjectTaggingInput| async move {
            info!("get_object_tagging: {:?}", i);
            get_object_tagging_output::Builder::default().build()
        })
        .delete_object(|i: DeleteObjectInput| async move {
            info!("delete_object: {:?}", i);
            delete_object_output::Builder::default().build()
        })
        .delete_objects(|i: DeleteObjectsInput| async move {
            info!("delete_objects: {:?}", i);
            delete_objects_output::Builder::default().build()
        })
        .delete_object_tagging(|i: DeleteObjectTaggingInput| async move {
            info!("delete_object_tagging: {:?}", i);
            delete_object_tagging_output::Builder::default().build()
        })
        .head_bucket(|i: HeadBucketInput| async move {
            info!("head_bucket: {:?}", i);
            Ok(head_bucket_output::Builder::default().build())
        })
        .head_object(|i: HeadObjectInput| async move {
            info!("head_object: {:?}", i);
            Ok(head_object_output::Builder::default().build())
        })
        .list_buckets(|i: ListBucketsInput| async move {
            info!("list_buckets: {:?}", i);
            ListBucketsOutput::builder()
                .buckets(
                    Bucket::builder()
                        .name("bucket1")
                        .creation_date(SystemTime::now().into())
                        .build(),
                )
                .build()
        })
        .list_objects(|i: ListObjectsInput| async move {
            info!("list_objects: {:?}", i);
            Ok(ListObjectsOutput::builder()
                .contents(
                    Object::builder()
                        .key("object1")
                        .size(1)
                        .last_modified(SystemTime::now().into())
                        .build(),
                )
                .build())
        })
        .list_objects_v2(|i: ListObjectsV2Input| async move {
            info!("list_objects_v2: {:?}", i);
            Ok(list_objects_v2_output::Builder::default().build())
        })
        .list_object_versions(|i: ListObjectVersionsInput| async move {
            info!("list_object_versions: {:?}", i);
            list_object_versions_output::Builder::default().build()
        })
        .list_parts(|i: ListPartsInput| async move {
            info!("list_parts: {:?}", i);
            list_parts_output::Builder::default().build()
        })
        .abort_multipart_upload(|i: AbortMultipartUploadInput| async move {
            info!("abort_multipart_upload: {:?}", i);
            Ok(AbortMultipartUploadOutput::builder().build())
        })
        .complete_multipart_upload(|i: CompleteMultipartUploadInput| async move {
            info!("complete_multipart_upload: {:?}", i);
            CompleteMultipartUploadOutput::builder().build()
        })
        .copy_object(|i: CopyObjectInput| async move {
            info!("copy_object: {:?}", i);
            Ok(CopyObjectOutput::builder().build())
        })
        .create_bucket(|i: CreateBucketInput| async move {
            info!("create_bucket: {:?}", i);
            Ok(CreateBucketOutput::builder().build())
        })
        .create_multipart_upload(|i: CreateMultipartUploadInput| async move {
            info!("create_multipart_upload: {:?}", i);
            CreateMultipartUploadOutput::builder().build()
        })
        .delete_bucket(|i: DeleteBucketInput| async move {
            info!("delete_bucket: {:?}", i);
            delete_bucket_output::Builder::default().build()
        })
        .delete_bucket_analytics_configuration(
            |i: DeleteBucketAnalyticsConfigurationInput| async move {
                info!("delete_bucket_analytics_configuration: {:?}", i);
                delete_bucket_analytics_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_cors(|i: DeleteBucketCorsInput| async move {
            info!("delete_bucket_cors: {:?}", i);
            delete_bucket_cors_output::Builder::default().build()
        })
        .delete_bucket_encryption(|i: DeleteBucketEncryptionInput| async move {
            info!("delete_bucket_encryption: {:?}", i);
            delete_bucket_encryption_output::Builder::default().build()
        })
        .delete_bucket_intelligent_tiering_configuration(
            |i: DeleteBucketIntelligentTieringConfigurationInput| async move {
                info!("delete_bucket_intelligent_tiering_configuration: {:?}", i);
                delete_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_inventory_configuration(
            |i: DeleteBucketInventoryConfigurationInput| async move {
                info!("delete_bucket_inventory_configuration: {:?}", i);
                delete_bucket_inventory_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_lifecycle(|i: DeleteBucketLifecycleInput| async move {
            info!("delete_bucket_lifecycle: {:?}", i);
            delete_bucket_lifecycle_output::Builder::default().build()
        })
        .delete_bucket_metrics_configuration(
            |i: DeleteBucketMetricsConfigurationInput| async move {
                info!("delete_bucket_metrics_configuration: {:?}", i);
                delete_bucket_metrics_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_ownership_controls(|i: DeleteBucketOwnershipControlsInput| async move {
            info!("delete_bucket_ownership_controls: {:?}", i);
            delete_bucket_ownership_controls_output::Builder::default().build()
        })
        .delete_bucket_policy(|i: DeleteBucketPolicyInput| async move {
            info!("delete_bucket_policy: {:?}", i);
            delete_bucket_policy_output::Builder::default().build()
        })
        .delete_bucket_replication(|i: DeleteBucketReplicationInput| async move {
            info!("delete_bucket_replication: {:?}", i);
            delete_bucket_replication_output::Builder::default().build()
        })
        .delete_bucket_tagging(|i: DeleteBucketTaggingInput| async move {
            info!("delete_bucket_tagging: {:?}", i);
            delete_bucket_tagging_output::Builder::default().build()
        })
        .delete_bucket_website(|i: DeleteBucketWebsiteInput| async move {
            info!("delete_bucket_website: {:?}", i);
            delete_bucket_website_output::Builder::default().build()
        })
        .delete_public_access_block(|i: DeletePublicAccessBlockInput| async move {
            info!("delete_public_access_block: {:?}", i);
            delete_public_access_block_output::Builder::default().build()
        })
        .get_bucket_accelerate_configuration(
            |i: GetBucketAccelerateConfigurationInput| async move {
                info!("get_bucket_accelerate_configuration: {:?}", i);
                get_bucket_accelerate_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_acl(|i: GetBucketAclInput| async move {
            info!("get_bucket_acl: {:?}", i);
            get_bucket_acl_output::Builder::default().build()
        })
        .get_bucket_analytics_configuration(|i: GetBucketAnalyticsConfigurationInput| async move {
            info!("get_bucket_analytics_configuration: {:?}", i);
            get_bucket_analytics_configuration_output::Builder::default().build()
        })
        .get_bucket_cors(|i: GetBucketCorsInput| async move {
            info!("get_bucket_cors: {:?}", i);
            get_bucket_cors_output::Builder::default().build()
        })
        .get_bucket_encryption(|i: GetBucketEncryptionInput| async move {
            info!("get_bucket_encryption: {:?}", i);
            get_bucket_encryption_output::Builder::default().build()
        })
        .get_bucket_intelligent_tiering_configuration(
            |i: GetBucketIntelligentTieringConfigurationInput| async move {
                info!("get_bucket_intelligent_tiering_configuration: {:?}", i);
                get_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_inventory_configuration(|i: GetBucketInventoryConfigurationInput| async move {
            info!("get_bucket_inventory_configuration: {:?}", i);
            get_bucket_inventory_configuration_output::Builder::default().build()
        })
        .get_bucket_lifecycle_configuration(|i: GetBucketLifecycleConfigurationInput| async move {
            info!("get_bucket_lifecycle_configuration: {:?}", i);
            get_bucket_lifecycle_configuration_output::Builder::default().build()
        })
        .get_bucket_location(|i: GetBucketLocationInput| async move {
            info!("get_bucket_location: {:?}", i);
            get_bucket_location_output::Builder::default().build()
        })
        .get_bucket_logging(|i: GetBucketLoggingInput| async move {
            info!("get_bucket_logging: {:?}", i);
            get_bucket_logging_output::Builder::default().build()
        })
        .get_bucket_metrics_configuration(|i: GetBucketMetricsConfigurationInput| async move {
            info!("get_bucket_metrics_configuration: {:?}", i);
            get_bucket_metrics_configuration_output::Builder::default().build()
        })
        .get_bucket_notification_configuration(
            |i: GetBucketNotificationConfigurationInput| async move {
                info!("get_bucket_notification_configuration: {:?}", i);
                get_bucket_notification_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_ownership_controls(|i: GetBucketOwnershipControlsInput| async move {
            info!("get_bucket_ownership_controls: {:?}", i);
            get_bucket_ownership_controls_output::Builder::default().build()
        })
        .get_bucket_policy(|i: GetBucketPolicyInput| async move {
            info!("get_bucket_policy: {:?}", i);
            get_bucket_policy_output::Builder::default().build()
        })
        .get_bucket_policy_status(|i: GetBucketPolicyStatusInput| async move {
            info!("get_bucket_policy_status: {:?}", i);
            get_bucket_policy_status_output::Builder::default().build()
        })
        .get_bucket_replication(|i: GetBucketReplicationInput| async move {
            info!("get_bucket_replication: {:?}", i);
            get_bucket_replication_output::Builder::default().build()
        })
        .get_bucket_request_payment(|i: GetBucketRequestPaymentInput| async move {
            info!("get_bucket_request_payment: {:?}", i);
            get_bucket_request_payment_output::Builder::default().build()
        })
        .get_bucket_tagging(|i: GetBucketTaggingInput| async move {
            info!("get_bucket_tagging: {:?}", i);
            get_bucket_tagging_output::Builder::default().build()
        })
        .get_bucket_versioning(|i: GetBucketVersioningInput| async move {
            info!("get_bucket_versioning: {:?}", i);
            get_bucket_versioning_output::Builder::default().build()
        })
        .get_bucket_website(|i: GetBucketWebsiteInput| async move {
            info!("get_bucket_website: {:?}", i);
            get_bucket_website_output::Builder::default().build()
        })
        .get_object_acl(|i: GetObjectAclInput| async move {
            info!("get_object_acl: {:?}", i);
            Ok(get_object_acl_output::Builder::default().build())
        })
        .get_object_legal_hold(|i: GetObjectLegalHoldInput| async move {
            info!("get_object_legal_hold: {:?}", i);
            get_object_legal_hold_output::Builder::default().build()
        })
        .get_object_lock_configuration(|i: GetObjectLockConfigurationInput| async move {
            info!("get_object_lock_configuration: {:?}", i);
            get_object_lock_configuration_output::Builder::default().build()
        })
        .get_object_retention(|i: GetObjectRetentionInput| async move {
            info!("get_object_retention: {:?}", i);
            get_object_retention_output::Builder::default().build()
        })
        .get_object_torrent(|i: GetObjectTorrentInput| async move {
            info!("get_object_torrent: {:?}", i);
            get_object_torrent_output::Builder::default().build()
        })
        .get_public_access_block(|i: GetPublicAccessBlockInput| async move {
            info!("get_public_access_block: {:?}", i);
            get_public_access_block_output::Builder::default().build()
        })
        .list_bucket_analytics_configurations(
            |i: ListBucketAnalyticsConfigurationsInput| async move {
                info!("list_bucket_analytics_configurations: {:?}", i);
                list_bucket_analytics_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_intelligent_tiering_configurations(
            |i: ListBucketIntelligentTieringConfigurationsInput| async move {
                info!("list_bucket_intelligent_tiering_configurations: {:?}", i);
                list_bucket_intelligent_tiering_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_inventory_configurations(
            |i: ListBucketInventoryConfigurationsInput| async move {
                info!("list_bucket_inventory_configurations: {:?}", i);
                list_bucket_inventory_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_metrics_configurations(|i: ListBucketMetricsConfigurationsInput| async move {
            info!("list_bucket_metrics_configurations: {:?}", i);
            list_bucket_metrics_configurations_output::Builder::default().build()
        })
        .list_multipart_uploads(|i: ListMultipartUploadsInput| async move {
            info!("list_multipart_uploads: {:?}", i);
            list_multipart_uploads_output::Builder::default().build()
        })
        .put_bucket_accelerate_configuration(
            |i: PutBucketAccelerateConfigurationInput| async move {
                info!("put_bucket_accelerate_configuration: {:?}", i);
                put_bucket_accelerate_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_acl(|i: PutBucketAclInput| async move {
            info!("put_bucket_acl: {:?}", i);
            put_bucket_acl_output::Builder::default().build()
        })
        .put_bucket_analytics_configuration(|i: PutBucketAnalyticsConfigurationInput| async move {
            info!("put_bucket_analytics_configuration: {:?}", i);
            put_bucket_analytics_configuration_output::Builder::default().build()
        })
        .put_bucket_cors(|i: PutBucketCorsInput| async move {
            info!("put_bucket_cors: {:?}", i);
            put_bucket_cors_output::Builder::default().build()
        })
        .put_bucket_encryption(|i: PutBucketEncryptionInput| async move {
            info!("put_bucket_encryption: {:?}", i);
            put_bucket_encryption_output::Builder::default().build()
        })
        .put_bucket_intelligent_tiering_configuration(
            |i: PutBucketIntelligentTieringConfigurationInput| async move {
                info!("put_bucket_intelligent_tiering_configuration: {:?}", i);
                put_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_inventory_configuration(|i: PutBucketInventoryConfigurationInput| async move {
            info!("put_bucket_inventory_configuration: {:?}", i);
            put_bucket_inventory_configuration_output::Builder::default().build()
        })
        .put_bucket_lifecycle_configuration(|i: PutBucketLifecycleConfigurationInput| async move {
            info!("put_bucket_lifecycle_configuration: {:?}", i);
            put_bucket_lifecycle_configuration_output::Builder::default().build()
        })
        .put_bucket_logging(|i: PutBucketLoggingInput| async move {
            info!("put_bucket_logging: {:?}", i);
            put_bucket_logging_output::Builder::default().build()
        })
        .put_bucket_metrics_configuration(|i: PutBucketMetricsConfigurationInput| async move {
            info!("put_bucket_metrics_configuration: {:?}", i);
            put_bucket_metrics_configuration_output::Builder::default().build()
        })
        .put_bucket_notification_configuration(
            |i: PutBucketNotificationConfigurationInput| async move {
                info!("put_bucket_notification_configuration: {:?}", i);
                put_bucket_notification_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_ownership_controls(|i: PutBucketOwnershipControlsInput| async move {
            info!("put_bucket_ownership_controls: {:?}", i);
            put_bucket_ownership_controls_output::Builder::default().build()
        })
        .put_bucket_policy(|i: PutBucketPolicyInput| async move {
            info!("put_bucket_policy: {:?}", i);
            put_bucket_policy_output::Builder::default().build()
        })
        .put_bucket_replication(|i: PutBucketReplicationInput| async move {
            info!("put_bucket_replication: {:?}", i);
            put_bucket_replication_output::Builder::default().build()
        })
        .put_bucket_request_payment(|i: PutBucketRequestPaymentInput| async move {
            info!("put_bucket_request_payment: {:?}", i);
            put_bucket_request_payment_output::Builder::default().build()
        })
        .put_bucket_tagging(|i: PutBucketTaggingInput| async move {
            info!("put_bucket_tagging: {:?}", i);
            put_bucket_tagging_output::Builder::default().build()
        })
        .put_bucket_versioning(|i: PutBucketVersioningInput| async move {
            info!("put_bucket_versioning: {:?}", i);
            put_bucket_versioning_output::Builder::default().build()
        })
        .put_bucket_website(|i: PutBucketWebsiteInput| async move {
            info!("put_bucket_website: {:?}", i);
            put_bucket_website_output::Builder::default().build()
        })
        .put_object_acl(|i: PutObjectAclInput| async move {
            info!("put_object_acl: {:?}", i);
            Ok(put_object_acl_output::Builder::default().build())
        })
        .put_object_legal_hold(|i: PutObjectLegalHoldInput| async move {
            info!("put_object_legal_hold: {:?}", i);
            put_object_legal_hold_output::Builder::default().build()
        })
        .put_object_lock_configuration(|i: PutObjectLockConfigurationInput| async move {
            info!("put_object_lock_configuration: {:?}", i);
            put_object_lock_configuration_output::Builder::default().build()
        })
        .put_object_retention(|i: PutObjectRetentionInput| async move {
            info!("put_object_retention: {:?}", i);
            put_object_retention_output::Builder::default().build()
        })
        .put_object_tagging(|i: PutObjectTaggingInput| async move {
            info!("put_object_tagging: {:?}", i);
            put_object_tagging_output::Builder::default().build()
        })
        .put_public_access_block(|i: PutPublicAccessBlockInput| async move {
            info!("put_public_access_block: {:?}", i);
            put_public_access_block_output::Builder::default().build()
        })
        .restore_object(|i: RestoreObjectInput| async move {
            info!("restore_object: {:?}", i);
            Ok(restore_object_output::Builder::default().build())
        })
        .upload_part(|i: UploadPartInput| async move {
            info!("upload_part: {:?}", i);
            upload_part_output::Builder::default().build()
        })
        .upload_part_copy(|i: UploadPartCopyInput| async move {
            info!("upload_part_copy: {:?}", i);
            upload_part_copy_output::Builder::default().build()
        })
        .write_get_object_response(|i: WriteGetObjectResponseInput| async move {
            info!("write_get_object_response: {:?}", i);
            write_get_object_response_output::Builder::default().build()
        })
        .build()
        .unwrap();

    {
        #[rustfmt::skip]
        let _: &OperationRegistry<hyper::Body,
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

// OperationRegistry<
//     hyper::Body,
//     impl Handler2<AbortMultipartUploadInput, AbortMultipartUploadOutput, AbortMultipartUploadError>,
//     (),
//     impl Handler1<CompleteMultipartUploadInput, CompleteMultipartUploadOutput>,
//     (),
//     impl Handler2<CopyObjectInput, CopyObjectOutput, CopyObjectError>,
//     (),
//     impl Handler2<CreateBucketInput, CreateBucketOutput, CreateBucketError>,
//     (),
//     impl Handler1<CreateMultipartUploadInput, CreateMultipartUploadOutput>,
//     (),
//     impl Handler1<DeleteBucketInput, DeleteBucketOutput>,
//     (),
//     impl Handler1<DeleteBucketAnalyticsConfigurationInput, DeleteBucketAnalyticsConfigurationOutput>,
//     (),
//     impl Handler1<DeleteBucketCorsInput, DeleteBucketCorsOutput>,
//     (),
//     impl Handler1<DeleteBucketEncryptionInput, DeleteBucketEncryptionOutput>,
//     (),
//     impl Handler1<
//         DeleteBucketIntelligentTieringConfigurationInput,
//         DeleteBucketIntelligentTieringConfigurationOutput,
//     >,
//     (),
//     impl Handler1<DeleteBucketInventoryConfigurationInput, DeleteBucketInventoryConfigurationOutput>,
//     (),
//     impl Handler1<DeleteBucketLifecycleInput, DeleteBucketLifecycleOutput>,
//     (),
//     impl Handler1<DeleteBucketMetricsConfigurationInput, DeleteBucketMetricsConfigurationOutput>,
//     (),
//     impl Handler1<DeleteBucketOwnershipControlsInput, DeleteBucketOwnershipControlsOutput>,
//     (),
//     impl Handler1<DeleteBucketPolicyInput, DeleteBucketPolicyOutput>,
//     (),
//     impl Handler1<DeleteBucketReplicationInput, DeleteBucketReplicationOutput>,
//     (),
//     impl Handler1<DeleteBucketTaggingInput, DeleteBucketTaggingOutput>,
//     (),
//     impl Handler1<DeleteBucketWebsiteInput, DeleteBucketWebsiteOutput>,
//     (),
//     impl Handler1<DeleteObjectInput, DeleteObjectOutput>,
//     (),
//     impl Handler1<DeleteObjectsInput, DeleteObjectsOutput>,
//     (),
//     impl Handler1<DeleteObjectTaggingInput, DeleteObjectTaggingOutput>,
//     (),
//     impl Handler1<DeletePublicAccessBlockInput, DeletePublicAccessBlockOutput>,
//     (),
//     impl Handler1<GetBucketAccelerateConfigurationInput, GetBucketAccelerateConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketAclInput, GetBucketAclOutput>,
//     (),
//     impl Handler1<GetBucketAnalyticsConfigurationInput, GetBucketAnalyticsConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketCorsInput, GetBucketCorsOutput>,
//     (),
//     impl Handler1<GetBucketEncryptionInput, GetBucketEncryptionOutput>,
//     (),
//     impl Handler1<
//         GetBucketIntelligentTieringConfigurationInput,
//         GetBucketIntelligentTieringConfigurationOutput,
//     >,
//     (),
//     impl Handler1<GetBucketInventoryConfigurationInput, GetBucketInventoryConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketLifecycleConfigurationInput, GetBucketLifecycleConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketLocationInput, GetBucketLocationOutput>,
//     (),
//     impl Handler1<GetBucketLoggingInput, GetBucketLoggingOutput>,
//     (),
//     impl Handler1<GetBucketMetricsConfigurationInput, GetBucketMetricsConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketNotificationConfigurationInput, GetBucketNotificationConfigurationOutput>,
//     (),
//     impl Handler1<GetBucketOwnershipControlsInput, GetBucketOwnershipControlsOutput>,
//     (),
//     impl Handler1<GetBucketPolicyInput, GetBucketPolicyOutput>,
//     (),
//     impl Handler1<GetBucketPolicyStatusInput, GetBucketPolicyStatusOutput>,
//     (),
//     impl Handler1<GetBucketReplicationInput, GetBucketReplicationOutput>,
//     (),
//     impl Handler1<GetBucketRequestPaymentInput, GetBucketRequestPaymentOutput>,
//     (),
//     impl Handler1<GetBucketTaggingInput, GetBucketTaggingOutput>,
//     (),
//     impl Handler1<GetBucketVersioningInput, GetBucketVersioningOutput>,
//     (),
//     impl Handler1<GetBucketWebsiteInput, GetBucketWebsiteOutput>,
//     (),
//     impl Handler1<GetObjectInput, GetObjectOutput>,
//     (),
//     impl Handler2<GetObjectAclInput, GetObjectAclOutput, GetObjectAclError>,
//     (),
//     impl Handler1<GetObjectLegalHoldInput, GetObjectLegalHoldOutput>,
//     (),
//     impl Handler1<GetObjectLockConfigurationInput, GetObjectLockConfigurationOutput>,
//     (),
//     impl Handler1<GetObjectRetentionInput, GetObjectRetentionOutput>,
//     (),
//     impl Handler1<GetObjectTaggingInput, GetObjectTaggingOutput>,
//     (),
//     impl Handler1<GetObjectTorrentInput, GetObjectTorrentOutput>,
//     (),
//     impl Handler1<GetPublicAccessBlockInput, GetPublicAccessBlockOutput>,
//     (),
//     impl Handler2<HeadBucketInput, HeadBucketOutput, HeadBucketError>,
//     (),
//     impl Handler2<HeadObjectInput, HeadObjectOutput, HeadObjectError>,
//     (),
//     impl Handler1<ListBucketAnalyticsConfigurationsInput, ListBucketAnalyticsConfigurationsOutput>,
//     (),
//     impl Handler1<
//         ListBucketIntelligentTieringConfigurationsInput,
//         ListBucketIntelligentTieringConfigurationsOutput,
//     >,
//     (),
//     impl Handler1<ListBucketInventoryConfigurationsInput, ListBucketInventoryConfigurationsOutput>,
//     (),
//     impl Handler1<ListBucketMetricsConfigurationsInput, ListBucketMetricsConfigurationsOutput>,
//     (),
//     impl Handler1<ListBucketsInput, ListBucketsOutput>,
//     (),
//     impl Handler1<ListMultipartUploadsInput, ListMultipartUploadsOutput>,
//     (),
//     impl Handler2<ListObjectsInput, ListObjectsOutput, ListObjectsError>,
//     (),
//     impl Handler2<ListObjectsV2Input, ListObjectsV2Output, ListObjectsV2Error>,
//     (),
//     impl Handler1<ListObjectVersionsInput, ListObjectVersionsOutput>,
//     (),
//     impl Handler1<ListPartsInput, ListPartsOutput>,
//     (),
//     impl Handler1<PutBucketAccelerateConfigurationInput, PutBucketAccelerateConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketAclInput, PutBucketAclOutput>,
//     (),
//     impl Handler1<PutBucketAnalyticsConfigurationInput, PutBucketAnalyticsConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketCorsInput, PutBucketCorsOutput>,
//     (),
//     impl Handler1<PutBucketEncryptionInput, PutBucketEncryptionOutput>,
//     (),
//     impl Handler1<
//         PutBucketIntelligentTieringConfigurationInput,
//         PutBucketIntelligentTieringConfigurationOutput,
//     >,
//     (),
//     impl Handler1<PutBucketInventoryConfigurationInput, PutBucketInventoryConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketLifecycleConfigurationInput, PutBucketLifecycleConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketLoggingInput, PutBucketLoggingOutput>,
//     (),
//     impl Handler1<PutBucketMetricsConfigurationInput, PutBucketMetricsConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketNotificationConfigurationInput, PutBucketNotificationConfigurationOutput>,
//     (),
//     impl Handler1<PutBucketOwnershipControlsInput, PutBucketOwnershipControlsOutput>,
//     (),
//     impl Handler1<PutBucketPolicyInput, PutBucketPolicyOutput>,
//     (),
//     impl Handler1<PutBucketReplicationInput, PutBucketReplicationOutput>,
//     (),
//     impl Handler1<PutBucketRequestPaymentInput, PutBucketRequestPaymentOutput>,
//     (),
//     impl Handler1<PutBucketTaggingInput, PutBucketTaggingOutput>,
//     (),
//     impl Handler1<PutBucketVersioningInput, PutBucketVersioningOutput>,
//     (),
//     impl Handler1<PutBucketWebsiteInput, PutBucketWebsiteOutput>,
//     (),
//     impl Handler1<PutObjectInput, PutObjectOutput>,
//     (),
//     impl Handler2<PutObjectAclInput, PutObjectAclOutput, PutObjectAclError>,
//     (),
//     impl Handler1<PutObjectLegalHoldInput, PutObjectLegalHoldOutput>,
//     (),
//     impl Handler1<PutObjectLockConfigurationInput, PutObjectLockConfigurationOutput>,
//     (),
//     impl Handler1<PutObjectRetentionInput, PutObjectRetentionOutput>,
//     (),
//     impl Handler1<PutObjectTaggingInput, PutObjectTaggingOutput>,
//     (),
//     impl Handler1<PutPublicAccessBlockInput, PutPublicAccessBlockOutput>,
//     (),
//     impl Handler2<RestoreObjectInput, RestoreObjectOutput, RestoreObjectError>,
//     (),
//     impl Handler1<UploadPartInput, UploadPartOutput>,
//     (),
//     impl Handler1<UploadPartCopyInput, UploadPartCopyOutput>,
//     (),
//     impl Handler1<WriteGetObjectResponseInput, WriteGetObjectResponseOutput>,
//     (),
// >
