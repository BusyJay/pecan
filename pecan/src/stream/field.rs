use std::{marker::PhantomData, ptr};

use crate::{
    codec,
    stream::{CodedInputStream, CodedOutputStream},
    Enumerate, Message,
};
use crate::{Error, Result};
use bytes::{buf::UninitSlice, Buf, BufMut, Bytes};

pub trait ReadFieldCodec<Field> {
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<Field>;
}

pub trait MergeFieldCodec<Field> {
    fn merge_from<B: Buf>(current: &mut Field, buf: &mut CodedInputStream<B>) -> Result<()>;
}

pub trait WriteFieldCodec<Field> {
    fn write_to<B: BufMut>(val: Field, buf: &mut CodedOutputStream<B>) -> Result<()>;
    fn size(val: Field) -> u64;
}

pub struct Varint;

impl ReadFieldCodec<bool> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<bool> {
        Varint::read_from(buf).map(|i: u64| i != 0)
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
    fn size(_: bool) -> u64 {
        1
    }
}

pub struct LengthPrefixed;

impl ReadFieldCodec<Bytes> for LengthPrefixed {
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<Bytes> {
        let len = buf.read_var_u64_raw()?;
        if len <= usize::MAX as u64 {
            return buf.copy_to_bytes(len as usize);
        }
        Err(Error::corrupted())
    }
}

impl<'a> WriteFieldCodec<&'a [u8]> for LengthPrefixed {
    fn write_to<B: BufMut>(val: &[u8], buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val.len() as u64, buf)?;
        buf.write_raw_bytes(val)
    }

    #[inline]
    fn size(val: &[u8]) -> u64 {
        Varint::size(val.len() as u64) + val.len() as u64
    }
}

pub struct Fixed32;

pub struct Fixed64;

impl ReadFieldCodec<f64> for Fixed64 {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<f64> {
        let u: u64 = Fixed64::read_from(buf)?;
        Ok(f64::from_bits(u))
    }
}

impl WriteFieldCodec<f64> for Fixed64 {
    #[inline]
    fn write_to<B: BufMut>(val: f64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = val.to_bits();
        Fixed64::write_to(u, buf)
    }

    #[inline]
    fn size(_: f64) -> u64 {
        8
    }
}

impl ReadFieldCodec<u32> for Fixed32 {
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<u32> {
        if buf.chunk.len() >= 4 {
            unsafe {
                let u = (buf.chunk.as_ptr() as *const u32).read_unaligned();
                let v = u32::from_le(u);
                buf.chunk = &buf.chunk[4..];
                return Ok(v);
            }
        }
        let mut bytes = [0; 4];
        buf.read_raw_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }
}

impl WriteFieldCodec<u32> for Fixed32 {
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
    fn size(_: u32) -> u64 {
        4
    }
}

impl ReadFieldCodec<u64> for Fixed64 {
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<u64> {
        if buf.chunk.len() >= 8 {
            unsafe {
                let u = (buf.chunk.as_ptr() as *const u64).read_unaligned();
                let v = u64::from_le(u);
                buf.chunk = &buf.chunk[8..];
                return Ok(v);
            }
        }
        let mut bytes = [0; 8];
        buf.read_raw_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

impl WriteFieldCodec<u64> for Fixed64 {
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
    fn size(_: u64) -> u64 {
        8
    }
}

impl ReadFieldCodec<f32> for Fixed32 {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<f32> {
        let u: u32 = Fixed32::read_from(buf)?;
        Ok(f32::from_bits(u))
    }
}

impl WriteFieldCodec<f32> for Fixed32 {
    #[inline]
    fn write_to<B: BufMut>(val: f32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = val.to_bits();
        Fixed32::write_to(u, buf)
    }

    #[inline]
    fn size(_: f32) -> u64 {
        4
    }
}

impl ReadFieldCodec<i32> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i32> {
        let v: i64 = Varint::read_from(buf)?;
        Ok(v as i32)
    }
}

impl WriteFieldCodec<i32> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as i64, buf)
    }

    #[inline]
    fn size(val: i32) -> u64 {
        Varint::size(val as i64)
    }
}

impl ReadFieldCodec<i64> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i64> {
        let u: u64 = Varint::read_from(buf)?;
        Ok(u as i64)
    }
}

impl WriteFieldCodec<i64> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as u64, buf)
    }

    #[inline]
    fn size(val: i64) -> u64 {
        Varint::size(val as u64)
    }
}

impl ReadFieldCodec<i32> for Fixed32 {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i32> {
        let u: u32 = Fixed32::read_from(buf)?;
        Ok(u as i32)
    }
}

impl WriteFieldCodec<i32> for Fixed32 {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Fixed32::write_to(val as u32, buf)
    }

    #[inline]
    fn size(_: i32) -> u64 {
        4
    }
}

impl ReadFieldCodec<i64> for Fixed64 {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i64> {
        let u: u64 = Fixed64::read_from(buf)?;
        Ok(u as i64)
    }
}

impl WriteFieldCodec<i64> for Fixed64 {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Fixed64::write_to(val as u64, buf)
    }

    #[inline]
    fn size(_: i64) -> u64 {
        8
    }
}

impl ReadFieldCodec<String> for LengthPrefixed {
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<String> {
        let len = buf.read_var_u64_raw()?;
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
                            return Ok(s);
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

macro_rules! impl_length_prefix {
    ($type:ty, $method:ident) => {
        impl<'a> WriteFieldCodec<&'a $type> for LengthPrefixed {
            #[inline]
            fn write_to<B: BufMut>(val: &$type, buf: &mut CodedOutputStream<B>) -> Result<()> {
                LengthPrefixed::write_to(impl_length_prefix!(eval val, $method), buf)
            }

            #[inline]
            fn size(val: &$type) -> u64 {
                LengthPrefixed::size(impl_length_prefix!(eval val, $method))
            }
        }
    };
    (eval $val:ident, inplace) => {
        $val
    };
    (eval $val:ident, $method:ident) => {
        $val.$method()
    };
}

impl_length_prefix!(str, as_bytes);
impl_length_prefix!(String, as_bytes);
impl_length_prefix!(Bytes, as_ref);

pub struct ZigZag;

impl ReadFieldCodec<i32> for ZigZag {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i32> {
        let u: u32 = Varint::read_from(buf)?;
        Ok(codec::unzigzag32(u))
    }
}

impl WriteFieldCodec<i32> for ZigZag {
    #[inline]
    fn write_to<B: BufMut>(val: i32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = codec::zigzag32(val);
        Varint::write_to(u, buf)
    }

    #[inline]
    fn size(val: i32) -> u64 {
        let u = codec::zigzag32(val);
        Varint::size(u)
    }
}

impl ReadFieldCodec<i64> for ZigZag {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<i64> {
        let u: u64 = Varint::read_from(buf)?;
        Ok(codec::unzigzag64(u))
    }
}

impl WriteFieldCodec<i64> for ZigZag {
    #[inline]
    fn write_to<B: BufMut>(val: i64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let u = codec::zigzag64(val);
        Varint::write_to(u, buf)
    }

    #[inline]
    fn size(val: i64) -> u64 {
        let u = codec::zigzag64(val);
        Varint::size(u)
    }
}

impl ReadFieldCodec<u32> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<u32> {
        let v: u64 = Varint::read_from(buf)?;
        Ok(v as u32)
    }
}

impl WriteFieldCodec<u32> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: u32, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val as u64, buf)
    }

    #[inline]
    fn size(val: u32) -> u64 {
        Varint::size(val as u64)
    }
}

impl ReadFieldCodec<u64> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<u64> {
        buf.read_var_u64_raw()
    }
}

impl WriteFieldCodec<u64> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: u64, buf: &mut CodedOutputStream<B>) -> Result<()> {
        buf.write_var_u64_raw(val)
    }

    fn size(mut val: u64) -> u64 {
        for i in 1..=9 {
            val >>= 7;
            if val == 0 {
                return i;
            }
        }
        10
    }
}

impl<E: Enumerate> ReadFieldCodec<E> for Varint {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<E> {
        let val: i32 = Varint::read_from(buf)?;
        Ok(E::from_value(val))
    }
}

impl<E: Enumerate> WriteFieldCodec<E> for Varint {
    #[inline]
    fn write_to<B: BufMut>(val: E, buf: &mut CodedOutputStream<B>) -> Result<()> {
        Varint::write_to(val.value(), buf)
    }

    #[inline]
    fn size(val: E) -> u64 {
        Varint::size(val.value())
    }
}

impl<M: Message> MergeFieldCodec<M> for LengthPrefixed {
    fn merge_from<B: Buf>(current: &mut M, buf: &mut CodedInputStream<B>) -> Result<()> {
        buf.read_length_limited(|buf| current.merge_from(buf))
    }
}

impl<M: Message + Default> ReadFieldCodec<M> for LengthPrefixed {
    #[inline]
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<M> {
        let mut m = Default::default();
        LengthPrefixed::merge_from(&mut m, buf)?;
        Ok(m)
    }
}

impl<'a, M: Message> WriteFieldCodec<&'a M> for LengthPrefixed {
    #[inline]
    fn write_to<B: BufMut>(val: &M, buf: &mut CodedOutputStream<B>) -> Result<()> {
        let l = val.size();
        Varint::write_to(l, buf)?;
        val.write_to(buf)
    }

    #[inline]
    fn size(val: &M) -> u64 {
        let l = val.size();
        Varint::size(l) + l
    }
}

pub struct RefArray<Codec> {
    _marker: PhantomData<Codec>,
}

impl<Item, Codec> MergeFieldCodec<Vec<Item>> for RefArray<Codec>
where
    Codec: ReadFieldCodec<Item>,
{
    fn merge_from<B: Buf>(current: &mut Vec<Item>, buf: &mut CodedInputStream<B>) -> Result<()> {
        current.push(Codec::read_from(buf)?);
        Ok(())
    }
}

impl<'a, Item, Codec> WriteFieldCodec<&'a [Item]> for RefArray<Codec>
where
    Codec: WriteFieldCodec<&'a Item>,
{
    fn write_to<B: BufMut>(_: &'a [Item], _: &mut CodedOutputStream<B>) -> Result<()> {
        unimplemented!("Writting repeated string/bytes/message requires tag");
    }

    #[inline]
    fn size(val: &'a [Item]) -> u64 {
        val.iter().map(Codec::size).sum()
    }
}

pub struct CopyArray<Codec> {
    _marker: PhantomData<Codec>,
}

impl<Item, Codec> MergeFieldCodec<Vec<Item>> for CopyArray<Codec>
where
    Codec: ReadFieldCodec<Item>,
{
    fn merge_from<B: Buf>(current: &mut Vec<Item>, buf: &mut CodedInputStream<B>) -> Result<()> {
        current.push(Codec::read_from(buf)?);
        Ok(())
    }
}

impl<'a, Item, Codec> WriteFieldCodec<&'a [Item]> for CopyArray<Codec>
where
    Item: Copy,
    Codec: WriteFieldCodec<Item>,
{
    fn write_to<B: BufMut>(_: &'a [Item], _: &mut CodedOutputStream<B>) -> Result<()> {
        unimplemented!("Writting repeated primitive requires tag");
    }

    #[inline]
    fn size(val: &'a [Item]) -> u64 {
        val.iter().map(|i| Codec::size(*i)).sum()
    }
}

pub struct PackedArray<ItemCodec> {
    _marker: PhantomData<ItemCodec>,
}

impl<Item, ItemCodec> MergeFieldCodec<Vec<Item>> for PackedArray<ItemCodec>
where
    ItemCodec: ReadFieldCodec<Item>,
{
    fn merge_from<B: Buf>(current: &mut Vec<Item>, buf: &mut CodedInputStream<B>) -> Result<()> {
        let len: u64 = Varint::read_from(buf)?;
        let limit = buf.progress() + len;
        while buf.progress() < limit {
            current.push(ItemCodec::read_from(buf)?);
        }
        if buf.progress() == limit {
            Ok(())
        } else {
            Err(Error::corrupted())
        }
    }
}

impl<Item, ItemCodec> ReadFieldCodec<Vec<Item>> for PackedArray<ItemCodec>
where
    ItemCodec: ReadFieldCodec<Item>,
{
    fn read_from<B: Buf>(buf: &mut CodedInputStream<B>) -> Result<Vec<Item>> {
        let mut res = Vec::new();
        Self::merge_from(&mut res, buf)?;
        Ok(res)
    }
}

impl<'a, Item, ItemCodec> WriteFieldCodec<&'a [Item]> for PackedArray<ItemCodec>
where
    Item: Copy,
    ItemCodec: WriteFieldCodec<Item>,
{
    fn write_to<B: BufMut>(val: &'a [Item], buf: &mut CodedOutputStream<B>) -> Result<()> {
        let l: u64 = val.iter().map(|v| ItemCodec::size(*v)).sum();
        Varint::write_to(l, buf)?;
        for v in val {
            ItemCodec::write_to(*v, buf)?;
        }
        Ok(())
    }

    fn size(val: &'a [Item]) -> u64 {
        let l: u64 = val.iter().map(|v| ItemCodec::size(*v)).sum();
        Varint::size(l) + l
    }
}
