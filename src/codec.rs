
use std::{u64, mem, slice, ptr, str};

use super::{Error, Result};

macro_rules! slow_var_decode {
    ($t:ty, $fallback_func:ident, $max:expr, $last:expr, $slow_func:ident) => {
        fn $slow_func(data: &mut &[u8]) -> Result<$t> {
            let mut res = 0;
            for i in 0..data.len() {
                let b = data[i];
                res |= (b as $t & 0x7f) << (i * 7);
                if b < 0x80 {
                    *data = unsafe { data.get_unchecked(i + 1..) };
                    return Ok(res);
                }
            }
            Err(Error::unexpected_eof())
        }

        fn $fallback_func(data: &mut &[u8]) -> Result<$t> {
            if data.len() >= $max || *data.last().unwrap() < 0x80 {
                let mut res = 0;
                for i in 0..$max - 1 {
                    let b = unsafe { *data.get_unchecked(i) };
                    res |= (b & 0x7f) as $t << (i * 7);
                    if b < 0x80 {
                        *data = unsafe { data.get_unchecked(i + 1..) };
                        return Ok(res);
                    }
                }
                let b = unsafe { *data.get_unchecked($max - 1) };
                if b <= $last {
                    res |= b as $t << (($max - 1) * 7);
                    *data = unsafe { data.get_unchecked($max..) };
                    return Ok(res);
                }
                return Err(Error::invalid_data());
            }

            $slow_func(data)
        }
    }
}

slow_var_decode!(u32, decode_varint_u32_fallback, 5, 15, decode_varint_u32_slow);
slow_var_decode!(u64, decode_varint_u64_fallback, 10, 1, decode_varint_u64_slow);

#[inline]
pub fn decode_varint_s32(data: &mut &[u8]) -> Result<i32> {
    let n = decode_varint_u32(data)?;
    Ok(unzig_zag_32(n))
}

#[inline]
pub fn decode_varint_i32(data: &mut &[u8]) -> Result<i32> {
    let n = decode_varint_u32(data)?;
    Ok(n as i32)
}

#[inline]
pub fn decode_varint_u32(data: &mut &[u8]) -> Result<u32> {
    if !data.is_empty() {
        // process with value < 127 independently at first
        // since it matches most of the cases.
        if data[0] < 0x80 {
            let res = u32::from(data[0]);
            *data = unsafe { data.get_unchecked(1..) };
            return Ok(res);
        }

        return decode_varint_u32_fallback(data);
    }

    Err(Error::unexpected_eof())
}

#[inline]
pub fn decode_varint_s64(data: &mut &[u8]) -> Result<i64> {
    let n = decode_varint_u64(data)?;
    Ok(unzig_zag_64(n))
}

#[inline]
pub fn decode_varint_i64(data: &mut &[u8]) -> Result<i64> {
    let n = decode_varint_u64(data)?;
    Ok(n as i64)
}

#[inline]
pub fn decode_varint_u64(data: &mut &[u8]) -> Result<u64> {
    if !data.is_empty() {
        // process with value < 127 independently at first
        // since it matches most of the cases.
        if data[0] < 0x80 {
            let res = u64::from(data[0]);
            *data = unsafe { data.get_unchecked(1..) };
            return Ok(res);
        }

        return decode_varint_u64_fallback(data);
    }

    Err(Error::unexpected_eof())
}

macro_rules! var_encode {
    ($t:ty, $arr_func:ident, $enc_func:ident, $max:expr) => {
        unsafe fn $arr_func(start: *mut u8, mut n: $t) -> usize {
            let mut end = start;
            while n >= 0x80 {
                ptr::write(end, 0x80 | (n as u8 & 0x7f));
                n >>= 7;
                end = end.offset(1);
            }
            ptr::write(end, n as u8);
            end.offset_from(start) as usize + 1
        }
        
        pub fn $enc_func(data: &mut Vec<u8>, n: $t) {
            let left = data.capacity() - data.len();
            if left >= $max {
                let old_len = data.len();
                unsafe {
                    let start = data.as_mut_ptr().offset(old_len as isize);
                    let len = $arr_func(start, n);
                    data.set_len(old_len + len);
                }
            }
            
            let mut buf = [0; $max];
            
            unsafe {
                let start = buf.as_mut_ptr();
                let len = $arr_func(start, n);
                let written = slice::from_raw_parts(start, len);
                data.extend_from_slice(written);
            }
        }
    }
}

var_encode!(u32, encode_varint_u32_to_array, encode_varint_u32, 5);
var_encode!(u64, encode_varint_u64_to_array, encode_varint_u64, 10);

#[inline]
pub fn encode_varint_s32(data: &mut Vec<u8>, n: i32) {
    let n = zig_zag_32(n);
    encode_varint_u32(data, n);
}

#[inline]
pub fn encode_varint_i32(data: &mut Vec<u8>, n: i32) {
    let n = n as u32;
    encode_varint_u32(data, n);
}

#[inline]
pub fn encode_varint_s64(data: &mut Vec<u8>, n: i64) {
    let n = zig_zag_64(n);
    encode_varint_u64(data, n);
}

#[inline]
pub fn encode_varint_i64(data: &mut Vec<u8>, n: i64) {
    let n = n as u64;
    encode_varint_u64(data, n);
}

#[inline]
pub fn varint_u32_bytes_len(n: u32) -> u32 {
    ((32 - n.leading_zeros()) / 7 + 1)
}

#[inline]
pub fn varint_i32_bytes_len(i: i32) -> u32 {
    varint_u32_bytes_len(i as u32)
}

#[inline]
pub fn varint_s32_bytes_len(n: i32) -> u32 {
    varint_u32_bytes_len(zig_zag_32(n))
}

#[inline]
pub fn varint_u64_bytes_len(n: u64) -> u32 {
    ((64 - n.leading_zeros()) / 7 + 1) as u32
}

#[inline]
pub fn varint_i64_bytes_len(i: i64) -> u32 {
    varint_u64_bytes_len(i as u64)
}

#[inline]
pub fn varint_s64_bytes_len(n: i64) -> u32 {
    varint_u64_bytes_len(zig_zag_64(n))
}

#[inline]
pub fn fixed_i64_bytes_len(_: i64) -> u32 {
    8
}

#[inline]
pub fn fixed_s64_bytes_len(_: i64) -> u32 {
    8
}

#[inline]
pub fn fixed_i32_bytes_len(_: i32) -> u32 {
    4
}

#[inline]
pub fn fixed_s32_bytes_len(_: i32) -> u32 {
    4
}

#[inline]
pub fn f32_bytes_len(_: f32) -> u32 {
    4
}

#[inline]
pub fn f64_bytes_len(_: f64) -> u32 {
    8
}

#[inline]
fn zig_zag_64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[inline]
fn unzig_zag_64(n: u64) -> i64 {
    ((n >> 1) ^ (!(n & 1)).wrapping_add(1)) as i64
}

#[inline]
fn zig_zag_32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

#[inline]
fn unzig_zag_32(n: u32) -> i32 {
    ((n >> 1) ^ (!(n & 1)).wrapping_add(1)) as i32
}

#[inline]
pub fn encode_tag(field_number: u32, wired_id: u32) -> u32 {
    (field_number << 3) | wired_id
}

macro_rules! encode_fixed {
    ($t:ty, $ti:ident, $len:expr, $enc_func:ident, $dec_func:ident) => {
        #[inline]
        pub fn $enc_func(data: &mut Vec<u8>, n: $t) {
            let n = n.to_le();
            let b: [u8; $len] = unsafe { mem::transmute(n) };
            data.extend_from_slice(&b)
        }

        #[inline]
        pub fn $dec_func(data: &[u8]) -> Result<$t> {
            let mut b: $t = 0;
            if data.len() >= $len {
                unsafe {
                    let buf: [u8; $len] = mem::transmute(b);
                    let mut buf = &mut b as *mut $t as *mut u8;
                    let mut data = data.as_ptr();
                    for i in 0..$len {
                        buf.write(data.read());
                        buf = buf.offset(1);
                        data = data.offset(1);
                    }
                }
                Ok($ti::from_le(b))
            } else {
                Err(Error::invalid_data())
            }
        }
    }
}

encode_fixed!(i32, i32, 4, encode_fixed_i32, decode_fixed_i32);
encode_fixed!(i64, i64, 8, encode_fixed_i64, decode_fixed_i64);

#[inline]
pub fn encode_fixed_f32(data: &mut Vec<u8>, n: f32) {
    let n = n.to_bits();
    encode_fixed_i32(data, n as i32)
}

#[inline]
pub fn decode_fixed_f32(data: &[u8]) -> Result<f32> {
    decode_fixed_i32(data).map(|n| f32::from_bits(n as u32))
}

#[inline]
pub fn encode_fixed_f64(data: &mut Vec<u8>, n: f64) {
    let n = n.to_bits();
    encode_fixed_i64(data, n as i64)
}

#[inline]
pub fn decode_fixed_f64(data: &[u8]) -> Result<f64> {
    decode_fixed_i64(data).map(|n| f64::from_bits(n as u64))
}

#[inline]
pub fn encode_fixed_s32(data: &mut Vec<u8>, n: i32) {
    encode_fixed_i32(data, zig_zag_32(n) as i32)
}

#[inline]
pub fn decode_fixed_s32(data: &[u8]) -> Result<i32> {
    decode_fixed_i32(data).map(|i| unzig_zag_32(i as u32))
}

#[inline]
pub fn encode_fixed_s64(data: &mut Vec<u8>, n: i64) {
    encode_fixed_i64(data, zig_zag_64(n) as i64)
}

#[inline]
pub fn decode_fixed_s64(data: &[u8]) -> Result<i64> {
    decode_fixed_i64(data).map(|i| unzig_zag_64(i as u64))
}

#[inline]
pub fn encode_bytes(data: &mut Vec<u8>, bytes: &[u8]) {
    encode_varint_u32(data, bytes.len() as u32);
    data.extend_from_slice(bytes);
}

#[inline]
pub fn decode_bytes<'a>(data: &mut &'a [u8]) -> Result<&'a [u8]> {
    let len = decode_varint_u32(data)? as usize;
    let (res, left) = if data.len() > len {
        data.split_at(len)
    } else {
        return Err(Error::unexpected_eof());
    };
    *data = left;
    Ok(res)
}

macro_rules! wired {
    ($t:ty, $wired_id:expr, $encode:ident, $func:ident) => {
        #[inline]
        pub fn $func(buf: &mut Vec<u8>, field_number: u32, val: $t) {
            let tag = encode_tag(field_number, $wired_id);
            encode_varint_u32(buf, tag);
            $encode(buf, val);
        }
    }
}

wired!(u64, 0, encode_varint_u64, wired_varint_u64);
wired!(u32, 0, encode_varint_u32, wired_varint_u32);
wired!(i64, 0, encode_varint_i64, wired_varint_i64);
wired!(i32, 0, encode_varint_i32, wired_varint_i32);
wired!(i64, 0, encode_varint_s64, wired_varint_s64);
wired!(i32, 0, encode_varint_s32, wired_varint_s32);
wired!(i32, 5, encode_fixed_i32, wired_fixed_i32);
wired!(i64, 1, encode_fixed_i64, wired_fixed_i64);
wired!(i32, 5, encode_fixed_s32, wired_fixed_s32);
wired!(i64, 1, encode_fixed_s64, wired_fixed_s64);
wired!(f32, 5, encode_fixed_f32, wired_fixed_f32);
wired!(f64, 1, encode_fixed_f64, wired_fixed_f64);

#[inline]
pub fn decode_wired(data: &mut &[u8], number: &mut u32, wired_id: &mut u8) -> Result<()> {
    let n = decode_varint_u32(data)?;
    *number = n >> 3;
    *wired_id = n as u8 & 0x07;
    Ok(())
}

macro_rules! decode_wired_type {
    ($func:ident, $wired_id:expr, $result:ty, $work:ident) => {
        #[inline]
        pub fn $func(data: &mut &[u8], number: u32, wired_id: u8) -> $result {
            if wired_id == $wired_id {
                $work(data)
            } else {
                Err(Error::invalid_wired(number, $wired_id, wired_id))
            }
        }
    };
}

decode_wired_type!(decode_wired_varint_u64, 0, Result<u64>, decode_varint_u64);
decode_wired_type!(deocde_wired_varint_u32, 0, Result<u32>, decode_varint_u32);
decode_wired_type!(deocde_wired_varint_i64, 0, Result<i64>, decode_varint_i64);
decode_wired_type!(deocde_wired_varint_i32, 0, Result<i32>, decode_varint_i32);
decode_wired_type!(deocde_wired_varint_s64, 0, Result<i64>, decode_varint_s64);
decode_wired_type!(deocde_wired_varint_s32, 0, Result<i32>, decode_varint_s32);
decode_wired_type!(deocde_wired_fixed_i32, 5, Result<i32>, decode_fixed_i32);
decode_wired_type!(deocde_wired_fixed_i64, 1, Result<i64>, decode_fixed_i64);
decode_wired_type!(deocde_wired_fixed_s32, 5, Result<i32>, decode_fixed_s32);
decode_wired_type!(deocde_wired_fixed_s64, 1, Result<i64>, decode_fixed_s64);
decode_wired_type!(deocde_wired_fixed_f32, 5, Result<f32>, decode_fixed_f32);
decode_wired_type!(deocde_wired_fixed_f64, 1, Result<f64>, decode_fixed_f64);

#[inline]
pub fn wired_bool(buf: &mut Vec<u8>, field_number: u32, val: bool) {
    let tag = encode_tag(field_number, 0);
    encode_varint_u32(buf, tag);
    buf.push(val as u8);
}

#[inline]
pub fn decode_wired_bool(data: &mut &[u8], number: u32, wired_id: u8) -> Result<bool> {
    if wired_id == 0 {
        let b = decode_varint_u32(data)?;
        if b == 1 {
            Ok(true)
        } else if b == 0 {
            Ok(false)
        } else {
            Err(Error::invalid_data())
        }
    } else {
        Err(Error::invalid_wired(number, 0, wired_id))
    }
}

#[inline]
pub fn wired_bytes(buf: &mut Vec<u8>, field_number: u32, val: &[u8]) {
    let tag = encode_tag(field_number, 2);
    encode_varint_u32(buf, tag);
    encode_bytes(buf, val);
}

#[inline]
pub fn wired_str(buf: &mut Vec<u8>, field_number: u32, val: &str) {
    wired_bytes(buf, field_number, val.as_bytes())
}

#[inline]
pub fn decode_wired_bytes<'a>(data: &mut &'a [u8], number: u32, wired_id: u8) -> Result<&'a [u8]> {
    if wired_id == 2 {
        decode_bytes(data)
    } else {
        Err(Error::invalid_wired(number, 2, wired_id))
    }
}

#[inline]
pub fn decode_wired_str<'a>(data: &mut &'a [u8], number: u32, wired_id: u8) -> Result<&'a str> {
    let bytes = decode_wired_bytes(data, number, wired_id)?;
    match str::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::invalid_data()),
    }
}

#[inline]
pub fn bytes_len(bytes: &[u8]) -> u32 {
    varint_u32_bytes_len(bytes.len() as u32) + bytes.len() as u32
}

#[inline]
pub fn str_bytes_len(s: &str) -> u32 {
    bytes_len(s.as_bytes())
}

pub fn decode_unknown_tag<'a>(buf: &mut &'a [u8], wired_id: u8) -> Result<()> {
    let cnt = match wired_id {
        0 => {
            let mut i = 0;
            while i < buf.len() {
                if buf[i] & 0x80 == 0 {
                    i += 1;
                    break;
                }
                i += 1;
            }
            i
        }
        1 => 8,
        2 => decode_varint_u32(buf)? as usize,
        5 => 4,
        n => return Err(Error::invalid_wired(0, 0, n)),
    };
    if buf.len() >= cnt {
        *buf = unsafe { buf.get_unchecked(cnt..) };
        Ok(())
    } else {
        Err(Error::invalid_data())
    }
}

macro_rules! check_return_early {
    ($data:ident, $number:ident, $wired_id:ident) => {
        if !$data.is_empty() {
            ::codec::decode_wired(data, &mut $number, &mut $wired_id)?;
        } else {
            return Ok(());
        }
    };
}

#[cfg(test)]
mod test {
    use std::{u32, i64};
    use super::*;

    macro_rules! make_test {
        ($test_name:ident, $cases:expr, $enc:ident, $dec:ident, $len:ident) => (
            #[test]
            fn $test_name() {
                let cases = $cases;

                for (bin, n) in cases {
                    let mut output = vec![];
                    $enc(&mut output, n);
                    assert_eq!(output, bin, "encode for {}", n);
                    assert_eq!($len(n), bin.len() as u32);
                    let res = $dec(&mut output.as_slice()).unwrap();
                    assert_eq!(res, n, "decode for {}", n);
                }
            }
        )
    }

    make_test!(test_varint_i64, vec![
        (vec![1], 1),
        (vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1], -1),
        (vec![2], 2),
        (vec![254, 255, 255, 255, 255, 255, 255, 255, 255, 1], -2),
        (vec![0], 0),
        (vec![128, 1], 128),
        (vec![150, 1], 150),
    ], encode_varint_i64, decode_varint_i64, varint_i64_bytes_len);

    make_test!(test_varint_s64, vec![
        (vec![2], 1),
        (vec![1], -1),
        (vec![4], 2),
        (vec![3], -2),
        (vec![0], 0),
        (vec![128, 2], 128),
        (vec![255, 1], -128),
        (vec![172, 2], 150),
        (vec![254, 255, 255, 255, 15], 2147483647),
        (vec![255, 255, 255, 255, 15], -2147483648),
        (vec![254, 255, 255, 255, 255, 255, 255, 255, 255, 1], 9223372036854775807),
        (vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1], -9223372036854775808),
    ], encode_varint_s64, decode_varint_s64, varint_s64_bytes_len);

    make_test!(test_varint_u64, vec![
        (vec![1], 1),
        (vec![150, 1], 150),
        (vec![255, 255, 255, 255, 15], u32::MAX as u64),
        (vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1], u64::MAX),
    ], encode_varint_u64, decode_varint_u64, varint_u64_bytes_len);

    make_test!(test_varint_u32, vec![
        (vec![1], 1),
        (vec![150, 1], 150),
        (vec![255, 255, 255, 255, 15], u32::MAX),
    ], encode_varint_u32, decode_varint_u32, varint_u32_bytes_len);

    make_test!(test_varint_i32, vec![
        (vec![1], 1),
        (vec![255, 255, 255, 255, 15], -1),
        (vec![254, 255, 255, 255, 15], -2),
        (vec![0], 0),
        (vec![128, 1], 128),
        (vec![150, 1], 150),
    ], encode_varint_i32, decode_varint_i32, varint_i32_bytes_len);

    make_test!(test_varint_s32, vec![
        (vec![2], 1),
        (vec![1], -1),
        (vec![4], 2),
        (vec![3], -2),
        (vec![0], 0),
        (vec![128, 2], 128),
        (vec![255, 1], -128),
        (vec![172, 2], 150),
        (vec![254, 255, 255, 255, 15], 2147483647),
        (vec![255, 255, 255, 255, 15], -2147483648),
    ], encode_varint_s32, decode_varint_s32, varint_s32_bytes_len);

    make_test!(test_fixed_i32, vec![
        (vec![1, 0, 0, 0], 1),
        (vec![255, 255, 255, 255], -1),
        (vec![2, 0, 0, 0], 2),
        (vec![254, 255, 255, 255], -2),
        (vec![0, 0, 0, 0], 0),
        (vec![128, 0, 0, 0], 128),
        (vec![128, 255, 255, 255], -128),
        (vec![150, 0, 0, 0], 150),
        (vec![255, 255, 255, 127], 2147483647),
        (vec![0, 0, 0, 128], -2147483648),
    ], encode_fixed_i32, decode_fixed_i32, fixed_i32_bytes_len);

    make_test!(test_fixed_i64, vec![
        (vec![1, 0, 0, 0, 0, 0, 0, 0], 1),
        (vec![255, 255, 255, 255, 255, 255, 255, 255], -1),
        (vec![2, 0, 0, 0, 0, 0, 0, 0], 2),
        (vec![254, 255, 255, 255, 255, 255, 255, 255], -2),
        (vec![0, 0, 0, 0, 0, 0, 0, 0], 0),
        (vec![128, 0, 0, 0, 0, 0, 0, 0], 128),
        (vec![128, 255, 255, 255, 255, 255, 255, 255], -128),
        (vec![150, 0, 0, 0, 0, 0, 0, 0], 150),
        (vec![255, 255, 255, 127, 0, 0, 0, 0], 2147483647),
        (vec![0, 0, 0, 128, 255, 255, 255, 255], -2147483648),
        (vec![255, 255, 255, 255, 255, 255, 255, 127], i64::MAX),
        (vec![0, 0, 0, 0, 0, 0, 0, 128], i64::MIN),
    ], encode_fixed_i64, decode_fixed_i64, fixed_i64_bytes_len);

    make_test!(test_f32, vec![
        (vec![0x00, 0x00, 0x80, 0x3f], 1f32),
        (vec![0x00, 0x00, 0x80, 0xbf], -1f32),
        (vec![0x00, 0x00, 0x00, 0x40], 2f32),
        (vec![0x00, 0x00, 0x00, 0xc0], -2f32),
        (vec![0x00, 0x00, 0x00, 0x00], 0f32),
        (vec![0x00, 0x00, 0x00, 0x43], 128f32),
        (vec![0x00, 0x00, 0x00, 0xc3], -128f32),
        (vec![0xc3, 0xf5, 0x48, 0x40], 3.14),
        (vec![0x00, 0x00, 0x00, 0x4f], 2147483647f32),
    ], encode_fixed_f32, decode_fixed_f32, f32_bytes_len);

    make_test!(test_f64, vec![
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f], 1f64),
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xbf], -1f64),
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40], 2f64),
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0], -2f64),
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0f64),
        (vec![0x1f, 0x85, 0xeb, 0x51, 0xb8, 0x1e, 0x09, 0x40], 3.14),
    ], encode_fixed_f64, decode_fixed_f64, f64_bytes_len);
}
