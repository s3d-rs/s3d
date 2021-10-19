#!/bin/bash
# run me from s3d project root.
set -e -x -o pipefail
cd smithy-rs
./gradlew clean
