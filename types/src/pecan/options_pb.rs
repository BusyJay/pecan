#![allow(non_camel_case_types)]
use pecan::prelude::*;
pub const FIELD_OPT: pecan::Extension<FieldOptions, LengthPrefixed> = pecan::Extension::new(18666);
#[derive(Clone, Debug)]
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
    fn merge_from<B: bytes::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.box_field = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: bytes::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if self.box_field {
            s.write_tag(8)?;
            Varint::write_to(self.box_field, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if self.box_field {
            l += 1 + Varint::len(self.box_field);
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
