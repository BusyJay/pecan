use crate::options_pb::{FieldOptions, FIELD_OPT};
use crate::{util, Generator};
use pecan::prelude::*;
use pecan::reflection::*;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

#[derive(Debug, Clone)]
enum FieldKind {
    Boolean,
    // repeated i32
    Primitive,
    Bytes,
    Message,
    HashMap(Box<FieldGenerator>, Box<FieldGenerator>),
    Group,
}

impl FieldKind {
    fn can_copy(&self) -> bool {
        matches!(self, FieldKind::Boolean | FieldKind::Primitive)
    }

    fn is_hash_map(&self) -> bool {
        matches!(self, FieldKind::HashMap(..))
    }

    fn is_message(&self) -> bool {
        matches!(self, FieldKind::Message | FieldKind::Group)
    }
}

fn is_message_type(e: &FieldDescriptorProto) -> bool {
    matches!(
        e.r#type(),
        FieldDescriptorProto_Type::TYPE_MESSAGE | FieldDescriptorProto_Type::TYPE_GROUP
    )
}

fn wire_type(e: &FieldDescriptorProto, proto3: bool) -> (u8, u8, TokenStream, TokenStream) {
    let (wire, codec) = match e.r#type() {
        FieldDescriptorProto_Type::TYPE_BOOL
        | FieldDescriptorProto_Type::TYPE_INT32
        | FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_UINT32
        | FieldDescriptorProto_Type::TYPE_UINT64
        | FieldDescriptorProto_Type::TYPE_ENUM => (0, "Varint"),
        FieldDescriptorProto_Type::TYPE_SINT32 | FieldDescriptorProto_Type::TYPE_SINT64 => {
            (0, "ZigZag")
        }
        FieldDescriptorProto_Type::TYPE_DOUBLE
        | FieldDescriptorProto_Type::TYPE_FIXED64
        | FieldDescriptorProto_Type::TYPE_SFIXED64 => (1, "Fixed64"),
        FieldDescriptorProto_Type::TYPE_BYTES
        | FieldDescriptorProto_Type::TYPE_STRING
        | FieldDescriptorProto_Type::TYPE_MESSAGE => (2, "LengthPrefixed"),
        FieldDescriptorProto_Type::TYPE_FLOAT
        | FieldDescriptorProto_Type::TYPE_FIXED32
        | FieldDescriptorProto_Type::TYPE_SFIXED32 => (5, "Fixed32"),
        FieldDescriptorProto_Type::TYPE_GROUP => (3, "Group"),
        ty => panic!("unsupported type {:?}", ty),
    };
    let codec = format_ident!("{}", codec);
    let codec = quote!(#codec);
    if e.label() == FieldDescriptorProto_Label::LABEL_REPEATED {
        if ![0, 1, 5].contains(&wire) {
            (wire, wire, quote! {RefArray::<#codec>}, codec)
        } else if e.options().packed() || proto3 && e.options.is_none() {
            (2, wire, quote! {PackedArray::<#codec>}, codec)
        } else {
            (wire, 2, quote! {CopyArray::<#codec>}, codec)
        }
    } else {
        (wire, wire, codec.clone(), codec)
    }
}

fn opt_type(g: &Generator) -> TokenStream {
    if g.escape_option() {
        quote! {std::option::Option}
    } else {
        quote! {Option}
    }
}

fn field_type_name(
    g: &Generator,
    e: &FieldDescriptorProto,
    opt: &mut FieldOptions,
) -> (TokenStream, TokenStream, FieldKind) {
    let (ty, kind) = match e.r#type() {
        FieldDescriptorProto_Type::TYPE_BOOL => (quote! {bool}, FieldKind::Boolean),
        FieldDescriptorProto_Type::TYPE_SFIXED32
        | FieldDescriptorProto_Type::TYPE_INT32
        | FieldDescriptorProto_Type::TYPE_SINT32 => (quote! {i32}, FieldKind::Primitive),
        FieldDescriptorProto_Type::TYPE_FIXED32 | FieldDescriptorProto_Type::TYPE_UINT32 => {
            (quote! {u32}, FieldKind::Primitive)
        }
        FieldDescriptorProto_Type::TYPE_SFIXED64
        | FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_SINT64 => (quote! {i64}, FieldKind::Primitive),
        FieldDescriptorProto_Type::TYPE_FIXED64 | FieldDescriptorProto_Type::TYPE_UINT64 => {
            (quote! {u64}, FieldKind::Primitive)
        }
        FieldDescriptorProto_Type::TYPE_FLOAT => (quote! {f32}, FieldKind::Primitive),
        FieldDescriptorProto_Type::TYPE_DOUBLE => (quote! {f64}, FieldKind::Primitive),
        FieldDescriptorProto_Type::TYPE_ENUM => {
            let p = g.db().r#type(e.type_name()).unwrap();
            (
                if p.package() == g.file().full_package() {
                    p.name().parse().unwrap()
                } else {
                    format!("{}::{}", p.package(), p.name()).parse().unwrap()
                },
                FieldKind::Primitive,
            )
        }
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let p = g.db().r#type(e.type_name()).unwrap();
            if p.message().map_or(false, |m| m.options().map_entry()) {
                let m = p.message().unwrap();
                let key_g = FieldGenerator::new(g, &m.field[0]);
                let val_g = FieldGenerator::new(g, &m.field[1]);
                let key_ty = key_g.inner_type.clone();
                let val_ty = val_g.inner_type.clone();
                let m = quote!(pecan::HashMap<#key_ty, #val_ty>);
                let container = opt_type(g);
                let wrapper = quote!(#container<#m>);
                return (
                    m,
                    wrapper,
                    FieldKind::HashMap(Box::new(key_g), Box::new(val_g)),
                );
            } else {
                (
                    if p.package() == g.file().full_package() {
                        p.name().parse().unwrap()
                    } else {
                        format!("{}::{}", p.package(), p.name()).parse().unwrap()
                    },
                    FieldKind::Message,
                )
            }
        }
        FieldDescriptorProto_Type::TYPE_GROUP => {
            let p = g.db().r#type(e.type_name()).unwrap();
            (p.name().parse().unwrap(), FieldKind::Group)
        }
        FieldDescriptorProto_Type::TYPE_BYTES => (quote!(pecan::Bytes), FieldKind::Bytes),
        FieldDescriptorProto_Type::TYPE_STRING => (quote!(String), FieldKind::Bytes),
        ty => panic!("unsupported type {:?}", ty),
    };

    let mut wrap_type = if is_message_type(e)
        && e.label() != FieldDescriptorProto_Label::LABEL_REPEATED
        && opt.box_field
    {
        quote! { Box<#ty> }
    } else {
        opt.box_field = false;
        ty.clone()
    };
    if e.label() == FieldDescriptorProto_Label::LABEL_OPTIONAL
        && (!g.file().proto3() || is_message_type(e))
    {
        let container = opt_type(g);
        wrap_type = quote! { #container<#wrap_type> };
    } else if e.label() == FieldDescriptorProto_Label::LABEL_REPEATED {
        wrap_type = quote! { Vec<#ty> };
    }
    (ty, wrap_type, kind)
}

fn default_value(
    e: &FieldDescriptorProto,
    inner_type: &TokenStream,
    kind: &FieldKind,
) -> TokenStream {
    match e.r#type() {
        FieldDescriptorProto_Type::TYPE_BOOL => quote! {false},
        FieldDescriptorProto_Type::TYPE_ENUM
        | FieldDescriptorProto_Type::TYPE_BYTES
        | FieldDescriptorProto_Type::TYPE_STRING
        | FieldDescriptorProto_Type::TYPE_MESSAGE
        | FieldDescriptorProto_Type::TYPE_GROUP => {
            if matches!(kind, FieldKind::HashMap(..)) {
                quote!(pecan::HashMap::new())
            } else {
                quote!(#inner_type::new())
            }
        }
        FieldDescriptorProto_Type::TYPE_FLOAT => quote!(0f32),
        FieldDescriptorProto_Type::TYPE_DOUBLE => quote!(0f64),
        _ => quote! {0},
    }
}

#[derive(Debug, Clone)]
pub struct FieldGenerator {
    name: String,
    r#type: TokenStream,
    inner_type: TokenStream,
    tag: u64,
    second_tag: u64,
    codec: TokenStream,
    inner_codec: TokenStream,
    default_value: TokenStream,
    kind: FieldKind,
    opt: crate::options_pb::FieldOptions,
    repeated: bool,
    optional: bool,
    one_of_index: Option<usize>,
}

impl FieldGenerator {
    pub fn new(generator: &Generator, f: &FieldDescriptorProto) -> FieldGenerator {
        let name = util::snake_name(f.name());
        let (wire_ty, sec_wire_ty, codec, inner_codec) = wire_type(f, generator.file().proto3());
        let mut opt = f
            .options()
            .extensions
            .get(&FIELD_OPT)
            .unwrap()
            .unwrap_or_default();
        let (inner_type, r#type, kind) = field_type_name(generator, f, &mut opt);

        let mut default_value = default_value(f, &inner_type, &kind);
        if opt.box_field {
            default_value = quote! { Box::new(#default_value) };
        }
        let tag = ((f.number() as u64) << 3) | wire_ty as u64;
        let optional = r#type.to_string().contains("Option <");
        let repeated = r#type.to_string().contains("Vec <");

        FieldGenerator {
            name,
            r#type,
            opt,
            inner_type,
            tag,
            second_tag: ((f.number() as u64) << 3) | sec_wire_ty as u64,
            codec,
            inner_codec,
            default_value,
            kind,
            repeated,
            optional,
            one_of_index: f.oneof_index.map(|v| v as usize),
        }
    }

    pub fn name(&self) -> Ident {
        format_ident!("{}", util::escape(&self.name))
    }

    pub fn tag(&self) -> Literal {
        Literal::u64_unsuffixed(self.tag)
    }

    fn end_tag_value(&self) -> u64 {
        (self.tag & (!0x7)) | 4
    }

    pub fn group_end_tag(&self) -> Literal {
        Literal::u64_unsuffixed(self.end_tag_value())
    }

    pub fn group_end_tag_len(&self) -> u64 {
        Varint::size(self.end_tag_value())
    }

    pub fn one_of_index(&self) -> Option<usize> {
        self.one_of_index
    }

    fn map_entry_merge_from(&self, ident: TokenStream) -> TokenStream {
        let tag = self.tag();
        let codec = &self.codec;
        if !self.kind.can_copy() {
            quote! {
                #tag => #codec::merge_from(&mut #ident, s)?,
            }
        } else {
            quote! {
                #tag => #ident = #codec::read_from(s)?,
            }
        }
    }

    fn map_merge_from(&self, kg: &FieldGenerator, vg: &FieldGenerator) -> TokenStream {
        let tag = self.tag();
        let (key_default, val_default) = (&kg.default_value, &vg.default_value);
        let key_mf = kg.map_entry_merge_from(quote!(key));
        let val_mf = vg.map_entry_merge_from(quote!(val));
        let accessor = format_ident!("{}_mut", self.name);
        quote! {
            #tag => {
                s.read_length_limited(|s| {
                    let mut key = #key_default;
                    let mut val = #val_default;
                    loop {
                        match s.read_tag()? {
                            #key_mf
                            #val_mf
                            0 => break,
                            _ => ()
                        }
                    }
                    self.#accessor().insert(key, val);
                    Ok(())
                })?;
            }
        }
    }

    pub fn fn_merge_from(&self) -> TokenStream {
        if let FieldKind::HashMap(kg, vg) = &self.kind {
            return self.map_merge_from(kg, vg);
        }
        let name = self.name();
        let tag = self.tag();
        let codec = &self.codec;
        if matches!(self.kind, FieldKind::Group) {
            let accessor = if !self.optional {
                quote! { self.#name }
            } else {
                let method = format_ident!("{}_mut", self.name);
                quote! { self.#method() }
            };
            let end_tag = self.group_end_tag();
            let handle = if !self.repeated {
                quote! { #accessor.merge_from(s) }
            } else {
                let default_val = &self.default_value;
                quote! {{
                    #accessor.push(#default_val);
                    #accessor.last_mut().unwrap().merge_from(s)
                }}
            };
            quote! {
                #tag => s.read_group(#end_tag, |s| #handle)?,
            }
        } else if !self.kind.can_copy() {
            let accessor = if !self.optional {
                if self.opt.box_field {
                    quote! { &mut *self.#name }
                } else {
                    quote! { &mut self.#name }
                }
            } else {
                let method = format_ident!("{}_mut", self.name);
                quote! { self.#method() }
            };
            quote! {
                #tag => #codec::merge_from(#accessor, s)?,
            }
        } else if self.optional {
            quote! {
                #tag => self.#name = Some(#codec::read_from(s)?),
            }
        } else if self.repeated {
            let mut q = quote! {
                #tag => #codec::merge_from(&mut self.#name, s)?,
            };
            if self.tag != self.second_tag {
                let second_tag = Literal::u64_unsuffixed(self.second_tag);
                let inner_codec = &self.inner_codec;
                let codec = if self.second_tag & 0x7 == 2 {
                    quote! {PackedArray::<#inner_codec>}
                } else {
                    quote! {CopyArray::<#inner_codec>}
                };
                q.extend(quote! {
                    #second_tag => #codec::merge_from(&mut self.#name, s)?,
                })
            }
            q
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
            let t = if !self.opt.box_field {
                action(quote!(v), &quote!(v))
            } else {
                action(quote!(v), &quote!(v.as_ref()))
            };
            quote! { if let Some(v) = #read_accessor { #t } }
        } else {
            let t = if !self.opt.box_field {
                action(quote! { self.#name }, &read_accessor)
            } else {
                action(quote! { self.#name }, &quote! { &*self.#name })
            };
            if self.repeated || matches!(self.kind, FieldKind::Bytes) {
                quote! { if !self.#name.is_empty() { #t } }
            } else if matches!(self.kind, FieldKind::Boolean) {
                quote! { if self.#name { #t } }
            } else if !self.kind.is_message() {
                let val = &self.default_value;
                quote! { if self.#name != #val { #t } }
            } else {
                t
            }
        }
    }

    fn map_entry_write_to_uncheck(&self, mut ident: TokenStream) -> TokenStream {
        let tag = self.tag();
        let codec = &self.codec;
        if self.kind.can_copy() {
            ident = quote!(*#ident);
        }
        quote! {
            s.write_tag(#tag)?;
            #codec::write_to(#ident, s)?;
        }
    }

    fn map_write_to_uncheck(&self, kg: &FieldGenerator, vg: &FieldGenerator) -> TokenStream {
        let tag = self.tag();
        let key_len = kg.map_entry_len(quote!(key));
        let val_len = vg.map_entry_len(quote!(val));
        let key_w = kg.map_entry_write_to_uncheck(quote!(key));
        let val_w = vg.map_entry_write_to_uncheck(quote!(val));
        self.check_empty(|_, v| {
            quote! {
                for (key, val) in #v {
                    s.write_tag(#tag)?;
                    let l = #key_len + #val_len;
                    Varint::write_to(l, s)?;
                    #key_w
                    #val_w
                }
            }
        })
    }

    pub fn fn_write_to_uncheck(&self) -> TokenStream {
        if let FieldKind::HashMap(kg, vg) = &self.kind {
            return self.map_write_to_uncheck(kg, vg);
        }
        let tag = self.tag();
        let codec = &self.codec;
        let end_tag = self.group_end_tag();
        if !self.repeated {
            self.check_empty(|v, v_ref| {
                if matches!(self.kind, FieldKind::Group) {
                    quote! {
                        s.write_tag(#tag)?;
                        #v.write_to_uncheck(s)?;
                        s.write_tag(#end_tag)?;
                    }
                } else {
                    quote! {
                        s.write_tag(#tag)?;
                        #codec::write_to(#v_ref, s)?;
                    }
                }
            })
        } else {
            self.check_empty(|_, v_ref| {
                if matches!(self.kind, FieldKind::Group) {
                    quote! {
                        for i in #v_ref {
                            s.write_tag(#tag)?;
                            i.write_to_uncheck(s)?;
                            s.write_tag(#end_tag)?;
                        }
                    }
                } else if self.tag & 0x7 == 2 && self.kind.can_copy() {
                    quote! {
                        s.write_tag(#tag)?;
                        #codec::write_to(#v_ref, s)?
                    }
                } else {
                    let item = if self.kind.can_copy() {
                        quote!(*i)
                    } else {
                        quote!(i)
                    };
                    let inner_codec = &self.inner_codec;
                    quote! {
                        for i in #v_ref {
                            s.write_tag(#tag)?;
                            #inner_codec::write_to(#item, s)?;
                        }
                    }
                }
            })
        }
    }

    fn map_entry_len(&self, mut ident: TokenStream) -> TokenStream {
        let (_, tag_len) = self.tag_len();
        let codec = &self.codec;
        if self.kind.can_copy() {
            ident = quote!(*#ident);
        }
        quote!(#tag_len + #codec::size(#ident))
    }

    fn map_len(&self, kg: &FieldGenerator, vg: &FieldGenerator) -> TokenStream {
        let (len_raw, tag_len) = self.tag_len();
        let key_l = kg.map_entry_len(quote!(key));
        let val_l = vg.map_entry_len(quote!(val));
        self.check_empty(|_, v| {
            let mut token = if len_raw == 1 {
                quote!(l += #v.len() as u64;)
            } else {
                quote!(l += #tag_len * #v.len() as u64;)
            };
            token.extend(quote! {
                for (key, val) in #v {
                    let el = #key_l + #val_l;
                    l += Varint::size(el) + el;
                }
            });
            token
        })
    }

    pub fn fn_len(&self) -> TokenStream {
        if let FieldKind::HashMap(kg, vg) = &self.kind {
            return self.map_len(kg, vg);
        }
        let (len_raw, tag_len) = self.tag_len();
        let end_tag_len_raw = self.group_end_tag_len();
        let group_tag_len = Literal::u64_unsuffixed(len_raw + end_tag_len_raw);
        let codec = &self.codec;
        if !self.repeated {
            self.check_empty(|v, v_ref| {
                if matches!(self.kind, FieldKind::Group) {
                    quote! { l += #group_tag_len + #v.size(); }
                } else {
                    quote! { l += #tag_len + #codec::size(#v_ref); }
                }
            })
        } else if !self.kind.can_copy() || self.tag & 0x7 != 2 {
            self.check_empty(|v, v_ref| {
                let vector_len = if len_raw == 1 {
                    quote! { #v.len() as u64 }
                } else {
                    quote! { #tag_len * #v.len() as u64 }
                };
                if matches!(self.kind, FieldKind::Group) {
                    quote! {
                        l += #group_tag_len * #v.len() as u64;
                        for i in #v_ref {
                            l += i.size();
                        }
                    }
                } else {
                    quote! {
                        l += #vector_len + #codec::size(#v_ref);
                    }
                }
            })
        } else {
            self.check_empty(|_, v| {
                quote! {
                    l += #tag_len + #codec::size(#v);
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

    pub fn field_clear(&self) -> TokenStream {
        let name = self.name();
        let default_value = &self.default_value;
        if self.optional {
            quote! { self.#name = None; }
        } else if self.repeated {
            quote! { self.#name.clear(); }
        } else if self.kind.can_copy() {
            quote! { self.#name = #default_value; }
        } else {
            quote! { self.#name.clear(); }
        }
    }

    pub fn tag_len(&self) -> (u64, Literal) {
        let len_raw = Varint::size(self.tag);
        (len_raw, Literal::u64_unsuffixed(len_raw))
    }

    pub fn tag_value(&self) -> u64 {
        self.tag
    }

    fn one_of_item(&self) -> Ident {
        let mut name = util::camel_name(&self.name);
        name.retain(|c| c != '_');
        format_ident!("{}", name)
    }

    fn getter_impl(
        &self,
        field_name: &Ident,
        container: TokenStream,
        field_copy: bool,
    ) -> TokenStream {
        let ty = self.inner_type.clone();
        let mut ty_ref = ty.clone();
        if !self.kind.can_copy() {
            ty_ref = quote!(&#ty);
        }
        let name = self.name();
        let (accessor, v) = if field_copy {
            (quote!(self.#field_name), quote!(v))
        } else if self.kind.can_copy() {
            (quote!(&self.#field_name), quote!(*v))
        } else {
            (quote!(&self.#field_name), quote!(v))
        };
        let default_value = if self.kind.can_copy() {
            self.default_value.clone()
        } else if self.kind.is_hash_map() {
            quote! {{
                pecan::lazy_static! {
                    static ref DEFAULT: #ty = pecan::HashMap::default();
                }

                &*DEFAULT
            }}
        } else {
            quote! { #ty::default_instance() }
        };
        quote! {
            pub fn #name(&self) -> #ty_ref {
                match #accessor {
                    #container(v) => #v,
                    _ => #default_value
                }
            }
        }
    }

    fn one_of_getter(&self, field_name: &Ident, full_type: &Ident, ty_copy: bool) -> TokenStream {
        let item = self.one_of_item();
        self.getter_impl(field_name, quote!(#full_type::#item), ty_copy)
    }

    fn getter(&self) -> TokenStream {
        let name = self.name();
        let ty = self.inner_type.clone();
        if self.kind.can_copy() {
            return quote! {
                pub fn #name(&self) -> #ty {
                    self.#name.unwrap_or_default()
                }
            };
        }

        self.getter_impl(&name, quote!(Some), false)
    }

    fn setter_impl(&self, field_name: &Ident, container: TokenStream) -> TokenStream {
        let method = format_ident!("set_{}", self.name);
        let ty = &self.r#inner_type;
        let set_ty = if self.opt.box_field && !self.repeated {
            quote! { Box<#ty> }
        } else {
            ty.clone()
        };
        quote! {
            pub fn #method(&mut self, val: #set_ty) {
                self.#field_name = #container(val);
            }
        }
    }

    fn one_of_setter(&self, field_name: &Ident, full_type: &Ident) -> TokenStream {
        let item = self.one_of_item();
        self.setter_impl(field_name, quote!(#full_type::#item))
    }

    fn setter(&self) -> TokenStream {
        let name = self.name();
        self.setter_impl(&name, quote!(Some))
    }

    fn one_of_mutter(&self, field_name: &Ident, full_type: &Ident) -> TokenStream {
        let item = self.one_of_item();
        let default_val = &self.default_value;
        let method = format_ident!("{}_mut", self.name);
        let ty = &self.r#inner_type;

        quote! {
            pub fn #method(&mut self) -> &mut #ty {
                if !matches!(self.#field_name, #full_type::#item(_)) {
                    self.#field_name = #full_type::#item(#default_val);
                }

                match &mut self.#field_name {
                    #full_type::#item(v) => v,
                    _ => unreachable!()
                }
            }
        }
    }

    fn mutter(&self) -> TokenStream {
        let name = self.name();
        let method = format_ident!("{}_mut", self.name);
        let ty = &self.r#inner_type;

        quote! {
            pub fn #method(&mut self) -> &mut #ty {
                self.#name.get_or_insert_with(Default::default)
            }
        }
    }

    pub fn accessor(&self) -> Option<TokenStream> {
        if !self.optional {
            return None;
        }
        let getter = self.getter();
        let mutter = self.mutter();
        let setter = self.setter();
        Some(quote! {
            #getter

            #mutter

            #setter
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

pub struct OneOfGenerator {
    type_name: String,
    field_name: String,
    options: Vec<FieldGenerator>,
}

impl OneOfGenerator {
    pub fn new(prefix: &str, oneof: &OneofDescriptorProto) -> OneOfGenerator {
        let type_name = util::camel_name(&format!("{}_{}", prefix, oneof.name()));
        let field_name = util::escape(&util::snake_name(oneof.name()));
        OneOfGenerator {
            type_name,
            field_name,
            options: vec![],
        }
    }

    fn field_name(&self) -> Ident {
        format_ident!("{}", self.field_name)
    }

    fn type_name(&self) -> Ident {
        format_ident!("{}", self.type_name)
    }

    pub fn register(&mut self, g: FieldGenerator) {
        self.options.push(g);
    }

    pub fn field_decl(&self) -> TokenStream {
        let name = self.field_name();
        let ty = self.type_name();
        quote! {
            pub #name: #ty,
        }
    }

    pub fn field_init(&self) -> TokenStream {
        let name = self.field_name();
        let ty = self.type_name();
        quote! {
            #name: #ty::None,
        }
    }

    pub fn field_clear(&self) -> TokenStream {
        let name = self.field_name();
        let ty = self.type_name();
        quote! { self.#name = #ty::None; }
    }

    pub fn fn_merge_from(&self) -> impl Iterator<Item = (u64, TokenStream)> + '_ {
        let ty = self.type_name();
        let name = self.field_name();
        self.options.iter().map(move |g| {
            let tag = g.tag();
            let codec = &g.codec;
            let val = g.tag_value();
            let token = if !g.kind.can_copy() {
                let method = format_ident!("{}_mut", g.name);
                quote! {
                    #tag => #codec::merge_from(self.#method(), s)?,
                }
            } else {
                // map field is not allowed in oneof.
                let item = g.one_of_item();
                quote! {
                    #tag => self.#name = #ty::#item(#codec::read_from(s)?),
                }
            };
            (val, token)
        })
    }

    fn can_copy(&self) -> bool {
        self.options.iter().all(|g| g.kind.can_copy())
    }

    fn check_empty(
        &self,
        action: impl Fn(&FieldGenerator, TokenStream) -> TokenStream,
    ) -> TokenStream {
        let ty = self.type_name();
        let name = self.field_name();
        let ty_copy = self.can_copy();
        let name = if !ty_copy {
            quote! { &self.#name }
        } else {
            quote! { self.#name }
        };
        let branches = self.options.iter().map(|g| {
            let item = g.one_of_item();
            let val = if g.kind.can_copy() && !ty_copy {
                quote! { *v }
            } else {
                quote! { v }
            };
            let handle = action(g, val);
            quote! {
                #ty::#item(v) => #handle,
            }
        });
        quote! {
            match #name {
                #ty::None => (),
                #(#branches)*
            }
        }
    }

    pub fn fn_write_to_uncheck(&self) -> (u64, TokenStream) {
        let token = self.check_empty(|g, val| {
            let tag = g.tag();
            let codec = &g.codec;
            quote! {{
                s.write_tag(#tag)?;
                #codec::write_to(#val, s)?;
            }}
        });
        (self.options[0].tag_value(), token)
    }

    pub fn fn_len(&self) -> (u64, TokenStream) {
        let token = self.check_empty(|g, val| {
            let (_, tag_len) = g.tag_len();
            let codec = &g.codec;
            quote!(l += #tag_len + #codec::size(#val))
        });
        (self.options[0].tag_value(), token)
    }

    pub fn accessor(&self) -> TokenStream {
        let ty = self.type_name();
        let name = self.field_name();
        let ty_copy = self.can_copy();
        let mut token = quote! {};
        for g in &self.options {
            let getter = g.one_of_getter(&name, &ty, ty_copy);
            let setter = g.one_of_setter(&name, &ty);
            let mutter = g.one_of_mutter(&name, &ty);
            token.extend(quote! {
                #getter

                #mutter

                #setter
            });
        }
        token
    }

    pub fn generate(&self) -> TokenStream {
        let ty = self.type_name();
        let mut derive = quote! {Debug, PartialEq, Clone};
        if self.can_copy() {
            derive = quote! {#derive, Copy}
        }
        let options = self.options.iter().map(|g| {
            let item = g.one_of_item();
            let ty = &g.inner_type;
            quote!(#item(#ty),)
        });
        quote! {
            #[derive(#derive)]
            pub enum #ty {
                #(#options)*
                None,
            }

            impl Default for #ty {
                #[inline]
                fn default() -> #ty {
                    #ty::None
                }
            }
        }
    }
}
