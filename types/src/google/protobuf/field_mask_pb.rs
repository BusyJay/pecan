#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct FieldMask {
    pub paths: Vec<String>,
    _unknown: Vec<u8>,
}
impl FieldMask {
    pub const fn new() -> FieldMask {
        FieldMask {
            paths: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for FieldMask {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => RefArray::<LengthPrefixed>::merge_from(&mut self.paths, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.paths.is_empty() {
            for i in &self.paths {
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
        if !self.paths.is_empty() {
            l += self.paths.len() as u64 + RefArray::<LengthPrefixed>::size(&self.paths);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FieldMask {
    fn default_instance() -> &'static FieldMask {
        static DEFAULT: FieldMask = FieldMask::new();
        &DEFAULT
    }
}
impl Default for FieldMask {
    #[inline]
    fn default() -> FieldMask {
        FieldMask::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n google/protobuf/field_mask.proto\x12\x0Fgoogle.protobuf\"!\n\tFieldMask\x12\x14\n\x05paths\x18\x01 \x03(\tR\x05pathsB\x85\x01\n\x13com.google.protobufB\x0EFieldMaskProtoP\x01Z2google.golang.org/protobuf/types/known/fieldmaskpb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1EGoogle.Protobuf.WellKnownTypesJ\xA1;\n\x07\x12\x05\x1E\0\xF4\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0,\n\t\n\x02\x08\x01\x12\x03#\0,\n\x08\n\x01\x08\x12\x03$\0/\n\t\n\x02\x08\x08\x12\x03$\0/\n\x08\n\x01\x08\x12\x03%\0\"\n\t\n\x02\x08\n\x12\x03%\0\"\n\x08\n\x01\x08\x12\x03&\0!\n\t\n\x02\x08$\x12\x03&\0!\n\x08\n\x01\x08\x12\x03'\0I\n\t\n\x02\x08\x0B\x12\x03'\0I\n\x08\n\x01\x08\x12\x03(\0\x1F\n\t\n\x02\x08\x1F\x12\x03(\0\x1F\n\xB2,\n\x02\x04\0\x12\x06\xF1\x01\0\xF4\x01\x01\x1A\xA3, `FieldMask` represents a set of symbolic field paths, for example:\n\n     paths: \"f.a\"\n     paths: \"f.b.d\"\n\n Here `f` represents a field in some root message, `a` and `b`\n fields in the message found in `f`, and `d` a field found in the\n message in `f.b`.\n\n Field masks are used to specify a subset of fields that should be\n returned by a get operation or modified by an update operation.\n Field masks also have a custom JSON encoding (see below).\n\n # Field Masks in Projections\n\n When used in the context of a projection, a response message or\n sub-message is filtered by the API to only contain those fields as\n specified in the mask. For example, if the mask in the previous\n example is applied to a response message as follows:\n\n     f {\n       a : 22\n       b {\n         d : 1\n         x : 2\n       }\n       y : 13\n     }\n     z: 8\n\n The result will not contain specific values for fields x,y and z\n (their value will be set to the default, and omitted in proto text\n output):\n\n\n     f {\n       a : 22\n       b {\n         d : 1\n       }\n     }\n\n A repeated field is not allowed except at the last position of a\n paths string.\n\n If a FieldMask object is not present in a get operation, the\n operation applies to all fields (as if a FieldMask of all fields\n had been specified).\n\n Note that a field mask does not necessarily apply to the\n top-level response message. In case of a REST get operation, the\n field mask applies directly to the response, but in case of a REST\n list operation, the mask instead applies to each individual message\n in the returned resource list. In case of a REST custom method,\n other definitions may be used. Where the mask applies will be\n clearly documented together with its declaration in the API.  In\n any case, the effect on the returned resource/resources is required\n behavior for APIs.\n\n # Field Masks in Update Operations\n\n A field mask in update operations specifies which fields of the\n targeted resource are going to be updated. The API is required\n to only change the values of the fields as specified in the mask\n and leave the others untouched. If a resource is passed in to\n describe the updated values, the API ignores the values of all\n fields not covered by the mask.\n\n If a repeated field is specified for an update operation, new values will\n be appended to the existing repeated field in the target resource. Note that\n a repeated field is only allowed in the last position of a `paths` string.\n\n If a sub-message is specified in the last position of the field mask for an\n update operation, then new value will be merged into the existing sub-message\n in the target resource.\n\n For example, given the target message:\n\n     f {\n       b {\n         d: 1\n         x: 2\n       }\n       c: [1]\n     }\n\n And an update message:\n\n     f {\n       b {\n         d: 10\n       }\n       c: [2]\n     }\n\n then if the field mask is:\n\n  paths: [\"f.b\", \"f.c\"]\n\n then the result will be:\n\n     f {\n       b {\n         d: 10\n         x: 2\n       }\n       c: [1, 2]\n     }\n\n An implementation may provide options to override this default behavior for\n repeated and message fields.\n\n In order to reset a field's value to the default, the field must\n be in the mask and set to the default value in the provided resource.\n Hence, in order to reset all fields of a resource, provide a default\n instance of the resource and set all fields in the mask, or do\n not provide a mask as described below.\n\n If a field mask is not present on update, the operation applies to\n all fields (as if a field mask of all fields has been specified).\n Note that in the presence of schema evolution, this may mean that\n fields the client does not know and has therefore not filled into\n the request will be reset to their default. If this is unwanted\n behavior, a specific service may require a client to always specify\n a field mask, producing an error if not.\n\n As with get operations, the location of the resource which\n describes the updated values in the request message depends on the\n operation kind. In any case, the effect of the field mask is\n required to be honored by the API.\n\n ## Considerations for HTTP REST\n\n The HTTP kind of an update operation which uses a field mask must\n be set to PATCH instead of PUT in order to satisfy HTTP semantics\n (PUT must only be used for full updates).\n\n # JSON Encoding of Field Masks\n\n In JSON, a field mask is encoded as a single string where paths are\n separated by a comma. Fields name in each path are converted\n to/from lower-camel naming conventions.\n\n As an example, consider the following message declarations:\n\n     message Profile {\n       User user = 1;\n       Photo photo = 2;\n     }\n     message User {\n       string display_name = 1;\n       string address = 2;\n     }\n\n In proto a field mask for `Profile` may look as such:\n\n     mask {\n       paths: \"user.display_name\"\n       paths: \"photo\"\n     }\n\n In JSON, the same mask is represented as below:\n\n     {\n       mask: \"user.displayName,photo\"\n     }\n\n # Field Masks and Oneof Fields\n\n Field masks treat fields in oneofs just as regular fields. Consider the\n following message:\n\n     message SampleMessage {\n       oneof test_oneof {\n         string name = 4;\n         SubMessage sub_message = 9;\n       }\n     }\n\n The field mask can be:\n\n     mask {\n       paths: \"name\"\n     }\n\n Or:\n\n     mask {\n       paths: \"sub_message\"\n     }\n\n Note that oneof type names (\"test_oneof\" in this case) cannot be used in\n paths.\n\n ## Field Mask Verification\n\n The implementation of any API method which has a FieldMask type field in the\n request should verify the included field paths, and return an\n `INVALID_ARGUMENT` error if any path is unmappable.\n\n\x0B\n\x03\x04\0\x01\x12\x04\xF1\x01\x08\x11\n,\n\x04\x04\0\x02\0\x12\x04\xF3\x01\x02\x1C\x1A\x1E The set of field mask paths.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\xF3\x01\x02\n\n\r\n\x05\x04\0\x02\0\x05\x12\x04\xF3\x01\x0B\x11\n\r\n\x05\x04\0\x02\0\x01\x12\x04\xF3\x01\x12\x17\n\r\n\x05\x04\0\x02\0\x03\x12\x04\xF3\x01\x1A\x1Bb\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
