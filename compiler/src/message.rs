use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use pecan_types::google::protobuf::descriptor_pb::*;
use quote::{format_ident, quote};
use crate::field::FieldGenerator;
use crate::Generator;

pub struct MessageGenerator {
    name: String,
    fields: HashMap<u64, FieldGenerator>,
    decl_order: Vec<u64>,
    sorted_order: Vec<u64>,
    extension_range: Vec<(i32, i32)>,
}

impl MessageGenerator {
    pub fn new(g: &Generator, proto: &DescriptorProto, name: String) -> MessageGenerator {
        let fgs: Vec<_> = proto
            .field
            .iter()
            .map(|f| FieldGenerator::new(g, f))
            .collect();
        let decl_order: Vec<_> = fgs.iter().map(|g| g.tag_value()).collect();
        let mut sorted_order = decl_order.clone();
        sorted_order.sort();
        let fields = fgs.into_iter().map(|g| (g.tag_value(), g)).collect();
        let extension_range = proto.extension_range.iter().map(|r| (r.start(), r.end())).collect();
        MessageGenerator { name, decl_order, sorted_order, fields, extension_range }
    }

    fn name(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn decl(&self) -> TokenStream {
        let name = self.name();
        let decl = self.decl_order.iter().map(|tag| self.fields.get(tag).unwrap().field_init());
        let extension = if self.extension_range.is_empty() {
            quote!{}
        } else {
            quote!{ extensions: pecan::ExtensionSet, }
        };
        quote! {
            #[derive(Clone, Default, Debug)]
            pub struct #name {
                #(#decl)*
                #(extension)
                _unknown: Vec<u8>,
            }
        }
    }

    fn init(&self) -> TokenStream {
        let name = self.name();
        let init = self.sorted_order.iter().map(|tag| self.fields.get(tag).unwrap().field_init());
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! { extensions: pecan::ExtensionSet::new(), }
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
        
    }

    pub fn generate(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let decl = self.decl();
        let init = self.init();
        let merge_from = sorted_fgs.iter().map(|g| g.fn_merge_from());
        let write_to = sorted_fgs.iter().map(|g| g.fn_write_to());
        let len = sorted_fgs.iter().map(|g| g.fn_len());
        let accessors = sorted_fgs.iter().flat_map(|g| g.accessor());
        
        quote! {
            #[derive(Clone, Default, Debug)]
            pub struct #name {
                #(#decl)*
                _unknown: Vec<u8>,
            }

            impl #name {
                pub const fn new() -> #name {
                    #name {
                        #(#init)*
                        _unknown: Vec::new(),
                    }
                }

                #(#accessors)*
            }

            impl pecan::Message for #name {
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

            impl pecan::DefaultInstance for #name {
                fn default_instance() -> &'static #name {
                    static DEFAULT: #name = #name::new();
                    &DEFAULT
                }
            }
        }
    }
}
