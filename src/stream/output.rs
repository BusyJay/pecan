use crate::enumerate::EnumType;
use crate::message::Message;
use crate::stream::BufMut;
use crate::{Error, Result};
use std::mem::MaybeUninit;
use std::ptr;

pub mod encoded {
    use crate::enumerate::EnumType;
    use crate::message::Message;
    use pecan_utils::codec;

    #[inline]
    pub fn var_u32_len(u: u32) -> usize {
        codec::varint_u32_bytes_len(u) as usize
    }

    #[inline]
    pub fn var_i32_len(i: i32) -> usize {
        var_u32_len(i as u32)
    }

    #[inline]
    pub fn var_u64_len(u: u64) -> usize {
        codec::varint_u64_bytes_len(u) as usize
    }

    #[inline]
    pub fn var_i64_len(i: i64) -> usize {
        var_u64_len(i as u64)
    }

    #[inline]
    pub fn bytes_len(s: &[u8]) -> usize {
        var_u32_len(s.len() as u32) + s.len()
    }

    #[inline]
    pub fn string_len(s: &str) -> usize {
        bytes_len(s.as_bytes())
    }

    #[inline]
    pub fn arr_var_u32_len(arr: &[u32]) -> usize {
        let sum: usize = arr.iter().map(|u| var_u32_len(*u)).sum();
        var_u32_len(sum as u32) + sum
    }

    #[inline]
    pub fn arr_var_i32_len(arr: &[i32]) -> usize {
        let sum: usize = arr.iter().map(|u| var_i32_len(*u)).sum();
        var_u32_len(sum as u32) + sum
    }

    #[inline]
    pub fn enum_len(e: impl EnumType) -> usize {
        var_i32_len(e.value())
    }

    #[inline]
    pub fn message_len(msg: &impl Message) -> usize {
        let l = msg.len();
        var_u32_len(l as u32) + l
    }

    #[inline]
    pub fn arr_message_len(tag_len: usize, arr: &[impl Message]) -> usize {
        arr.iter()
            .fold(tag_len * arr.len(), |n, s| n + message_len(s))
    }

    #[inline]
    pub fn arr_string_len(tag_len: usize, arr: &[String]) -> usize {
        arr.iter()
            .fold(tag_len * arr.len(), |n, s| n + string_len(s))
    }
}

pub struct CodedOutputStream<B> {
    buf: B,
    end: *mut u8,
    start: *mut u8,
    cur: *mut u8,
}

// Uses const array instead once rust-lang/rust#44580 is resolved.
macro_rules! write_n_bytes {
    ($func:ident, $n:expr) => {
        #[inline]
        pub fn $func(&mut self, arr: [u8; $n]) -> Result<()> {
            if self.remaining()? > $n {
                self.cur = unsafe {
                    arr.as_ptr().copy_to_nonoverlapping(self.cur, $n);
                    self.cur.add($n)
                };
                return Ok(());
            }
            self.write_raw_bytes(&arr)
        }
    };
}

impl<B: BufMut> CodedOutputStream<B> {
    #[inline]
    pub fn new(mut buf: B) -> CodedOutputStream<B> {
        let (start, end) = unsafe {
            let bytes = buf.bytes_mut();
            if !bytes.is_empty() {
                let start = bytes.as_mut_ptr() as *mut u8;
                (start, start.add(bytes.len()))
            } else {
                (ptr::null_mut(), ptr::null_mut())
            }
        };
        CodedOutputStream {
            buf,
            start,
            end,
            cur: start,
        }
    }

    #[inline]
    pub fn into_inner(mut self) -> Result<B> {
        self.flush()?;
        Ok(self.buf)
    }

    #[inline]
    fn remaining(&mut self) -> Result<usize> {
        if self.cur < self.end {
            Ok(self.end as usize - self.cur as usize)
        } else if self.cur.is_null() {
            Err(Error::OutOfSpace)
        } else {
            self.flush()?;
            self.remaining()
        }
    }

    fn flush(&mut self) -> Result<()> {
        if self.cur == self.start {
            return Ok(());
        }
        unsafe {
            self.buf
                .advance_mut(self.cur as usize - self.start as usize);
        }
        let bytes = self.buf.bytes_mut();
        if !bytes.is_empty() {
            self.start = bytes.as_mut_ptr() as *mut u8;
            self.end = unsafe { self.start.add(bytes.len()) };
            self.cur = self.start;
            return Ok(());
        }

        self.cur = ptr::null_mut();
        self.end = ptr::null_mut();
        self.start = ptr::null_mut();
        Err(Error::OutOfSpace)
    }

    write_n_bytes!(write_raw_1_byte, 1);
    write_n_bytes!(write_raw_2_byte, 2);
    write_n_bytes!(write_raw_3_byte, 3);
    write_n_bytes!(write_raw_4_byte, 4);
    write_n_bytes!(write_raw_5_byte, 5);
    write_n_bytes!(write_raw_6_byte, 6);
    write_n_bytes!(write_raw_7_byte, 7);
    write_n_bytes!(write_raw_8_byte, 8);
    write_n_bytes!(write_raw_9_byte, 9);
    write_n_bytes!(write_raw_10_byte, 10);

    #[inline]
    fn write_raw_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let mut remaining = bytes.len();
        let mut cur = bytes.as_ptr();
        loop {
            let buf_remaining = self.remaining()?;
            if buf_remaining >= remaining {
                unsafe {
                    cur.copy_to_nonoverlapping(self.cur, remaining);
                    self.cur = self.cur.add(remaining);
                }
                return Ok(());
            }
            unsafe {
                cur.copy_to_nonoverlapping(self.cur, buf_remaining);
                cur = cur.add(buf_remaining);
                self.cur = self.cur.add(buf_remaining);
                remaining -= buf_remaining;
            }
            self.flush()?;
        }
    }

    pub fn write_var_u32(&mut self, mut n: u32) -> Result<()> {
        if n < 0x80 {
            return self.write_raw_1_byte([n as u8]);
        }
        let b1 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_2_byte([b1, n as u8]);
        }
        let b2 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_3_byte([b1, b2, n as u8]);
        }
        let b3 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_4_byte([b1, b2, b3, n as u8]);
        }
        let b4 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        self.write_raw_5_byte([b1, b2, b3, b4, n as u8])
    }

    pub fn write_var_i32_array(&mut self, arr: &[i32]) -> Result<()> {
        let l: usize = arr.iter().map(|i| encoded::var_i32_len(*i)).sum();
        self.write_var_u32(l as u32)?;
        for i in arr {
            self.write_var_i32(*i)?;
        }
        Ok(())
    }

    pub fn write_var_u64(&mut self, mut n: u64) -> Result<()> {
        if n < 0x80 {
            return self.write_raw_1_byte([n as u8]);
        }
        let b1 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_2_byte([b1, n as u8]);
        }
        let b2 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_3_byte([b1, b2, n as u8]);
        }
        let b3 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_4_byte([b1, b2, b3, n as u8]);
        }
        let b4 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_5_byte([b1, b2, b3, b4, n as u8]);
        }
        let b5 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_6_byte([b1, b2, b3, b4, b5, n as u8]);
        }
        let b6 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_7_byte([b1, b2, b3, b4, b5, b6, n as u8]);
        }
        let b7 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_8_byte([b1, b2, b3, b4, b5, b6, b7, n as u8]);
        }
        let b8 = (n as u8 & 0x7f) | 0x80;
        n >>= 7;
        if n < 0x80 {
            return self.write_raw_9_byte([b1, b2, b3, b4, b5, b6, b7, b8, n as u8]);
        }
        let b9 = (n as u8) & 0x7f | 0x80;
        n >>= 7;
        self.write_raw_10_byte([b1, b2, b3, b4, b5, b6, b7, b8, b9, n as u8])
    }

    #[inline]
    pub fn write_var_i64(&mut self, i: i64) -> Result<()> {
        self.write_var_u64(i as u64)
    }

    #[inline]
    pub fn write_string(&mut self, s: &str) -> Result<()> {
        self.write_bytes(s.as_bytes())
    }

    #[inline]
    pub fn write_bytes(&mut self, s: &[u8]) -> Result<()> {
        let len = s.len();
        self.write_var_u32(len as u32)?;
        self.write_raw_bytes(s)
    }

    #[inline]
    pub fn write_var_i32(&mut self, i: i32) -> Result<()> {
        self.write_var_u32(i as u32)
    }

    #[inline]
    pub fn write_enum(&mut self, e: impl EnumType) -> Result<()> {
        self.write_var_i32(e.value())
    }

    #[inline]
    pub fn write_fixed_u64(&mut self, u: u64) -> Result<()> {
        let bytes = u64::to_le_bytes(u);
        self.write_raw_8_byte(bytes)
    }

    #[inline]
    pub fn write_f64(&mut self, f: f64) -> Result<()> {
        let u = f64::to_bits(f);
        self.write_fixed_u64(u)
    }

    #[inline]
    pub fn write_bool(&mut self, b: bool) -> Result<()> {
        self.write_raw_1_byte([if b { 1 } else { 0 }])
    }

    #[inline]
    pub fn write_unknown(&mut self, unknown: &[u8]) -> Result<()> {
        self.write_raw_bytes(unknown)
    }

    #[inline]
    pub fn write_message(&mut self, msg: &impl Message) -> Result<()> {
        let l = msg.len();
        self.write_var_u32(l as u32)?;
        msg.write_to(self)
    }
}

impl<B: BufMut> BufMut for CodedOutputStream<B> {
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.advance_mut(cnt)
    }

    #[inline]
    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.buf.bytes_mut()
    }

    #[inline]
    fn reserve(&mut self, addition: usize) {
        self.buf.reserve(addition)
    }
}
