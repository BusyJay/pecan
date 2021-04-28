#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use pecan::prelude::*;
pub const FIELD_OPT: pecan::Extension<FieldOptions, LengthPrefixed> = pecan::Extension::new(18666);
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOptions {
    pub box_field: bool,
    _unknown: Vec<u8>,
}
impl FieldOptions {
    pub const fn new() -> FieldOptions {
        FieldOptions {
            box_field: false,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for FieldOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.box_field = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if self.box_field {
            s.write_tag(8)?;
            Varint::write_to(self.box_field, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.box_field {
            l += 1 + Varint::size(self.box_field);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FieldOptions {
    fn default_instance() -> &'static FieldOptions {
        static DEFAULT: FieldOptions = FieldOptions::new();
        &DEFAULT
    }
}
impl Default for FieldOptions {
    #[inline]
    fn default() -> FieldOptions {
        FieldOptions::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x13pecan/options.proto\x12\x05pecan\x1A google/protobuf/descriptor.proto\"+\n\x0CFieldOptions\x12\x1B\n\tbox_field\x18\x01 \x01(\x08R\x08boxField:P\n\tfield_opt\x12\x1D.google.protobuf.FieldOptions\x18\x9D\x12 \x01(\x0B2\x13.pecan.FieldOptionsR\x08fieldOptJ\xBC\x01\n\x06\x12\x04\0\0\x0C\x01\n\x08\n\x01\x0C\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0*\n\x08\n\x01\x02\x12\x03\x04\0\x0E\n\n\n\x02\x04\0\x12\x04\x06\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x14\n\x0B\n\x04\x04\0\x02\0\x12\x03\x07\x04\x17\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03\x07\x04\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03\x07\t\x12\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03\x07\x15\x16\n\t\n\x01\x07\x12\x04\n\0\x0C\x01\n\t\n\x02\x07\0\x12\x03\x0B\x04\"\n\n\n\x03\x07\0\x02\x12\x03\n\x07#\n\n\n\x03\x07\0\x06\x12\x03\x0B\x04\x10\n\n\n\x03\x07\0\x01\x12\x03\x0B\x11\x1A\n\n\n\x03\x07\0\x03\x12\x03\x0B\x1D!b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
