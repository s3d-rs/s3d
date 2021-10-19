#!/bin/bash
# run me from s3d project root.
set -e -x -o pipefail

echo -e "\n\n>>> Building codegen \n\n"

cd smithy-rs
# ./gradlew -Paws.services=+sts,+sso,+s3 :aws:sdk:assemble
./gradlew :codegen-s3d:assemble
cd ..

echo -e "\n\n>>> Update generated code in source tree \n\n"

rm -rf codegen/codegen-server-s3
rm -rf codegen/codegen-client-s3

mkdir -p codegen/codegen-server-s3
mkdir -p codegen/codegen-client-s3

cp -R codegen/build/smithyprojections/codegen-s3d/s3/rust-server-codegen/ codegen/codegen-server-s3/
cp -R codegen/build/smithyprojections/codegen-s3d/s3/rust-codegen/ codegen/codegen-client-s3/

sed -i.bak 's:smithy-rs/rust-runtime/aws-endpoint:smithy-rs/aws/rust-runtime/aws-endpoint:' codegen/codegen-server-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-http:smithy-rs/aws/rust-runtime/aws-http:' codegen/codegen-server-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-sig-auth:smithy-rs/aws/rust-runtime/aws-sig-auth:' codegen/codegen-server-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-sigv4:smithy-rs/aws/rust-runtime/aws-sigv4:' codegen/codegen-server-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-types:smithy-rs/aws/rust-runtime/aws-types:' codegen/codegen-server-s3/Cargo.toml
rm codegen/codegen-server-s3/Cargo.toml.bak

sed -i.bak 's:smithy-rs/rust-runtime/aws-endpoint:smithy-rs/aws/rust-runtime/aws-endpoint:' codegen/codegen-client-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-http:smithy-rs/aws/rust-runtime/aws-http:' codegen/codegen-client-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-sig-auth:smithy-rs/aws/rust-runtime/aws-sig-auth:' codegen/codegen-client-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-sigv4:smithy-rs/aws/rust-runtime/aws-sigv4:' codegen/codegen-client-s3/Cargo.toml
sed -i.bak 's:smithy-rs/rust-runtime/aws-types:smithy-rs/aws/rust-runtime/aws-types:' codegen/codegen-client-s3/Cargo.toml
rm codegen/codegen-client-s3/Cargo.toml.bak

echo -e "\n\n>>> Building s3d-rs \n\n"

cargo run

echo -e "\n\n>>> Done \n\n"
