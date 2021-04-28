#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
    _unknown: Vec<u8>,
}
impl Timestamp {
    pub const fn new() -> Timestamp {
        Timestamp {
            seconds: 0,
            nanos: 0,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for Timestamp {
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
impl pecan::DefaultInstance for Timestamp {
    fn default_instance() -> &'static Timestamp {
        static DEFAULT: Timestamp = Timestamp::new();
        &DEFAULT
    }
}
impl Default for Timestamp {
    #[inline]
    fn default() -> Timestamp {
        Timestamp::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Fgoogle/protobuf/timestamp.proto\x12\x0Fgoogle.protobuf\";\n\tTimestamp\x12\x18\n\x07seconds\x18\x01 \x01(\x03R\x07seconds\x12\x14\n\x05nanos\x18\x02 \x01(\x05R\x05nanosB\x85\x01\n\x13com.google.protobufB\x0ETimestampProtoP\x01Z2google.golang.org/protobuf/types/known/timestamppb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xC5/\n\x07\x12\x05\x1E\0\x92\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0\x1F\n\t\n\x02\x08\x1F\x12\x03#\0\x1F\n\x08\n\x01\x08\x12\x03$\0I\n\t\n\x02\x08\x0B\x12\x03$\0I\n\x08\n\x01\x08\x12\x03%\0,\n\t\n\x02\x08\x01\x12\x03%\0,\n\x08\n\x01\x08\x12\x03&\0/\n\t\n\x02\x08\x08\x12\x03&\0/\n\x08\n\x01\x08\x12\x03'\0\"\n\t\n\x02\x08\n\x12\x03'\0\"\n\x08\n\x01\x08\x12\x03(\0!\n\t\n\x02\x08$\x12\x03(\0!\n\xDE\x1D\n\x02\x04\0\x12\x06\x87\x01\0\x92\x01\x01\x1A\xCF\x1D A Timestamp represents a point in time independent of any time zone or local\n calendar, encoded as a count of seconds and fractions of seconds at\n nanosecond resolution. The count is relative to an epoch at UTC midnight on\n January 1, 1970, in the proleptic Gregorian calendar which extends the\n Gregorian calendar backwards to year one.\n\n All minutes are 60 seconds long. Leap seconds are \"smeared\" so that no leap\n second table is needed for interpretation, using a [24-hour linear\n smear](https://developers.google.com/time/smear).\n\n The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By\n restricting to that range, we ensure that we can convert to and from [RFC\n 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings.\n\n # Examples\n\n Example 1: Compute Timestamp from POSIX `time()`.\n\n     Timestamp timestamp;\n     timestamp.set_seconds(time(NULL));\n     timestamp.set_nanos(0);\n\n Example 2: Compute Timestamp from POSIX `gettimeofday()`.\n\n     struct timeval tv;\n     gettimeofday(&tv, NULL);\n\n     Timestamp timestamp;\n     timestamp.set_seconds(tv.tv_sec);\n     timestamp.set_nanos(tv.tv_usec * 1000);\n\n Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.\n\n     FILETIME ft;\n     GetSystemTimeAsFileTime(&ft);\n     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;\n\n     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z\n     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.\n     Timestamp timestamp;\n     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));\n     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));\n\n Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.\n\n     long millis = System.currentTimeMillis();\n\n     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)\n         .setNanos((int) ((millis % 1000) * 1000000)).build();\n\n\n Example 5: Compute Timestamp from Java `Instant.now()`.\n\n     Instant now = Instant.now();\n\n     Timestamp timestamp =\n         Timestamp.newBuilder().setSeconds(now.getEpochSecond())\n             .setNanos(now.getNano()).build();\n\n\n Example 6: Compute Timestamp from current time in Python.\n\n     timestamp = Timestamp()\n     timestamp.GetCurrentTime()\n\n # JSON Mapping\n\n In JSON format, the Timestamp type is encoded as a string in the\n [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the\n format is \"{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z\"\n where {year} is always expressed using four digits while {month}, {day},\n {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional\n seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),\n are optional. The \"Z\" suffix indicates the timezone (\"UTC\"); the timezone\n is required. A proto3 JSON serializer should always use UTC (as indicated by\n \"Z\") when printing the Timestamp type and a proto3 JSON parser should be\n able to accept both UTC and other timezones (as indicated by an offset).\n\n For example, \"2017-01-15T01:30:15.01Z\" encodes 15.01 seconds past\n 01:30 UTC on January 15, 2017.\n\n In JavaScript, one can convert a Date object to this format using the\n standard\n [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)\n method. In Python, a standard `datetime.datetime` object can be converted\n to this format using\n [`strftime`](https://docs.python.org/2/library/time.html#time.strftime) with\n the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use\n the Joda Time's [`ISODateTimeFormat.dateTime()`](\n http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D\n ) to obtain a formatter capable of generating timestamps in this format.\n\n\n\n\x0B\n\x03\x04\0\x01\x12\x04\x87\x01\x08\x11\n\x9D\x01\n\x04\x04\0\x02\0\x12\x04\x8B\x01\x02\x14\x1A\x8E\x01 Represents seconds of UTC time since Unix epoch\n 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to\n 9999-12-31T23:59:59Z inclusive.\n\n\r\n\x05\x04\0\x02\0\x05\x12\x04\x8B\x01\x02\x07\n\r\n\x05\x04\0\x02\0\x01\x12\x04\x8B\x01\x08\x0F\n\r\n\x05\x04\0\x02\0\x03\x12\x04\x8B\x01\x12\x13\n\xE5\x01\n\x04\x04\0\x02\x01\x12\x04\x91\x01\x02\x12\x1A\xD6\x01 Non-negative fractions of a second at nanosecond resolution. Negative\n second values with fractions must still have non-negative nanos values\n that count forward in time. Must be from 0 to 999,999,999\n inclusive.\n\n\r\n\x05\x04\0\x02\x01\x05\x12\x04\x91\x01\x02\x07\n\r\n\x05\x04\0\x02\x01\x01\x12\x04\x91\x01\x08\r\n\r\n\x05\x04\0\x02\x01\x03\x12\x04\x91\x01\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
