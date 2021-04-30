use core::slice;

use crate::{Error, Result};

macro_rules! us {
    ($bytes:ident, $off:expr) => {
        unsafe { $bytes.get_unchecked($off) }
    };
}

pub fn decode_var_u64(bytes: &mut &[u8]) -> Result<u64> {
    if !bytes.is_empty() {
        if bytes[0] <= 0x7f {
            let b = bytes[0] as u64;
            *bytes = us!(bytes, 1..);
            return Ok(b);
        }
        let mut res = bytes[0] as u64 & 0x7f;
        let mut c: usize = 1;
        if bytes[bytes.len() - 1] <= 0x7f || bytes.len() <= 10 {
            for b in &bytes[1..] {
                res |= (0x7f & *b as u64) << (7 * c);
                c += 1;
                if *b <= 0x7f {
                    if c <= 9 || c == 10 && *b == 1 {
                        *bytes = us!(bytes, c..);
                        return Ok(res);
                    } else {
                        return Err(Error::corrupted());
                    }
                }
            }
            if c < 10 {
                return Err(Error::WantMore(res, c as u8));
            } else {
                return Err(Error::corrupted());
            }
        } else {
            while c < 9 {
                let b = *us!(bytes, c);
                res |= (0x7f & b as u64) << (7 * c);
                c += 1;
                if b <= 0x7f {
                    *bytes = us!(bytes, c..);
                    return Ok(res);
                }
            }
            if *us!(bytes, 9) == 1 {
                *bytes = us!(bytes, 10..);
                return Ok((1 << 63) | res);
            } else {
                return Err(Error::corrupted());
            }
        }
    }
    Err(Error::WantMore(0, 0))
}

pub fn decode_var_u64_resume(bytes: &mut &[u8], mut res: u64, lp: u8) -> Result<u64> {
    let mut off = 0;
    while off + (lp as usize) < 9 {
        let cur = off as u8 + lp;
        if off < bytes.len() {
            let b = bytes[off];
            res |= (0x7f & b as u64) << (7 * cur);
            off += 1;
            if b <= 0x7f {
                *bytes = us!(bytes, off..);
                return Ok(res);
            }
        } else {
            return Err(Error::WantMore(res, cur));
        }
    }
    if bytes.len() > off {
        if bytes[off] == 1 {
            *bytes = us!(bytes, off + 1..);
            return Ok((1 << 63) | res);
        } else {
            return Err(Error::corrupted());
        }
    }
    Err(Error::WantMore(res, off as u8 + lp))
}

#[inline]
pub fn encode_var_u64(buf: &mut &mut [u8], mut val: u64) -> Result<()> {
    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    for i in 0..std::cmp::min(len, 11) {
        if val < 0x80 {
            unsafe {
                ptr.add(i).write(val as u8);
                *buf = slice::from_raw_parts_mut(ptr.add(i + 1), len - i - 1);
            }
            return Ok(());
        } else {
            unsafe {
                ptr.add(i).write((val as u8) | 0x80);
            }
        }
        val >>= 7;
    }
    Err(Error::WantMore(val, len as u8))
}

#[inline]
pub fn zigzag32(v: i32) -> u32 {
    ((v << 1) ^ (v >> 31)) as u32
}

#[inline]
pub fn zigzag64(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

#[inline]
pub fn unzigzag32(v: u32) -> i32 {
    ((v >> 1) as i32) ^ -(v as i32 & 1)
}

#[inline]
pub fn unzigzag64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -(v as i64 & 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{i32, i64, u32, u64};

    #[test]
    fn test_normal() {
        let mut buf = [0; 11];
        for i in &[
            0,
            7,
            127,
            128,
            256,
            u32::MAX as u64,
            i32::MAX as u64,
            u64::MAX,
            i64::MAX as u64,
        ] {
            let mut data: &mut [u8] = &mut buf;
            encode_var_u64(&mut data, *i).unwrap();
            let data_len = data.len();
            let wrote_len = buf.len() - data_len;
            let mut read = &buf[..];
            let res = decode_var_u64(&mut read).unwrap();
            assert_eq!(res, *i, "{:?}", buf);
            let read_len = buf.len() - read.len();
            assert_eq!(wrote_len, read_len, "{}", i);
        }
    }

    #[test]
    fn test_resume() {
        for i in &[
            0,
            7,
            127,
            128,
            256,
            u32::MAX as u64,
            i32::MAX as u64,
            u64::MAX,
            i64::MAX as u64,
        ] {
            let mut buf = [0; 11];
            let mut data: &mut [u8] = &mut buf;
            encode_var_u64(&mut data, *i).unwrap();
            let total = 11 - data.len();
            for mut step in 0..11 {
                let mut buf = vec![];
                let mut val = *i;
                loop {
                    let mut b = vec![0; step];
                    let mut data = b.as_mut_slice();
                    match encode_var_u64(&mut data, val) {
                        Ok(()) => {
                            let read = step - data.len();
                            unsafe { b.set_len(read) }
                            buf.push(b);
                            break;
                        }
                        Err(Error::WantMore(v, t)) => {
                            assert_eq!(t as usize, step);
                            val = v;
                            buf.push(b);
                        }
                        Err(e) => panic!("{:?}", e),
                    }
                    step += 1;
                }
                assert_eq!(buf.iter().map(|b| b.len()).sum::<usize>(), total, "{}", i);
                let mut read = &*buf[0];
                match decode_var_u64(&mut read) {
                    Ok(n) => {
                        assert_eq!(n, *i);
                        assert_eq!(&buf[0].len() - read.len(), total);
                    }
                    Err(Error::WantMore(mut val, mut lp)) => {
                        assert_eq!(lp as usize, buf[0].len());
                        for i in 1..buf.len() - 1 {
                            let mut read = &*buf[i];
                            match decode_var_u64_resume(&mut read, val, lp) {
                                Err(Error::WantMore(v, l)) => {
                                    assert_eq!(l as usize - lp as usize, buf[i].len());
                                    val = v;
                                    lp = l;
                                }
                                res => panic!("{:?}", res),
                            }
                        }
                        let mut read = &*buf[buf.len() - 1];
                        let val = decode_var_u64_resume(&mut read, val, lp).unwrap();
                        assert_eq!(val, *i);
                        assert_eq!(lp as usize + buf[buf.len() - 1].len() - read.len(), total);
                    }
                    res => panic!("{:?}", res),
                }
            }
        }
    }

    #[test]
    fn test_zigzag() {
        let cases: &[(i32, u32)] = &[
            (0, 0),
            (-1, 1),
            (1, 2),
            (-2, 3),
            (2147483647, 4294967294),
            (-2147483648, 4294967295),
        ];
        for (i, exp) in cases {
            assert_eq!(super::zigzag32(*i), *exp);
            assert_eq!(super::unzigzag32(*exp), *i);
            assert_eq!(super::zigzag64(*i as i64), *exp as u64);
            assert_eq!(super::unzigzag64(*exp as u64), *i as i64);
        }
    }
}
