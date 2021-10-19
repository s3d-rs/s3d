---
title: User Guide
---

# User Guide 

To run the daemon in foreground:

```bash
s3d run
```

Configuration using environment variables:

- `S3D_LOCAL_DIR` - path to the local storage dir, default `$HOME/.s3d`.
- `S3D_ENDPOINT` - S3 listen address, default `http://localhost:33333`.
- `S3_ENDPOINT` - remote S3 address, default empty (SDK will choose default -> AWS).
- `AWS_ACCESS_KEY_ID` - AWS access key ID, default empty (SDK will choose default).
- `AWS_SECRET_ACCESS_KEY` - AWS secret access key, default empty (SDK will choose default).
- `AWS_SESSION_TOKEN` - AWS session token, default empty (SDK will choose default).

`s3d` uses the filesystem dir as a local storage, which is used for queueing, caching, and synching data from and to the remote storage.

`s3d` reads the remote S3 config and credential files and environment variables
just like any other S3 SDK client in order to connect to its remote storage.
In addition, to support S3 compatible endpoints, it reads the `S3_ENDPOINT` environment variable.

The credentials provided for `s3d` in the aws config files should be valid for the main storage,
and the identity provided to `s3d` is the one it will use in all the requests to the main storage.

To check and report the status of the daemon and the remote S3 storage, use:

```bash
s3d status
```

# Upload-queue

Environment variables:

- `S3D_UPLOAD_QUEUE` - true/false, default false.
- `S3D_UPLOAD_QUEUE_DIR` - directory to store the queue, default `$S3D_LOCAL_DIR/upload-queue`.
- `S3D_UPLOAD_QUEUE_FILTER` - object filter to push, default all.
- `S3D_UPLOAD_QUEUE_MAX_SIZE` - maximum size of the queue in bytes, default 1GB.
- `S3D_UPLOAD_QUEUE_MAX_FILES` - maximum number of files in the queue, default 100.
- `S3D_UPLOAD_QUEUE_MAX_AGE` - maximum age of uploads in the queue in seconds, default 3600.

When enabled, `s3d` first writes new objects to files in the local store, and will push them to the main storage in the background. This is to mitigate connection issues and improve performance.

When the limits are exceeded, new upload requests will not be added to the queue, instead it will wait for pending uploads to push and make room for it.

See filters syntax for fine grain control of which data to push. In order to dynamically change the filtering of an object that was not pushed, use put-object-tagging which can be used on an existing in the uploads queue.

# Read-cache

Environment variables:

- `S3D_READ_CACHE` - true/false, default false.
- `S3D_READ_CACHE_DIR` - directory to store the cache, default `$S3D_LOCAL_DIR/read-cache`.
- `S3D_READ_CACHE_FILTER` - object filter to cache, default all.
- `S3D_READ_CACHE_MAX_SIZE` - maximum size of the cache in bytes, default 1GB.
- `S3D_READ_CACHE_MAX_FILES` - maximum number of files in the cache, default 100.
- `S3D_READ_CACHE_MAX_AGE` - maximum age of files in the cache in seconds, default 3600.

When enabled, `s3d` will store objects in the local store on read, in order to reduce egress costs and latency on repeated reads from the main storage.

When the limits are exceeded, old items from the cache will be pruned before adding new items.

See filters syntax for fine grain control of which data to cache.

# Sync-folder

When enabled, `s3d` will perform a continuous bidirectional background sync of the remote buckets with a local dir (aka "dropbox folder").

The following environment variables can be used to configure the sync-folder:

- `S3D_SYNC_FOLDER` - true/false, default false.
- `S3D_SYNC_FOLDER_DIR` - directory to store the folder, default `$S3D_LOCAL_DIR/sync-folder`.
- `S3D_SYNC_FOLDER_FILTER` - object filter to sync, default all.
- `S3D_SYNC_FOLDER_MAX_SIZE` - maximum size of the folder in bytes, default 1GB.
- `S3D_SYNC_FOLDER_MAX_FILES` - maximum number of files in the folder, default 100.
- `S3D_SYNC_FOLDER_MAX_AGE` - maximum age of (unsync-ed) files in the folder in seconds, default 3600.

When the limits are exceeded, sync will skip adding new data to the local folder.
See filters syntax for fine grain control of which data to sync.

# Fuse-mount

When enabled, `s3d` will set up a FUSE mount point, which exposes the same buckets and objects through a POSIX-like file interface.

The following environment variables can be used to configure the fuse-mount:

- `S3D_FUSE_MOUNT` - true/false, default false.
- `S3D_FUSE_MOUNT_DIR` - directory to bind the mount point, default `$S3D_LOCAL_DIR/fuse-mount`.

# Filters

By default, `s3d` will include all objects eligible for upload-queue, read-cache, and sync-folder. However for fine control over which objects to include, filters can be configured.

Here are a few examples of a filters syntax:

```re
bucket[tag:key]
bucket[tag:key=value]
bucket[tag:key!=value]
bucket[tag:key=value][tag:key=value]
bucket/prefix*
bucket/prefix*[tag:key]
bucket/prefix*[tag:key=value]
bucket/prefix*[tag:key!=value]
bucket/prefix*[tag:key1=value][tag:key2=value]
bucket/prefix*[hdr:content-type=value]
bucket/prefix*[hdr:content-length<100]
bucket/prefix*[md:custom-meta-data=value]
bucket[tag:key1=val1]/prefix*[tag:key2=val2][hdr:content-type='video/*']
```

Tags provide a way to update the filtering of existing objects,
for example using the S3 put-object-tagging API:

```bash
alias s3api='aws --endpoint localhost:33333 s3api'
s3api put-object-tagging --bucket bucket --key key --tagging '{"TagSet":[
  { "Key": "s3d.upload", "Value": "false" }
]}'
```

Notice that put-object-tagging is overriding the entire tag set, so in order to add a tag to existing set, you will need to use get-object-tagging, append to the TagSet array and then put-object-tagging.

# Deploy

See examples of using `s3d` in container images and kubernetes yamls in `deploy/` dir:

```bash
IMG="<username>/s3d:<tag>"
docker build . -t $IMG
docker push $IMG
# update image in yaml ... TODO kustomize
kubectl create -f deploy/s3d-kube-deploy.yaml
```
