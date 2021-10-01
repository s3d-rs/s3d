---
title: Config
---

## Config file

`s3d` loads a yaml config file `s3d.yaml` which makes it easy to set up and configure in standalone linux or containerized environments like Kubernetes.

Use `s3d config --help` to see the available configuration options. 

## Example

This is a sample `s3d.yaml` for an overview of the available configuration options.


```yaml

# TODO ... this is just an example

s3d:

  # The hub connection and configuration
  hub:
    type: s3-compatible
    endpoint: s3.compatible.acme.com:8443
    # Read modes:
    # `read-hub`    - reads directly from hub
    # `read-local`  - reads only from local cache from previous fetch/pull/reads
    # `read-cache`  - reads from hub or cached contents if hub validates the same etag and last-modified-time
    # `read-ttl`    - keep using stored contents until expired by TTL ("time to live") to allow working during disconnections
    read-mode: <mode>
    # Write modes:
    # `write-hub`   - writes directly to hub
    # `write-local` - stores locally and will only write to hub when push is requested.
    # `write-queue` - stores locally and write to hub asap
    write-mode: <mode>
    # IAM modes:
    # `iam-none`  - clients can access without providing credentials and s3d will use global credentials file
    # `iam-hub`   - use the same identities as the hub
    # `iam-local` - s3d will assign to every local user (uid) its own identity and access key (through files)
    iam: <mode>
    # Secure tokens (short-lived access keys)
    # `sts-none`  - no secure tokens
    # `sts-hub`   - use the hub's STS service to create/refresh tokens
    # `sts-local` - s3d will assign to every local user (uid) its own secure token (through files)
    sts: <mode>
    # Encryption modes:
    # `none`
    # `server-side`   - the hub will manage the encryption
    # `client-side`   - s3d will encrypt and decrypt the data, and reading from the hub will require the key
    encryption: <mode>

  # S3 server configuration
  s3:
    port: 8080
    auth: none | sigv4
    https: none | allow | require

  # FUSE filesystem configuration
  fuse:
    mountpoint: /mnt/s3d
    # uid mapping modes - TODO
    uid-mode: <mode>
```
