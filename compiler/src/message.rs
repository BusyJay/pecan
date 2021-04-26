use std::collections::HashMap;

use crate::field::FieldGenerator;
use crate::Generator;
use pecan_types::google::protobuf::descriptor_pb::*;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

pub struct MessageGenerator {
    name: String,
    fields: HashMap<u64, FieldGenerator>,
    decl_order: Vec<u64>,
    sorted_order: Vec<u64>,
    extension_range: Vec<(i32, i32)>,
}

impl MessageGenerator {
    pub fn new(g: &Generator, proto: &DescriptorProto, name: &str) -> Option<MessageGenerator> {
        if proto.options().map_entry() {
            return None;
        }
        let fgs: Vec<_> = proto
            .field
            .iter()
            .map(|f| FieldGenerator::new(g, f))
            .collect();
        let decl_order: Vec<_> = fgs.iter().map(|g| g.tag_value()).collect();
        let mut sorted_order = decl_order.clone();
        sorted_order.sort_unstable();
        let fields = fgs.into_iter().map(|g| (g.tag_value(), g)).collect();
        let extension_range = proto
            .extension_range
            .iter()
            .map(|r| (r.start(), r.end()))
            .collect();
        Some(MessageGenerator {
            name: name.to_string(),
            decl_order,
            sorted_order,
            fields,
            extension_range,
        })
    }

    fn name(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn decl(&self) -> TokenStream {
        let name = self.name();
        let decl = self
            .decl_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().field_decl());
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! { pub extensions: pecan::ExtensionMap, }
        };
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #name {
                #(#decl)*
                #extension
                _unknown: Vec<u8>,
            }
        }
    }

    fn init(&self) -> TokenStream {
        let name = self.name();
        let init = self
            .sorted_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().field_init());
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! { extensions: pecan::ExtensionMap::new(), }
        };
        quote! {
            pub const fn new() -> #name {
                #name {
                    #(#init)*
                    #extension
                    _unknown: Vec::new(),
                }
            }
        }
    }

    fn merge_from(&self) -> TokenStream {
        let merge_from = self
            .sorted_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().fn_merge_from());
        let extension = if self.extension_range.is_empty() {
            quote! {
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        } else {
            let (mut begin, mut end) = (vec![], vec![]);
            for (b, e) in &self.extension_range {
                begin.push(Literal::u64_unsuffixed((*b as u64) << 3));
                end.push(Literal::u64_unsuffixed((((*e) as u64) << 3) | 7));
            }
            quote! {
                tag => {
                    #(
                        if (#begin..=#end).contains(&tag) {
                            s.read_extension(tag, &mut self.extensions)?;
                            continue;
                        }
                    )*
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        };
        quote! {
            fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
                loop {
                    match s.read_tag()? {
                        #(#merge_from)*
                        0 => return Ok(()),
                        #extension
                    }
                }
            }
        }
    }

    pub fn write_to(&self) -> TokenStream {
        let write_to = self
            .sorted_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().fn_write_to());
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! {
                if !self.extensions.is_empty() {
                    s.write_extensions(&self.extensions)?;
                }
            }
        };
        quote! {
            fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
                #(#write_to)*
                #extension
                if !self._unknown.is_empty() {
                    s.write_raw_bytes(&self._unknown)?;
                }
                Ok(())
            }
        }
    }

    pub fn len(&self) -> TokenStream {
        let len = self
            .sorted_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().fn_len());
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! {
                if !self.extensions.is_empty() {
                    l += self.extensions.len();
                }
            }
        };
        quote! {
            fn len(&self) -> u64 {
                let mut l = 0;
                #(#len)*
                #extension
                if !self._unknown.is_empty() {
                    l += self._unknown.len() as u64;
                }
                l
            }
        }
    }

    fn accessors(&self) -> TokenStream {
        let accessors = self
            .sorted_order
            .iter()
            .map(|tag| self.fields.get(tag).unwrap().accessor());
        quote! {
            #(#accessors)*
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let decl = self.decl();
        let init = self.init();
        let merge_from = self.merge_from();
        let write_to = self.write_to();
        let len = self.len();
        let accessors = self.accessors();

        quote! {
            #decl

            impl #name {
                #init

                #accessors
            }

            impl pecan::Message for #name {
                #merge_from

                #write_to

                #len
            }

            impl pecan::DefaultInstance for #name {
                fn default_instance() -> &'static #name {
                    static DEFAULT: #name = #name::new();
                    &DEFAULT
                }
            }

            impl Default for #name {
                #[inline]
                fn default() -> #name {
                    #name::new()
                }
            }
        }
    }
}
