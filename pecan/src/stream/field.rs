use std::ptr;

use bytes::{Buf, BufMut, Bytes, buf::UninitSlice};
use crate::{Enumerate, Message, stream::{CodedInputStream, CodedOutputStream}};
use crate::{Result, Error, codec};

pub trait MergeFieldCodec<Field> {
    fn merge_from<B: Buf>(current: &mut Field, buf: &mut CodedInputStream<B>) -> Result<()>;
}

pub trait WriteFieldCodec<Field> {
    fn write_to<B: BufMut>(val: Field, buf: &mut CodedOutputStream<B>) -> Result<()>;
    fn len(val: Field) -> u64;
}

pub struct Varint;

impl MergeFieldCodec<bool> for Varint {
    #[inline]
    fn merge_from<B: Buf>(current: &mut bool, buf: &mut CodedInputStream<B>) -> Result<()> {
        loop {
            if !buf.chunk.is_empty() {
                let b = match buf.chunk[0] {
                    0 => false,
                    1 => true,
                    _ => return Err(Error::corrupted()),
                };
                buf.chunk = &buf.chunk[1..];
                *current = b;
            }
            buf.renew()?;
        }
    }
}

impl WriteFieldCodec<bool> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: bool, buf: &mut CodedOutputStream<B>) -> Result<()> {
        loop {
            if buf.chunk.len() > 0 {
                unsafe {
                    buf.chunk.as_mut_ptr().write(val as u8);
                    buf.chunk = UninitSlice::from_raw_parts_mut(
                        buf.chunk.as_mut_ptr().add(1),
                        buf.chunk.len() - 1,
                    );
                }
                return Ok(());
            }
            buf.renew()?;
        }
    }

    #[inline]
    fn len(_: bool) -> u64 {
        1
    }
}

pub struct LengthPrefixed;

impl MergeFieldCodec<Bytes> for LengthPrefixed {
    fn merge_from<B: Buf>(current: &mut Bytes, buf: &mut CodedInputStream<B>) -> Result<()> {
        let len = buf.read_var_u64()?;
        if len <= usize::MAX as u64 {
            buf.flush();
            let len = len as usize;
            if buf.buf.remaining() >= len {
                *current = buf.buf.copy_to_bytes(len as usize);
                return Ok(())
            } else {
                return Err(Error::Eof);
            }
        }
        Err(Error::corrupted())
    }
}

impl<'a> WriteFieldCodec<&'a [u8]> for LengthPrefixed {
    fn write_to<B: BufMut>(val: &[u8], buf: &mut CodedOutputStream<B>) -> Result<()> {
        buf.write_raw_bytes(val)
    }

    fn len(val: &[u8]) -> u64 {
        Varint::len(val.len() as u64) + val.len() as u64
    }
}

pub struct Fixed;

impl MergeFieldCodec<f64> for Fixed {
    #[inline]
    fn merge_from<B: Buf>(current: &mut f64, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u: u64 = 0;
        Fixed::merge_from(&mut u, buf)?;
        *current = f64::from_bits(u);
        Ok(())
    }
}

impl WriteFieldCodec<f64> for Fixed {
    #[inline]
    fn write_to<B: BufMut>(val: f64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = val.to_bits();
        Fixed::write_to(u, buf)
    }

    #[inline]
    fn len(_: f64) -> u64 {
        8
    }
}

impl MergeFieldCodec<u32> for Fixed {
    fn merge_from<B: Buf>(current: &mut u32, buf: &mut CodedInputStream<B>) -> Result<()> {
        if buf.chunk.len() >= 4 {
            unsafe {
                let u = (buf.chunk.as_ptr() as *const u32).read_unaligned();
                *current = u32::from_le(u);
                buf.chunk = &buf.chunk[4..];
                return Ok(());
            }
        }
        let mut bytes = [0; 4];
        buf.read_n_bytes(&mut bytes)?;
        *current = u32::from_le_bytes(bytes);
        Ok(())
    }
}

impl WriteFieldCodec<u32> for Fixed {
    fn write_to<B: BufMut>(val: u32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let bytes = val.to_le_bytes();
        if buf.chunk.len() >= 8 {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), buf.chunk.as_mut_ptr(), 4);
                buf.chunk = UninitSlice::from_raw_parts_mut(
                    buf.chunk.as_mut_ptr().add(4),
                    buf.chunk.len() - 4,
                );
            }
            return Ok(());
        }
        buf.write_raw_bytes(&bytes)
    }

    #[inline]
    fn len(_: u32) -> u64 {
        4
    }
}

impl MergeFieldCodec<u64> for Fixed {
    fn merge_from<B: Buf>(current: &mut u64, buf: &mut CodedInputStream<B>) -> Result<()> {
        if buf.chunk.len() >= 8 {
            unsafe {
                let u = (buf.chunk.as_ptr() as *const u64).read_unaligned();
                *current = u64::from_le(u);
                buf.chunk = &buf.chunk[8..];
                return Ok(());
            }
        }
        let mut bytes = [0; 8];
        buf.read_n_bytes(&mut bytes)?;
        *current = u64::from_le_bytes(bytes);
        Ok(())
    }
}

impl WriteFieldCodec<u64> for Fixed {
    fn write_to<B: BufMut>(val: u64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let bytes = val.to_le_bytes();
        if buf.chunk.len() >= 8 {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), buf.chunk.as_mut_ptr(), 8);
                buf.chunk = UninitSlice::from_raw_parts_mut(
                    buf.chunk.as_mut_ptr().add(8),
                    buf.chunk.len() - 8,
                );
            }
            return Ok(());
        }
        buf.write_raw_bytes(&bytes)
    }

    #[inline]
    fn len(_: u64) -> u64 {
        8
    }
}

impl MergeFieldCodec<f32> for Fixed {
    #[inline]
    fn merge_from<B: Buf>(current: &mut f32, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u: u32 = 0;
        Fixed::merge_from(&mut u, buf)?;
        *current = f32::from_bits(u);
        Ok(())
    }
}

impl WriteFieldCodec<f32> for Fixed {
    #[inline]
    fn write_to<B: BufMut>(val: f32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = val.to_bits();
        Fixed::write_to(u, buf)
    }

    #[inline]
    fn len(_: f32) -> u64 {
        4
    }
}

impl MergeFieldCodec<i32> for Varint {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i32, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut v: i64 = 0;
        Varint::merge_from(&mut v, buf)?;
        if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            *current = v as i32;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }
}

impl WriteFieldCodec<i32> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as i64, buf)
    }

    #[inline]
    fn len(val: i32) -> u64 {
        Varint::len(val as i64)
    }
}

impl MergeFieldCodec<i64> for Varint {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i64, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u: u64 = 0;
        Varint::merge_from(&mut u, buf)?;
        *current = u as i64;
        Ok(())
    }
}

impl WriteFieldCodec<i64> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as u64, buf)
    }

    #[inline]
    fn len(val: i64) -> u64 {
        Varint::len(val as u64)
    }
}

impl MergeFieldCodec<i32> for Fixed {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i32, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u: u32 = 0;
        Fixed::merge_from(&mut u, buf)?;
        *current = u as i32;
        Ok(())
    }
}

impl WriteFieldCodec<i32> for Fixed {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Fixed::write_to(val as u32, buf)
    }

    #[inline]
    fn len(_: i32) -> u64 {
        4
    }
}

impl MergeFieldCodec<i64> for Fixed {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i64, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u = 0;
        Fixed::merge_from(&mut u, buf)?;
        *current = u as i64;
        Ok(())
    }
}

impl WriteFieldCodec<i64> for Fixed {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Fixed::write_to(val as u64, buf)
    }

    #[inline]
    fn len(_: i64) -> u64 {
        8
    }
}

impl MergeFieldCodec<String> for LengthPrefixed {
    fn merge_from<B: Buf>(current: &mut String, buf: &mut CodedInputStream<B>) -> Result<()> {
        let len = buf.read_var_u64()?;
        if len <= usize::MAX as u64 {
            let len = len as usize;
            let mut v = Vec::with_capacity(len);
            loop {
                if buf.chunk.len() >= len - v.len() {
                    let (res, rest) = buf.chunk.split_at(len - v.len());
                    buf.chunk = rest;
                    v.extend_from_slice(res);
                    match String::from_utf8(v) {
                        Ok(s) => {
                            *current = s;
                            return Ok(());
                        }
                        Err(_) => return Err(Error::corrupted()),
                    }
                }
                buf.renew()?;
            }
        }
        Err(Error::corrupted())
    }
}

impl<'a> WriteFieldCodec<&'a str> for LengthPrefixed {
    fn write_to<B: BufMut>(val: &str, buf: &mut CodedOutputStream<B>) -> Result<()> {
        LengthPrefixed::write_to(val.as_bytes(), buf)
    }

    fn len(val: &str) -> u64 {
        LengthPrefixed::len(val.as_bytes())
    }
}

pub struct ZigZag;

impl MergeFieldCodec<i32> for ZigZag {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i32, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut i: i64 = 0;
        ZigZag::merge_from(&mut i, buf)?;
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            *current = i as i32;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }
}

impl WriteFieldCodec<i32> for ZigZag {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        ZigZag::write_to(val as i64, buf)
    }

    #[inline]
    fn len(val: i32) -> u64 {
        ZigZag::len(val as i64)
    }
}

impl MergeFieldCodec<i64> for ZigZag {
    #[inline]
    fn merge_from<B: Buf>(current: &mut i64, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u: u64 = 0;
        Varint::merge_from(&mut u, buf)?;
        *current = u.rotate_right(1) as i64;
        Ok(())
    }
}

impl WriteFieldCodec<i64> for ZigZag {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val.rotate_left(1) as u64, buf)
    }

    #[inline]
    fn len(val: i64) -> u64 {
        Varint::len(val.rotate_left(1) as u64)
    }
}

impl MergeFieldCodec<u32> for Varint {
    #[inline]
    fn merge_from<B: Buf>(current: &mut u32, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut u = 0;
        Varint::merge_from(&mut u, buf)?;
        if u >= u32::MIN as u64 && u <= u32::MAX as u64 {
            *current = u as u32;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }
}

impl WriteFieldCodec<u32> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: u32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as u64, buf)
    }

    #[inline]
    fn len(val: u32) -> u64 {
        Varint::len(val as u64)
    }
}

impl MergeFieldCodec<u64> for Varint {
    fn merge_from<B: Buf>(current: &mut u64, buf: &mut CodedInputStream<B>) -> Result<()> {
        let (mut tmp, mut lp) = match codec::decode_var_u64(&mut buf.chunk) {
            Ok(v) => {
                *current = v;
                return Ok(())
            },
            Err(Error::WantMore(tmp, lp)) => (tmp, lp),
            Err(e) => return Err(e),
        };

        loop {
            buf.renew()?;
            match codec::decode_var_u64_resume(&mut buf.chunk, tmp, lp) {
                Ok(v) => {
                    *current = v;
                    return Ok(())
                },
                Err(Error::WantMore(t, l)) => {
                    tmp = t;
                    lp = l;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl WriteFieldCodec<u64> for Varint {
    fn write_to<B: BufMut>(mut val: u64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        loop {
            match codec::encode_var_u64(&mut buf.chunk, val) {
                Ok(()) => return Ok(()),
                Err(Error::WantMore(v, _)) => {
                    buf.renew()?;
                    val = v;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn len(mut val: u64) -> u64 {
        for i in 1..=9 {
            val >>= 7;
            if val == 0 {
                return i;
            }
        }
        10
    }
}

impl<E: Enumerate> MergeFieldCodec<E> for Varint {
    #[inline]
    fn merge_from<B: Buf>(current: &mut E, buf: &mut CodedInputStream<B>) -> Result<()> {
        let mut val = 0;
        Varint::merge_from(&mut val, buf)?;
        *current = E::from_value(val);
        Ok(())
    }
}

impl<E: Enumerate> WriteFieldCodec<E> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: E, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val.value(), buf)
    }

    #[inline]
    fn len(val: E) -> u64 {
        Varint::len(val.value())
    }
}

impl<M: Message> MergeFieldCodec<M> for LengthPrefixed {
    fn merge_from<B: Buf>(current: &mut M, buf: &mut CodedInputStream<B>) -> Result<()> {
        let s = buf.read_var_u64()?;
        if buf.depth < buf.depth_limit {
            buf.depth += 1;
        } else {
            return Err(Error::DepthExcceedLimit(buf.depth_limit));
        }
        let last_limit = buf.limit;
        buf.limit = buf.progress() + s;
        current.merge_from(buf)?;
        buf.depth -= 1;
        if buf.limit == buf.progress() {
            buf.limit = last_limit;
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }
}

impl<'a, M: Message> WriteFieldCodec<&'a M> for LengthPrefixed {
    fn write_to<B: BufMut>(val: &M, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let l = val.len();
        Varint::write_to(l, buf)?;
        val.write_to(buf)
    }

    fn len(val: &M) -> u64 {
        let l = val.len();
        Varint::len(l) + l
    }
}
