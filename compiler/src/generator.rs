use crate::db::{Database, File};
use crate::field::FieldGenerator;
use crate::util::*;
use pecan_types::google::protobuf::descriptor_pb::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct Generator<'a> {
    db: &'a Database,
    file: &'a File,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(db: &'a Database, file: &'a File) -> Generator<'a> {
        Generator { db, file }
    }

    fn generate_enum(&self, e: &EnumDescriptorProto) -> TokenStream {
        let name = type_name(e.name.as_ref().unwrap(), "");
        self.generate_enum_impl(&name, e)
    }

    fn generate_enum_impl(&self, name: &str, e: &EnumDescriptorProto) -> TokenStream {
        let name_ident = format_ident!("{}", name);
        let mut raw_values: Vec<_> = e
            .value
            .iter()
            .map(|v| (escape(v.name.as_ref().unwrap()), v.number.unwrap()))
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
            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

            impl std::fmt::Display for #name_ident {
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
        let msg_name = type_name(d.name.as_ref().unwrap(), "");
        self.generate_message_impl(&msg_name, d)
    }

    fn generate_message_impl(&self, msg_name: &str, d: &DescriptorProto) -> TokenStream {
        let mut token = TokenStream::new();
        for e in &d.enum_type {
            let name = type_name(e.name.as_ref().unwrap(), msg_name);
            token.extend(self.generate_enum_impl(&name, e));
        }
        for e in &d.nested_type {
            let name = type_name(e.name.as_ref().unwrap(), msg_name);
            token.extend(self.generate_message_impl(&name, e));
        }
        let msg_name = format_ident!("{}", msg_name);
        let mut fgs: Vec<_> = d
            .field
            .iter()
            .map(|f| FieldGenerator::new(self, f))
            .collect();
        let decl: Vec<_> = fgs.iter().map(|g| g.field_decl()).collect();
        fgs.sort_by_key(|g| g.tag_value());
        let init = fgs.iter().map(|g| g.field_init());
        let merge_from = fgs.iter().map(|g| g.fn_merge_from());
        let write_to = fgs.iter().map(|g| g.fn_write_to());
        let len = fgs.iter().map(|g| g.fn_len());
        let accessors = fgs.iter().flat_map(|g| g.accessor());
        token.extend(quote! {
            #[derive(Clone, Default, Debug)]
            pub struct #msg_name {
                #(#decl)*
                _unknown: Vec<u8>,
            }

            impl #msg_name {
                pub const fn new() -> #msg_name {
                    #msg_name {
                        #(#init)*
                        _unknown: Vec::new(),
                    }
                }

                #(#accessors)*
            }

            impl pecan::Message for #msg_name {
                fn merge_from<B: bytes::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
                    loop {
                        match s.read_tag()? {
                            #(#merge_from)*
                            0 => return Ok(()),
                            tag => s.read_unknown_field(tag, &mut self._unknown)?,
                        }
                    }
                }

                fn write_to<B: bytes::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
                    #(#write_to)*
                    if !self._unknown.is_empty() {
                        s.write_raw_bytes(&self._unknown)?;
                    }
                    Ok(())
                }

                fn len(&self) -> u64 {
                    let mut l = 0;
                    #(#len)*
                    if !self._unknown.is_empty() {
                        l += self._unknown.len() as u64;
                    }
                    l
                }
            }

            impl pecan::DefaultInstance for #msg_name {
                fn default_instance() -> &'static #msg_name {
                    static DEFAULT: #msg_name = #msg_name::new();
                    &DEFAULT
                }
            }
        });
        token
    }

    pub fn generate(&self) -> String {
        let mut token = quote! {
            #![allow(non_camel_case_types)]

            use pecan::prelude::*;
        };
        for e in &self.file.proto().enum_type {
            token.extend(self.generate_enum(e));
        }
        for e in &self.file.proto().message_type {
            token.extend(self.generate_message(e));
        }
        rustfmt(&token.to_string())
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
