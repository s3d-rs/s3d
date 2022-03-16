//! # build.rs for s3d
//!
//! This is the cargo build script which is called during build.
//! We use it to generate code that for the S3 protocol.
//!
//! It reads the S3 smithy model as input, and writes generated code out to `$OUT_DIR/`,
//! which is then included! in the src/build_gen.rs file.
//!
//! The S3 protocol is defined in a Smithy JSON AST model:
//! - https://github.com/awslabs/smithy-rs/blob/main/aws/sdk/aws-models/s3.json
//!

// the build script flow is split to several modules
mod build_code_writer;
mod build_gen_cli;
mod build_gen_converters;
mod build_smithy_model;

use crate::{
    build_gen_cli::CLIGenerator,
    build_gen_converters::ConvertersGenerator,
    build_smithy_model::{FromJson, SmithyModel},
};
use std::{env, path::Path};

/// The main build script
/// See https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(out_dir.as_str());
    let model_path = Path::new("smithy-rs/aws/sdk/aws-models/s3.json");
    println!("cargo:rerun-if-changed={}", model_path.display());
    println!("cargo:rerun-if-changed=build_code_writer.rs");
    println!("cargo:rerun-if-changed=build_gen_cli.rs");
    println!("cargo:rerun-if-changed=build_gen_converters.rs");
    println!("cargo:rerun-if-changed=build_smithy_model.rs");
    println!("cargo:rerun-if-changed=build.rs");
    let model = SmithyModel::from_json_file(&model_path);
    CLIGenerator::new(&model, &out_path.join("s3_cli.rs")).generate();
    ConvertersGenerator::new(&model, &out_path.join("s3_conv.rs")).generate();
}
