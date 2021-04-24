use crate::{codec, Enumerate, Error, Message, Result};
use bytes::{buf::UninitSlice, Buf, BufMut, Bytes};
use std::{i32, mem, ptr};

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
            let tag = self.read_var_u64()?;
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

    pub fn read_var_u64(&mut self) -> Result<u64> {
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

    pub fn read_var_u32(&mut self) -> Result<u32> {
        let u = self.read_var_u64()?;
        if u <= u32::MAX as u64 {
            Ok(u as u32)
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_zz_i64(&mut self) -> Result<i64> {
        let u = self.read_var_u64()?;
        Ok(u.rotate_right(1) as i64)
    }

    pub fn read_zz_i32(&mut self) -> Result<i32> {
        let i = self.read_var_i64()?;
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            Ok(i as i32)
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_var_u64()?;
        if len <= usize::MAX as u64 {
            let len = len as usize;
            let mut v = Vec::with_capacity(len);
            loop {
                if self.chunk.len() >= len - v.len() {
                    let (res, rest) = self.chunk.split_at(len - v.len());
                    self.chunk = rest;
                    v.extend_from_slice(res);
                    return String::from_utf8(v).map_err(|_| Error::corrupted());
                }
                self.renew()?;
            }
        }
        Err(Error::corrupted())
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let u = self.read_u64()?;
        Ok(u as i64)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let u = self.read_u32()?;
        Ok(u as i32)
    }

    pub fn read_var_i64(&mut self) -> Result<i64> {
        let u = self.read_var_u64()?;
        Ok(u as i64)
    }

    pub fn read_var_i32(&mut self) -> Result<i32> {
        let u = self.read_var_u32()?;
        Ok(u as i32)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let u = self.read_u32()?;
        Ok(f32::from_bits(u))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        if self.chunk.len() >= 8 {
            unsafe {
                let u = (self.chunk.as_ptr() as *const u64).read_unaligned();
                self.chunk = &self.chunk[8..];
                return Ok(u);
            }
        }
        let mut bytes = [0; 8];
        self.read_n_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        if self.chunk.len() >= 4 {
            unsafe {
                let u = (self.chunk.as_ptr() as *const u32).read_unaligned();
                self.chunk = &self.chunk[4..];
                return Ok(u);
            }
        }
        let mut bytes = [0; 4];
        self.read_n_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        let u = self.read_u64()?;
        Ok(f64::from_bits(u))
    }

    pub fn read_bytes(&mut self) -> Result<Bytes> {
        let len = self.read_var_u64()?;
        if len <= usize::MAX as u64 {
            self.flush();
            let len = len as usize;
            if self.buf.remaining() >= len {
                return Ok(self.buf.copy_to_bytes(len as usize));
            } else {
                return Err(Error::Eof);
            }
        }
        Err(Error::corrupted())
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        loop {
            if !self.chunk.is_empty() {
                let b = match self.chunk[0] {
                    0 => false,
                    1 => true,
                    _ => return Err(Error::corrupted()),
                };
                self.chunk = &self.chunk[1..];
                return Ok(b);
            }
            self.renew()?;
        }
    }

    fn read_n_bytes(&mut self, mut target: &mut [u8]) -> Result<()> {
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

    pub fn merge_message_to<M: Message>(&mut self, m: &mut M) -> Result<()> {
        let s = self.read_var_u64()?;
        if self.depth < self.depth_limit {
            self.depth += 1;
        } else {
            return Err(Error::DepthExcceedLimit(self.depth_limit));
        }
        let last_limit = self.limit;
        self.limit = self.progress() + s;
        m.merge_from(self)?;
        self.depth -= 1;
        if self.limit == self.progress() {
            self.limit = last_limit;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_message_to<M: Default + Message>(&mut self, ms: &mut Vec<M>) -> Result<()> {
        ms.push(Default::default());
        let m = ms.last_mut().unwrap();
        self.merge_message_to(m)
    }

    pub fn read_string_to(&mut self, arr: &mut Vec<String>) -> Result<()> {
        arr.push(self.read_string()?);
        Ok(())
    }

    pub fn read_var_i32_to(&mut self, arr: &mut Vec<i32>) -> Result<()> {
        let len = self.read_var_u64()?;
        let limit = self.progress() + len;
        while self.progress() < limit {
            arr.push(self.read_var_i32()?);
        }
        if self.progress() == limit {
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_packed_array<N>(
        &mut self,
        arr: &mut Vec<N>,
        mut f: impl FnMut(&mut Self) -> Result<N>,
    ) -> Result<()> {
        let len = self.read_var_u64()?;
        let limit = self.progress() + len;
        while self.progress() < limit {
            arr.push(f(self)?);
        }
        if self.progress() == limit {
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_enum<E: Enumerate>(&mut self) -> Result<E> {
        let v = self.read_var_i64()?;
        if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            Ok(E::from_value(v as i32))
        } else {
            Err(Error::corrupted())
        }
    }

    pub fn read_unknown_field(&mut self, tag: u64, unknown: &mut impl BufMut) -> Result<()> {
        let mut os = CodedOutputStream::new(unknown);
        os.write_tag(tag)?;
        match tag & 0x7 {
            0 => {
                let v = self.read_var_u64()?;
                os.write_var_u64(v)
            }
            1 => {
                let v = self.read_u64()?;
                os.write_u64(v)
            }
            2 => {
                // TODO: no copy is better.
                let bs = self.read_bytes()?;
                os.write_bytes(&bs)
            }
            3 | 4 => unimplemented!("{}", tag),
            5 => {
                let v = self.read_u32()?;
                os.write_u32(v)
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

    pub fn write_var_u64(&mut self, val: u64) -> Result<()> {
        self.write_var_u64_raw(val)
    }

    pub fn write_var_u32(&mut self, val: u32) -> Result<()> {
        self.write_var_u64(val as u64)
    }

    pub fn write_zz_i64(&mut self, val: i64) -> Result<()> {
        self.write_var_i64(val.rotate_left(1))
    }

    pub fn write_zz_i32(&mut self, val: i32) -> Result<()> {
        self.write_var_i32(val.rotate_left(1))
    }

    pub fn write_string(&mut self, s: &str) -> Result<()> {
        self.write_bytes(s.as_bytes())
    }

    pub fn write_i64(&mut self, val: i64) -> Result<()> {
        self.write_u64(val as u64)
    }

    pub fn write_i32(&mut self, val: i32) -> Result<()> {
        self.write_u32(val as u32)
    }

    pub fn write_var_i64(&mut self, val: i64) -> Result<()> {
        self.write_var_u64(val as u64)
    }

    pub fn write_var_i32(&mut self, val: i32) -> Result<()> {
        self.write_var_i64(val as i64)
    }

    pub fn write_f32(&mut self, f: f32) -> Result<()> {
        self.write_u32(f.to_bits())
    }

    pub fn write_u64(&mut self, u: u64) -> Result<()> {
        let bytes = u.to_le_bytes();
        if self.chunk.len() >= 8 {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), self.chunk.as_mut_ptr(), 8);
                self.chunk = UninitSlice::from_raw_parts_mut(
                    self.chunk.as_mut_ptr().add(8),
                    self.chunk.len() - 8,
                );
            }
            return Ok(());
        }
        self.write_raw_bytes(&bytes)
    }

    pub fn write_u32(&mut self, u: u32) -> Result<()> {
        let bytes = u.to_le_bytes();
        if self.chunk.len() >= 8 {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), self.chunk.as_mut_ptr(), 4);
                self.chunk = UninitSlice::from_raw_parts_mut(
                    self.chunk.as_mut_ptr().add(4),
                    self.chunk.len() - 4,
                );
            }
            return Ok(());
        }
        self.write_raw_bytes(&bytes)
    }

    pub fn write_f64(&mut self, f: f64) -> Result<()> {
        self.write_u64(f.to_bits())
    }

    pub fn write_bytes(&mut self, s: &[u8]) -> Result<()> {
        self.write_var_u64_raw(s.len() as u64)?;
        self.write_raw_bytes(s)
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

    pub fn write_bool(&mut self, b: bool) -> Result<()> {
        loop {
            if self.chunk.len() > 0 {
                unsafe {
                    self.chunk.as_mut_ptr().write(b as u8);
                    self.chunk = UninitSlice::from_raw_parts_mut(
                        self.chunk.as_mut_ptr().add(1),
                        self.chunk.len() - 1,
                    );
                }
                return Ok(());
            }
            self.renew()?;
        }
    }

    pub fn write_message<M: Message>(&mut self, msg: &M) -> Result<()> {
        let l = msg.len();
        self.write_var_u64_raw(l)?;
        msg.write_to(self)
    }

    pub fn write_enum<E: Enumerate>(&mut self, e: E) -> Result<()> {
        self.write_var_i64(e.value() as i64)
    }

    pub fn write_packed_array<N: Copy>(
        &mut self,
        len: u64,
        arr: &[N],
        mut f: impl FnMut(&mut Self, N) -> Result<()>,
    ) -> Result<()> {
        self.write_var_u64_raw(len)?;
        for n in arr {
            f(self, *n)?;
        }
        Ok(())
    }
}

impl<'a, B: BufMut> Drop for CodedOutputStream<'a, B> {
    fn drop(&mut self) {
        unsafe { self.buf.advance_mut(self.last_len - self.chunk.len()) }
    }
}

pub fn var_u64_len(mut s: u64) -> u64 {
    for i in 1..=9 {
        s >>= 7;
        if s == 0 {
            return i;
        }
    }
    10
}

pub fn var_u32_len(s: u32) -> u64 {
    var_u64_len(s as u64)
}

pub fn zz_i64_len(s: i64) -> u64 {
    var_i64_len(s.rotate_left(1))
}

pub fn zz_i32_len(s: i32) -> u64 {
    var_i32_len(s.rotate_left(1))
}

pub fn string_len(s: &str) -> u64 {
    bytes_len(s.as_bytes())
}

pub fn i64_len(_: i64) -> u64 {
    8
}

pub fn i32_len(_: i32) -> u64 {
    4
}

pub fn var_i64_len(s: i64) -> u64 {
    var_u64_len(s as u64)
}

pub fn var_i32_len(s: i32) -> u64 {
    var_i64_len(s as i64)
}

pub fn f32_len(_: f32) -> u64 {
    4
}

pub fn u64_len(_: u64) -> u64 {
    8
}

pub fn u32_len(_: u32) -> u32 {
    4
}

pub fn f64_len(_: f64) -> u64 {
    8
}

pub fn bytes_len(s: &[u8]) -> u64 {
    var_u64_len(s.len() as u64) + s.len() as u64
}

pub fn bool_len(_: bool) -> u64 {
    1
}

pub fn message_len(m: &impl Message) -> u64 {
    let l = m.len();
    var_u64_len(l) + l
}

pub fn enum_len<E: Enumerate>(e: E) -> u64 {
    var_i64_len(e.value() as i64)
}

pub fn packed_array_len<N: Copy>(arr: &[N], mut f: impl FnMut(N) -> u64) -> u64 {
    let total_len: u64 = arr.into_iter().map(|n| f(*n)).sum();
    var_u64_len(total_len) + total_len
}
