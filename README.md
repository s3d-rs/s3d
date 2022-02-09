<div id="top"></div>
<div align="center">
  <a alt="crate" href="https://crates.io/crates/s3d">
    <img src="https://img.shields.io/crates/v/s3d.svg?color=success&logo=rust&logoColor=white" />
  </a>
  <a alt="build" href="https://github.com/s3d-rs/s3d/actions">
    <!-- <img src="https://github.com/s3d-rs/s3d/workflows/build/badge.svg" /> -->
    <img src="https://img.shields.io/github/checks-status/s3d-rs/s3d/main?logo=github&logoColor=white" />
  </a>
  <a alt="discord" href="https://discord.com/channels/897764851580035072">
    <img src="https://img.shields.io/discord/897764851580035072?color=success&logo=discord&logoColor=white" />
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/github/license/s3d-rs/s3d?color=success" />
  </a>
  <!--
  <a alt="releases" href="https://github.com/s3d-rs/s3d/releases/latest">
    <img src="https://img.shields.io/github/v/release/s3d-rs/s3d" />
  </a>
  <a alt="s3d at docs.rs" href="http://docs.rs/s3d">
    <img src="https://docs.rs/s3d/badge.svg" />
  </a>
  -->
</div>
<br />
<div align="center" style="background-color: hsla(0,0,0%,0.1); text-align: center">
  <a alt="s3d logo" href="https://s3d.rs" style="background-color: hsla(0,0,0%,0.1); text-align: center">
    <img alt="s3d" src="s3d.png" width="250" />
  </a>
</div>

# The S3 Daemon

`s3d` is a daemon for data access using S3 API. A modern cousin of `nfsd`, `ftpd`, `httpd`, etc. It is designed to be simple, tiny, blazing fast, and portable in order to fit in a variety of environments from developer machines, containers, kubernetes, edge devices, etc.

By default, `s3d` serves the S3 API as a gateway to a main remote S3 storage (AWS/compatible), with ultimate protocol compatiblity (based on the AWS SDK and Smithy API), it adds critical features needed by remote clients such as queueing, caching, and synching, in order to optimize performance, increase data availability, and service continuity for its clients.

The need for a daemon running alongside applications emerges in Edge computing use cases, where data is stored and processed locally at the edge as it gets collected, while some of the data gets synced to and from a main data storage. Refer to [Wikipedia - Edge computing](https://en.wikipedia.org/wiki/Edge_computing): _"Edge computing is a distributed computing paradigm that brings computation and data storage closer to the sources of data. This is expected to improve response times and save bandwidth. ..."_.

# Features

Maturity legend: 🥉 - design phase, 🥈 - development phase, 🥇 - released.

- 🥈 &nbsp; **S3 API**
  - Generated S3 protocol code with [awslabs/smithy-rs](https://github.com/awslabs/smithy-rs) which builds the AWS SDK for Rust.
  - Provides compatible parsing of API operations, inputs, outputs, errors, and the server skeleton.
- 🥈 &nbsp; **Write Queue**
  - Writing new objects to local filesystem first to tolerate connection issues.
  - Pushing in the background to remote storage.
- 🥉 &nbsp; **Read Cache**
  - Store cached and prefetched objects in local filesystem.
  - Reduce egress costs and latency of reads from remote storage.
- 🥉 &nbsp; **Filters**
  - Fine control over which objects to include/exclude for upload/cache/sync.
  - Filter by bucket name, bucket tags, object keys (or prefixes), object tags, and object meta-data.
  - Optional integration with [OpenPolicyAgent (OPA)](https://www.openpolicyagent.org) for advanced filtering.
- 🥉 &nbsp; **Sync Folder**
  - Continuous, bidirectional and background sync of local dirs to remote buckets.
  - This is a simple way to get data that begins as local files to the remote S3 storage.
- 🥉 &nbsp; **Fuse Mount**
  - Virtual filesystem mountpoint provided by the daemon (see [kernel fuse docs](https://www.kernel.org/doc/html/latest/filesystems/fuse.html)).
  - The filesystem can be accessed normally for applications that do not use the S3 API.
- 🥉 &nbsp; **Metrics**
  - Optional integration with [OpenTelemetry](https://opentelemetry.io).
  - Expose logging, metrics, and tracing of S3 operations and `s3d` features.

# Docs

- 🧑‍🚀 &nbsp; **[User guide](docs/user-guide.md)** - how to use features and configurations.
- 🥷 &nbsp; **[Developer guide](docs/developer-guide.md)** - how to build and test.
- 🧝 &nbsp; **[Architecture page](docs/architecture.md)** - designs of components, software and roadmap ideas.

# Status

> 🧪🧪 &nbsp; **Experimental** &nbsp; 🧪🧪
>
> This project is still in it's early days, which means it's a great time to affect its direction, and it welcomes contributions and open discussions.
> 
> Keep in mind that all internal and external interfaces are considered unstable and might change without notice.

# Getting Started

Until the first releases are available, please refer to the [Developer guide](docs/developer-guide.md) for building `s3d` from source.

Here are some commands to explore:

```bash
make                        # build from source (see dev guide for prerequisites)
eval $(make env)            # defines aliases such as s3d -> build output binary
aws configure list          # make sure remote S3 credentials are configured
s3d run                     # runs daemon (foreground)
s3d status                  # check daemon status
s3d status bucket/key       # check bucket or object status
s3d ls bucket/prefix        # list buckets or objects
s3d get bucket/key > file   # get object data to stdout (meta-data to stderr)
s3d put bucket/key < file   # put object data from stdin
s3d set bucket/key          \
  --tag s3d.upload=true     \
  --tag s3d.cache=pin       # set tags for bucket or object to be used in filters
s3d <command> --help        # show usage for command
```

# Project

- [Github repo](https://github.com/s3d-rs/s3d) - The project lives here, set your <button>👁️  Watch</button> options to get notified on releases, issues, etc.
- [Discord chat](https://discord.com/channels/897764851580035072) - use this [invite link](https://discord.gg/kPWHDuCdhh) to join.
- [Redhat-et](https://github.com/redhat-et) - this project was initiated at Red Hat Emerging Technologies.
- [License](LICENSE) - Apache 2.0
