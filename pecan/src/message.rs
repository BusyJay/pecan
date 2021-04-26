use crate::{CodedInputStream, CodedOutputStream, Result};
use bytes::{Buf, BufMut, Bytes};

pub trait Message {
    fn merge_from<B: Buf>(&mut self, s: &mut CodedInputStream<B>) -> Result<()>;
    fn write_to<B: BufMut>(&self, s: &mut CodedOutputStream<B>) -> Result<()>;
    fn len(&self) -> u64;

    fn merge_from_buf<B: Buf>(&mut self, s: &mut B) -> Result<()> {
        let mut input = CodedInputStream::new(s);
        self.merge_from(&mut input)
    }

    fn write_to_buf<B: BufMut>(&self, bytes: &mut B) -> Result<()> {
        let mut output = CodedOutputStream::new(bytes);
        self.write_to(&mut output)
    }
}

pub trait DefaultInstance {
    fn default_instance() -> &'static Self;
}

impl DefaultInstance for Bytes {
    fn default_instance() -> &'static Self {
        static DEFAULT: Bytes = Bytes::new();
        &DEFAULT
    }
}

impl DefaultInstance for String {
    fn default_instance() -> &'static Self {
        static DEFAULT: String = String::new();
        &DEFAULT
    }
}

pub trait Enumerate {
    fn value(self) -> i32;
    fn from_value(v: i32) -> Self;
}
