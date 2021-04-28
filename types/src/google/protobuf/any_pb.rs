#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Any {
    pub type_url: String,
    pub value: pecan::Bytes,
    _unknown: Vec<u8>,
}
impl Any {
    pub const fn new() -> Any {
        Any {
            type_url: String::new(),
            value: pecan::Bytes::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Any {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.type_url = LengthPrefixed::read_from(s)?,
                18 => self.value = LengthPrefixed::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.type_url.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.type_url, s)?;
        }
        if !self.value.is_empty() {
            s.write_tag(18)?;
            LengthPrefixed::write_to(&self.value, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.type_url.is_empty() {
            l += 1 + LengthPrefixed::size(&self.type_url);
        }
        if !self.value.is_empty() {
            l += 1 + LengthPrefixed::size(&self.value);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Any {
    fn default_instance() -> &'static Any {
        static DEFAULT: Any = Any::new();
        &DEFAULT
    }
}
impl Default for Any {
    #[inline]
    fn default() -> Any {
        Any::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x19google/protobuf/any.proto\x12\x0Fgoogle.protobuf\"6\n\x03Any\x12\x19\n\x08type_url\x18\x01 \x01(\tR\x07typeUrl\x12\x14\n\x05value\x18\x02 \x01(\x0CR\x05valueBv\n\x13com.google.protobufB\x08AnyProtoP\x01Z,google.golang.org/protobuf/types/known/anypb\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xF9*\n\x07\x12\x05\x1E\0\x9D\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0C\n\t\n\x02\x08\x0B\x12\x03#\0C\n\x08\n\x01\x08\x12\x03$\0,\n\t\n\x02\x08\x01\x12\x03$\0,\n\x08\n\x01\x08\x12\x03%\0)\n\t\n\x02\x08\x08\x12\x03%\0)\n\x08\n\x01\x08\x12\x03&\0\"\n\t\n\x02\x08\n\x12\x03&\0\"\n\x08\n\x01\x08\x12\x03'\0!\n\t\n\x02\x08$\x12\x03'\0!\n\xFD\x10\n\x02\x04\0\x12\x05|\0\x9D\x01\x01\x1A\xEF\x10 `Any` contains an arbitrary serialized protocol buffer message along with a\n URL that describes the type of the serialized message.\n\n Protobuf library provides support to pack/unpack Any values in the form\n of utility functions or additional generated methods of the Any type.\n\n Example 1: Pack and unpack a message in C++.\n\n     Foo foo = ...;\n     Any any;\n     any.PackFrom(foo);\n     ...\n     if (any.UnpackTo(&foo)) {\n       ...\n     }\n\n Example 2: Pack and unpack a message in Java.\n\n     Foo foo = ...;\n     Any any = Any.pack(foo);\n     ...\n     if (any.is(Foo.class)) {\n       foo = any.unpack(Foo.class);\n     }\n\n  Example 3: Pack and unpack a message in Python.\n\n     foo = Foo(...)\n     any = Any()\n     any.Pack(foo)\n     ...\n     if any.Is(Foo.DESCRIPTOR):\n       any.Unpack(foo)\n       ...\n\n  Example 4: Pack and unpack a message in Go\n\n      foo := &pb.Foo{...}\n      any, err := anypb.New(foo)\n      if err != nil {\n        ...\n      }\n      ...\n      foo := &pb.Foo{}\n      if err := any.UnmarshalTo(foo); err != nil {\n        ...\n      }\n\n The pack methods provided by protobuf library will by default use\n 'type.googleapis.com/full.type.name' as the type URL and the unpack\n methods only use the fully qualified type name after the last '/'\n in the type URL, for example \"foo.bar.com/x/y.z\" will yield type\n name \"y.z\".\n\n\n JSON\n ====\n The JSON representation of an `Any` value uses the regular\n representation of the deserialized, embedded message, with an\n additional field `@type` which contains the type URL. Example:\n\n     package google.profile;\n     message Person {\n       string first_name = 1;\n       string last_name = 2;\n     }\n\n     {\n       \"@type\": \"type.googleapis.com/google.profile.Person\",\n       \"firstName\": <string>,\n       \"lastName\": <string>\n     }\n\n If the embedded message type is well-known and has a custom JSON\n representation, that representation will be embedded adding a field\n `value` which holds the custom JSON in addition to the `@type`\n field. Example (for message [google.protobuf.Duration][]):\n\n     {\n       \"@type\": \"type.googleapis.com/google.protobuf.Duration\",\n       \"value\": \"1.212s\"\n     }\n\n\n\n\n\x03\x04\0\x01\x12\x03|\x08\x0B\n\xD7\n\n\x04\x04\0\x02\0\x12\x04\x99\x01\x02\x16\x1A\xC8\n A URL/resource name that uniquely identifies the type of the serialized\n protocol buffer message. This string must contain at least\n one \"/\" character. The last segment of the URL's path must represent\n the fully qualified name of the type (as in\n `path/google.protobuf.Duration`). The name should be in a canonical form\n (e.g., leading \".\" is not accepted).\n\n In practice, teams usually precompile into the binary all types that they\n expect it to use in the context of Any. However, for URLs which use the\n scheme `http`, `https`, or no scheme, one can optionally set up a type\n server that maps type URLs to message definitions as follows:\n\n * If no scheme is provided, `https` is assumed.\n * An HTTP GET on the URL must yield a [google.protobuf.Type][]\n   value in binary format, or produce an error.\n * Applications are allowed to cache lookup results based on the\n   URL, or have them precompiled into a binary to avoid any\n   lookup. Therefore, binary compatibility needs to be preserved\n   on changes to types. (Use versioned type names to manage\n   breaking changes.)\n\n Note: this functionality is not currently available in the official\n protobuf release, and it is not used for type URLs beginning with\n type.googleapis.com.\n\n Schemes other than `http`, `https` (or the empty scheme) might be\n used with implementation specific semantics.\n\n\n\r\n\x05\x04\0\x02\0\x05\x12\x04\x99\x01\x02\x08\n\r\n\x05\x04\0\x02\0\x01\x12\x04\x99\x01\t\x11\n\r\n\x05\x04\0\x02\0\x03\x12\x04\x99\x01\x14\x15\nW\n\x04\x04\0\x02\x01\x12\x04\x9C\x01\x02\x12\x1AI Must be a valid serialized protocol buffer of the above specified type.\n\n\r\n\x05\x04\0\x02\x01\x05\x12\x04\x9C\x01\x02\x07\n\r\n\x05\x04\0\x02\x01\x01\x12\x04\x9C\x01\x08\r\n\r\n\x05\x04\0\x02\x01\x03\x12\x04\x9C\x01\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
