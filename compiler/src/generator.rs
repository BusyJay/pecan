use crate::util::*;
use crate::{
    db::{Database, File},
    field::FieldGenerator,
    message::MessageGenerator,
};
use bytes::BytesMut;
use pecan::prelude::*;
use pecan::reflection::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct Generator<'a> {
    db: &'a Database,
    file: &'a File,
    escape_option: bool,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(db: &'a Database, file: &'a File) -> Generator<'a> {
        let escape_option = file
            .proto()
            .enum_type
            .iter()
            .any(|e| e.name().eq_ignore_ascii_case("option"))
            || file
                .proto()
                .message_type
                .iter()
                .any(|m| m.name().eq_ignore_ascii_case("option"));
        Generator {
            db,
            file,
            escape_option,
        }
    }

    fn generate_enum(&self, e: &EnumDescriptorProto) -> TokenStream {
        let name = type_name(e.name(), "");
        self.generate_enum_impl(&name, e)
    }

    fn generate_enum_impl(&self, name: &str, e: &EnumDescriptorProto) -> TokenStream {
        let name_ident = format_ident!("{}", name);
        let mut raw_values: Vec<_> = e
            .value
            .iter()
            .map(|v| (escape(v.name()), v.number()))
            .collect();
        let all_values: Vec<_> = raw_values
            .iter()
            .map(|(k, v)| (format_ident!("{}", k), Literal::i32_unsuffixed(*v)))
            .collect();
        let (all_key, all_value) = split(&all_values);

        raw_values.dedup_by_key(|(_, v)| *v);
        let dedup_values: Vec<_> = raw_values
            .iter()
            .map(|(k, v)| (k, Literal::i32_unsuffixed(*v)))
            .collect();
        let (dedup_key, dedup_value) = split(&dedup_values);

        let raw_repr = format!("{}({{}})", name);
        let r_name = std::iter::repeat(&name_ident);
        quote! {
            #[derive(Default, Clone, Copy, PartialEq, Eq)]
            pub struct #name_ident(i32);

            impl pecan::Enumerate for #name_ident {
                #[inline]
                fn value(self) -> i32 {
                    self.0
                }

                #[inline]
                fn from_value(v: i32) -> #name_ident {
                    #name_ident(v)
                }
            }

            impl #name_ident {
                #(pub const #all_key: #r_name = #r_name(#all_value);)*

                pub const fn new() -> #name_ident {
                    #name_ident(0)
                }
            }

            impl std::fmt::Debug for #name_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self.0 {
                        #(#dedup_value => write!(f, #dedup_key),)*
                        v => write!(f, #raw_repr, v),
                    }
                }
            }
        }
    }

    fn generate_message(&self, d: &DescriptorProto) -> TokenStream {
        let msg_name = type_name(d.name(), "");
        let prefix = package_prefix(self.file.proto());
        let type_id = format!("{}{}", prefix, d.name());
        self.generate_message_impl(&msg_name, &type_id, d)
    }

    fn generate_message_impl(
        &self,
        msg_name: &str,
        type_id: &str,
        d: &DescriptorProto,
    ) -> TokenStream {
        let mut token = TokenStream::new();
        for e in &d.enum_type {
            let name = type_name(e.name(), msg_name);
            token.extend(self.generate_enum_impl(&name, e));
        }
        for e in &d.nested_type {
            let name = type_name(e.name(), msg_name);
            let type_id = format!("{}.{}", type_id, e.name());
            token.extend(self.generate_message_impl(&name, &type_id, e));
        }
        if let Some(g) = MessageGenerator::new(self, d, type_id, msg_name) {
            token.extend(g.generate());
        }
        token
    }

    fn generate_extension(&self, f: &FieldDescriptorProto) -> TokenStream {
        FieldGenerator::new(self, f).extension()
    }

    fn descriptor(&self) -> TokenStream {
        let mut desc = BytesMut::new();
        self.file.proto().write_to_buf(&mut desc).unwrap();
        let data = desc.freeze();
        let data_slice = Literal::byte_string(&*data);
        quote! {
            static DESCRIPTOR_RAW: &[u8] = #data_slice;
            pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
        }
    }

    pub fn generate(&self) -> String {
        let mut token = quote! {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            #![allow(dead_code)]
            
            #[allow(unused_imports)]
            use pecan::prelude::*;
        };
        for e in &self.file.proto().extension {
            token.extend(self.generate_extension(e));
        }
        for e in &self.file.proto().enum_type {
            token.extend(self.generate_enum(e));
        }
        for e in &self.file.proto().message_type {
            token.extend(self.generate_message(e));
        }
        token.extend(self.descriptor());
        rustfmt(&token.to_string())
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn escape_option(&self) -> bool {
        self.escape_option
    }
}
