// This file is generated by pecan 0.1.0, DO NOT EDIT!
// @generated
// source: benchmarks/benchmarks.proto

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
    10, 27, 98, 101, 110, 99, 104, 109, 97, 114, 107, 115, 47, 98, 101, 110,
    99, 104, 109, 97, 114, 107, 115, 46, 112, 114, 111, 116, 111, 18, 10, 98,
    101, 110, 99, 104, 109, 97, 114, 107, 115, 66, 32, 10, 30, 99, 111, 109,
    46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102,
    46, 98, 101, 110, 99, 104, 109, 97, 114, 107, 115, 98, 6, 112, 114, 111,
    116, 111, 51,
];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BenchmarkDataset {
    pub name: String,
    pub message_name: String,
    payload: Vec<Vec<u8>>,
    cache_size: u32,
    unknown: Vec<u8>,
}

impl Message for BenchmarkDataset {
    fn merge_from(&mut self, s: &mut CodedInputStream<impl Buf>) -> Result<()> {
        loop {
            let tag = s.read_tag()?;
            match tag {
                10 => self.name = s.read_string()?,
                18 => self.message_name = s.read_string()?,
                26 => self.payload.push(s.read_bytes()?),
                0 => return Ok(()),
                _ => s.skip_field(&mut self.unknown, tag)?,
            }
        }
    }

    fn write_to(&self, s: &mut CodedOutputStream<impl BufMut>) -> Result<()> {
        if !&self.name.is_empty() {
            s.write_raw_1_byte([10])?;
            s.write_string(&self.name)?;
        }
        if !&self.message_name.is_empty() {
            s.write_raw_1_byte([18])?;
            s.write_string(&self.message_name)?;
        }
        if !self.payload.is_empty() {
            for v in &self.payload {
                s.write_raw_1_byte([26])?;
                s.write_bytes(v)?;
            }
        }
        if !self.unknown.is_empty() {
            s.write_unknown(&self.unknown)?;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        let mut n = self.unknown.len();
        if !self.name.is_empty() {
            n += {
                let l = self.name.len();
                1 + codec::varint_u32_bytes_len(l as u32) as usize + l
            };
        }
        if !self.message_name.is_empty() {
            n += {
                let l = self.message_name.len();
                1 + codec::varint_u32_bytes_len(l as u32) as usize + l
            };
        }
        if !self.payload.is_empty() {
            n += self.payload.iter().fold(0, |n, m| {
                let l = m.len();
                n + 1 + codec::varint_u32_bytes_len(l as u32) as usize + l
            });
        }
        n
    }
}

impl BenchmarkDataset {
    pub const fn new() -> BenchmarkDataset {
        BenchmarkDataset {
            name: String::new(),
            message_name: String::new(),
            payload: Vec::new(),
            cache_size: 0,
            unknown: Vec::new(),
        }
    }

    pub fn default_instance() -> &'static BenchmarkDataset {
        static DEFAULT: BenchmarkDataset = BenchmarkDataset::new();
        &DEFAULT
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn clear_name(&mut self) { self.name = Default::default(); }

    pub fn set_name(&mut self, v: String) { self.name = v; }

    pub fn name_mut(&mut self) -> &mut String { &mut self.name }

    pub fn message_name(&self) -> &str { &self.message_name }

    pub fn clear_message_name(&mut self) { self.message_name = Default::default(); }

    pub fn set_message_name(&mut self, v: String) { self.message_name = v; }

    pub fn message_name_mut(&mut self) -> &mut String { &mut self.message_name }

    pub fn payload(&self) -> &[Vec<u8>] { &self.payload }

    pub fn clear_payload(&mut self) { self.payload.clear(); }

    pub fn set_payload(&mut self, v: impl Into<Vec<Vec<u8>>>) { self.payload = v.into(); }

    pub fn payload_mut(&mut self) -> &mut Vec<Vec<u8>> { &mut self.payload }
}