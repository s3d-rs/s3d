---
title: Developer Guide
---

# Developer Guide

Clone from repo (use a fork if you want to contribute back upstream):

```bash
git clone --recurse-submodules https://github.com/s3d-rs/s3d.git
cd s3d
```

Build and execute in one command:

```bash
cargo run -- <args>
```

Or in two commands:

```bash
cargo build
./target/debug/s3d <args>
```

Additional developer scripts are in `hack/` dir, most useful is the env script loaded to your shell:

```bash
source hack/env.sh
```

In order to update and build the smithy-rs submodule, you need to have `java` and run:

```bash
hack/smithy-update.sh # updates the smithy-rs submodule HEAD
hack/smithy-build.sh # builds smithy-rs and codegen for S3 API
```
