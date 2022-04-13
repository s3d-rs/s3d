use crate::codegen::utils::CodeWriter;
use crate::codegen::smithy_model::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

/// ConvertersGenerator generates functions to convert input and output structs
/// between smithy client and server because they are not the same.
/// See https://github.com/awslabs/smithy-rs/issues/1099
pub struct GenConverters<'a> {
    pub model: &'a SmithyModel,
    pub writer: CodeWriter,
    pub client_crate: String,
    pub server_crate: String,
}

impl<'a> GenConverters<'a> {
    pub fn new(model: &'a SmithyModel, out_path: &Path) -> Self {
        Self {
            model,
            writer: CodeWriter::new(out_path),
            client_crate: String::from("aws_sdk_s3"),
            server_crate: String::from("s3d_smithy_codegen_server_s3"),
        }
    }

    pub fn generate(mut self) {
        let client_crate = format_ident!("{}", self.client_crate);
        let server_crate = format_ident!("{}", self.server_crate);

        for op in self.model.iter_ops() {
            {
                let input_id = format_ident!("{}Input", op.name);
                let conv_to_client_input =
                    format_ident!("conv_to_client_{}", snake(&input_id.to_string()));
                let conv_to_client_input_gen =
                    if let Some(mut input_member) = op.members.get("input").cloned() {
                        input_member.name = format!("input/{}", input_id);
                        self.gen_conv_to_client(&input_member, quote! { input  })
                    } else {
                        quote! { #client_crate::input::#input_id::builder().build().unwrap() }
                    };
                self.writer.write_code(quote! {
                    pub fn #conv_to_client_input(input: #server_crate::input::#input_id) -> #client_crate::input::#input_id {
                        #conv_to_client_input_gen
                    }
                });
            }
            {
                let output_id = format_ident!("{}Output", op.name);
                let conv_from_client_output =
                    format_ident!("conv_from_client_{}", snake(&output_id.to_string()));
                let conv_from_client_output_gen =
                    if let Some(mut output_member) = op.members.get("output").cloned() {
                        output_member.name = format!("output/{}", output_id);
                        self.gen_conv_from_client(&output_member, quote! { output  })
                    } else {
                        quote! { #server_crate::output::#output_id {} }
                    };
                self.writer.write_code(quote! {
                    pub fn #conv_from_client_output(output: #client_crate::output::#output_id) -> #server_crate::output::#output_id {
                        #conv_from_client_output_gen
                    }
                });
            }
        }

        self.writer.done();
    }

    fn gen_conv_to_client(&mut self, member: &SmithyMember, from: TokenStream) -> TokenStream {
        let client_crate = format_ident!("{}", self.client_crate);
        let server_crate = format_ident!("{}", self.server_crate);
        let shape = self.model.get_shape_of(member);
        let member_split: Vec<_> = member.name.split('/').collect();
        let pkg_name = format_ident!(
            "{}",
            match member_split[0] {
                "input" => "input",
                "output" => "output",
                _ => "model",
            }
        );
        let type_name = format_ident!(
            "{}",
            match member_split[0] {
                "input" => member_split[1],
                "output" => member_split[1],
                _ => shape.name.as_str(),
            }
        );

        match shape.typ {
            SmithyType::Structure => {
                let members: Vec<TokenStream> = shape
                    .members
                    .values()
                    .map(|m| {
                        let m_ident = m.ident();
                        let set_ident = m.set_ident();
                        let s = self.model.get_shape_of(m);
                        let convert = if m.has_trait(SM_REQUIRED) || s.typ.is_always_required() {
                            let c = self.gen_conv_to_client(m, quote! { v. #m_ident });
                            quote! { Some(#c)}
                        } else {
                            let c = self.gen_conv_to_client(m, quote! { v });
                            quote! { v. #m_ident .map(|v| #c) }
                        };
                        quote! { b = b.#set_ident(#convert); }
                    })
                    .collect();
                let build_it = if pkg_name == "input" {
                    quote! { b.build().unwrap() }
                } else {
                    quote! { b.build() }
                };
                quote! {{
                    let v = #from;
                    let mut b = #client_crate::#pkg_name::#type_name::builder();
                    #(#members)*
                    #build_it
                }}
            }

            SmithyType::List => {
                let m = &shape.members["member"];
                let s = self.model.get_shape_of(m);
                let convert = self.gen_conv_to_client(m, quote! { v });
                if s.typ.is_always_required()
                    || (s.typ == SmithyType::String && !s.has_trait(SM_ENUM))
                {
                    quote! { #from .clone() }
                } else {
                    quote! { #from .into_iter().map(|v| #convert).collect() }
                }
            }

            // SmithyType::Map => {}
            SmithyType::Union => {
                let members: Vec<TokenStream> = shape
                    .members
                    .values()
                    .map(|m| {
                        let enum_name = format_ident!("{}", m.name);
                        let c = self.gen_conv_to_client(m, quote! { v });
                        quote! {
                            #server_crate::#pkg_name::#type_name::#enum_name(v) =>
                                #client_crate::#pkg_name::#type_name::#enum_name(#c),
                        }
                    })
                    .collect();
                quote! {{
                    match #from {
                        #(#members)*
                        _ => panic!("unknown union value"),
                    }
                }}
            }

            SmithyType::Blob => {
                quote! { #from }
            }

            SmithyType::String => {
                if shape.has_trait(SM_ENUM) {
                    quote! { #client_crate::#pkg_name::#type_name::from(#from .as_str()) }
                } else {
                    quote! { #from .to_owned() }
                }
            }

            _ => {
                quote! { #from .to_owned() }
            }
        }
    }

    fn gen_conv_from_client(&mut self, member: &SmithyMember, from: TokenStream) -> TokenStream {
        let client_crate = format_ident!("{}", self.client_crate);
        let server_crate = format_ident!("{}", self.server_crate);
        let shape = self.model.get_shape_of(member);
        let member_split: Vec<_> = member.name.split('/').collect();
        let pkg_name = format_ident!(
            "{}",
            match member_split[0] {
                "input" => "input",
                "output" => "output",
                _ => "model",
            }
        );
        let type_name = format_ident!(
            "{}",
            match member_split[0] {
                "input" => member_split[1],
                "output" => member_split[1],
                _ => shape.name.as_str(),
            }
        );

        match shape.typ {
            SmithyType::Structure => {
                let mut has_required = false;
                let members: Vec<TokenStream> = shape
                    .members
                    .values()
                    .map(|m| {
                        let m_ident = m.ident();
                        let set_ident = m.set_ident();
                        let s = self.model.get_shape_of(m);
                        if m.has_trait(SM_REQUIRED)
                            && !s.typ.is_always_required()
                            && (s.typ != SmithyType::String || s.has_trait(SM_ENUM))
                        {
                            has_required = true;
                        }
                        let convert = if s.typ.is_always_required() {
                            let c = self.gen_conv_from_client(m, quote! { v. #m_ident });
                            quote! { Some(#c)}
                        } else {
                            let c = self.gen_conv_from_client(m, quote! { v });
                            quote! { v. #m_ident .map(|v| #c) }
                        };
                        quote! { b = b.#set_ident(#convert); }
                    })
                    .collect();
                let build_it = if pkg_name == "input" || has_required {
                    quote! { b.build().unwrap() }
                } else {
                    quote! { b.build() }
                };
                quote! {{
                    let v = #from;
                    let mut b = #server_crate::#pkg_name::#type_name::builder();
                    #(#members)*
                    #build_it
                }}
            }

            SmithyType::List => {
                let m = &shape.members["member"];
                let s = self.model.get_shape_of(m);
                let convert = self.gen_conv_from_client(m, quote! { v });
                if s.typ.is_always_required()
                    || (s.typ == SmithyType::String && !s.has_trait(SM_ENUM))
                {
                    quote! { #from .clone() }
                } else {
                    quote! { #from .into_iter().map(|v| #convert).collect() }
                }
            }

            // SmithyType::Map => {}
            SmithyType::Union => {
                let members: Vec<TokenStream> = shape
                    .members
                    .values()
                    .map(|m| {
                        let enum_name = format_ident!("{}", m.name);
                        let c = self.gen_conv_from_client(m, quote! { v });
                        quote! {
                            #client_crate::#pkg_name::#type_name::#enum_name(v) =>
                                #server_crate::#pkg_name::#type_name::#enum_name(#c),
                        }
                    })
                    .collect();
                quote! {{
                    match #from {
                        #(#members)*
                        _ => panic!("unknown union value"),
                    }
                }}
            }

            SmithyType::Blob => {
                quote! { #from }
            }

            SmithyType::String => {
                if shape.has_trait(SM_ENUM) {
                    quote! { #server_crate::#pkg_name::#type_name::from(#from .as_str()) }
                } else {
                    quote! { #from .to_owned() }
                }
            }

            _ => {
                quote! { #from .to_owned() }
            }
        }
    }
}
