use crate::{CodedInputStream, CodedOutputStream, Result};
use bytes::{Buf, BufMut, Bytes};

pub trait Message {
    fn merge_from<B: Buf>(&mut self, s: &mut CodedInputStream<B>) -> Result<()>;
    fn write_to<B: BufMut>(&self, s: &mut CodedOutputStream<B>) -> Result<()>;
    fn size(&self) -> u64;
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
