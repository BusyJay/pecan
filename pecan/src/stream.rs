pub mod field;

use crate::{codec, extension::ExtensionMap, Error, Result};
use bytes::{buf::UninitSlice, Buf, BufMut, Bytes, BytesMut};
use field::{Fixed32, Fixed64, LengthPrefixed, ReadFieldCodec, Varint, WriteFieldCodec};
use std::{mem, ptr};

pub struct CodedInputStream<'a, B: Buf> {
    buf: &'a mut B,
    chunk: &'a [u8],
    last_len: usize,
    flushed: u64,
    limit: u64,
    depth: usize,
    depth_limit: usize,
}

impl<'a, B: Buf> CodedInputStream<'a, B> {
    pub fn new(buf: &'a mut B) -> CodedInputStream<'a, B> {
        let chunk: &[u8] = unsafe { mem::transmute(buf.chunk()) };
        CodedInputStream {
            last_len: chunk.len(),
            flushed: 0,
            limit: buf.remaining() as u64,
            chunk,
            buf,
            depth: 0,
            depth_limit: 20,
        }
    }

    pub fn set_depth_limit(&mut self, limit: usize) {
        self.depth_limit = limit;
    }

    fn renew(&mut self) -> Result<()> {
        self.buf.advance(self.last_len);
        self.flushed += self.last_len as u64;
        self.chunk = unsafe { mem::transmute(self.buf.chunk()) };
        self.last_len = self.chunk.len();
        if !self.chunk.is_empty() {
            Ok(())
        } else {
            Err(Error::eof())
        }
    }

    fn progress(&self) -> u64 {
        self.flushed + self.last_len as u64 - self.chunk.len() as u64
    }

    pub fn read_tag(&mut self) -> Result<u64> {
        if self.progress() < self.limit {
            let tag = self.read_var_u64_raw()?;
            if self.progress() < self.limit && tag != 0 {
                Ok(tag)
            } else {
                Err(Error::corrupted())
            }
        } else if self.progress() == self.limit {
            Ok(0)
        } else {
            Err(Error::corrupted())
        }
    }

    fn read_var_u64_raw(&mut self) -> Result<u64> {
        let (mut tmp, mut lp) = match codec::decode_var_u64(&mut self.chunk) {
            Ok(v) => return Ok(v),
            Err(Error::WantMore(tmp, lp)) => (tmp, lp),
            Err(e) => return Err(e),
        };

        loop {
            self.renew()?;
            match codec::decode_var_u64_resume(&mut self.chunk, tmp, lp) {
                Ok(v) => return Ok(v),
                Err(Error::WantMore(t, l)) => {
                    tmp = t;
                    lp = l;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn copy_to_bytes(&mut self, len: usize) -> Result<Bytes> {
        self.flush();
        if self.buf.remaining() >= len {
            self.flushed += len as u64;
            return Ok(self.buf.copy_to_bytes(len));
        } else {
            return Err(Error::Eof);
        }
    }

    fn read_raw_bytes(&mut self, mut target: &mut [u8]) -> Result<()> {
        loop {
            if self.chunk.len() >= target.len() {
                let (lp, rp) = self.chunk.split_at(target.len());
                target.copy_from_slice(lp);
                self.chunk = rp;
                return Ok(());
            }
            let (lp, rp) = target.split_at_mut(self.chunk.len());
            lp.copy_from_slice(&self.chunk);
            self.renew()?;
            target = rp;
        }
    }

    pub fn read_extension(&mut self, tag: u64, extension: &mut ExtensionMap) -> Result<()> {
        let mut data = BytesMut::new();
        if let Some(buf) = extension.get_raw(tag) {
            data.extend(buf);
        }
        self.read_unknown_field(tag, &mut data)?;
        extension.insert_raw(tag, data.freeze());
        Ok(())
    }

    pub fn read_unknown_field(&mut self, tag: u64, unknown: &mut impl BufMut) -> Result<()> {
        let mut os = CodedOutputStream::new(unknown);
        os.write_tag(tag)?;
        match tag & 0x7 {
            0 => {
                let v: u64 = Varint::read_from(self)?;
                Varint::write_to(v, &mut os)
            }
            1 => {
                let v: u64 = Fixed64::read_from(self)?;
                Fixed64::write_to(v, &mut os)
            }
            2 => {
                // TODO: no copy is better.
                let bs: Bytes = LengthPrefixed::read_from(self)?;
                LengthPrefixed::write_to(&bs, &mut os)
            }
            3 | 4 => unimplemented!("{}", tag),
            5 => {
                let v: u32 = Fixed32::read_from(self)?;
                Fixed32::write_to(v, &mut os)
            }
            _ => return Err(Error::corrupted()),
        }
    }

    fn flush(&mut self) {
        self.flushed += (self.last_len - self.chunk.len()) as u64;
        self.buf.advance(self.last_len - self.chunk.len());
        self.last_len = 0;
        self.chunk = &[];
    }

    pub fn is_empty(&self) -> bool {
        self.progress() >= self.limit || self.chunk.is_empty() && !self.buf.has_remaining()
    }

    #[inline]
    fn push_limit(&mut self, size: u64) -> Result<u64> {
        if self.depth < self.depth_limit {
            self.depth += 1;
        } else {
            return Err(Error::DepthExcceedLimit(self.depth_limit));
        }
        let last_limit = self.limit;
        self.limit = self.progress() + size;
        Ok(last_limit)
    }

    #[inline]
    fn pop_limit(&mut self, last_limit: u64) -> Result<()> {
        self.depth -= 1;
        if self.limit == self.progress() {
            self.limit = last_limit;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }

    #[inline]
    pub fn read_length_limited(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        let size = self.read_var_u64_raw()?;
        let last_limit = self.push_limit(size)?;
        f(self)?;
        self.pop_limit(last_limit)
    }
}

impl<'a, B: Buf> Drop for CodedInputStream<'a, B> {
    fn drop(&mut self) {
        self.flush()
    }
}

pub struct CodedOutputStream<'a, B: BufMut> {
    buf: &'a mut B,
    chunk: &'a mut UninitSlice,
    last_len: usize,
}

impl<'a, B: BufMut> CodedOutputStream<'a, B> {
    pub fn new(buf: &mut B) -> CodedOutputStream<B> {
        let chunk: &mut UninitSlice = unsafe { mem::transmute(buf.chunk_mut()) };
        CodedOutputStream {
            last_len: chunk.len(),
            buf,
            chunk,
        }
    }

    fn renew(&mut self) -> Result<()> {
        unsafe {
            self.buf.advance_mut(self.last_len);
        }
        self.chunk = unsafe { mem::transmute(self.buf.chunk_mut()) };
        self.last_len = self.chunk.len();
        if self.chunk.len() > 0 {
            Ok(())
        } else {
            return Err(Error::Eof);
        }
    }

    fn write_var_u64_raw(&mut self, mut val: u64) -> Result<()> {
        loop {
            match codec::encode_var_u64(&mut self.chunk, val) {
                Ok(()) => return Ok(()),
                Err(Error::WantMore(v, _)) => {
                    self.renew()?;
                    val = v;
                }
                Err(e) => return Err(e),
            }
        }
    }

    #[inline]
    pub fn write_tag(&mut self, tag: u64) -> Result<()> {
        self.write_var_u64_raw(tag)
    }

    pub fn write_raw_bytes(&mut self, mut s: &[u8]) -> Result<()> {
        loop {
            if self.chunk.len() >= s.len() {
                unsafe {
                    ptr::copy_nonoverlapping(s.as_ptr(), self.chunk.as_mut_ptr(), s.len());
                    self.chunk = UninitSlice::from_raw_parts_mut(
                        self.chunk.as_mut_ptr().add(s.len()),
                        self.chunk.len() - s.len(),
                    );
                }
                return Ok(());
            }
            let (lp, rp) = s.split_at(self.chunk.len());
            unsafe {
                ptr::copy_nonoverlapping(lp.as_ptr(), self.chunk.as_mut_ptr(), lp.len());
            }
            self.renew()?;
            s = rp;
        }
    }

    pub fn write_extensions(&mut self, extensions: &ExtensionMap) -> Result<()> {
        if let Some(values) = extensions.values_raw() {
            for val in values {
                self.write_raw_bytes(val)?;
            }
        }
        Ok(())
    }
}

impl<'a, B: BufMut> Drop for CodedOutputStream<'a, B> {
    fn drop(&mut self) {
        unsafe { self.buf.advance_mut(self.last_len - self.chunk.len()) }
    }
}
