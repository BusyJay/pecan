use crate::{util, Generator};
use pecan::prelude::*;
use pecan_types::google::protobuf::descriptor_pb::*;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

#[derive(Clone, Copy, PartialEq, Debug)]
enum FieldKind {
    Boolean,
    // repeated i32
    Primitive,
    Bytes,
    Message,
}

impl FieldKind {
    fn can_copy(self) -> bool {
        matches!(self, FieldKind::Boolean | FieldKind::Primitive)
    }
}

#[derive(Debug, Clone)]
pub struct FieldGenerator {
    name: String,
    r#type: TokenStream,
    inner_type: TokenStream,
    tag: u64,
    codec: Ident,
    default_value: TokenStream,
    kind: FieldKind,
    repeated: bool,
    optional: bool,
}

impl FieldGenerator {
    pub fn new(generator: &Generator, f: &FieldDescriptorProto) -> FieldGenerator {
        let name = util::snake_name(f.name());
        let pb_ty = f.r#type();
        let (inner_type, wire_ty, codec, kind) = match pb_ty {
            FieldDescriptorProto_Type::TYPE_BOOL => (quote!(bool), 0, "Varint", FieldKind::Boolean),
            FieldDescriptorProto_Type::TYPE_BYTES => {
                (quote!(bytes::Bytes), 2, "LengthPrefixed", FieldKind::Bytes)
            }
            FieldDescriptorProto_Type::TYPE_DOUBLE => {
                (quote!(f64), 1, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_FIXED32 => {
                (quote!(u32), 5, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_FIXED64 => {
                (quote!(u64), 1, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_FLOAT => {
                (quote!(f32), 5, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_INT32 => {
                (quote!(i32), 0, "Varint", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_INT64 => {
                (quote!(i64), 0, "Varint", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                (quote!(i32), 5, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                (quote!(i64), 1, "Fixed", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_STRING => {
                (quote!(String), 2, "LengthPrefixed", FieldKind::Bytes)
            }
            FieldDescriptorProto_Type::TYPE_SINT32 => {
                (quote!(i32), 0, "ZigZag", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_SINT64 => {
                (quote!(i64), 0, "ZigZag", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_UINT32 => {
                (quote!(u32), 0, "Varint", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_UINT64 => {
                (quote!(u64), 0, "Varint", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_ENUM => {
                let p = generator.db().r#type(f.type_name()).unwrap();
                let ty = if p.package() == generator.file().full_package() {
                    p.name().parse().unwrap()
                } else {
                    format!("{}::{}", p.package(), p.name()).parse().unwrap()
                };
                (ty, 0, "Varint", FieldKind::Primitive)
            }
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                let p = generator.db().r#type(f.type_name()).unwrap();
                let ty = if p.package() == generator.file().full_package() {
                    p.name().parse().unwrap()
                } else {
                    format!("{}::{}", p.package(), p.name()).parse().unwrap()
                };
                (ty, 2, "LengthPrefixed", FieldKind::Message)
            }
            ty => panic!("unsupported type: {}", ty),
        };
        let label = f.label();
        let default_value = match pb_ty {
            FieldDescriptorProto_Type::TYPE_DOUBLE => quote! { 0f64 },
            FieldDescriptorProto_Type::TYPE_FLOAT => quote! { 0f32 },
            FieldDescriptorProto_Type::TYPE_BYTES
            | FieldDescriptorProto_Type::TYPE_STRING
            | FieldDescriptorProto_Type::TYPE_ENUM
            | FieldDescriptorProto_Type::TYPE_MESSAGE => quote! { #inner_type::new() },
            FieldDescriptorProto_Type::TYPE_BOOL => quote! { false },
            _ => quote! { 0 },
        };
        let number = f.number() as i64;
        let tag;
        let r#type;
        let (mut repeated, mut optional) = (false, false);
        if label == FieldDescriptorProto_Label::LABEL_OPTIONAL
            && generator.file().proto3()
            && pb_ty != FieldDescriptorProto_Type::TYPE_MESSAGE
            || label == FieldDescriptorProto_Label::LABEL_REQUIRED
        {
            tag = (number << 3) | wire_ty;
            r#type = quote! { #inner_type };
        } else if label == FieldDescriptorProto_Label::LABEL_OPTIONAL {
            tag = (number << 3) | wire_ty;
            r#type = quote! { Option<#inner_type> };
            optional = true;
        } else if label == FieldDescriptorProto_Label::LABEL_REPEATED {
            tag = (number << 3) | 2;
            r#type = quote! { Vec<#inner_type> };
            repeated = true;
        } else {
            panic!("unsupported label {}", label)
        };

        FieldGenerator {
            name,
            r#type,
            inner_type,
            tag: tag as u64,
            codec: format_ident!("{}{}", codec, if repeated { "Array" } else { "" }),
            default_value,
            kind,
            repeated,
            optional,
        }
    }

    pub fn name(&self) -> Ident {
        format_ident!("{}", util::escape(&self.name))
    }

    pub fn tag(&self) -> Literal {
        Literal::u64_unsuffixed(self.tag)
    }

    pub fn fn_merge_from(&self) -> TokenStream {
        let name = self.name();
        let tag = self.tag();
        let codec = &self.codec;
        if self.kind == FieldKind::Message {
            if !self.optional {
                quote! {
                    #tag => #codec::merge_from(&mut self.#name, s)?,
                }
            } else {
                let accessor = format_ident!("{}_mut", self.name);
                quote! {
                    #tag => #codec::merge_from(self.#accessor(), s)?,
                }
            }
        } else if self.optional {
            quote! {
                #tag => self.#name = Some(#codec::read_from(s)?),
            }
        } else if self.repeated {
            quote! {
                #tag => #codec::merge_from(&mut self.#name, s)?,
            }
        } else {
            quote! {
                #tag => self.#name = #codec::read_from(s)?,
            }
        }
    }

    fn check_empty(
        &self,
        action: impl FnOnce(TokenStream, &TokenStream) -> TokenStream,
    ) -> TokenStream {
        let name = self.name();
        let read_accessor = if self.kind.can_copy() && !self.repeated {
            quote! { self.#name }
        } else {
            quote! { &self.#name }
        };
        if self.optional {
            let t = action(quote!(v), &quote!(v));
            quote! { if let Some(v) = #read_accessor { #t } }
        } else {
            let t = action(quote! { self.#name }, &read_accessor);
            if self.repeated || self.kind == FieldKind::Bytes {
                quote! { if !self.#name.is_empty() { #t } }
            } else if self.kind == FieldKind::Boolean {
                quote! { if self.#name { #t } }
            } else {
                let val = &self.default_value;
                quote! { if self.#name != #val { #t } }
            }
        }
    }

    pub fn fn_write_to(&self) -> TokenStream {
        let tag = self.tag();
        let codec = &self.codec;
        if !self.repeated {
            self.check_empty(|_, v| {
                quote! {
                    s.write_tag(#tag)?;
                    #codec::write_to(#v, s)?;
                }
            })
        } else if !self.kind.can_copy() {
            self.check_empty(|_, v| {
                quote! {
                    for i in #v {
                        s.write_tag(#tag)?;
                        LengthPrefixed::write_to(i, s)?;
                    }
                }
            })
        } else {
            self.check_empty(|_, v| {
                quote! {
                    s.write_tag(#tag)?;
                    #codec::write_to(#v, s)?;
                }
            })
        }
    }

    pub fn fn_len(&self) -> TokenStream {
        let len_raw = Varint::len(self.tag);
        let tag_len = Literal::u64_unsuffixed(len_raw);
        let codec = &self.codec;
        if !self.repeated {
            self.check_empty(|_, v| {
                quote! { l += #tag_len + #codec::len(#v); }
            })
        } else if !self.kind.can_copy() {
            self.check_empty(|v, v_ref| {
                let vector_len = if len_raw == 1 {
                    quote! { #v.len() }
                } else {
                    quote! { #tag_len * #v.len() }
                };
                quote! {
                    l += #vector_len as u64 + #codec::len(#v_ref);
                }
            })
        } else {
            self.check_empty(|_, v| {
                quote! {
                    l += #tag_len + #codec::len(#v);
                }
            })
        }
    }

    pub fn field_decl(&self) -> TokenStream {
        let name = self.name();
        let ty = &self.r#type;
        quote! {
            pub #name: #ty,
        }
    }

    pub fn field_init(&self) -> TokenStream {
        let name = self.name();
        if self.optional {
            quote! {
                #name: None,
            }
        } else if self.repeated {
            quote! {
                #name: Vec::new(),
            }
        } else {
            let val = &self.default_value;
            quote! {
                #name: #val,
            }
        }
    }

    pub fn tag_value(&self) -> u64 {
        self.tag
    }

    pub fn accessor(&self) -> Option<TokenStream> {
        if !self.optional {
            return None;
        }
        let name = self.name();
        let mutter = format_ident!("{}_mut", self.name);
        let setter = format_ident!("set_{}", self.name);
        let val = &self.default_value;
        let ty = &self.r#inner_type;
        let getter = if self.kind.can_copy() {
            quote! {
                pub fn #name(&self) -> #ty {
                    self.#name.unwrap_or_default()
                }
            }
        } else {
            quote! {
                pub fn #name(&self) -> &#ty {
                    match &self.#name {
                        Some(v) => v,
                        None => #ty::default_instance(),
                    }
                }
            }
        };
        Some(quote! {
            #getter

            pub fn #mutter(&mut self) -> &mut #ty {
                if self.#name.is_none() {
                    self.#name = Some(#val);
                }

                self.#name.as_mut().unwrap()
            }

            pub fn #setter(&mut self, val: #ty) {
                self.#name = Some(val);
            }
        })
    }

    pub fn extension(&self) -> TokenStream {
        let name = util::const_name(&self.name);
        let name_ident = format_ident!("{}", name);
        let tag = self.tag();
        let ty = if self.optional {
            &self.inner_type
        } else {
            &self.r#type
        };
        let codec = &self.codec;
        quote! {
            pub const #name_ident: pecan::Extension<#ty, #codec> = pecan::Extension::new(#tag);
        }
    }
}
