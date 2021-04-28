#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
    _unknown: Vec<u8>,
}
impl Duration {
    pub const fn new() -> Duration {
        Duration {
            seconds: 0,
            nanos: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Duration {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.seconds = Varint::read_from(s)?,
                16 => self.nanos = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        s.write_tag(8)?;
        Varint::write_to(self.seconds, s)?;
        s.write_tag(16)?;
        Varint::write_to(self.nanos, s)?;
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        l += 1 + Varint::len(self.seconds);
        l += 1 + Varint::len(self.nanos);
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Duration {
    fn default_instance() -> &'static Duration {
        static DEFAULT: Duration = Duration::new();
        &DEFAULT
    }
}
impl Default for Duration {
    #[inline]
    fn default() -> Duration {
        Duration::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Egoogle/protobuf/duration.proto\x12\x0Fgoogle.protobuf\":\n\x08Duration\x12\x18\n\x07seconds\x18\x01 \x01(\x03R\x07seconds\x12\x14\n\x05nanos\x18\x02 \x01(\x05R\x05nanosB\x83\x01\n\x13com.google.protobufB\rDurationProtoP\x01Z1google.golang.org/protobuf/types/known/durationpb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xDA#\n\x06\x12\x04\x1E\0s\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0\x1F\n\t\n\x02\x08\x1F\x12\x03#\0\x1F\n\x08\n\x01\x08\x12\x03$\0H\n\t\n\x02\x08\x0B\x12\x03$\0H\n\x08\n\x01\x08\x12\x03%\0,\n\t\n\x02\x08\x01\x12\x03%\0,\n\x08\n\x01\x08\x12\x03&\0.\n\t\n\x02\x08\x08\x12\x03&\0.\n\x08\n\x01\x08\x12\x03'\0\"\n\t\n\x02\x08\n\x12\x03'\0\"\n\x08\n\x01\x08\x12\x03(\0!\n\t\n\x02\x08$\x12\x03(\0!\n\x9E\x10\n\x02\x04\0\x12\x04f\0s\x01\x1A\x91\x10 A Duration represents a signed, fixed-length span of time represented\n as a count of seconds and fractions of seconds at nanosecond\n resolution. It is independent of any calendar and concepts like \"day\"\n or \"month\". It is related to Timestamp in that the difference between\n two Timestamp values is a Duration and it can be added or subtracted\n from a Timestamp. Range is approximately +-10,000 years.\n\n # Examples\n\n Example 1: Compute Duration from two Timestamps in pseudo code.\n\n     Timestamp start = ...;\n     Timestamp end = ...;\n     Duration duration = ...;\n\n     duration.seconds = end.seconds - start.seconds;\n     duration.nanos = end.nanos - start.nanos;\n\n     if (duration.seconds < 0 && duration.nanos > 0) {\n       duration.seconds += 1;\n       duration.nanos -= 1000000000;\n     } else if (duration.seconds > 0 && duration.nanos < 0) {\n       duration.seconds -= 1;\n       duration.nanos += 1000000000;\n     }\n\n Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.\n\n     Timestamp start = ...;\n     Duration duration = ...;\n     Timestamp end = ...;\n\n     end.seconds = start.seconds + duration.seconds;\n     end.nanos = start.nanos + duration.nanos;\n\n     if (end.nanos < 0) {\n       end.seconds -= 1;\n       end.nanos += 1000000000;\n     } else if (end.nanos >= 1000000000) {\n       end.seconds += 1;\n       end.nanos -= 1000000000;\n     }\n\n Example 3: Compute Duration from datetime.timedelta in Python.\n\n     td = datetime.timedelta(days=3, minutes=10)\n     duration = Duration()\n     duration.FromTimedelta(td)\n\n # JSON Mapping\n\n In JSON format, the Duration type is encoded as a string rather than an\n object, where the string ends in the suffix \"s\" (indicating seconds) and\n is preceded by the number of seconds, with nanoseconds expressed as\n fractional seconds. For example, 3 seconds with 0 nanoseconds should be\n encoded in JSON format as \"3s\", while 3 seconds and 1 nanosecond should\n be expressed in JSON format as \"3.000000001s\", and 3 seconds and 1\n microsecond should be expressed in JSON format as \"3.000001s\".\n\n\n\n\n\n\x03\x04\0\x01\x12\x03f\x08\x10\n\xDC\x01\n\x04\x04\0\x02\0\x12\x03j\x02\x14\x1A\xCE\x01 Signed seconds of the span of time. Must be from -315,576,000,000\n to +315,576,000,000 inclusive. Note: these bounds are computed from:\n 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03j\x02\x07\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03j\x08\x0F\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03j\x12\x13\n\x83\x03\n\x04\x04\0\x02\x01\x12\x03r\x02\x12\x1A\xF5\x02 Signed fractions of a second at nanosecond resolution of the span\n of time. Durations less than one second are represented with a 0\n `seconds` field and a positive or negative `nanos` field. For durations\n of one second or more, a non-zero value for the `nanos` field must be\n of the same sign as the `seconds` field. Must be from -999,999,999\n to +999,999,999 inclusive.\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03r\x02\x07\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03r\x08\r\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03r\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
