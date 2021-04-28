#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Api {
    pub name: String,
    pub methods: Vec<Method>,
    pub options: Vec<crate::google::protobuf::type_pb::Option>,
    pub version: String,
    pub source_context: Option<crate::google::protobuf::source_context_pb::SourceContext>,
    pub mixins: Vec<Mixin>,
    pub syntax: crate::google::protobuf::type_pb::Syntax,
    _unknown: Vec<u8>,
}
impl Api {
    pub const fn new() -> Api {
        Api {
            name: String::new(),
            methods: Vec::new(),
            options: Vec::new(),
            version: String::new(),
            source_context: None,
            mixins: Vec::new(),
            syntax: crate::google::protobuf::type_pb::Syntax::new(),
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
impl pecan::Message for Api {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => LengthPrefixedArray::merge_from(&mut self.methods, s)?,
                26 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                34 => self.version = LengthPrefixed::read_from(s)?,
                42 => LengthPrefixed::merge_from(self.source_context_mut(), s)?,
                50 => LengthPrefixedArray::merge_from(&mut self.mixins, s)?,
                56 => self.syntax = Varint::read_from(s)?,
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
        if !self.methods.is_empty() {
            for i in &self.methods {
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
        if !self.version.is_empty() {
            s.write_tag(34)?;
            LengthPrefixed::write_to(&self.version, s)?;
        }
        if let Some(v) = &self.source_context {
            s.write_tag(42)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.mixins.is_empty() {
            for i in &self.mixins {
                s.write_tag(50)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        s.write_tag(56)?;
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
        if !self.methods.is_empty() {
            l += self.methods.len() as u64 + LengthPrefixedArray::size(&self.methods);
        }
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        if !self.version.is_empty() {
            l += 1 + LengthPrefixed::size(&self.version);
        }
        if let Some(v) = &self.source_context {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.mixins.is_empty() {
            l += self.mixins.len() as u64 + LengthPrefixedArray::size(&self.mixins);
        }
        l += 1 + Varint::size(self.syntax);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Api {
    fn default_instance() -> &'static Api {
        static DEFAULT: Api = Api::new();
        &DEFAULT
    }
}
impl Default for Api {
    #[inline]
    fn default() -> Api {
        Api::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub request_type_url: String,
    pub request_streaming: bool,
    pub response_type_url: String,
    pub response_streaming: bool,
    pub options: Vec<crate::google::protobuf::type_pb::Option>,
    pub syntax: crate::google::protobuf::type_pb::Syntax,
    _unknown: Vec<u8>,
}
impl Method {
    pub const fn new() -> Method {
        Method {
            name: String::new(),
            request_type_url: String::new(),
            request_streaming: false,
            response_type_url: String::new(),
            response_streaming: false,
            options: Vec::new(),
            syntax: crate::google::protobuf::type_pb::Syntax::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Method {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => self.request_type_url = LengthPrefixed::read_from(s)?,
                24 => self.request_streaming = Varint::read_from(s)?,
                34 => self.response_type_url = LengthPrefixed::read_from(s)?,
                40 => self.response_streaming = Varint::read_from(s)?,
                50 => LengthPrefixedArray::merge_from(&mut self.options, s)?,
                56 => self.syntax = Varint::read_from(s)?,
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
        if !self.request_type_url.is_empty() {
            s.write_tag(18)?;
            LengthPrefixed::write_to(&self.request_type_url, s)?;
        }
        if self.request_streaming {
            s.write_tag(24)?;
            Varint::write_to(self.request_streaming, s)?;
        }
        if !self.response_type_url.is_empty() {
            s.write_tag(34)?;
            LengthPrefixed::write_to(&self.response_type_url, s)?;
        }
        if self.response_streaming {
            s.write_tag(40)?;
            Varint::write_to(self.response_streaming, s)?;
        }
        if !self.options.is_empty() {
            for i in &self.options {
                s.write_tag(50)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        s.write_tag(56)?;
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
        if !self.request_type_url.is_empty() {
            l += 1 + LengthPrefixed::size(&self.request_type_url);
        }
        if self.request_streaming {
            l += 1 + Varint::size(self.request_streaming);
        }
        if !self.response_type_url.is_empty() {
            l += 1 + LengthPrefixed::size(&self.response_type_url);
        }
        if self.response_streaming {
            l += 1 + Varint::size(self.response_streaming);
        }
        if !self.options.is_empty() {
            l += self.options.len() as u64 + LengthPrefixedArray::size(&self.options);
        }
        l += 1 + Varint::size(self.syntax);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Method {
    fn default_instance() -> &'static Method {
        static DEFAULT: Method = Method::new();
        &DEFAULT
    }
}
impl Default for Method {
    #[inline]
    fn default() -> Method {
        Method::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Mixin {
    pub name: String,
    pub root: String,
    _unknown: Vec<u8>,
}
impl Mixin {
    pub const fn new() -> Mixin {
        Mixin {
            name: String::new(),
            root: String::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Mixin {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => self.root = LengthPrefixed::read_from(s)?,
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
        if !self.root.is_empty() {
            s.write_tag(18)?;
            LengthPrefixed::write_to(&self.root, s)?;
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
        if !self.root.is_empty() {
            l += 1 + LengthPrefixed::size(&self.root);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Mixin {
    fn default_instance() -> &'static Mixin {
        static DEFAULT: Mixin = Mixin::new();
        &DEFAULT
    }
}
impl Default for Mixin {
    #[inline]
    fn default() -> Mixin {
        Mixin::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x19google/protobuf/api.proto\x12\x0Fgoogle.protobuf\x1A$google/protobuf/source_context.proto\x1A\x1Agoogle/protobuf/type.proto\"\xC1\x02\n\x03Api\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x121\n\x07methods\x18\x02 \x03(\x0B2\x17.google.protobuf.MethodR\x07methods\x121\n\x07options\x18\x03 \x03(\x0B2\x17.google.protobuf.OptionR\x07options\x12\x18\n\x07version\x18\x04 \x01(\tR\x07version\x12E\n\x0Esource_context\x18\x05 \x01(\x0B2\x1E.google.protobuf.SourceContextR\rsourceContext\x12.\n\x06mixins\x18\x06 \x03(\x0B2\x16.google.protobuf.MixinR\x06mixins\x12/\n\x06syntax\x18\x07 \x01(\x0E2\x17.google.protobuf.SyntaxR\x06syntax\"\xB2\x02\n\x06Method\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12(\n\x10request_type_url\x18\x02 \x01(\tR\x0ErequestTypeUrl\x12+\n\x11request_streaming\x18\x03 \x01(\x08R\x10requestStreaming\x12*\n\x11response_type_url\x18\x04 \x01(\tR\x0FresponseTypeUrl\x12-\n\x12response_streaming\x18\x05 \x01(\x08R\x11responseStreaming\x121\n\x07options\x18\x06 \x03(\x0B2\x17.google.protobuf.OptionR\x07options\x12/\n\x06syntax\x18\x07 \x01(\x0E2\x17.google.protobuf.SyntaxR\x06syntax\"/\n\x05Mixin\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x12\n\x04root\x18\x02 \x01(\tR\x04rootBv\n\x13com.google.protobufB\x08ApiProtoP\x01Z,google.golang.org/protobuf/types/known/apipb\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\x81<\n\x07\x12\x05\x1E\0\xCF\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\t\n\x02\x03\0\x12\x03\"\0.\n\t\n\x02\x03\x01\x12\x03#\0$\n\x08\n\x01\x08\x12\x03%\0;\n\t\n\x02\x08%\x12\x03%\0;\n\x08\n\x01\x08\x12\x03&\0,\n\t\n\x02\x08\x01\x12\x03&\0,\n\x08\n\x01\x08\x12\x03'\0)\n\t\n\x02\x08\x08\x12\x03'\0)\n\x08\n\x01\x08\x12\x03(\0\"\n\t\n\x02\x08\n\x12\x03(\0\"\n\x08\n\x01\x08\x12\x03)\0!\n\t\n\x02\x08$\x12\x03)\0!\n\x08\n\x01\x08\x12\x03*\0C\n\t\n\x02\x08\x0B\x12\x03*\0C\n\xAB\x04\n\x02\x04\0\x12\x045\0`\x01\x1A\x9E\x04 Api is a light-weight descriptor for an API Interface.\n\n Interfaces are also described as \"protocol buffer services\" in some contexts,\n such as by the \"service\" keyword in a .proto file, but they are different\n from API Services, which represent a concrete implementation of an interface\n as opposed to simply a description of methods and bindings. They are also\n sometimes simply referred to as \"APIs\" in other contexts, such as the name of\n this message itself. See https://cloud.google.com/apis/design/glossary for\n detailed terminology.\n\n\n\n\x03\x04\0\x01\x12\x035\x08\x0B\n{\n\x04\x04\0\x02\0\x12\x038\x02\x12\x1An The fully qualified name of this interface, including package name\n followed by the interface's simple name.\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x038\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x038\t\r\n\x0C\n\x05\x04\0\x02\0\x03\x12\x038\x10\x11\nC\n\x04\x04\0\x02\x01\x12\x03;\x02\x1E\x1A6 The methods of this interface, in unspecified order.\n\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03;\x02\n\n\x0C\n\x05\x04\0\x02\x01\x06\x12\x03;\x0B\x11\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03;\x12\x19\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03;\x1C\x1D\n6\n\x04\x04\0\x02\x02\x12\x03>\x02\x1E\x1A) Any metadata attached to the interface.\n\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\0\x02\x02\x06\x12\x03>\x0B\x11\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03>\x12\x19\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03>\x1C\x1D\n\x82\x08\n\x04\x04\0\x02\x03\x12\x03U\x02\x15\x1A\xF4\x07 A version string for this interface. If specified, must have the form\n `major-version.minor-version`, as in `1.10`. If the minor version is\n omitted, it defaults to zero. If the entire version field is empty, the\n major version is derived from the package name, as outlined below. If the\n field is not empty, the version in the package name will be verified to be\n consistent with what is provided here.\n\n The versioning schema uses [semantic\n versioning](http://semver.org) where the major version number\n indicates a breaking change and the minor version an additive,\n non-breaking change. Both version numbers are signals to users\n what to expect from different versions, and should be carefully\n chosen based on the product plan.\n\n The major version is also reflected in the package name of the\n interface, which must end in `v<major-version>`, as in\n `google.feature.v1`. For major versions 0 and 1, the suffix can\n be omitted. Zero major versions must only be used for\n experimental, non-GA interfaces.\n\n\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03U\x02\x08\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03U\t\x10\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03U\x13\x14\n[\n\x04\x04\0\x02\x04\x12\x03Y\x02#\x1AN Source context for the protocol buffer service represented by this\n message.\n\n\x0C\n\x05\x04\0\x02\x04\x06\x12\x03Y\x02\x0F\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03Y\x10\x1E\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03Y!\"\n2\n\x04\x04\0\x02\x05\x12\x03\\\x02\x1C\x1A% Included interfaces. See [Mixin][].\n\n\x0C\n\x05\x04\0\x02\x05\x04\x12\x03\\\x02\n\n\x0C\n\x05\x04\0\x02\x05\x06\x12\x03\\\x0B\x10\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x03\\\x11\x17\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x03\\\x1A\x1B\n0\n\x04\x04\0\x02\x06\x12\x03_\x02\x14\x1A# The source syntax of the service.\n\n\x0C\n\x05\x04\0\x02\x06\x06\x12\x03_\x02\x08\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x03_\t\x0F\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x03_\x12\x13\n=\n\x02\x04\x01\x12\x04c\0x\x01\x1A1 Method represents a method of an API interface.\n\n\n\n\x03\x04\x01\x01\x12\x03c\x08\x0E\n.\n\x04\x04\x01\x02\0\x12\x03e\x02\x12\x1A! The simple name of this method.\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03e\x02\x08\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03e\t\r\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03e\x10\x11\n/\n\x04\x04\x01\x02\x01\x12\x03h\x02\x1E\x1A\" A URL of the input message type.\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03h\x02\x08\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03h\t\x19\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03h\x1C\x1D\n0\n\x04\x04\x01\x02\x02\x12\x03k\x02\x1D\x1A# If true, the request is streamed.\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03k\x02\x06\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03k\x07\x18\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03k\x1B\x1C\n2\n\x04\x04\x01\x02\x03\x12\x03n\x02\x1F\x1A% The URL of the output message type.\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03n\x02\x08\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03n\t\x1A\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03n\x1D\x1E\n1\n\x04\x04\x01\x02\x04\x12\x03q\x02\x1E\x1A$ If true, the response is streamed.\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03q\x02\x06\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03q\x07\x19\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03q\x1C\x1D\n3\n\x04\x04\x01\x02\x05\x12\x03t\x02\x1E\x1A& Any metadata attached to the method.\n\n\x0C\n\x05\x04\x01\x02\x05\x04\x12\x03t\x02\n\n\x0C\n\x05\x04\x01\x02\x05\x06\x12\x03t\x0B\x11\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03t\x12\x19\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03t\x1C\x1D\n0\n\x04\x04\x01\x02\x06\x12\x03w\x02\x14\x1A# The source syntax of this method.\n\n\x0C\n\x05\x04\x01\x02\x06\x06\x12\x03w\x02\x08\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03w\t\x0F\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03w\x12\x13\n\xCA\x13\n\x02\x04\x02\x12\x06\xC8\x01\0\xCF\x01\x01\x1A\xBB\x13 Declares an API Interface to be included in this interface. The including\n interface must redeclare all the methods from the included interface, but\n documentation and options are inherited as follows:\n\n - If after comment and whitespace stripping, the documentation\n   string of the redeclared method is empty, it will be inherited\n   from the original method.\n\n - Each annotation belonging to the service config (http,\n   visibility) which is not set in the redeclared method will be\n   inherited.\n\n - If an http annotation is inherited, the path pattern will be\n   modified as follows. Any version prefix will be replaced by the\n   version of the including interface plus the [root][] path if\n   specified.\n\n Example of a simple mixin:\n\n     package google.acl.v1;\n     service AccessControl {\n       // Get the underlying ACL object.\n       rpc GetAcl(GetAclRequest) returns (Acl) {\n         option (google.api.http).get = \"/v1/{resource=**}:getAcl\";\n       }\n     }\n\n     package google.storage.v2;\n     service Storage {\n       rpc GetAcl(GetAclRequest) returns (Acl);\n\n       // Get a data record.\n       rpc GetData(GetDataRequest) returns (Data) {\n         option (google.api.http).get = \"/v2/{resource=**}\";\n       }\n     }\n\n Example of a mixin configuration:\n\n     apis:\n     - name: google.storage.v2.Storage\n       mixins:\n       - name: google.acl.v1.AccessControl\n\n The mixin construct implies that all methods in `AccessControl` are\n also declared with same name and request/response types in\n `Storage`. A documentation generator or annotation processor will\n see the effective `Storage.GetAcl` method after inheriting\n documentation and annotations as follows:\n\n     service Storage {\n       // Get the underlying ACL object.\n       rpc GetAcl(GetAclRequest) returns (Acl) {\n         option (google.api.http).get = \"/v2/{resource=**}:getAcl\";\n       }\n       ...\n     }\n\n Note how the version in the path pattern changed from `v1` to `v2`.\n\n If the `root` field in the mixin is specified, it should be a\n relative path under which inherited HTTP paths are placed. Example:\n\n     apis:\n     - name: google.storage.v2.Storage\n       mixins:\n       - name: google.acl.v1.AccessControl\n         root: acls\n\n This implies the following inherited HTTP annotation:\n\n     service Storage {\n       // Get the underlying ACL object.\n       rpc GetAcl(GetAclRequest) returns (Acl) {\n         option (google.api.http).get = \"/v2/acls/{resource=**}:getAcl\";\n       }\n       ...\n     }\n\n\x0B\n\x03\x04\x02\x01\x12\x04\xC8\x01\x08\r\nL\n\x04\x04\x02\x02\0\x12\x04\xCA\x01\x02\x12\x1A> The fully qualified name of the interface which is included.\n\n\r\n\x05\x04\x02\x02\0\x05\x12\x04\xCA\x01\x02\x08\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xCA\x01\t\r\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xCA\x01\x10\x11\n[\n\x04\x04\x02\x02\x01\x12\x04\xCE\x01\x02\x12\x1AM If non-empty specifies a path under which inherited HTTP paths\n are rooted.\n\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\xCE\x01\x02\x08\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\xCE\x01\t\r\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\xCE\x01\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
