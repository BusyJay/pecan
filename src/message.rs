use crate::stream::{Buf, BufMut, CodedInputStream, CodedOutputStream};
use crate::Result;
use std::cell::UnsafeCell;
use std::fmt::{self, Debug};

pub trait Message {
    fn merge_from(&mut self, input: &mut CodedInputStream<impl Buf>) -> Result<()>;
    fn write_to(&self, output: &mut CodedOutputStream<impl BufMut>) -> Result<()>;
    fn len(&self) -> usize;

    fn merge_from_bytes(&mut self, s: &[u8]) -> Result<()> {
        let mut input = CodedInputStream::new(s);
        self.merge_from(&mut input)
    }

    fn write_to_vec(&self, s: &mut Vec<u8>) -> Result<()> {
        let mut output = CodedOutputStream::new(s);
        self.write_to(&mut output)?;
        output.flush()
    }

    fn write_as_bytes(&self) -> Result<Vec<u8>> {
        let mut v = Vec::with_capacity(self.len());
        self.write_to_vec(&mut v)?;
        Ok(v)
    }
}

pub trait MergeFrom {
    fn merge_from(&mut self, other: Self);
}

pub struct CacheSize {
    size: UnsafeCell<u32>,
}

impl Default for CacheSize {
    fn default() -> Self {
        CacheSize::new(0)
    }
}

impl PartialEq for CacheSize {
    fn eq(&self, _: &CacheSize) -> bool {
        true
    }
}

impl CacheSize {
    pub const fn new(size: u32) -> CacheSize {
        CacheSize {
            size: UnsafeCell::new(size),
        }
    }

    pub unsafe fn set_size(&self, size: u32) {
        *self.size.get() = size;
    }
}

impl Debug for CacheSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe { *self.size.get() })
    }
}

impl Clone for CacheSize {
    fn clone(&self) -> Self {
        CacheSize {
            size: UnsafeCell::new(unsafe { *self.size.get() }),
        }
    }
}
