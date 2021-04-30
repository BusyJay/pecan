#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct SourceContext {
    pub file_name: String,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl SourceContext {
    pub const fn new() -> SourceContext {
        SourceContext {
            file_name: String::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for SourceContext {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixed::merge_from(&mut self.file_name, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.file_name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.file_name, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.file_name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.file_name);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self.file_name.clear();
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SourceContext {
    fn default_instance() -> &'static SourceContext {
        static DEFAULT: SourceContext = SourceContext::new();
        &DEFAULT
    }
}
impl Default for SourceContext {
    #[inline]
    fn default() -> SourceContext {
        SourceContext::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n$google/protobuf/source_context.proto\x12\x0Fgoogle.protobuf\",\n\rSourceContext\x12\x1B\n\tfile_name\x18\x01 \x01(\tR\x08fileNameB\x8A\x01\n\x13com.google.protobufB\x12SourceContextProtoP\x01Z6google.golang.org/protobuf/types/known/sourcecontextpb\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xC1\x10\n\x06\x12\x04\x1E\0/\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0,\n\t\n\x02\x08\x01\x12\x03#\0,\n\x08\n\x01\x08\x12\x03$\03\n\t\n\x02\x08\x08\x12\x03$\03\n\x08\n\x01\x08\x12\x03%\0\"\n\t\n\x02\x08\n\x12\x03%\0\"\n\x08\n\x01\x08\x12\x03&\0!\n\t\n\x02\x08$\x12\x03&\0!\n\x08\n\x01\x08\x12\x03'\0M\n\t\n\x02\x08\x0B\x12\x03'\0M\n\x83\x01\n\x02\x04\0\x12\x04+\0/\x01\x1Aw `SourceContext` represents information about the source of a\n protobuf element, like the file in which it is defined.\n\n\n\n\x03\x04\0\x01\x12\x03+\x08\x15\n\xA3\x01\n\x04\x04\0\x02\0\x12\x03.\x02\x17\x1A\x95\x01 The path-qualified name of the .proto file that contained the associated\n protobuf element.  For example: `\"google/protobuf/source_context.proto\"`.\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03.\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03.\t\x12\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03.\x15\x16b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
