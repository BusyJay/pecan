#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NullValue(i32);
impl pecan::Enumerate for NullValue {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> NullValue {
        NullValue(v)
    }
}
impl NullValue {
    pub const NULL_VALUE: NullValue = NullValue(0);
    pub const fn new() -> NullValue {
        NullValue(0)
    }
}
impl std::fmt::Debug for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "NULL_VALUE"),
            v => write!(f, "NullValue({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub fields: Option<pecan::HashMap<String, Value>>,
    _unknown: Vec<u8>,
}
impl Struct {
    pub const fn new() -> Struct {
        Struct {
            fields: None,
            _unknown: Vec::new(),
        }
    }
    pub fn fields(&self) -> &pecan::HashMap<String, Value> {
        match &self.fields {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , Value > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn fields_mut(&mut self) -> &mut pecan::HashMap<String, Value> {
        self.fields.get_or_insert_with(Default::default)
    }
    pub fn set_fields(&mut self, val: pecan::HashMap<String, Value>) {
        self.fields = Some(val);
    }
}
impl pecan::Message for Struct {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = Value::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                18 => LengthPrefixed::merge_from(&mut val, s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.fields_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = &self.fields {
            for (key, val) in v {
                s.write_tag(10)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(18)?;
                LengthPrefixed::write_to(val, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.fields {
            l += v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                l += Varint::size(el) + el;
            }
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Struct {
    fn default_instance() -> &'static Struct {
        static DEFAULT: Struct = Struct::new();
        &DEFAULT
    }
}
impl Default for Struct {
    #[inline]
    fn default() -> Struct {
        Struct::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Value_Kind {
    NullValue(NullValue),
    NumberValue(f64),
    StringValue(String),
    BoolValue(bool),
    StructValue(Struct),
    ListValue(ListValue),
    None,
}
impl Default for Value_Kind {
    #[inline]
    fn default() -> Value_Kind {
        Value_Kind::None
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub kind: Value_Kind,
    _unknown: Vec<u8>,
}
impl Value {
    pub const fn new() -> Value {
        Value {
            kind: Value_Kind::None,
            _unknown: Vec::new(),
        }
    }
    pub fn null_value(&self) -> NullValue {
        match &self.kind {
            Value_Kind::NullValue(v) => *v,
            _ => NullValue::new(),
        }
    }
    pub fn null_value_mut(&mut self) -> &mut NullValue {
        if !matches!(self.kind, Value_Kind::NullValue(_)) {
            self.kind = Value_Kind::NullValue(NullValue::new());
        }
        match &mut self.kind {
            Value_Kind::NullValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_null_value(&mut self, val: NullValue) {
        self.kind = Value_Kind::NullValue(val);
    }
    pub fn number_value(&self) -> f64 {
        match &self.kind {
            Value_Kind::NumberValue(v) => *v,
            _ => 0f64,
        }
    }
    pub fn number_value_mut(&mut self) -> &mut f64 {
        if !matches!(self.kind, Value_Kind::NumberValue(_)) {
            self.kind = Value_Kind::NumberValue(0f64);
        }
        match &mut self.kind {
            Value_Kind::NumberValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_number_value(&mut self, val: f64) {
        self.kind = Value_Kind::NumberValue(val);
    }
    pub fn string_value(&self) -> &String {
        match &self.kind {
            Value_Kind::StringValue(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn string_value_mut(&mut self) -> &mut String {
        if !matches!(self.kind, Value_Kind::StringValue(_)) {
            self.kind = Value_Kind::StringValue(String::new());
        }
        match &mut self.kind {
            Value_Kind::StringValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_string_value(&mut self, val: String) {
        self.kind = Value_Kind::StringValue(val);
    }
    pub fn bool_value(&self) -> bool {
        match &self.kind {
            Value_Kind::BoolValue(v) => *v,
            _ => false,
        }
    }
    pub fn bool_value_mut(&mut self) -> &mut bool {
        if !matches!(self.kind, Value_Kind::BoolValue(_)) {
            self.kind = Value_Kind::BoolValue(false);
        }
        match &mut self.kind {
            Value_Kind::BoolValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_bool_value(&mut self, val: bool) {
        self.kind = Value_Kind::BoolValue(val);
    }
    pub fn struct_value(&self) -> &Struct {
        match &self.kind {
            Value_Kind::StructValue(v) => v,
            _ => Struct::default_instance(),
        }
    }
    pub fn struct_value_mut(&mut self) -> &mut Struct {
        if !matches!(self.kind, Value_Kind::StructValue(_)) {
            self.kind = Value_Kind::StructValue(Struct::new());
        }
        match &mut self.kind {
            Value_Kind::StructValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_struct_value(&mut self, val: Struct) {
        self.kind = Value_Kind::StructValue(val);
    }
    pub fn list_value(&self) -> &ListValue {
        match &self.kind {
            Value_Kind::ListValue(v) => v,
            _ => ListValue::default_instance(),
        }
    }
    pub fn list_value_mut(&mut self) -> &mut ListValue {
        if !matches!(self.kind, Value_Kind::ListValue(_)) {
            self.kind = Value_Kind::ListValue(ListValue::new());
        }
        match &mut self.kind {
            Value_Kind::ListValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_list_value(&mut self, val: ListValue) {
        self.kind = Value_Kind::ListValue(val);
    }
}
impl pecan::Message for Value {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.kind = Value_Kind::NullValue(Varint::read_from(s)?),
                17 => self.kind = Value_Kind::NumberValue(Fixed64::read_from(s)?),
                26 => self.kind = Value_Kind::StringValue(LengthPrefixed::read_from(s)?),
                32 => self.kind = Value_Kind::BoolValue(Varint::read_from(s)?),
                42 => LengthPrefixed::merge_from(self.struct_value_mut(), s)?,
                50 => LengthPrefixed::merge_from(self.list_value_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        match &self.kind {
            Value_Kind::None => (),
            Value_Kind::NullValue(v) => {
                s.write_tag(8)?;
                Varint::write_to(*v, s)?;
            }
            Value_Kind::NumberValue(v) => {
                s.write_tag(17)?;
                Fixed64::write_to(*v, s)?;
            }
            Value_Kind::StringValue(v) => {
                s.write_tag(26)?;
                LengthPrefixed::write_to(v, s)?;
            }
            Value_Kind::BoolValue(v) => {
                s.write_tag(32)?;
                Varint::write_to(*v, s)?;
            }
            Value_Kind::StructValue(v) => {
                s.write_tag(42)?;
                LengthPrefixed::write_to(v, s)?;
            }
            Value_Kind::ListValue(v) => {
                s.write_tag(50)?;
                LengthPrefixed::write_to(v, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        match &self.kind {
            Value_Kind::None => (),
            Value_Kind::NullValue(v) => l += 1 + Varint::size(*v),
            Value_Kind::NumberValue(v) => l += 1 + Fixed64::size(*v),
            Value_Kind::StringValue(v) => l += 1 + LengthPrefixed::size(v),
            Value_Kind::BoolValue(v) => l += 1 + Varint::size(*v),
            Value_Kind::StructValue(v) => l += 1 + LengthPrefixed::size(v),
            Value_Kind::ListValue(v) => l += 1 + LengthPrefixed::size(v),
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Value {
    fn default_instance() -> &'static Value {
        static DEFAULT: Value = Value::new();
        &DEFAULT
    }
}
impl Default for Value {
    #[inline]
    fn default() -> Value {
        Value::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ListValue {
    pub values: Vec<Value>,
    _unknown: Vec<u8>,
}
impl ListValue {
    pub const fn new() -> ListValue {
        ListValue {
            values: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for ListValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.values, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.values.is_empty() {
            for i in &self.values {
                s.write_tag(10)?;
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
        if !self.values.is_empty() {
            l += self.values.len() as u64 + LengthPrefixedArray::size(&self.values);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for ListValue {
    fn default_instance() -> &'static ListValue {
        static DEFAULT: ListValue = ListValue::new();
        &DEFAULT
    }
}
impl Default for ListValue {
    #[inline]
    fn default() -> ListValue {
        ListValue::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Cgoogle/protobuf/struct.proto\x12\x0Fgoogle.protobuf\"\x98\x01\n\x06Struct\x12;\n\x06fields\x18\x01 \x03(\x0B2#.google.protobuf.Struct.FieldsEntryR\x06fields\x1AQ\n\x0BFieldsEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12,\n\x05value\x18\x02 \x01(\x0B2\x16.google.protobuf.ValueR\x05value:\x028\x01\"\xB2\x02\n\x05Value\x12;\n\nnull_value\x18\x01 \x01(\x0E2\x1A.google.protobuf.NullValueH\0R\tnullValue\x12#\n\x0Cnumber_value\x18\x02 \x01(\x01H\0R\x0BnumberValue\x12#\n\x0Cstring_value\x18\x03 \x01(\tH\0R\x0BstringValue\x12\x1F\n\nbool_value\x18\x04 \x01(\x08H\0R\tboolValue\x12<\n\x0Cstruct_value\x18\x05 \x01(\x0B2\x17.google.protobuf.StructH\0R\x0BstructValue\x12;\n\nlist_value\x18\x06 \x01(\x0B2\x1A.google.protobuf.ListValueH\0R\tlistValueB\x06\n\x04kind\";\n\tListValue\x12.\n\x06values\x18\x01 \x03(\x0B2\x16.google.protobuf.ValueR\x06values*\x1B\n\tNullValue\x12\x0E\n\nNULL_VALUE\x10\0B\x7F\n\x13com.google.protobufB\x0BStructProtoP\x01Z/google.golang.org/protobuf/types/known/structpb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\x99\x1D\n\x06\x12\x04\x1E\0^\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0\x1F\n\t\n\x02\x08\x1F\x12\x03#\0\x1F\n\x08\n\x01\x08\x12\x03$\0F\n\t\n\x02\x08\x0B\x12\x03$\0F\n\x08\n\x01\x08\x12\x03%\0,\n\t\n\x02\x08\x01\x12\x03%\0,\n\x08\n\x01\x08\x12\x03&\0,\n\t\n\x02\x08\x08\x12\x03&\0,\n\x08\n\x01\x08\x12\x03'\0\"\n\t\n\x02\x08\n\x12\x03'\0\"\n\x08\n\x01\x08\x12\x03(\0!\n\t\n\x02\x08$\x12\x03(\0!\n\xB3\x03\n\x02\x04\0\x12\x042\05\x01\x1A\xA6\x03 `Struct` represents a structured data value, consisting of fields\n which map to dynamically typed values. In some languages, `Struct`\n might be supported by a native representation. For example, in\n scripting languages like JS a struct is represented as an\n object. The details of that representation are described together\n with the proto support for the language.\n\n The JSON representation for `Struct` is JSON object.\n\n\n\n\x03\x04\0\x01\x12\x032\x08\x0E\n9\n\x04\x04\0\x02\0\x12\x034\x02 \x1A, Unordered map of dynamically typed values.\n\n\x0C\n\x05\x04\0\x02\0\x06\x12\x034\x02\x14\n\x0C\n\x05\x04\0\x02\0\x01\x12\x034\x15\x1B\n\x0C\n\x05\x04\0\x02\0\x03\x12\x034\x1E\x1F\n\xC3\x02\n\x02\x04\x01\x12\x04=\0M\x01\x1A\xB6\x02 `Value` represents a dynamically typed value which can be either\n null, a number, a string, a boolean, a recursive struct value, or a\n list of values. A producer of value is expected to set one of that\n variants, absence of any variant indicates an error.\n\n The JSON representation for `Value` is JSON value.\n\n\n\n\x03\x04\x01\x01\x12\x03=\x08\r\n\"\n\x04\x04\x01\x08\0\x12\x04?\x02L\x03\x1A\x14 The kind of value.\n\n\x0C\n\x05\x04\x01\x08\0\x01\x12\x03?\x08\x0C\n'\n\x04\x04\x01\x02\0\x12\x03A\x04\x1D\x1A\x1A Represents a null value.\n\n\x0C\n\x05\x04\x01\x02\0\x06\x12\x03A\x04\r\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03A\x0E\x18\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03A\x1B\x1C\n)\n\x04\x04\x01\x02\x01\x12\x03C\x04\x1C\x1A\x1C Represents a double value.\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03C\x04\n\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03C\x0B\x17\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03C\x1A\x1B\n)\n\x04\x04\x01\x02\x02\x12\x03E\x04\x1C\x1A\x1C Represents a string value.\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03E\x04\n\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03E\x0B\x17\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03E\x1A\x1B\n*\n\x04\x04\x01\x02\x03\x12\x03G\x04\x18\x1A\x1D Represents a boolean value.\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03G\x04\x08\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03G\t\x13\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03G\x16\x17\n-\n\x04\x04\x01\x02\x04\x12\x03I\x04\x1C\x1A  Represents a structured value.\n\n\x0C\n\x05\x04\x01\x02\x04\x06\x12\x03I\x04\n\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03I\x0B\x17\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03I\x1A\x1B\n-\n\x04\x04\x01\x02\x05\x12\x03K\x04\x1D\x1A  Represents a repeated `Value`.\n\n\x0C\n\x05\x04\x01\x02\x05\x06\x12\x03K\x04\r\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03K\x0E\x18\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03K\x1B\x1C\n\xA9\x01\n\x02\x05\0\x12\x04S\0V\x01\x1A\x9C\x01 `NullValue` is a singleton enumeration to represent the null value for the\n `Value` type union.\n\n  The JSON representation for `NullValue` is JSON `null`.\n\n\n\n\x03\x05\0\x01\x12\x03S\x05\x0E\n\x1A\n\x04\x05\0\x02\0\x12\x03U\x02\x11\x1A\r Null value.\n\n\x0C\n\x05\x05\0\x02\0\x01\x12\x03U\x02\x0C\n\x0C\n\x05\x05\0\x02\0\x02\x12\x03U\x0F\x10\n\x82\x01\n\x02\x04\x02\x12\x04[\0^\x01\x1Av `ListValue` is a wrapper around a repeated field of values.\n\n The JSON representation for `ListValue` is JSON array.\n\n\n\n\x03\x04\x02\x01\x12\x03[\x08\x11\n:\n\x04\x04\x02\x02\0\x12\x03]\x02\x1C\x1A- Repeated field of dynamically typed values.\n\n\x0C\n\x05\x04\x02\x02\0\x04\x12\x03]\x02\n\n\x0C\n\x05\x04\x02\x02\0\x06\x12\x03]\x0B\x10\n\x0C\n\x05\x04\x02\x02\0\x01\x12\x03]\x11\x17\n\x0C\n\x05\x04\x02\x02\0\x03\x12\x03]\x1A\x1Bb\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
