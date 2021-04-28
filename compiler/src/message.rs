use std::collections::{HashMap, HashSet};

use crate::field::{FieldGenerator, OneOfGenerator};
use crate::Generator;
use pecan::reflection::*;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

pub enum Field {
    Normal(u64),
    OneOf(usize),
}

pub struct MessageGenerator {
    name: String,
    fields: HashMap<u64, FieldGenerator>,
    decl_order: Vec<Field>,
    one_of_fields: Vec<OneOfGenerator>,
    extension_range: Vec<(i32, i32)>,
    group: i32,
}

impl MessageGenerator {
    pub fn new(
        g: &Generator,
        proto: &DescriptorProto,
        type_id: &str,
        name: &str,
    ) -> Option<MessageGenerator> {
        if proto.options().map_entry() {
            return None;
        }
        let mut decl_order = vec![];
        let mut register_idx = HashSet::new();
        let mut one_of_fields = vec![];
        for decl in &proto.oneof_decl {
            one_of_fields.push(OneOfGenerator::new(name, decl));
        }
        let mut fgs = vec![];
        for proto in &proto.field {
            let g = FieldGenerator::new(g, proto);
            if let Some(idx) = g.one_of_index() {
                if !register_idx.contains(&idx) {
                    decl_order.push(Field::OneOf(idx));
                    register_idx.insert(idx);
                }
                one_of_fields[idx].register(g);
            } else {
                decl_order.push(Field::Normal(g.tag_value()));
                fgs.push(g);
            }
        }

        let fields = fgs.into_iter().map(|g| (g.tag_value(), g)).collect();
        let extension_range = proto
            .extension_range
            .iter()
            .map(|r| (r.start(), r.end()))
            .collect();
        Some(MessageGenerator {
            name: name.to_string(),
            one_of_fields,
            decl_order,
            fields,
            extension_range,
            group: g.db().r#type(type_id).expect(&type_id).group(),
        })
    }

    fn name(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn decl(&self) -> TokenStream {
        let name = self.name();
        let mut decls = vec![];
        for f in &self.decl_order {
            let decl = match f {
                Field::Normal(n) => self.fields.get(n).unwrap().field_decl(),
                Field::OneOf(n) => self.one_of_fields[*n].field_decl(),
            };
            decls.push(decl);
        }
        let extension = if self.extension_range.is_empty() {
            quote! {}
        } else {
            quote! { pub extensions: pecan::ExtensionMap, }
        };
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #name {
                #(#decls)*
                #extension
                _unknown: Vec<u8>,
            }
        }
    }

    fn init(&self) -> TokenStream {
        let name = self.name();
        let mut init = vec![];
        for f in &self.decl_order {
            let token = match f {
                Field::Normal(tag) => self.fields[tag].field_init(),
                Field::OneOf(off) => self.one_of_fields[*off].field_init(),
            };
            init.push(token);
        }
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
        let mut merge_from = vec![];
        for (tag, g) in &self.fields {
            merge_from.push((*tag, g.fn_merge_from()));
        }
        for g in &self.one_of_fields {
            merge_from.extend(g.fn_merge_from());
        }
        merge_from.sort_by_key(|i| i.0);
        let merge_from = merge_from.into_iter().map(|i| i.1);
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
        let end = if self.group == 0 {
            quote!(0 => return Ok(()),)
        } else {
            let tag = Literal::u64_unsuffixed(((self.group as u64) << 3) | 4);
            quote!(0 | #tag => {
                s.set_last_tag(#tag);
                return Ok(());
            })
        };
        quote! {
            fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
                loop {
                    match s.read_tag()? {
                        #(#merge_from)*
                        #end
                        #extension
                    }
                }
            }
        }
    }

    pub fn write_to(&self) -> TokenStream {
        let mut write_to = vec![];
        for (tag, g) in &self.fields {
            write_to.push((*tag, g.fn_write_to()));
        }
        for g in &self.one_of_fields {
            write_to.push(g.fn_write_to());
        }
        write_to.sort_by_key(|i| i.0);
        let write_to = write_to.into_iter().map(|i| i.1);
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
        let mut len = vec![];
        for (tag, g) in &self.fields {
            len.push((*tag, g.fn_len()));
        }
        for g in &self.one_of_fields {
            len.push(g.fn_len());
        }
        len.sort_by_key(|i| i.0);
        let len = len.into_iter().map(|i| i.1);
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
        let mut accessors = vec![];
        for f in &self.decl_order {
            let token = match f {
                Field::Normal(tag) => self.fields[tag].accessor(),
                Field::OneOf(off) => Some(self.one_of_fields[*off].accessor()),
            };
            if let Some(t) = token {
                accessors.push(t)
            }
        }
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
        let one_of_enums = self.one_of_fields.iter().map(|g| g.generate());

        quote! {
            #(#one_of_enums)*

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
