// This file is generated by pecan 0.1.0, DO NOT EDIT!
// @generated
// source: google/protobuf/timestamp.proto

#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use pecan::{
    codec,
    EnumType,
    Message,
    Result,
    Buf,
    BufMut,
    CodedInputStream,
    CodedOutputStream,
    encoded,
};
use std::collections::HashMap;

pub static DESCRIPTOR: &[u8] = &[
    10, 31, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117,
    102, 47, 116, 105, 109, 101, 115, 116, 97, 109, 112, 46, 112, 114, 111, 116,
    111, 18, 15, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98,
    117, 102, 66, 126, 10, 19, 99, 111, 109, 46, 103, 111, 111, 103, 108, 101,
    46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 14, 84, 105, 109, 101, 115,
    116, 97, 109, 112, 80, 114, 111, 116, 111, 80, 1, 90, 43, 103, 105, 116,
    104, 117, 98, 46, 99, 111, 109, 47, 103, 111, 108, 97, 110, 103, 47, 112,
    114, 111, 116, 111, 98, 117, 102, 47, 112, 116, 121, 112, 101, 115, 47, 116,
    105, 109, 101, 115, 116, 97, 109, 112, 248, 1, 1, 162, 2, 3, 71, 80,
    66, 170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111,
    98, 117, 102, 46, 87, 101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112,
    101, 115, 98, 6, 112, 114, 111, 116, 111, 51,
];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
    cache_size: u32,
    unknown: Vec<u8>,
}

impl Message for Timestamp {
    fn merge_from(&mut self, s: &mut CodedInputStream<impl Buf>) -> Result<()> {
        loop {
            let tag = s.read_tag()?;
            match tag {
                8 => self.seconds = s.read_var_i64()?,
                16 => self.nanos = s.read_var_i32()?,
                0 => return Ok(()),
                _ => s.skip_field(&mut self.unknown, tag)?,
            }
        }
    }

    fn write_to(&self, s: &mut CodedOutputStream<impl BufMut>) -> Result<()> {
        if 0 != self.seconds {
            s.write_raw_1_byte([8])?;
            s.write_var_i64(self.seconds)?;
        }
        if 0 != self.nanos {
            s.write_raw_1_byte([16])?;
            s.write_var_i32(self.nanos)?;
        }
        if !self.unknown.is_empty() {
            s.write_unknown(&self.unknown)?;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        let mut n = self.unknown.len();
        if 0 != self.seconds {
            n += 1 + codec::varint_i64_bytes_len(self.seconds) as usize;
        }
        if 0 != self.nanos {
            n += 1 + codec::varint_i64_bytes_len(self.nanos as i64) as usize;
        }
        n
    }
}

impl Timestamp {
    pub const fn new() -> Timestamp {
        Timestamp {
            seconds: 0,
            nanos: 0,
            cache_size: 0,
            unknown: Vec::new(),
        }
    }

    pub fn default_instance() -> &'static Timestamp {
        static DEFAULT: Timestamp = Timestamp::new();
        &DEFAULT
    }

    pub fn seconds(&self) -> i64 { self.seconds }

    pub fn clear_seconds(&mut self) { self.seconds = 0; }

    pub fn set_seconds(&mut self, v: i64) { self.seconds = v; }

    pub fn nanos(&self) -> i32 { self.nanos }

    pub fn clear_nanos(&mut self) { self.nanos = 0; }

    pub fn set_nanos(&mut self, v: i32) { self.nanos = v; }
}
