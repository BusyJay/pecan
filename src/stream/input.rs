use crate::enumerate::EnumType;
use crate::message::Message;
use crate::stream::Buf;
use crate::{Error, Result};
use pecan_utils::codec;
use std::{ptr, usize};

macro_rules! impl_fixed {
    ($t:ident, $func:ident, $width:expr) => {
        pub fn $func(&mut self) -> Result<$t> {
            if self.read + $width > self.len_limit {
                return Err(Error::truncated());
            }
            let mut data = [0u8; $width];
            let mut read = 0;
            loop {
                let len = unsafe {
                    let dst = data.as_mut_ptr().add(read);
                    let bytes = self.buf.bytes();
                    if bytes.len() >= $width - read {
                        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, $width - read);
                        $width - read
                    } else {
                        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
                        bytes.len()
                    }
                };
                self.buf.advance(len);
                self.read += len;
                read += len;
                if read == $width {
                    return Ok($t::from_le_bytes(data));
                }
            }
        }
    };
}

macro_rules! impl_numeric_array {
    ($t:ident, $func:ident, $read_one:ident) => {
        pub fn $func(&mut self, dst: &mut Vec<$t>) -> Result<()> {
            let len = self.read_raw_varint32()? as usize;
            let old_limit = self.push_limit(len)?;
            while self.read < self.len_limit {
                dst.push(self.$read_one()?);
            }
            self.pop_limit(old_limit);
            Ok(())
        }
    }
}

pub struct CodedInputStream<B> {
    buf: B,
    read: usize,
    len_limit: usize,
    depth: usize,
    recursive_limit: usize,
}

impl<B: Buf> CodedInputStream<B> {
    #[inline]
    pub fn new(buf: B) -> CodedInputStream<B> {
        CodedInputStream::with_recursive_limit(buf, 100)
    }

    #[inline]
    pub fn with_recursive_limit(buf: B, limit: usize) -> CodedInputStream<B> {
        CodedInputStream {
            len_limit: buf.available(),
            read: 0,
            buf,
            depth: 0,
            recursive_limit: limit,
        }
    }

    #[inline]
    pub fn read_tag(&mut self) -> Result<u32> {
        if self.len_limit == usize::MAX {
            if !self.buf.bytes().is_empty() {
                match self.read_raw_varint32() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                }
            } else {
                Ok(0)
            }
        } else if self.len_limit != self.read {
            match self.read_raw_varint32() {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        } else {
            Ok(0)
        }
    }

    #[inline]
    pub fn read_string(&mut self) -> Result<String> {
        let l = self.read_raw_varint32()? as usize;
        if l == 0 {
            return Ok(String::new());
        }
        if self.read + l > self.len_limit {
            return Err(Error::truncated());
        }

        let s = {
            let bytes = self.buf.bytes();
            if bytes.len() >= l {
                match std::str::from_utf8(&bytes[..l]) {
                    Ok(s) => Some(s.to_owned()),
                    Err(_) => {
                        return Err(Error::corrupted())
                    }
                }
            } else {
                None
            }
        };
        let s = match s {
            Some(s) => {
                self.buf.advance(l);
                self.read += l;
                s
            }
            None => {
                let bytes = self.read_fixed_bytes(l)?;
                match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(Error::corrupted())
                    }
                }
            }
        };
        Ok(s)
    }

    #[inline]
    pub fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let l = self.read_raw_varint32()? as usize;
        if l == 0 {
            return Ok(Vec::new());
        }
        if self.read + l > self.len_limit {
            return Err(Error::truncated());
        }
        self.read_fixed_bytes(l)
    }

    fn read_fixed_bytes(&mut self, l: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(l);
        let mut read = 0;
        while l > read {
            let to_copy = {
                let bytes = self.buf.bytes();
                if bytes.is_empty() {
                    return Err(Error::truncated());
                }
                let to_copy = if bytes.len() < l - read {
                    bytes.len()
                } else {
                    l - read
                };
                unsafe {
                    ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr().add(read), to_copy)
                }
                to_copy
            };
            self.buf.advance(to_copy);
            read += to_copy;
        }
        self.read += read;
        unsafe { buf.set_len(l); }
        Ok(buf)
    }

    impl_fixed!(u32, read_fixed32, 4);
    impl_fixed!(u64, read_fixed64, 8);
    impl_fixed!(i32, read_sfixed32, 4);
    impl_fixed!(i64, read_sfixed64, 8);

    #[inline]
    pub fn read_f32(&mut self) -> Result<f32> {
        let u = self.read_fixed32()?;
        Ok(f32::from_bits(u))
    }

    #[inline]
    pub fn read_f64(&mut self) -> Result<f64> {
        let u = self.read_fixed64()?;
        Ok(f64::from_bits(u))
    }

    #[inline]
    pub fn read_var_u32(&mut self) -> Result<u32> {
        self.read_raw_varint32()
    }

    #[inline]
    pub fn read_var_u64(&mut self) -> Result<u64> {
        self.read_raw_varint64()
    }

    #[inline]
    pub fn read_var_i32(&mut self) -> Result<i32> {
        self.read_raw_varint64().map(|u| u as i64 as i32)
    }

    #[inline]
    pub fn read_var_i64(&mut self) -> Result<i64> {
        self.read_raw_varint64().map(|u| u as i64)
    }

    #[inline]
    pub fn read_var_s32(&mut self) -> Result<i32> {
        self.read_raw_varint32().map(codec::unzig_zag_32)
    }

    #[inline]
    pub fn read_var_s64(&mut self) -> Result<i64> {
        self.read_raw_varint64().map(codec::unzig_zag_64)
    }

    #[inline]
    pub fn read_bool(&mut self) -> Result<bool> {
        if self.read == self.len_limit {
            return Err(Error::truncated());
        }
        let b = {
            let bytes = self.buf.bytes();
            if bytes.is_empty() {
                return Err(Error::truncated());
            }
            unsafe { *bytes.get_unchecked(0) }
        };
        self.buf.advance(1);
        self.read += 1;
        match b {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::corrupted()),
        }
    }

    #[inline]
    pub fn read_enum<E: EnumType>(&mut self) -> Result<E> {
        let value = self.read_raw_varint32()? as i32;
        Ok(E::from(value))
    }

    pub fn read_message_like(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        let len = self.read_raw_varint32()? as usize;
        if self.depth == self.recursive_limit {
            return Err(Error::ExceedRecursiveLimit(self.depth));
        }
        let old_limit = self.push_limit(len)?;
        self.depth += 1;
        f(self)?;
        if self.read != self.len_limit {
            return Err(Error::corrupted());
        }
        self.depth -= 1;
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_message(&mut self, msg: &mut impl Message) -> Result<()> {
        self.read_message_like(|s| msg.merge_from(s))
    }

    pub fn read_message_to(&mut self, msgs: &mut Vec<impl Message + Default>) -> Result<()> {
        msgs.push(Default::default());
        let pos = msgs.len() - 1;
        let last = unsafe { msgs.get_unchecked_mut(pos) };
        self.read_message(last)
    }

    impl_numeric_array!(bool, read_bool_array, read_bool);
    impl_numeric_array!(i32, read_var_i32_array, read_var_i32);
    impl_numeric_array!(u32, read_var_u32_array, read_var_u32);
    impl_numeric_array!(i32, read_var_s32_array, read_var_s32);
    impl_numeric_array!(f32, read_f32_array, read_f32);
    impl_numeric_array!(i32, read_sfixed32_array, read_sfixed32);
    impl_numeric_array!(u32, read_fixed32_array, read_fixed32);
    impl_numeric_array!(i64, read_var_i64_array, read_var_i64);
    impl_numeric_array!(u64, read_var_u64_array, read_var_u64);
    impl_numeric_array!(i64, read_var_s64_array, read_var_s64);
    impl_numeric_array!(f64, read_f64_array, read_f64);
    impl_numeric_array!(i64, read_sfixed64_array, read_sfixed64);
    impl_numeric_array!(u64, read_fixed64_array, read_fixed64);

    pub fn read_enum_array(&mut self, dst: &mut Vec<impl EnumType>) -> Result<()> {
        let len = self.read_raw_varint32()? as usize;
        let old_limit = self.push_limit(len)?;
        while self.read < self.len_limit {
            dst.push(self.read_var_i32()?.into());
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    fn push_limit(&mut self, new_limit: usize) -> Result<usize> {
        let new_len_limit = self.read + new_limit;
        if new_len_limit > self.len_limit {
            return Err(Error::truncated());
        }
        let old_limit = self.len_limit;
        self.len_limit = new_len_limit;
        Ok(old_limit)
    }

    fn pop_limit(&mut self, old_limit: usize) {
        self.len_limit = old_limit;
    }

    fn skip_varint(&mut self, unknown: &mut impl Discard) -> Result<()> {
        let u = self.read_raw_varint64()?;
        unknown.discard_varint(u);
        Ok(())
    }

    fn skip_group(&mut self, unknown: &mut impl Discard, tag: u32) -> Result<()> {
        self.depth += 1;
        loop {
            let next_tag = self.read_tag()?;
            if next_tag == 0 {
                self.depth -= 1;
                return Err(Error::truncated());
            }
            if next_tag & 0x07 == 4 {
                if next_tag >> 3 == tag >> 3 {
                    unknown.discard_varint(next_tag as u64);
                    self.depth -= 1;
                    return Ok(());
                } else {
                    self.depth -= 1;
                    return Err(Error::corrupted());
                }
            }
            self.skip_field_impl(unknown, next_tag)?;
        }
    }

    fn skip_field_impl(&mut self, unknown: &mut impl Discard, tag: u32) -> Result<()> {
        if tag >> 3 == 0 {
            // 0 field number is illegal.
            return Err(Error::corrupted());
        }
        unknown.discard_varint(tag as u64);
        let mut to_skip = match tag & 0x07 {
            0 => return self.skip_varint(unknown),
            1 => 8,
            2 => {
                let len = self.read_raw_varint32()?;
                unknown.discard_varint(len as u64);
                len as usize
            }
            3 => return self.skip_group(unknown, tag),
            4 => return Err(Error::corrupted()),
            5 => 4,
            _ => return Err(Error::corrupted()),
        };
        if to_skip + self.read > self.len_limit {
            return Err(Error::truncated());
        }
        while to_skip > 0 {
            let l = {
                let bytes = self.buf.bytes();
                if bytes.is_empty() {
                    return Err(Error::truncated());
                }
                if bytes.len() > to_skip {
                    unknown.discard_slice(unsafe { bytes.get_unchecked(..to_skip) });
                    to_skip
                } else {
                    unknown.discard_slice(bytes);
                    bytes.len()
                }
            };
            self.buf.advance(l);
            to_skip -= l;
        }
        Ok(())
    }

    #[inline]
    pub fn skip_field(&mut self, unknown: &mut Vec<u8>, tag: u32) -> Result<()> {
        self.skip_field_impl(unknown, tag)
    }

    #[inline]
    pub fn discard_field(&mut self, tag: u32) -> Result<()> {
        self.skip_field_impl(&mut BlackHole, tag)
    }

    #[inline]
    fn read_raw_varint32(&mut self) -> Result<u32> {
        match codec::decode_varint_u32(self.buf.bytes()) {
            Ok((u, n)) => {
                if self.read + n as usize <= self.len_limit {
                    self.read += n as usize;
                    self.buf.advance(n as usize);
                    Ok(u)
                } else {
                    Err(Error::truncated())
                }
            }
            Err(codec::Error::Truncated) => self.slow_read_raw_varint32(),
            Err(codec::Error::Corrupted) => Err(Error::corrupted()),
        }
    }

    fn slow_read_raw_varint32(&mut self) -> Result<u32> {
        let mut data: u32 = 0;
        let mut read: usize = 0;
        'outer: loop {
            let l = {
                let buf = self.buf.bytes();
                for i in 0..std::cmp::min(buf.len(), 4 - read) {
                    let b = unsafe { *buf.get_unchecked(i) };
                    data |= ((b & 0x7f) as u32) << ((i + read) * 7);
                    if b < 0x80 {
                        read = i + 1;
                        break 'outer;
                    }
                }
                if buf.len() > 4 - read {
                    let b = unsafe { *buf.get_unchecked(4 - read) };
                    if b <= 15 {
                        data |= (b as u32) << 28;
                        read = 4 - read + 1;
                        break 'outer;
                    }
                    if (b & 0x80) > 0 {
                        if buf.len() >= 10 - read {
                            let b = unsafe { *buf.get_unchecked(9 - read) };
                            if b == 1 {
                                read = 10 - read;
                                break 'outer;
                            } else {
                                return Err(Error::corrupted());
                            }
                        }
                    } else {
                        return Err(Error::corrupted());
                    }
                } else if buf.is_empty() {
                    return Err(Error::truncated());
                }
                buf.len()
            };
            read += l;
            if l + self.read <= self.len_limit {
                self.read += l;
                self.buf.advance(l);
            } else {
                return Err(Error::truncated());
            }
            if read < 5 {
                continue;
            }
            loop {
                let l = {
                    let buf = self.buf.bytes();
                    if buf.len() >= 10 - read {
                        let b = unsafe { *buf.get_unchecked(9 - read) };
                        if b == 1 {
                            read = 10 - read;
                            break 'outer;
                        } else {
                            return Err(Error::corrupted());
                        }
                    } else if buf.is_empty() {
                        return Err(Error::truncated());
                    }
                    buf.len()
                };
                read += l;
                if l + self.read <= self.len_limit {
                    self.read += l;
                    self.buf.advance(l);
                } else {
                    return Err(Error::truncated());
                }
            }
        }
        if read + self.read <= self.len_limit {
            self.read += read;
            self.buf.advance(read);
            Ok(data)
        } else {
            Err(Error::truncated())
        }
    }

    #[inline]
    fn read_raw_varint64(&mut self) -> Result<u64> {
        match codec::decode_varint_u64(self.buf.bytes()) {
            Ok((u, n)) => {
                if self.read + n as usize <= self.len_limit {
                    self.read += n as usize;
                    self.buf.advance(n as usize);
                    Ok(u)
                } else {
                    Err(Error::truncated())
                }
            },
            Err(codec::Error::Truncated) => self.slow_read_raw_varint64(),
            Err(codec::Error::Corrupted) => Err(Error::corrupted()),
        }
    }

    fn slow_read_raw_varint64(&mut self) -> Result<u64> {
        let mut data = 0;
        let mut read: usize = 0;
        'outer: loop {
            let l = {
                let buf = self.buf.bytes();
                for i in 0..std::cmp::min(buf.len(), 9 - read) {
                    let b = unsafe { *buf.get_unchecked(i) };
                    data |= ((b & 0x7f) as u64) << ((read + i) * 7);
                    if b < 0x80 {
                        read = i + 1;
                        break 'outer;
                    }
                }
                if buf.len() > 9 - read {
                    let b = unsafe { *buf.get_unchecked(9 - read) };
                    if b <= 1 {
                        data |= (b as u64) << 63;
                        read = 10 - read;
                        break 'outer;
                    } else {
                        return Err(Error::corrupted());
                    }
                } else if buf.is_empty() {
                    return Err(Error::truncated());
                }
                buf.len()
            };
            read += l;
            if l + self.read <= self.len_limit {
                self.read += l;
                self.buf.advance(l);
            } else {
                return Err(Error::truncated());
            }
        }
        if read + self.read <= self.len_limit {
            self.read += read;
            self.buf.advance(read);
            Ok(data)
        } else {
            Err(Error::truncated())
        }
    }
}

trait Discard {
    fn discard_varint(&mut self, tag: u64);
    fn discard_slice(&mut self, bytes: &[u8]);
}

impl Discard for Vec<u8> {
    #[inline]
    fn discard_varint(&mut self, tag: u64) {
        codec::encode_varint_u64(self, tag)
    }

    #[inline]
    fn discard_slice(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes)
    }
}

struct BlackHole;

impl Discard for BlackHole {
    #[inline]
    fn discard_varint(&mut self, _: u64) {}

    #[inline]
    fn discard_slice(&mut self, _: &[u8]) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_var_i64() {
        let s: &[u8] = &[0x80];
        let mut cis = CodedInputStream::new(s);
        let e = cis.read_var_i64().unwrap_err();
        assert!(e.is_truncated(), "{:?}", e);
    }
}
