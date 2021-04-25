#![allow(non_camel_case_types)]
use pecan::prelude::*;
#[derive(Clone, Default, Debug)]
pub struct PecanFieldOptions {
    pub box_field: bool,
    _unknown: Vec<u8>,
}
impl PecanFieldOptions {
    pub const fn new() -> PecanFieldOptions {
        PecanFieldOptions {
            box_field: false,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for PecanFieldOptions {
    fn merge_from<B: bytes::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.box_field = s.read_bool()?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: bytes::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if self.box_field {
            s.write_tag(8)?;
            s.write_bool(self.box_field)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if self.box_field {
            l += 1 + pecan::stream::bool_len(self.box_field);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for PecanFieldOptions {
    fn default_instance() -> &'static PecanFieldOptions {
        static DEFAULT: PecanFieldOptions = PecanFieldOptions::new();
        &DEFAULT
    }
}
