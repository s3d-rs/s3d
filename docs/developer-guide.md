---
title: Developer Guide
---

# Prerequisites:

- [Rust](https://www.rust-lang.org/tools/install) - stable channel, currently rustc 1.58.1 (db9d1b20b 2022-01-20).
- [Java 14](https://jdk.java.net/archive/) - currently openjdk 14.0.2 (2020-07-14).

Notice that JDK <= 14 is still the required for smithy-rs, but this restriction would be removed with the move to gradle 7 - see [tracking issue](https://github.com/awslabs/smithy-rs/issues/1167).

# Build from source

Clone the repo (use a fork if you want to contribute back upstream):
```bash
git clone https://github.com/s3d-rs/s3d.git
cd s3d
```

Build debug mode:
```bash
make
```

Build release mode:
```bash
make RELEASE=1
```

# Run locally

Run from target dir:
```bash
./target/debug/s3d <args>
```

Load shell env to simplify running (run `make env` to show the commands):
```bash
eval $(make env)
s3d # aliased to ./target/debug/s3d
```
