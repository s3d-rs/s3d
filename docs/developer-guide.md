---
title: Developer Guide
---

# Developer Guide

Clone from repo (use a fork if you want to contribute back upstream):

```bash
git clone https://github.com/s3d-rs/s3d.git
cd s3d
```

Requried toolchains:
- rust (cargo)
- java (gradle)

Build debug mode:
```bash
make
```

Run locally:
```bash
./target/debug/s3d <args>
```

Developer scripts are in `hack/` dir, most useful is the env script loaded to your shell:
```bash
source hack/env.sh
```

Build release mode:
```bash
make RELEASE=1
```
