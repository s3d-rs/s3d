use crate::build_code_writer::CodeWriter;
use crate::build_smithy_model::*;
use quote::{format_ident, quote};
use std::{collections::HashSet, path::Path};

/// CLIGenerator generates clap commands for every operation.
pub struct CLIGenerator<'a> {
    pub model: &'a SmithyModel,
    pub writer: CodeWriter,
}

impl<'a> CLIGenerator<'a> {
    pub fn new(model: &'a SmithyModel, out_path: &Path) -> Self {
        Self {
            model,
            writer: CodeWriter::new(out_path),
        }
    }

    pub fn generate(mut self) {
        let ops_names: Vec<_> = self.model.iter_ops().map(|op| op.ident()).collect();
        let ops_command = self
            .model
            .iter_ops()
            .map(|op| format_ident!("{}Command", op.name));
        let ops_about = self.model.iter_ops().map(|op| op.get_doc_summary());
        let mut structs_set = HashSet::<String>::new();

        // generate the subcommands enum with each S3 operation as a cli subcommand
        self.writer.write_code(quote! {

            #[derive(clap::Subcommand, Debug, Clone)]
            pub enum S3OpsCommands {
                #(
                    #[clap(about = #ops_about, long_about = None)]
                    #ops_names(#ops_command),
                )*
            }

            impl S3OpsCommands {
                pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
                    match self {
                        #(
                            S3OpsCommands::#ops_names(cmd) => cmd.run(s3).await,
                        )*
                    }
                }
            }
        });

        for op in self.model.iter_ops() {
            let cmd = format_ident!("{}Command", op.name);
            let op_snake = format_ident!("{}", snake(&op.name));
            let input_shape = op.members.get("input").map(|m| self.model.get_shape_of(m));
            let empty = SmithyMemberMap::new();
            let members = input_shape.map_or(&empty, |s| &s.members);
            let set_inputs = members.values().map(|m| m.set_ident());

            let command_args = members.values().map(|m| {
                let ident = m.ident();
                let long_name = m.snake.replace("_", "-");
                let arg_name = m.name.clone();
                let help =  m.get_doc_summary();
                let required = m.has_trait(SM_REQUIRED);
                let shape = self.model.get_shape_of(m);
                let rust_type = match shape.typ {
                    SmithyType::Boolean => quote! { bool },
                    SmithyType::Byte => quote! { i8 },
                    SmithyType::Short => quote! { i16 },
                    SmithyType::Integer => quote! { i32 },
                    SmithyType::Long => quote! { i64 },
                    SmithyType::Float => quote! { f32 },
                    SmithyType::Double => quote! { f64 },
                    SmithyType::String => quote! { String },
                    SmithyType::Timestamp => quote! { String },
                    SmithyType::Blob => quote! { Vec<u8> },
                    SmithyType::Map => quote! { String },
                    SmithyType::Structure => {
                        let struct_name = format_ident!("{}Args", shape.name);
                        if !structs_set.contains(&shape.name) {
                            structs_set.insert(shape.name.clone());
                            let args = shape.members.values().map(|k| k.ident());
                            let long_names = shape.members.values().map(|k| k.snake.replace("_", "-"));
                            self.writer.write_code(quote! {
                                #[derive(clap::Args, Debug, Clone)]
                                pub struct #struct_name {
                                    #(
                                        #[clap(long = #long_names)]
                                        #args: Option<String>,
                                    )*
                                }
                            });
                        }
                        quote! { #struct_name }
                    }
                    _ => todo!(
                        "unsupported input shape type {:?} shape name {} member name {} required {}",
                        shape.typ,
                        shape.name,
                        m.name,
                        required
                    ),
                };
                if shape.typ == SmithyType::Structure {
                    quote! {
                        #[clap(flatten)]
                        #ident: #rust_type
                    }
                } else {
                    let rust_type = if required {
                        rust_type
                    } else {
                        quote! { Option<#rust_type> }
                    };
                    quote! {
                        #[clap(long = #long_name, name = #arg_name, help = #help, long_help = None)]
                        #ident: #rust_type
                    }
                }
            }).collect::<Vec<_>>();

            let inputs_from = members.values().map(|m| {
                let id = m.ident();
                let required = m.has_trait(SM_REQUIRED);
                let shape = self.model.get_shape_of(m);
                match shape.typ {
                    SmithyType::Boolean
                    | SmithyType::Byte
                    | SmithyType::Short
                    | SmithyType::Integer
                    | SmithyType::Long
                    | SmithyType::Float
                    | SmithyType::Double => {
                        if required {
                            quote! { Some(self.#id) }
                        } else {
                            quote! { self.#id }
                        }
                    }
                    SmithyType::String => {
                        if shape.has_trait(SM_ENUM) {
                            quote! { None }
                        } else if required {
                            quote! { Some(self.#id.to_owned()) }
                        } else {
                            quote! { self.#id.to_owned() }
                        }
                    }
                    _ => quote! { None },
                }
            });

            self.writer.write_code(quote! {

                #[derive(clap::Parser, Debug, Clone)]
                pub struct #cmd {
                    #(
                        #command_args,
                    )*
                }

                impl #cmd {
                    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
                        log::info!("{:#?}", self);
                        let res = s3.#op_snake()
                            #(
                                .#set_inputs(#inputs_from)
                            )*
                            .send()
                            .await;
                        match res {
                            Ok(out) => {
                                log::info!("{:#?}", out);
                            }
                            Err(err) => match err {
                                aws_smithy_http::result::SdkError::ServiceError {err,raw} => {
                                    log::error!("{:#?}", err);
                                }
                                _ => {
                                    log::error!("{:#?}", err);
                                }
                            }
                        }
                    }
                }

            });
        }

        self.writer.done();
    }
}

/*
/// Generate the basic enum of operation kinds + macros to quickly generate code for each operation.
/// The enum is flat - meaning it defines no attached state to any of the operations.
/// It might be interesting to consider a more complex enum if needed by the daemon,
/// or perhaps that would instead go to it's own enum, with auto-generated-mapping to this one.
fn gen_ops_enum(&mut self) {
    let ops_names: Vec<_> = self.model.iter_ops().map(|op| op.ident()).collect();

    self.writer.write_code(quote! {

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum S3Ops {
            #(#ops_names),*
        }

        /// This macro calls a provided $macro for each S3 operation to generate code per op.
        macro_rules! generate_ops_code {
            ($macro:ident) => {
                #( $macro!(#ops_names); )*
            }
        }

        /// This macro calls a provided $macro for each S3 operation to generate item per op.
        macro_rules! generate_ops_items {
            ($macro:ident) => {
                #( $macro!(#ops_names), )*
            }
        }

        /// This macro matches a variable of S3Ops type and expands the provided $macro
        /// for each S3 operation to generate code handler per op.
        macro_rules! generate_ops_match {
            ($macro:ident, $op:expr) => {
                match ($op) {
                    #( S3Ops::#ops_names => $macro!(#ops_names), )*
                }
            }
        }

        pub(crate) use generate_ops_code;
        pub(crate) use generate_ops_items;
        pub(crate) use generate_ops_match;
    });
}
*/
