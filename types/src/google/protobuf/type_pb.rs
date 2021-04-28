#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syntax(i32);
impl pecan::Enumerate for Syntax {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> Syntax {
        Syntax(v)
    }
}
impl Syntax {
    pub const SYNTAX_PROTO2: Syntax = Syntax(0);
    pub const SYNTAX_PROTO3: Syntax = Syntax(1);
    pub const fn new() -> Syntax {
        Syntax(0)
    }
}
impl std::fmt::Debug for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "SYNTAX_PROTO2"),
            1 => write!(f, "SYNTAX_PROTO3"),
            v => write!(f, "Syntax({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub name: String,
    pub fields: Vec<Field>,
    pub oneofs: Vec<String>,
    pub options: Vec<Option>,
    pub source_context:
        std::option::Option<crate::google::protobuf::source_context_pb::SourceContext>,
    pub syntax: Syntax,
    _unknown: Vec<u8>,
}
impl Type {
    pub const fn new() -> Type {
        Type {
            name: String::new(),
            fields: Vec::new(),
            oneofs: Vec::new(),
            options: Vec::new(),
            source_context: None,
            syntax: Syntax::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn source_context(&self) -> &crate::google::protobuf::source_context_pb::SourceContext {
        match &self.source_context {
            Some(v) => v,
            _ => crate::google::protobuf::source_context_pb::SourceContext::default_instance(),
        }
    }
    pub fn source_context_mut(
        &mut self,
    ) -> &mut crate::google::protobuf::source_context_pb::SourceContext {
        self.source_context.get_or_insert_with(Default::default)
    }
    pub fn set_source_context(
        &mut self,
        val: crate::google::protobuf::source_context_pb::SourceContext,
    ) {
        self.source_context = Some(val);
    }
}
impl pecan::Message for Type {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => LengthPrefixedArray::merge_from(&mut self.fields, s)?,
                26 => LengthPrefixedArray::merge_from(&mut self.oneofs, s)?,
                34 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                42 => LengthPrefixed::merge_from(self.source_context_mut(), s)?,
                48 => self.syntax = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        if !self.fields.is_empty() {
            for i in &self.fields {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.oneofs.is_empty() {
            for i in &self.oneofs {
                s.write_tag(26)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.options.is_empty() {
            for i in &self.options {
                s.write_tag(34)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.source_context {
            s.write_tag(42)?;
            LengthPrefixed::write_to(v, s)?;
        }
        s.write_tag(48)?;
        Varint::write_to(self.syntax, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.name);
        }
        if !self.fields.is_empty() {
            l += self.fields.len() as u64 + LengthPrefixedArray::size(&self.fields);
        }
        if !self.oneofs.is_empty() {
            l += self.oneofs.len() as u64 + LengthPrefixedArray::size(&self.oneofs);
        }
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        if let Some(v) = &self.source_context {
            l += 1 + LengthPrefixed::size(v);
        }
        l += 1 + Varint::size(self.syntax);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Type {
    fn default_instance() -> &'static Type {
        static DEFAULT: Type = Type::new();
        &DEFAULT
    }
}
impl Default for Type {
    #[inline]
    fn default() -> Type {
        Type::new()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Field_Kind(i32);
impl pecan::Enumerate for Field_Kind {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> Field_Kind {
        Field_Kind(v)
    }
}
impl Field_Kind {
    pub const TYPE_UNKNOWN: Field_Kind = Field_Kind(0);
    pub const TYPE_DOUBLE: Field_Kind = Field_Kind(1);
    pub const TYPE_FLOAT: Field_Kind = Field_Kind(2);
    pub const TYPE_INT64: Field_Kind = Field_Kind(3);
    pub const TYPE_UINT64: Field_Kind = Field_Kind(4);
    pub const TYPE_INT32: Field_Kind = Field_Kind(5);
    pub const TYPE_FIXED64: Field_Kind = Field_Kind(6);
    pub const TYPE_FIXED32: Field_Kind = Field_Kind(7);
    pub const TYPE_BOOL: Field_Kind = Field_Kind(8);
    pub const TYPE_STRING: Field_Kind = Field_Kind(9);
    pub const TYPE_GROUP: Field_Kind = Field_Kind(10);
    pub const TYPE_MESSAGE: Field_Kind = Field_Kind(11);
    pub const TYPE_BYTES: Field_Kind = Field_Kind(12);
    pub const TYPE_UINT32: Field_Kind = Field_Kind(13);
    pub const TYPE_ENUM: Field_Kind = Field_Kind(14);
    pub const TYPE_SFIXED32: Field_Kind = Field_Kind(15);
    pub const TYPE_SFIXED64: Field_Kind = Field_Kind(16);
    pub const TYPE_SINT32: Field_Kind = Field_Kind(17);
    pub const TYPE_SINT64: Field_Kind = Field_Kind(18);
    pub const fn new() -> Field_Kind {
        Field_Kind(0)
    }
}
impl std::fmt::Debug for Field_Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "TYPE_UNKNOWN"),
            1 => write!(f, "TYPE_DOUBLE"),
            2 => write!(f, "TYPE_FLOAT"),
            3 => write!(f, "TYPE_INT64"),
            4 => write!(f, "TYPE_UINT64"),
            5 => write!(f, "TYPE_INT32"),
            6 => write!(f, "TYPE_FIXED64"),
            7 => write!(f, "TYPE_FIXED32"),
            8 => write!(f, "TYPE_BOOL"),
            9 => write!(f, "TYPE_STRING"),
            10 => write!(f, "TYPE_GROUP"),
            11 => write!(f, "TYPE_MESSAGE"),
            12 => write!(f, "TYPE_BYTES"),
            13 => write!(f, "TYPE_UINT32"),
            14 => write!(f, "TYPE_ENUM"),
            15 => write!(f, "TYPE_SFIXED32"),
            16 => write!(f, "TYPE_SFIXED64"),
            17 => write!(f, "TYPE_SINT32"),
            18 => write!(f, "TYPE_SINT64"),
            v => write!(f, "Field_Kind({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Field_Cardinality(i32);
impl pecan::Enumerate for Field_Cardinality {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> Field_Cardinality {
        Field_Cardinality(v)
    }
}
impl Field_Cardinality {
    pub const CARDINALITY_UNKNOWN: Field_Cardinality = Field_Cardinality(0);
    pub const CARDINALITY_OPTIONAL: Field_Cardinality = Field_Cardinality(1);
    pub const CARDINALITY_REQUIRED: Field_Cardinality = Field_Cardinality(2);
    pub const CARDINALITY_REPEATED: Field_Cardinality = Field_Cardinality(3);
    pub const fn new() -> Field_Cardinality {
        Field_Cardinality(0)
    }
}
impl std::fmt::Debug for Field_Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "CARDINALITY_UNKNOWN"),
            1 => write!(f, "CARDINALITY_OPTIONAL"),
            2 => write!(f, "CARDINALITY_REQUIRED"),
            3 => write!(f, "CARDINALITY_REPEATED"),
            v => write!(f, "Field_Cardinality({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub kind: Field_Kind,
    pub cardinality: Field_Cardinality,
    pub number: i32,
    pub name: String,
    pub type_url: String,
    pub oneof_index: i32,
    pub packed: bool,
    pub options: Vec<Option>,
    pub json_name: String,
    pub default_value: String,
    _unknown: Vec<u8>,
}
impl Field {
    pub const fn new() -> Field {
        Field {
            kind: Field_Kind::new(),
            cardinality: Field_Cardinality::new(),
            number: 0,
            name: String::new(),
            type_url: String::new(),
            oneof_index: 0,
            packed: false,
            options: Vec::new(),
            json_name: String::new(),
            default_value: String::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Field {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.kind = Varint::read_from(s)?,
                16 => self.cardinality = Varint::read_from(s)?,
                24 => self.number = Varint::read_from(s)?,
                34 => self.name = LengthPrefixed::read_from(s)?,
                50 => self.type_url = LengthPrefixed::read_from(s)?,
                56 => self.oneof_index = Varint::read_from(s)?,
                64 => self.packed = Varint::read_from(s)?,
                74 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                82 => self.json_name = LengthPrefixed::read_from(s)?,
                90 => self.default_value = LengthPrefixed::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.kind, s)?;
        s.write_tag(16)?;
        Varint::write_to(self.cardinality, s)?;
        s.write_tag(24)?;
        Varint::write_to(self.number, s)?;
        if !self.name.is_empty() {
            s.write_tag(34)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        if !self.type_url.is_empty() {
            s.write_tag(50)?;
            LengthPrefixed::write_to(&self.type_url, s)?;
        }
        s.write_tag(56)?;
        Varint::write_to(self.oneof_index, s)?;
        if self.packed {
            s.write_tag(64)?;
            Varint::write_to(self.packed, s)?;
        }
        if !self.options.is_empty() {
            for i in &self.options {
                s.write_tag(74)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.json_name.is_empty() {
            s.write_tag(82)?;
            LengthPrefixed::write_to(&self.json_name, s)?;
        }
        if !self.default_value.is_empty() {
            s.write_tag(90)?;
            LengthPrefixed::write_to(&self.default_value, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::size(self.kind);
        l += 1 + Varint::size(self.cardinality);
        l += 1 + Varint::size(self.number);
        if !self.name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.name);
        }
        if !self.type_url.is_empty() {
            l += 1 + LengthPrefixed::size(&self.type_url);
        }
        l += 1 + Varint::size(self.oneof_index);
        if self.packed {
            l += 1 + Varint::size(self.packed);
        }
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        if !self.json_name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.json_name);
        }
        if !self.default_value.is_empty() {
            l += 1 + LengthPrefixed::size(&self.default_value);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Field {
    fn default_instance() -> &'static Field {
        static DEFAULT: Field = Field::new();
        &DEFAULT
    }
}
impl Default for Field {
    #[inline]
    fn default() -> Field {
        Field::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub enumvalue: Vec<EnumValue>,
    pub options: Vec<Option>,
    pub source_context:
        std::option::Option<crate::google::protobuf::source_context_pb::SourceContext>,
    pub syntax: Syntax,
    _unknown: Vec<u8>,
}
impl Enum {
    pub const fn new() -> Enum {
        Enum {
            name: String::new(),
            enumvalue: Vec::new(),
            options: Vec::new(),
            source_context: None,
            syntax: Syntax::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn source_context(&self) -> &crate::google::protobuf::source_context_pb::SourceContext {
        match &self.source_context {
            Some(v) => v,
            _ => crate::google::protobuf::source_context_pb::SourceContext::default_instance(),
        }
    }
    pub fn source_context_mut(
        &mut self,
    ) -> &mut crate::google::protobuf::source_context_pb::SourceContext {
        self.source_context.get_or_insert_with(Default::default)
    }
    pub fn set_source_context(
        &mut self,
        val: crate::google::protobuf::source_context_pb::SourceContext,
    ) {
        self.source_context = Some(val);
    }
}
impl pecan::Message for Enum {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => LengthPrefixedArray::merge_from(&mut self.enumvalue, s)?,
                26 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                34 => LengthPrefixed::merge_from(self.source_context_mut(), s)?,
                40 => self.syntax = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        if !self.enumvalue.is_empty() {
            for i in &self.enumvalue {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.options.is_empty() {
            for i in &self.options {
                s.write_tag(26)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.source_context {
            s.write_tag(34)?;
            LengthPrefixed::write_to(v, s)?;
        }
        s.write_tag(40)?;
        Varint::write_to(self.syntax, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.name);
        }
        if !self.enumvalue.is_empty() {
            l += self.enumvalue.len() as u64 + LengthPrefixedArray::size(&self.enumvalue);
        }
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        if let Some(v) = &self.source_context {
            l += 1 + LengthPrefixed::size(v);
        }
        l += 1 + Varint::size(self.syntax);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Enum {
    fn default_instance() -> &'static Enum {
        static DEFAULT: Enum = Enum::new();
        &DEFAULT
    }
}
impl Default for Enum {
    #[inline]
    fn default() -> Enum {
        Enum::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
    pub options: Vec<Option>,
    _unknown: Vec<u8>,
}
impl EnumValue {
    pub const fn new() -> EnumValue {
        EnumValue {
            name: String::new(),
            number: 0,
            options: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for EnumValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                16 => self.number = Varint::read_from(s)?,
                26 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        s.write_tag(16)?;
        Varint::write_to(self.number, s)?;
        if !self.options.is_empty() {
            for i in &self.options {
                s.write_tag(26)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.name);
        }
        l += 1 + Varint::size(self.number);
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumValue {
    fn default_instance() -> &'static EnumValue {
        static DEFAULT: EnumValue = EnumValue::new();
        &DEFAULT
    }
}
impl Default for EnumValue {
    #[inline]
    fn default() -> EnumValue {
        EnumValue::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Option {
    pub name: String,
    pub value: std::option::Option<crate::google::protobuf::any_pb::Any>,
    _unknown: Vec<u8>,
}
impl Option {
    pub const fn new() -> Option {
        Option {
            name: String::new(),
            value: None,
            _unknown: Vec::new(),
        }
    }
    pub fn value(&self) -> &crate::google::protobuf::any_pb::Any {
        match &self.value {
            Some(v) => v,
            _ => crate::google::protobuf::any_pb::Any::default_instance(),
        }
    }
    pub fn value_mut(&mut self) -> &mut crate::google::protobuf::any_pb::Any {
        self.value.get_or_insert_with(Default::default)
    }
    pub fn set_value(&mut self, val: crate::google::protobuf::any_pb::Any) {
        self.value = Some(val);
    }
}
impl pecan::Message for Option {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => LengthPrefixed::merge_from(self.value_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        if let Some(v) = &self.value {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.name);
        }
        if let Some(v) = &self.value {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Option {
    fn default_instance() -> &'static Option {
        static DEFAULT: Option = Option::new();
        &DEFAULT
    }
}
impl Default for Option {
    #[inline]
    fn default() -> Option {
        Option::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Agoogle/protobuf/type.proto\x12\x0Fgoogle.protobuf\x1A\x19google/protobuf/any.proto\x1A$google/protobuf/source_context.proto\"\x8D\x02\n\x04Type\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12.\n\x06fields\x18\x02 \x03(\x0B2\x16.google.protobuf.FieldR\x06fields\x12\x16\n\x06oneofs\x18\x03 \x03(\tR\x06oneofs\x121\n\x07options\x18\x04 \x03(\x0B2\x17.google.protobuf.OptionR\x07options\x12E\n\x0Esource_context\x18\x05 \x01(\x0B2\x1E.google.protobuf.SourceContextR\rsourceContext\x12/\n\x06syntax\x18\x06 \x01(\x0E2\x17.google.protobuf.SyntaxR\x06syntax\"\xB4\x06\n\x05Field\x12/\n\x04kind\x18\x01 \x01(\x0E2\x1B.google.protobuf.Field.KindR\x04kind\x12D\n\x0Bcardinality\x18\x02 \x01(\x0E2\".google.protobuf.Field.CardinalityR\x0Bcardinality\x12\x16\n\x06number\x18\x03 \x01(\x05R\x06number\x12\x12\n\x04name\x18\x04 \x01(\tR\x04name\x12\x19\n\x08type_url\x18\x06 \x01(\tR\x07typeUrl\x12\x1F\n\x0Boneof_index\x18\x07 \x01(\x05R\noneofIndex\x12\x16\n\x06packed\x18\x08 \x01(\x08R\x06packed\x121\n\x07options\x18\t \x03(\x0B2\x17.google.protobuf.OptionR\x07options\x12\x1B\n\tjson_name\x18\n \x01(\tR\x08jsonName\x12#\n\rdefault_value\x18\x0B \x01(\tR\x0CdefaultValue\"\xC8\x02\n\x04Kind\x12\x10\n\x0CTYPE_UNKNOWN\x10\0\x12\x0F\n\x0BTYPE_DOUBLE\x10\x01\x12\x0E\n\nTYPE_FLOAT\x10\x02\x12\x0E\n\nTYPE_INT64\x10\x03\x12\x0F\n\x0BTYPE_UINT64\x10\x04\x12\x0E\n\nTYPE_INT32\x10\x05\x12\x10\n\x0CTYPE_FIXED64\x10\x06\x12\x10\n\x0CTYPE_FIXED32\x10\x07\x12\r\n\tTYPE_BOOL\x10\x08\x12\x0F\n\x0BTYPE_STRING\x10\t\x12\x0E\n\nTYPE_GROUP\x10\n\x12\x10\n\x0CTYPE_MESSAGE\x10\x0B\x12\x0E\n\nTYPE_BYTES\x10\x0C\x12\x0F\n\x0BTYPE_UINT32\x10\r\x12\r\n\tTYPE_ENUM\x10\x0E\x12\x11\n\rTYPE_SFIXED32\x10\x0F\x12\x11\n\rTYPE_SFIXED64\x10\x10\x12\x0F\n\x0BTYPE_SINT32\x10\x11\x12\x0F\n\x0BTYPE_SINT64\x10\x12\"t\n\x0BCardinality\x12\x17\n\x13CARDINALITY_UNKNOWN\x10\0\x12\x18\n\x14CARDINALITY_OPTIONAL\x10\x01\x12\x18\n\x14CARDINALITY_REQUIRED\x10\x02\x12\x18\n\x14CARDINALITY_REPEATED\x10\x03\"\xFF\x01\n\x04Enum\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x128\n\tenumvalue\x18\x02 \x03(\x0B2\x1A.google.protobuf.EnumValueR\tenumvalue\x121\n\x07options\x18\x03 \x03(\x0B2\x17.google.protobuf.OptionR\x07options\x12E\n\x0Esource_context\x18\x04 \x01(\x0B2\x1E.google.protobuf.SourceContextR\rsourceContext\x12/\n\x06syntax\x18\x05 \x01(\x0E2\x17.google.protobuf.SyntaxR\x06syntax\"j\n\tEnumValue\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x16\n\x06number\x18\x02 \x01(\x05R\x06number\x121\n\x07options\x18\x03 \x03(\x0B2\x17.google.protobuf.OptionR\x07options\"H\n\x06Option\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12*\n\x05value\x18\x02 \x01(\x0B2\x14.google.protobuf.AnyR\x05value*.\n\x06Syntax\x12\x11\n\rSYNTAX_PROTO2\x10\0\x12\x11\n\rSYNTAX_PROTO3\x10\x01B{\n\x13com.google.protobufB\tTypeProtoP\x01Z-google.golang.org/protobuf/types/known/typepb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xC38\n\x07\x12\x05\x1E\0\xBA\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\t\n\x02\x03\0\x12\x03\"\0#\n\t\n\x02\x03\x01\x12\x03#\0.\n\x08\n\x01\x08\x12\x03%\0;\n\t\n\x02\x08%\x12\x03%\0;\n\x08\n\x01\x08\x12\x03&\0\x1F\n\t\n\x02\x08\x1F\x12\x03&\0\x1F\n\x08\n\x01\x08\x12\x03'\0,\n\t\n\x02\x08\x01\x12\x03'\0,\n\x08\n\x01\x08\x12\x03(\0*\n\t\n\x02\x08\x08\x12\x03(\0*\n\x08\n\x01\x08\x12\x03)\0\"\n\t\n\x02\x08\n\x12\x03)\0\"\n\x08\n\x01\x08\x12\x03*\0!\n\t\n\x02\x08$\x12\x03*\0!\n\x08\n\x01\x08\x12\x03+\0D\n\t\n\x02\x08\x0B\x12\x03+\0D\n-\n\x02\x04\0\x12\x04.\0;\x01\x1A! A protocol buffer message type.\n\n\n\n\x03\x04\0\x01\x12\x03.\x08\x0C\n0\n\x04\x04\0\x02\0\x12\x030\x02\x12\x1A# The fully qualified message name.\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x030\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x030\t\r\n\x0C\n\x05\x04\0\x02\0\x03\x12\x030\x10\x11\n\"\n\x04\x04\0\x02\x01\x12\x032\x02\x1C\x1A\x15 The list of fields.\n\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x032\x02\n\n\x0C\n\x05\x04\0\x02\x01\x06\x12\x032\x0B\x10\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x032\x11\x17\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x032\x1A\x1B\nO\n\x04\x04\0\x02\x02\x12\x034\x02\x1D\x1AB The list of types appearing in `oneof` definitions in this type.\n\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x034\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x034\x0B\x11\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x034\x12\x18\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x034\x1B\x1C\n+\n\x04\x04\0\x02\x03\x12\x036\x02\x1E\x1A\x1E The protocol buffer options.\n\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x036\x02\n\n\x0C\n\x05\x04\0\x02\x03\x06\x12\x036\x0B\x11\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x036\x12\x19\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x036\x1C\x1D\n\"\n\x04\x04\0\x02\x04\x12\x038\x02#\x1A\x15 The source context.\n\n\x0C\n\x05\x04\0\x02\x04\x06\x12\x038\x02\x0F\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x038\x10\x1E\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x038!\"\n!\n\x04\x04\0\x02\x05\x12\x03:\x02\x14\x1A\x14 The source syntax.\n\n\x0C\n\x05\x04\0\x02\x05\x06\x12\x03:\x02\x08\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x03:\t\x0F\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x03:\x12\x13\n0\n\x02\x04\x01\x12\x05>\0\x8B\x01\x01\x1A# A single field of a message type.\n\n\n\n\x03\x04\x01\x01\x12\x03>\x08\r\n\"\n\x04\x04\x01\x04\0\x12\x04@\x02g\x03\x1A\x14 Basic field types.\n\n\x0C\n\x05\x04\x01\x04\0\x01\x12\x03@\x07\x0B\n$\n\x06\x04\x01\x04\0\x02\0\x12\x03B\x04\x15\x1A\x15 Field type unknown.\n\n\x0E\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03B\x04\x10\n\x0E\n\x07\x04\x01\x04\0\x02\0\x02\x12\x03B\x13\x14\n#\n\x06\x04\x01\x04\0\x02\x01\x12\x03D\x04\x14\x1A\x14 Field type double.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x01\x01\x12\x03D\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03D\x12\x13\n\"\n\x06\x04\x01\x04\0\x02\x02\x12\x03F\x04\x13\x1A\x13 Field type float.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x02\x01\x12\x03F\x04\x0E\n\x0E\n\x07\x04\x01\x04\0\x02\x02\x02\x12\x03F\x11\x12\n\"\n\x06\x04\x01\x04\0\x02\x03\x12\x03H\x04\x13\x1A\x13 Field type int64.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x03\x01\x12\x03H\x04\x0E\n\x0E\n\x07\x04\x01\x04\0\x02\x03\x02\x12\x03H\x11\x12\n#\n\x06\x04\x01\x04\0\x02\x04\x12\x03J\x04\x14\x1A\x14 Field type uint64.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x04\x01\x12\x03J\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\x04\x02\x12\x03J\x12\x13\n\"\n\x06\x04\x01\x04\0\x02\x05\x12\x03L\x04\x13\x1A\x13 Field type int32.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x05\x01\x12\x03L\x04\x0E\n\x0E\n\x07\x04\x01\x04\0\x02\x05\x02\x12\x03L\x11\x12\n$\n\x06\x04\x01\x04\0\x02\x06\x12\x03N\x04\x15\x1A\x15 Field type fixed64.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x06\x01\x12\x03N\x04\x10\n\x0E\n\x07\x04\x01\x04\0\x02\x06\x02\x12\x03N\x13\x14\n$\n\x06\x04\x01\x04\0\x02\x07\x12\x03P\x04\x15\x1A\x15 Field type fixed32.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x07\x01\x12\x03P\x04\x10\n\x0E\n\x07\x04\x01\x04\0\x02\x07\x02\x12\x03P\x13\x14\n!\n\x06\x04\x01\x04\0\x02\x08\x12\x03R\x04\x12\x1A\x12 Field type bool.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x08\x01\x12\x03R\x04\r\n\x0E\n\x07\x04\x01\x04\0\x02\x08\x02\x12\x03R\x10\x11\n#\n\x06\x04\x01\x04\0\x02\t\x12\x03T\x04\x14\x1A\x14 Field type string.\n\n\x0E\n\x07\x04\x01\x04\0\x02\t\x01\x12\x03T\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\t\x02\x12\x03T\x12\x13\nF\n\x06\x04\x01\x04\0\x02\n\x12\x03V\x04\x14\x1A7 Field type group. Proto2 syntax only, and deprecated.\n\n\x0E\n\x07\x04\x01\x04\0\x02\n\x01\x12\x03V\x04\x0E\n\x0E\n\x07\x04\x01\x04\0\x02\n\x02\x12\x03V\x11\x13\n$\n\x06\x04\x01\x04\0\x02\x0B\x12\x03X\x04\x16\x1A\x15 Field type message.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x0B\x01\x12\x03X\x04\x10\n\x0E\n\x07\x04\x01\x04\0\x02\x0B\x02\x12\x03X\x13\x15\n\"\n\x06\x04\x01\x04\0\x02\x0C\x12\x03Z\x04\x14\x1A\x13 Field type bytes.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x0C\x01\x12\x03Z\x04\x0E\n\x0E\n\x07\x04\x01\x04\0\x02\x0C\x02\x12\x03Z\x11\x13\n#\n\x06\x04\x01\x04\0\x02\r\x12\x03\\\x04\x15\x1A\x14 Field type uint32.\n\n\x0E\n\x07\x04\x01\x04\0\x02\r\x01\x12\x03\\\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\r\x02\x12\x03\\\x12\x14\n!\n\x06\x04\x01\x04\0\x02\x0E\x12\x03^\x04\x13\x1A\x12 Field type enum.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x0E\x01\x12\x03^\x04\r\n\x0E\n\x07\x04\x01\x04\0\x02\x0E\x02\x12\x03^\x10\x12\n%\n\x06\x04\x01\x04\0\x02\x0F\x12\x03`\x04\x17\x1A\x16 Field type sfixed32.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x0F\x01\x12\x03`\x04\x11\n\x0E\n\x07\x04\x01\x04\0\x02\x0F\x02\x12\x03`\x14\x16\n%\n\x06\x04\x01\x04\0\x02\x10\x12\x03b\x04\x17\x1A\x16 Field type sfixed64.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x10\x01\x12\x03b\x04\x11\n\x0E\n\x07\x04\x01\x04\0\x02\x10\x02\x12\x03b\x14\x16\n#\n\x06\x04\x01\x04\0\x02\x11\x12\x03d\x04\x15\x1A\x14 Field type sint32.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x11\x01\x12\x03d\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\x11\x02\x12\x03d\x12\x14\n#\n\x06\x04\x01\x04\0\x02\x12\x12\x03f\x04\x15\x1A\x14 Field type sint64.\n\n\x0E\n\x07\x04\x01\x04\0\x02\x12\x01\x12\x03f\x04\x0F\n\x0E\n\x07\x04\x01\x04\0\x02\x12\x02\x12\x03f\x12\x14\nC\n\x04\x04\x01\x04\x01\x12\x04j\x02s\x03\x1A5 Whether a field is optional, required, or repeated.\n\n\x0C\n\x05\x04\x01\x04\x01\x01\x12\x03j\x07\x12\n5\n\x06\x04\x01\x04\x01\x02\0\x12\x03l\x04\x1C\x1A& For fields with unknown cardinality.\n\n\x0E\n\x07\x04\x01\x04\x01\x02\0\x01\x12\x03l\x04\x17\n\x0E\n\x07\x04\x01\x04\x01\x02\0\x02\x12\x03l\x1A\x1B\n%\n\x06\x04\x01\x04\x01\x02\x01\x12\x03n\x04\x1D\x1A\x16 For optional fields.\n\n\x0E\n\x07\x04\x01\x04\x01\x02\x01\x01\x12\x03n\x04\x18\n\x0E\n\x07\x04\x01\x04\x01\x02\x01\x02\x12\x03n\x1B\x1C\n9\n\x06\x04\x01\x04\x01\x02\x02\x12\x03p\x04\x1D\x1A* For required fields. Proto2 syntax only.\n\n\x0E\n\x07\x04\x01\x04\x01\x02\x02\x01\x12\x03p\x04\x18\n\x0E\n\x07\x04\x01\x04\x01\x02\x02\x02\x12\x03p\x1B\x1C\n%\n\x06\x04\x01\x04\x01\x02\x03\x12\x03r\x04\x1D\x1A\x16 For repeated fields.\n\n\x0E\n\x07\x04\x01\x04\x01\x02\x03\x01\x12\x03r\x04\x18\n\x0E\n\x07\x04\x01\x04\x01\x02\x03\x02\x12\x03r\x1B\x1C\n\x1E\n\x04\x04\x01\x02\0\x12\x03v\x02\x10\x1A\x11 The field type.\n\n\x0C\n\x05\x04\x01\x02\0\x06\x12\x03v\x02\x06\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03v\x07\x0B\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03v\x0E\x0F\n%\n\x04\x04\x01\x02\x01\x12\x03x\x02\x1E\x1A\x18 The field cardinality.\n\n\x0C\n\x05\x04\x01\x02\x01\x06\x12\x03x\x02\r\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03x\x0E\x19\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03x\x1C\x1D\n \n\x04\x04\x01\x02\x02\x12\x03z\x02\x13\x1A\x13 The field number.\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03z\x02\x07\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03z\x08\x0E\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03z\x11\x12\n\x1E\n\x04\x04\x01\x02\x03\x12\x03|\x02\x12\x1A\x11 The field name.\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03|\x02\x08\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03|\t\r\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03|\x10\x11\n\x96\x01\n\x04\x04\x01\x02\x04\x12\x03\x7F\x02\x16\x1A\x88\x01 The field type URL, without the scheme, for message or enumeration\n types. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`.\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03\x7F\x02\x08\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03\x7F\t\x11\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03\x7F\x14\x15\n\xA5\x01\n\x04\x04\x01\x02\x05\x12\x04\x82\x01\x02\x18\x1A\x96\x01 The index of the field type in `Type.oneofs`, for message or enumeration\n types. The first type has index 1; zero means the type is not in the list.\n\n\r\n\x05\x04\x01\x02\x05\x05\x12\x04\x82\x01\x02\x07\n\r\n\x05\x04\x01\x02\x05\x01\x12\x04\x82\x01\x08\x13\n\r\n\x05\x04\x01\x02\x05\x03\x12\x04\x82\x01\x16\x17\nF\n\x04\x04\x01\x02\x06\x12\x04\x84\x01\x02\x12\x1A8 Whether to use alternative packed wire representation.\n\n\r\n\x05\x04\x01\x02\x06\x05\x12\x04\x84\x01\x02\x06\n\r\n\x05\x04\x01\x02\x06\x01\x12\x04\x84\x01\x07\r\n\r\n\x05\x04\x01\x02\x06\x03\x12\x04\x84\x01\x10\x11\n,\n\x04\x04\x01\x02\x07\x12\x04\x86\x01\x02\x1E\x1A\x1E The protocol buffer options.\n\n\r\n\x05\x04\x01\x02\x07\x04\x12\x04\x86\x01\x02\n\n\r\n\x05\x04\x01\x02\x07\x06\x12\x04\x86\x01\x0B\x11\n\r\n\x05\x04\x01\x02\x07\x01\x12\x04\x86\x01\x12\x19\n\r\n\x05\x04\x01\x02\x07\x03\x12\x04\x86\x01\x1C\x1D\n$\n\x04\x04\x01\x02\x08\x12\x04\x88\x01\x02\x18\x1A\x16 The field JSON name.\n\n\r\n\x05\x04\x01\x02\x08\x05\x12\x04\x88\x01\x02\x08\n\r\n\x05\x04\x01\x02\x08\x01\x12\x04\x88\x01\t\x12\n\r\n\x05\x04\x01\x02\x08\x03\x12\x04\x88\x01\x15\x17\nX\n\x04\x04\x01\x02\t\x12\x04\x8A\x01\x02\x1C\x1AJ The string value of the default value of this field. Proto2 syntax only.\n\n\r\n\x05\x04\x01\x02\t\x05\x12\x04\x8A\x01\x02\x08\n\r\n\x05\x04\x01\x02\t\x01\x12\x04\x8A\x01\t\x16\n\r\n\x05\x04\x01\x02\t\x03\x12\x04\x8A\x01\x19\x1B\n%\n\x02\x04\x02\x12\x06\x8E\x01\0\x99\x01\x01\x1A\x17 Enum type definition.\n\n\x0B\n\x03\x04\x02\x01\x12\x04\x8E\x01\x08\x0C\n\x1F\n\x04\x04\x02\x02\0\x12\x04\x90\x01\x02\x12\x1A\x11 Enum type name.\n\n\r\n\x05\x04\x02\x02\0\x05\x12\x04\x90\x01\x02\x08\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\x90\x01\t\r\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x90\x01\x10\x11\n'\n\x04\x04\x02\x02\x01\x12\x04\x92\x01\x02#\x1A\x19 Enum value definitions.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\x02\x02\x01\x06\x12\x04\x92\x01\x0B\x14\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\x92\x01\x15\x1E\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x92\x01!\"\n(\n\x04\x04\x02\x02\x02\x12\x04\x94\x01\x02\x1E\x1A\x1A Protocol buffer options.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x94\x01\x02\n\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\x94\x01\x0B\x11\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\x94\x01\x12\x19\n\r\n\x05\x04\x02\x02\x02\x03\x12\x04\x94\x01\x1C\x1D\n#\n\x04\x04\x02\x02\x03\x12\x04\x96\x01\x02#\x1A\x15 The source context.\n\n\r\n\x05\x04\x02\x02\x03\x06\x12\x04\x96\x01\x02\x0F\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\x96\x01\x10\x1E\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\x96\x01!\"\n\"\n\x04\x04\x02\x02\x04\x12\x04\x98\x01\x02\x14\x1A\x14 The source syntax.\n\n\r\n\x05\x04\x02\x02\x04\x06\x12\x04\x98\x01\x02\x08\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\x98\x01\t\x0F\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\x98\x01\x12\x13\n&\n\x02\x04\x03\x12\x06\x9C\x01\0\xA3\x01\x01\x1A\x18 Enum value definition.\n\n\x0B\n\x03\x04\x03\x01\x12\x04\x9C\x01\x08\x11\n \n\x04\x04\x03\x02\0\x12\x04\x9E\x01\x02\x12\x1A\x12 Enum value name.\n\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\x9E\x01\x02\x08\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\x9E\x01\t\r\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\x9E\x01\x10\x11\n\"\n\x04\x04\x03\x02\x01\x12\x04\xA0\x01\x02\x13\x1A\x14 Enum value number.\n\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\xA0\x01\x02\x07\n\r\n\x05\x04\x03\x02\x01\x01\x12\x04\xA0\x01\x08\x0E\n\r\n\x05\x04\x03\x02\x01\x03\x12\x04\xA0\x01\x11\x12\n(\n\x04\x04\x03\x02\x02\x12\x04\xA2\x01\x02\x1E\x1A\x1A Protocol buffer options.\n\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04\xA2\x01\x02\n\n\r\n\x05\x04\x03\x02\x02\x06\x12\x04\xA2\x01\x0B\x11\n\r\n\x05\x04\x03\x02\x02\x01\x12\x04\xA2\x01\x12\x19\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\xA2\x01\x1C\x1D\ng\n\x02\x04\x04\x12\x06\xA7\x01\0\xB2\x01\x01\x1AY A protocol buffer option, which can be attached to a message, field,\n enumeration, etc.\n\n\x0B\n\x03\x04\x04\x01\x12\x04\xA7\x01\x08\x0E\n\xFC\x01\n\x04\x04\x04\x02\0\x12\x04\xAC\x01\x02\x12\x1A\xED\x01 The option's name. For protobuf built-in options (options defined in\n descriptor.proto), this is the short name. For example, `\"map_entry\"`.\n For custom options, it should be the fully-qualified name. For example,\n `\"google.api.http\"`.\n\n\r\n\x05\x04\x04\x02\0\x05\x12\x04\xAC\x01\x02\x08\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\xAC\x01\t\r\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xAC\x01\x10\x11\n\xA0\x02\n\x04\x04\x04\x02\x01\x12\x04\xB1\x01\x02\x10\x1A\x91\x02 The option's value packed in an Any message. If the value is a primitive,\n the corresponding wrapper type defined in google/protobuf/wrappers.proto\n should be used. If the value is an enum, it should be stored as an int32\n value using the google.protobuf.Int32Value type.\n\n\r\n\x05\x04\x04\x02\x01\x06\x12\x04\xB1\x01\x02\x05\n\r\n\x05\x04\x04\x02\x01\x01\x12\x04\xB1\x01\x06\x0B\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\xB1\x01\x0E\x0F\nI\n\x02\x05\0\x12\x06\xB5\x01\0\xBA\x01\x01\x1A; The syntax in which a protocol buffer element is defined.\n\n\x0B\n\x03\x05\0\x01\x12\x04\xB5\x01\x05\x0B\n \n\x04\x05\0\x02\0\x12\x04\xB7\x01\x02\x14\x1A\x12 Syntax `proto2`.\n\n\r\n\x05\x05\0\x02\0\x01\x12\x04\xB7\x01\x02\x0F\n\r\n\x05\x05\0\x02\0\x02\x12\x04\xB7\x01\x12\x13\n \n\x04\x05\0\x02\x01\x12\x04\xB9\x01\x02\x14\x1A\x12 Syntax `proto3`.\n\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\xB9\x01\x02\x0F\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xB9\x01\x12\x13b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
