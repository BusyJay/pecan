use crate::{CodedInputStream, CodedOutputStream, Error, Result};
use bytes::{Buf, BufMut, Bytes};
use std::sync::atomic::{AtomicU32, Ordering};
use std::{
    fmt::{self, Debug},
    i32,
};

pub trait Message {
    fn merge_from<B: Buf>(&mut self, s: &mut CodedInputStream<B>) -> Result<()>;
    fn write_to_uncheck<B: BufMut>(&self, s: &mut CodedOutputStream<B>) -> Result<()>;
    fn size(&self) -> u64;
    fn cached_size(&self) -> u32;
    fn clear(&mut self);

    fn write_to<B: BufMut>(&self, s: &mut CodedOutputStream<B>) -> Result<()> {
        let l = self.size();
        if l <= i32::MAX as u64 {
            self.write_to_uncheck(s)
        } else {
            Err(Error::SizeExcceedLimit(l))
        }
    }
}

pub trait BufUnMarshal<Input> {
    fn merge_from_buf(&mut self, s: &mut Input) -> Result<()>;
}

pub trait BufMarshal<Output> {
    fn write_to_buf(&self, s: &mut Output) -> Result<()>;
    fn size_buf(&self) -> u64;
}

pub trait BufMessage<Input, Output>: BufUnMarshal<Input> + BufMarshal<Output> {}

impl<M, Input> BufUnMarshal<Input> for M
where
    M: Message,
    Input: Buf,
{
    #[inline]
    fn merge_from_buf(&mut self, s: &mut Input) -> Result<()> {
        let mut input = CodedInputStream::new(s);
        self.merge_from(&mut input)
    }
}

impl<M, Output> BufMarshal<Output> for M
where
    M: Message,
    Output: BufMut,
{
    #[inline]
    fn write_to_buf(&self, s: &mut Output) -> Result<()> {
        let mut output = CodedOutputStream::new(s);
        self.write_to(&mut output)
    }
    #[inline]
    fn size_buf(&self) -> u64 {
        self.size()
    }
}

impl<M, Input, Output> BufMessage<Input, Output> for M
where
    M: Message,
    Input: Buf,
    Output: BufMut,
{
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

pub struct CachedSize(AtomicU32);

impl CachedSize {
    #[inline]
    pub const fn new() -> CachedSize {
        CachedSize(AtomicU32::new(0))
    }

    #[inline]
    pub fn set(&self, size: u64) {
        // Relaxed is safe. Because CachedSize is only expected to return
        // correct result during serialization, and the whole message will
        // be locked by ref, so the data size has to be the same no matter
        // how many times it's calculated. If the message is being updated
        // concurrently, it has to be protected by some lock, which also
        // guarentees safety.
        self.0.store(size as u32, Ordering::Relaxed);
    }

    #[inline]
    pub fn get(&self) -> u32 {
        self.0.load(Ordering::Relaxed)
    }
}

impl PartialEq for CachedSize {
    #[inline]
    fn eq(&self, rhs: &CachedSize) -> bool {
        self.get() == rhs.get()
    }
}

impl Clone for CachedSize {
    #[inline]
    fn clone(&self) -> CachedSize {
        CachedSize(AtomicU32::new(self.get()))
    }
}

impl Debug for CachedSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
