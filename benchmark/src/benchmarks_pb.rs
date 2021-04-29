#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct BenchmarkDataset {
    pub name: String,
    pub message_name: String,
    pub payload: Vec<pecan::Bytes>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl BenchmarkDataset {
    pub const fn new() -> BenchmarkDataset {
        BenchmarkDataset {
            name: String::new(),
            message_name: String::new(),
            payload: Vec::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for BenchmarkDataset {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = LengthPrefixed::read_from(s)?,
                18 => self.message_name = LengthPrefixed::read_from(s)?,
                26 => RefArray::<LengthPrefixed>::merge_from(&mut self.payload, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.name.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name, s)?;
        }
        if !self.message_name.is_empty() {
            s.write_tag(18)?;
            LengthPrefixed::write_to(&self.message_name, s)?;
        }
        if !self.payload.is_empty() {
            for i in &self.payload {
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
        if !self.message_name.is_empty() {
            l += 1 + LengthPrefixed::size(&self.message_name);
        }
        if !self.payload.is_empty() {
            l += self.payload.len() as u64 + RefArray::<LengthPrefixed>::size(&self.payload);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for BenchmarkDataset {
    fn default_instance() -> &'static BenchmarkDataset {
        static DEFAULT: BenchmarkDataset = BenchmarkDataset::new();
        &DEFAULT
    }
}
impl Default for BenchmarkDataset {
    #[inline]
    fn default() -> BenchmarkDataset {
        BenchmarkDataset::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x10benchmarks.proto\x12\nbenchmarks\"c\n\x10BenchmarkDataset\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12!\n\x0Cmessage_name\x18\x02 \x01(\tR\x0BmessageName\x12\x18\n\x07payload\x18\x03 \x03(\x0CR\x07payloadB \n\x1Ecom.google.protobuf.benchmarksJ\xA0\x16\n\x06\x12\x04\x1E\0>\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x1F\0\x13\n\x08\n\x01\x08\x12\x03 \07\n\t\n\x02\x08\x01\x12\x03 \07\n\n\n\x02\x04\0\x12\x04\"\0>\x01\n\n\n\x03\x04\0\x01\x12\x03\"\x08\x18\n\x8C\x01\n\x04\x04\0\x02\0\x12\x03%\x02\x12\x1A\x7F Name of the benchmark dataset.  This should be unique across all datasets.\n Should only contain word characters: [a-zA-Z0-9_]\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03%\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03%\t\r\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03%\x10\x11\n\xD9\x02\n\x04\x04\0\x02\x01\x12\x03.\x02\x1A\x1A\xCB\x02 Fully-qualified name of the protobuf message for this dataset.\n It will be one of the messages defined benchmark_messages_proto2.proto\n or benchmark_messages_proto3.proto.\n\n Implementations that do not support reflection can implement this with\n an explicit \"if/else\" chain that lists every known message defined\n in those files.\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03.\x02\x08\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03.\t\x15\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03.\x18\x19\n\x98\x04\n\x04\x04\0\x02\x02\x12\x03=\x02\x1D\x1A\x8A\x04 The payload(s) for this dataset.  They should be parsed or serialized\n in sequence, in a loop, ie.\n\n  while (!benchmarkDone) {  // Benchmark runner decides when to exit.\n    for (i = 0; i < benchmark.payload.length; i++) {\n      parse(benchmark.payload[i])\n    }\n  }\n\n This is intended to let datasets include a variety of data to provide\n potentially more realistic results than just parsing the same message\n over and over.  A single message parsed repeatedly could yield unusually\n good branch prediction performance.\n\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03=\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03=\x0B\x10\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03=\x11\x18\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03=\x1B\x1Cb\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
