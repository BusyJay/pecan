
use std::{u64, mem};
use std::io::Write;

use super::Result;

macro_rules! make_encode_varint {
    ($name:ident, $t:ty) => (
        fn $name<W: Write>(mut i: $t, w: &mut W) -> Result<()> {
            while i >= 0x80 {
                try!(w.write_all(&[(i | 128) as u8]));
                i >>= 7;
            }
            w.write_all(&[(i & 127) as u8]).map_err(From::from)
        }
    )
}

make_encode_varint!(encode_varint64, u64);
make_encode_varint!(encode_varint32, u32);

pub trait WiredType {
    fn get_wired_id(&self) -> u8;

    fn encode_as<W: Write>(&self, field_id: u32, w: &mut W) -> Result<()> {
        let id_tag = (field_id << 3) | self.get_wired_id() as u32;
        try!(encode_varint32(id_tag, w));
        self.encode_to(w)
    }

    fn encode_to<W: Write>(&self, w: &mut W) -> Result<()>;
}

macro_rules! wired_varint {
    ($t:ty, $at:ty, $func:ident) => (
        impl WiredType for $t {
            fn get_wired_id(&self) -> u8 {
                0
            }

            fn encode_to<W: Write>(&self, w: &mut W) -> Result<()> {
                $func(*self as $at, w)
            }
        }
    )
}

wired_varint!(u64, u64, encode_varint64);
wired_varint!(u32, u32, encode_varint32);
wired_varint!(i64, u64, encode_varint64);
wired_varint!(i32, u32, encode_varint32);
wired_varint!(bool, u32, encode_varint32);

macro_rules! wired_sint {
    ($t:ty, $at:ty, $off:expr, $func:ident) => (
        impl WiredType for $t {
            fn get_wired_id(&self) -> u8 {
                0
            }

            fn encode_to<W: Write>(&self, w: &mut W) -> Result<()> {
                let i = ((self.0 << 1) ^ (self.0 >> $off)) as $at;
                $func(i, w)
            }
        }
    )
}

pub struct Sint64(i64);

wired_sint!(Sint64, u64, 63, encode_varint64);

pub struct Sint32(i32);

wired_sint!(Sint32, u32, 31, encode_varint32);

macro_rules! wired_fixed {
    ($t:ty, $wired_id:expr, $wide:expr) => (
        impl WiredType for $t {
            fn get_wired_id(&self) -> u8 {
                $wired_id
            }

            fn encode_to<W: Write>(&self, w: &mut W) -> Result<()> {
                let f = self.0.to_le();
                let bs: [u8; $wide] = unsafe {
                    mem::transmute(f)
                };
                w.write_all(&bs).map_err(From::from)
            }
        }
    )
}

pub struct Fixed64(u64);

wired_fixed!(Fixed64, 1, 8);

pub struct SFixed64(i64);

wired_fixed!(SFixed64, 1, 8);

pub struct Fixed32(u32);

wired_fixed!(Fixed32, 5, 4);

pub struct SFixed32(i32);

wired_fixed!(SFixed32, 5, 4);

macro_rules! wired_float {
    ($t:ty, $at:ty, $wired_id:expr, $wide:expr) => (
        impl WiredType for $t {
            fn get_wired_id(&self) -> u8 {
                $wired_id
            }

            fn encode_to<W: Write>(&self, w: &mut W) -> Result<()> {
                let u: $at = unsafe { mem::transmute(*self) };
                let f = u.to_le();
                let bs: [u8; $wide] = unsafe {
                    mem::transmute(f)
                };
                w.write_all(&bs).map_err(From::from)
            }
        }
    )
}

wired_float!(f64, u64, 1, 8);
wired_float!(f32, u32, 5, 4);

macro_rules! wired_bytes {
    ($t:ty) => (
        impl WiredType for $t {
            fn get_wired_id(&self) -> u8 {
                2
            }

            fn encode_to<W: Write>(&self, w: &mut W) -> Result<()> {
                encode_varint64(self.len() as u64, w);
                w.write_all(self.as_ref()).map_err(From::from)
            }
        }
    )
}

wired_bytes!(String);
wired_bytes!(Vec<u8>);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! make_test {
        ($test_name:ident, $cases:expr, $method:ident) => (
            #[test]
            fn $test_name() {
                let cases = $cases;

                for (input, exp) in cases {
                    let mut output = vec![];
                    $method(input, &mut output).unwrap();
                    assert_eq!(output, exp);
                }
            }
        )
    }

    make_test!(test_encode_varint64, vec![
        (1, vec![1]),
        (0, vec![0]),
        (128, vec![128, 1]),
        (150, vec![150, 1]),
    ], encode_varint64);

    make_test!(test_encode_varint32, vec![
        (1, vec![1]),
        (0, vec![0]),
        (128, vec![128, 1]),
        (150, vec![150, 1]),
    ], encode_varint32);
}
