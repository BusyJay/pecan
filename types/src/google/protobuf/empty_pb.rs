#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Empty {
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Empty {
    pub const fn new() -> Empty {
        Empty {
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Empty {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for Empty {
    fn default_instance() -> &'static Empty {
        static DEFAULT: Empty = Empty::new();
        &DEFAULT
    }
}
impl Default for Empty {
    #[inline]
    fn default() -> Empty {
        Empty::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Bgoogle/protobuf/empty.proto\x12\x0Fgoogle.protobuf\"\x07\n\x05EmptyB}\n\x13com.google.protobufB\nEmptyProtoP\x01Z.google.golang.org/protobuf/types/known/emptypb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xFE\x10\n\x06\x12\x04\x1E\03\x10\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0E\n\t\n\x02\x08\x0B\x12\x03#\0E\n\x08\n\x01\x08\x12\x03$\0,\n\t\n\x02\x08\x01\x12\x03$\0,\n\x08\n\x01\x08\x12\x03%\0+\n\t\n\x02\x08\x08\x12\x03%\0+\n\x08\n\x01\x08\x12\x03&\0\"\n\t\n\x02\x08\n\x12\x03&\0\"\n\x08\n\x01\x08\x12\x03'\0!\n\t\n\x02\x08$\x12\x03'\0!\n\x08\n\x01\x08\x12\x03(\0\x1F\n\t\n\x02\x08\x1F\x12\x03(\0\x1F\n\xFB\x02\n\x02\x04\0\x12\x033\0\x10\x1A\xEF\x02 A generic empty message that you can re-use to avoid defining duplicated\n empty messages in your APIs. A typical example is to use it as the request\n or the response type of an API method. For instance:\n\n     service Foo {\n       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);\n     }\n\n The JSON representation for `Empty` is empty JSON object `{}`.\n\n\n\n\x03\x04\0\x01\x12\x033\x08\rb\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
