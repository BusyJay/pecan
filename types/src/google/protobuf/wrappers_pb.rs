#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct DoubleValue {
    pub value: f64,
    _unknown: Vec<u8>,
}
impl DoubleValue {
    pub const fn new() -> DoubleValue {
        DoubleValue {
            value: 0f64,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for DoubleValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                9 => self.value = Fixed64::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(9)?;
        Fixed64::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Fixed64::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for DoubleValue {
    fn default_instance() -> &'static DoubleValue {
        static DEFAULT: DoubleValue = DoubleValue::new();
        &DEFAULT
    }
}
impl Default for DoubleValue {
    #[inline]
    fn default() -> DoubleValue {
        DoubleValue::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FloatValue {
    pub value: f32,
    _unknown: Vec<u8>,
}
impl FloatValue {
    pub const fn new() -> FloatValue {
        FloatValue {
            value: 0f32,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for FloatValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                13 => self.value = Fixed32::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(13)?;
        Fixed32::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Fixed32::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FloatValue {
    fn default_instance() -> &'static FloatValue {
        static DEFAULT: FloatValue = FloatValue::new();
        &DEFAULT
    }
}
impl Default for FloatValue {
    #[inline]
    fn default() -> FloatValue {
        FloatValue::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Int64Value {
    pub value: i64,
    _unknown: Vec<u8>,
}
impl Int64Value {
    pub const fn new() -> Int64Value {
        Int64Value {
            value: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Int64Value {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.value = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Int64Value {
    fn default_instance() -> &'static Int64Value {
        static DEFAULT: Int64Value = Int64Value::new();
        &DEFAULT
    }
}
impl Default for Int64Value {
    #[inline]
    fn default() -> Int64Value {
        Int64Value::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UInt64Value {
    pub value: u64,
    _unknown: Vec<u8>,
}
impl UInt64Value {
    pub const fn new() -> UInt64Value {
        UInt64Value {
            value: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for UInt64Value {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.value = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for UInt64Value {
    fn default_instance() -> &'static UInt64Value {
        static DEFAULT: UInt64Value = UInt64Value::new();
        &DEFAULT
    }
}
impl Default for UInt64Value {
    #[inline]
    fn default() -> UInt64Value {
        UInt64Value::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Int32Value {
    pub value: i32,
    _unknown: Vec<u8>,
}
impl Int32Value {
    pub const fn new() -> Int32Value {
        Int32Value {
            value: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Int32Value {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.value = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Int32Value {
    fn default_instance() -> &'static Int32Value {
        static DEFAULT: Int32Value = Int32Value::new();
        &DEFAULT
    }
}
impl Default for Int32Value {
    #[inline]
    fn default() -> Int32Value {
        Int32Value::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UInt32Value {
    pub value: u32,
    _unknown: Vec<u8>,
}
impl UInt32Value {
    pub const fn new() -> UInt32Value {
        UInt32Value {
            value: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for UInt32Value {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.value = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.value, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::size(self.value);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for UInt32Value {
    fn default_instance() -> &'static UInt32Value {
        static DEFAULT: UInt32Value = UInt32Value::new();
        &DEFAULT
    }
}
impl Default for UInt32Value {
    #[inline]
    fn default() -> UInt32Value {
        UInt32Value::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BoolValue {
    pub value: bool,
    _unknown: Vec<u8>,
}
impl BoolValue {
    pub const fn new() -> BoolValue {
        BoolValue {
            value: false,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for BoolValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.value = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if self.value {
            s.write_tag(8)?;
            Varint::write_to(self.value, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.value {
            l += 1 + Varint::size(self.value);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for BoolValue {
    fn default_instance() -> &'static BoolValue {
        static DEFAULT: BoolValue = BoolValue::new();
        &DEFAULT
    }
}
impl Default for BoolValue {
    #[inline]
    fn default() -> BoolValue {
        BoolValue::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StringValue {
    pub value: String,
    _unknown: Vec<u8>,
}
impl StringValue {
    pub const fn new() -> StringValue {
        StringValue {
            value: String::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for StringValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.value = LengthPrefixed::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.value.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.value, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.value.is_empty() {
            l += 1 + LengthPrefixed::size(&self.value);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for StringValue {
    fn default_instance() -> &'static StringValue {
        static DEFAULT: StringValue = StringValue::new();
        &DEFAULT
    }
}
impl Default for StringValue {
    #[inline]
    fn default() -> StringValue {
        StringValue::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BytesValue {
    pub value: pecan::Bytes,
    _unknown: Vec<u8>,
}
impl BytesValue {
    pub const fn new() -> BytesValue {
        BytesValue {
            value: pecan::Bytes::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for BytesValue {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.value = LengthPrefixed::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.value.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.value, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.value.is_empty() {
            l += 1 + LengthPrefixed::size(&self.value);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for BytesValue {
    fn default_instance() -> &'static BytesValue {
        static DEFAULT: BytesValue = BytesValue::new();
        &DEFAULT
    }
}
impl Default for BytesValue {
    #[inline]
    fn default() -> BytesValue {
        BytesValue::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Egoogle/protobuf/wrappers.proto\x12\x0Fgoogle.protobuf\"#\n\x0BDoubleValue\x12\x14\n\x05value\x18\x01 \x01(\x01R\x05value\"\"\n\nFloatValue\x12\x14\n\x05value\x18\x01 \x01(\x02R\x05value\"\"\n\nInt64Value\x12\x14\n\x05value\x18\x01 \x01(\x03R\x05value\"#\n\x0BUInt64Value\x12\x14\n\x05value\x18\x01 \x01(\x04R\x05value\"\"\n\nInt32Value\x12\x14\n\x05value\x18\x01 \x01(\x05R\x05value\"#\n\x0BUInt32Value\x12\x14\n\x05value\x18\x01 \x01(\rR\x05value\"!\n\tBoolValue\x12\x14\n\x05value\x18\x01 \x01(\x08R\x05value\"#\n\x0BStringValue\x12\x14\n\x05value\x18\x01 \x01(\tR\x05value\"\"\n\nBytesValue\x12\x14\n\x05value\x18\x01 \x01(\x0CR\x05valueB\x83\x01\n\x13com.google.protobufB\rWrappersProtoP\x01Z1google.golang.org/protobuf/types/known/wrapperspb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xC6\x1F\n\x06\x12\x04(\0z\x01\n\xDB\x10\n\x01\x0C\x12\x03(\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2\x8C\x04 Wrappers for primitive (non-message) types. These types are useful\n for embedding primitives in the `google.protobuf.Any` type and for places\n where we need to distinguish between the absence of a primitive\n typed field and its default value.\n\n These wrappers have no meaningful use within repeated fields as they lack\n the ability to detect presence on individual elements.\n These wrappers have no meaningful use within a map or a oneof since\n individual entries of a map or fields of a oneof can already detect presence.\n\n\x08\n\x01\x02\x12\x03*\0\x18\n\x08\n\x01\x08\x12\x03,\0;\n\t\n\x02\x08%\x12\x03,\0;\n\x08\n\x01\x08\x12\x03-\0\x1F\n\t\n\x02\x08\x1F\x12\x03-\0\x1F\n\x08\n\x01\x08\x12\x03.\0H\n\t\n\x02\x08\x0B\x12\x03.\0H\n\x08\n\x01\x08\x12\x03/\0,\n\t\n\x02\x08\x01\x12\x03/\0,\n\x08\n\x01\x08\x12\x030\0.\n\t\n\x02\x08\x08\x12\x030\0.\n\x08\n\x01\x08\x12\x031\0\"\n\t\n\x02\x08\n\x12\x031\0\"\n\x08\n\x01\x08\x12\x032\0!\n\t\n\x02\x08$\x12\x032\0!\ng\n\x02\x04\0\x12\x047\0:\x01\x1A[ Wrapper message for `double`.\n\n The JSON representation for `DoubleValue` is JSON number.\n\n\n\n\x03\x04\0\x01\x12\x037\x08\x13\n \n\x04\x04\0\x02\0\x12\x039\x02\x13\x1A\x13 The double value.\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x039\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x039\t\x0E\n\x0C\n\x05\x04\0\x02\0\x03\x12\x039\x11\x12\ne\n\x02\x04\x01\x12\x04?\0B\x01\x1AY Wrapper message for `float`.\n\n The JSON representation for `FloatValue` is JSON number.\n\n\n\n\x03\x04\x01\x01\x12\x03?\x08\x12\n\x1F\n\x04\x04\x01\x02\0\x12\x03A\x02\x12\x1A\x12 The float value.\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03A\x02\x07\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03A\x08\r\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03A\x10\x11\ne\n\x02\x04\x02\x12\x04G\0J\x01\x1AY Wrapper message for `int64`.\n\n The JSON representation for `Int64Value` is JSON string.\n\n\n\n\x03\x04\x02\x01\x12\x03G\x08\x12\n\x1F\n\x04\x04\x02\x02\0\x12\x03I\x02\x12\x1A\x12 The int64 value.\n\n\x0C\n\x05\x04\x02\x02\0\x05\x12\x03I\x02\x07\n\x0C\n\x05\x04\x02\x02\0\x01\x12\x03I\x08\r\n\x0C\n\x05\x04\x02\x02\0\x03\x12\x03I\x10\x11\ng\n\x02\x04\x03\x12\x04O\0R\x01\x1A[ Wrapper message for `uint64`.\n\n The JSON representation for `UInt64Value` is JSON string.\n\n\n\n\x03\x04\x03\x01\x12\x03O\x08\x13\n \n\x04\x04\x03\x02\0\x12\x03Q\x02\x13\x1A\x13 The uint64 value.\n\n\x0C\n\x05\x04\x03\x02\0\x05\x12\x03Q\x02\x08\n\x0C\n\x05\x04\x03\x02\0\x01\x12\x03Q\t\x0E\n\x0C\n\x05\x04\x03\x02\0\x03\x12\x03Q\x11\x12\ne\n\x02\x04\x04\x12\x04W\0Z\x01\x1AY Wrapper message for `int32`.\n\n The JSON representation for `Int32Value` is JSON number.\n\n\n\n\x03\x04\x04\x01\x12\x03W\x08\x12\n\x1F\n\x04\x04\x04\x02\0\x12\x03Y\x02\x12\x1A\x12 The int32 value.\n\n\x0C\n\x05\x04\x04\x02\0\x05\x12\x03Y\x02\x07\n\x0C\n\x05\x04\x04\x02\0\x01\x12\x03Y\x08\r\n\x0C\n\x05\x04\x04\x02\0\x03\x12\x03Y\x10\x11\ng\n\x02\x04\x05\x12\x04_\0b\x01\x1A[ Wrapper message for `uint32`.\n\n The JSON representation for `UInt32Value` is JSON number.\n\n\n\n\x03\x04\x05\x01\x12\x03_\x08\x13\n \n\x04\x04\x05\x02\0\x12\x03a\x02\x13\x1A\x13 The uint32 value.\n\n\x0C\n\x05\x04\x05\x02\0\x05\x12\x03a\x02\x08\n\x0C\n\x05\x04\x05\x02\0\x01\x12\x03a\t\x0E\n\x0C\n\x05\x04\x05\x02\0\x03\x12\x03a\x11\x12\no\n\x02\x04\x06\x12\x04g\0j\x01\x1Ac Wrapper message for `bool`.\n\n The JSON representation for `BoolValue` is JSON `true` and `false`.\n\n\n\n\x03\x04\x06\x01\x12\x03g\x08\x11\n\x1E\n\x04\x04\x06\x02\0\x12\x03i\x02\x11\x1A\x11 The bool value.\n\n\x0C\n\x05\x04\x06\x02\0\x05\x12\x03i\x02\x06\n\x0C\n\x05\x04\x06\x02\0\x01\x12\x03i\x07\x0C\n\x0C\n\x05\x04\x06\x02\0\x03\x12\x03i\x0F\x10\ng\n\x02\x04\x07\x12\x04o\0r\x01\x1A[ Wrapper message for `string`.\n\n The JSON representation for `StringValue` is JSON string.\n\n\n\n\x03\x04\x07\x01\x12\x03o\x08\x13\n \n\x04\x04\x07\x02\0\x12\x03q\x02\x13\x1A\x13 The string value.\n\n\x0C\n\x05\x04\x07\x02\0\x05\x12\x03q\x02\x08\n\x0C\n\x05\x04\x07\x02\0\x01\x12\x03q\t\x0E\n\x0C\n\x05\x04\x07\x02\0\x03\x12\x03q\x11\x12\ne\n\x02\x04\x08\x12\x04w\0z\x01\x1AY Wrapper message for `bytes`.\n\n The JSON representation for `BytesValue` is JSON string.\n\n\n\n\x03\x04\x08\x01\x12\x03w\x08\x12\n\x1F\n\x04\x04\x08\x02\0\x12\x03y\x02\x12\x1A\x12 The bytes value.\n\n\x0C\n\x05\x04\x08\x02\0\x05\x12\x03y\x02\x07\n\x0C\n\x05\x04\x08\x02\0\x01\x12\x03y\x08\r\n\x0C\n\x05\x04\x08\x02\0\x03\x12\x03y\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
