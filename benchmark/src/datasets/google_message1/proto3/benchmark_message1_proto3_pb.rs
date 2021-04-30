#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage1 {
    pub field1: String,
    pub field9: String,
    pub field18: String,
    pub field80: bool,
    pub field81: bool,
    pub field2: i32,
    pub field3: i32,
    pub field280: i32,
    pub field6: i32,
    pub field22: i64,
    pub field4: String,
    pub field5: Vec<u64>,
    pub field59: bool,
    pub field7: String,
    pub field16: i32,
    pub field130: i32,
    pub field12: bool,
    pub field17: bool,
    pub field13: bool,
    pub field14: bool,
    pub field104: i32,
    pub field100: i32,
    pub field101: i32,
    pub field102: String,
    pub field103: String,
    pub field29: i32,
    pub field30: bool,
    pub field60: i32,
    pub field271: i32,
    pub field272: i32,
    pub field150: i32,
    pub field23: i32,
    pub field24: bool,
    pub field25: i32,
    pub field15: Option<GoogleMessage1SubMessage>,
    pub field78: bool,
    pub field67: i32,
    pub field68: i32,
    pub field128: i32,
    pub field129: String,
    pub field131: i32,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl GoogleMessage1 {
    pub const fn new() -> GoogleMessage1 {
        GoogleMessage1 {
            field1: String::new(),
            field9: String::new(),
            field18: String::new(),
            field80: false,
            field81: false,
            field2: 0,
            field3: 0,
            field280: 0,
            field6: 0,
            field22: 0,
            field4: String::new(),
            field5: Vec::new(),
            field59: false,
            field7: String::new(),
            field16: 0,
            field130: 0,
            field12: false,
            field17: false,
            field13: false,
            field14: false,
            field104: 0,
            field100: 0,
            field101: 0,
            field102: String::new(),
            field103: String::new(),
            field29: 0,
            field30: false,
            field60: 0,
            field271: 0,
            field272: 0,
            field150: 0,
            field23: 0,
            field24: false,
            field25: 0,
            field15: None,
            field78: false,
            field67: 0,
            field68: 0,
            field128: 0,
            field129: String::new(),
            field131: 0,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field15(&self) -> &GoogleMessage1SubMessage {
        match &self.field15 {
            Some(v) => v,
            _ => GoogleMessage1SubMessage::default_instance(),
        }
    }
    pub fn field15_mut(&mut self) -> &mut GoogleMessage1SubMessage {
        self.field15.get_or_insert_with(Default::default)
    }
    pub fn set_field15(&mut self, val: GoogleMessage1SubMessage) {
        self.field15 = Some(val);
    }
}
impl pecan::Message for GoogleMessage1 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixed::merge_from(&mut self.field1, s)?,
                16 => self.field2 = Varint::read_from(s)?,
                24 => self.field3 = Varint::read_from(s)?,
                34 => LengthPrefixed::merge_from(&mut self.field4, s)?,
                42 => PackedArray::<Fixed64>::merge_from(&mut self.field5, s)?,
                41 => CopyArray::<Fixed64>::merge_from(&mut self.field5, s)?,
                48 => self.field6 = Varint::read_from(s)?,
                58 => LengthPrefixed::merge_from(&mut self.field7, s)?,
                74 => LengthPrefixed::merge_from(&mut self.field9, s)?,
                96 => self.field12 = Varint::read_from(s)?,
                104 => self.field13 = Varint::read_from(s)?,
                112 => self.field14 = Varint::read_from(s)?,
                122 => LengthPrefixed::merge_from(self.field15_mut(), s)?,
                128 => self.field16 = Varint::read_from(s)?,
                136 => self.field17 = Varint::read_from(s)?,
                146 => LengthPrefixed::merge_from(&mut self.field18, s)?,
                176 => self.field22 = Varint::read_from(s)?,
                184 => self.field23 = Varint::read_from(s)?,
                192 => self.field24 = Varint::read_from(s)?,
                200 => self.field25 = Varint::read_from(s)?,
                232 => self.field29 = Varint::read_from(s)?,
                240 => self.field30 = Varint::read_from(s)?,
                472 => self.field59 = Varint::read_from(s)?,
                480 => self.field60 = Varint::read_from(s)?,
                536 => self.field67 = Varint::read_from(s)?,
                544 => self.field68 = Varint::read_from(s)?,
                624 => self.field78 = Varint::read_from(s)?,
                640 => self.field80 = Varint::read_from(s)?,
                648 => self.field81 = Varint::read_from(s)?,
                800 => self.field100 = Varint::read_from(s)?,
                808 => self.field101 = Varint::read_from(s)?,
                818 => LengthPrefixed::merge_from(&mut self.field102, s)?,
                826 => LengthPrefixed::merge_from(&mut self.field103, s)?,
                832 => self.field104 = Varint::read_from(s)?,
                1024 => self.field128 = Varint::read_from(s)?,
                1034 => LengthPrefixed::merge_from(&mut self.field129, s)?,
                1040 => self.field130 = Varint::read_from(s)?,
                1048 => self.field131 = Varint::read_from(s)?,
                1200 => self.field150 = Varint::read_from(s)?,
                2168 => self.field271 = Varint::read_from(s)?,
                2176 => self.field272 = Varint::read_from(s)?,
                2240 => self.field280 = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.field1.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.field1, s)?;
        }
        if self.field2 != 0 {
            s.write_tag(16)?;
            Varint::write_to(self.field2, s)?;
        }
        if self.field3 != 0 {
            s.write_tag(24)?;
            Varint::write_to(self.field3, s)?;
        }
        if !self.field4.is_empty() {
            s.write_tag(34)?;
            LengthPrefixed::write_to(&self.field4, s)?;
        }
        if !self.field5.is_empty() {
            s.write_tag(42)?;
            PackedArray::<Fixed64>::write_to(&self.field5, s)?
        }
        if self.field6 != 0 {
            s.write_tag(48)?;
            Varint::write_to(self.field6, s)?;
        }
        if !self.field7.is_empty() {
            s.write_tag(58)?;
            LengthPrefixed::write_to(&self.field7, s)?;
        }
        if !self.field9.is_empty() {
            s.write_tag(74)?;
            LengthPrefixed::write_to(&self.field9, s)?;
        }
        if self.field12 {
            s.write_tag(96)?;
            Varint::write_to(self.field12, s)?;
        }
        if self.field13 {
            s.write_tag(104)?;
            Varint::write_to(self.field13, s)?;
        }
        if self.field14 {
            s.write_tag(112)?;
            Varint::write_to(self.field14, s)?;
        }
        if let Some(v) = &self.field15 {
            s.write_tag(122)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if self.field16 != 0 {
            s.write_tag(128)?;
            Varint::write_to(self.field16, s)?;
        }
        if self.field17 {
            s.write_tag(136)?;
            Varint::write_to(self.field17, s)?;
        }
        if !self.field18.is_empty() {
            s.write_tag(146)?;
            LengthPrefixed::write_to(&self.field18, s)?;
        }
        if self.field22 != 0 {
            s.write_tag(176)?;
            Varint::write_to(self.field22, s)?;
        }
        if self.field23 != 0 {
            s.write_tag(184)?;
            Varint::write_to(self.field23, s)?;
        }
        if self.field24 {
            s.write_tag(192)?;
            Varint::write_to(self.field24, s)?;
        }
        if self.field25 != 0 {
            s.write_tag(200)?;
            Varint::write_to(self.field25, s)?;
        }
        if self.field29 != 0 {
            s.write_tag(232)?;
            Varint::write_to(self.field29, s)?;
        }
        if self.field30 {
            s.write_tag(240)?;
            Varint::write_to(self.field30, s)?;
        }
        if self.field59 {
            s.write_tag(472)?;
            Varint::write_to(self.field59, s)?;
        }
        if self.field60 != 0 {
            s.write_tag(480)?;
            Varint::write_to(self.field60, s)?;
        }
        if self.field67 != 0 {
            s.write_tag(536)?;
            Varint::write_to(self.field67, s)?;
        }
        if self.field68 != 0 {
            s.write_tag(544)?;
            Varint::write_to(self.field68, s)?;
        }
        if self.field78 {
            s.write_tag(624)?;
            Varint::write_to(self.field78, s)?;
        }
        if self.field80 {
            s.write_tag(640)?;
            Varint::write_to(self.field80, s)?;
        }
        if self.field81 {
            s.write_tag(648)?;
            Varint::write_to(self.field81, s)?;
        }
        if self.field100 != 0 {
            s.write_tag(800)?;
            Varint::write_to(self.field100, s)?;
        }
        if self.field101 != 0 {
            s.write_tag(808)?;
            Varint::write_to(self.field101, s)?;
        }
        if !self.field102.is_empty() {
            s.write_tag(818)?;
            LengthPrefixed::write_to(&self.field102, s)?;
        }
        if !self.field103.is_empty() {
            s.write_tag(826)?;
            LengthPrefixed::write_to(&self.field103, s)?;
        }
        if self.field104 != 0 {
            s.write_tag(832)?;
            Varint::write_to(self.field104, s)?;
        }
        if self.field128 != 0 {
            s.write_tag(1024)?;
            Varint::write_to(self.field128, s)?;
        }
        if !self.field129.is_empty() {
            s.write_tag(1034)?;
            LengthPrefixed::write_to(&self.field129, s)?;
        }
        if self.field130 != 0 {
            s.write_tag(1040)?;
            Varint::write_to(self.field130, s)?;
        }
        if self.field131 != 0 {
            s.write_tag(1048)?;
            Varint::write_to(self.field131, s)?;
        }
        if self.field150 != 0 {
            s.write_tag(1200)?;
            Varint::write_to(self.field150, s)?;
        }
        if self.field271 != 0 {
            s.write_tag(2168)?;
            Varint::write_to(self.field271, s)?;
        }
        if self.field272 != 0 {
            s.write_tag(2176)?;
            Varint::write_to(self.field272, s)?;
        }
        if self.field280 != 0 {
            s.write_tag(2240)?;
            Varint::write_to(self.field280, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.field1.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field1);
        }
        if self.field2 != 0 {
            l += 1 + Varint::size(self.field2);
        }
        if self.field3 != 0 {
            l += 1 + Varint::size(self.field3);
        }
        if !self.field4.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field4);
        }
        if !self.field5.is_empty() {
            l += 1 + PackedArray::<Fixed64>::size(&self.field5);
        }
        if self.field6 != 0 {
            l += 1 + Varint::size(self.field6);
        }
        if !self.field7.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field7);
        }
        if !self.field9.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field9);
        }
        if self.field12 {
            l += 1 + Varint::size(self.field12);
        }
        if self.field13 {
            l += 1 + Varint::size(self.field13);
        }
        if self.field14 {
            l += 1 + Varint::size(self.field14);
        }
        if let Some(v) = &self.field15 {
            l += 1 + LengthPrefixed::size(v);
        }
        if self.field16 != 0 {
            l += 2 + Varint::size(self.field16);
        }
        if self.field17 {
            l += 2 + Varint::size(self.field17);
        }
        if !self.field18.is_empty() {
            l += 2 + LengthPrefixed::size(&self.field18);
        }
        if self.field22 != 0 {
            l += 2 + Varint::size(self.field22);
        }
        if self.field23 != 0 {
            l += 2 + Varint::size(self.field23);
        }
        if self.field24 {
            l += 2 + Varint::size(self.field24);
        }
        if self.field25 != 0 {
            l += 2 + Varint::size(self.field25);
        }
        if self.field29 != 0 {
            l += 2 + Varint::size(self.field29);
        }
        if self.field30 {
            l += 2 + Varint::size(self.field30);
        }
        if self.field59 {
            l += 2 + Varint::size(self.field59);
        }
        if self.field60 != 0 {
            l += 2 + Varint::size(self.field60);
        }
        if self.field67 != 0 {
            l += 2 + Varint::size(self.field67);
        }
        if self.field68 != 0 {
            l += 2 + Varint::size(self.field68);
        }
        if self.field78 {
            l += 2 + Varint::size(self.field78);
        }
        if self.field80 {
            l += 2 + Varint::size(self.field80);
        }
        if self.field81 {
            l += 2 + Varint::size(self.field81);
        }
        if self.field100 != 0 {
            l += 2 + Varint::size(self.field100);
        }
        if self.field101 != 0 {
            l += 2 + Varint::size(self.field101);
        }
        if !self.field102.is_empty() {
            l += 2 + LengthPrefixed::size(&self.field102);
        }
        if !self.field103.is_empty() {
            l += 2 + LengthPrefixed::size(&self.field103);
        }
        if self.field104 != 0 {
            l += 2 + Varint::size(self.field104);
        }
        if self.field128 != 0 {
            l += 2 + Varint::size(self.field128);
        }
        if !self.field129.is_empty() {
            l += 2 + LengthPrefixed::size(&self.field129);
        }
        if self.field130 != 0 {
            l += 2 + Varint::size(self.field130);
        }
        if self.field131 != 0 {
            l += 2 + Varint::size(self.field131);
        }
        if self.field150 != 0 {
            l += 2 + Varint::size(self.field150);
        }
        if self.field271 != 0 {
            l += 2 + Varint::size(self.field271);
        }
        if self.field272 != 0 {
            l += 2 + Varint::size(self.field272);
        }
        if self.field280 != 0 {
            l += 2 + Varint::size(self.field280);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self.field1.clear();
        self.field9.clear();
        self.field18.clear();
        self.field80 = false;
        self.field81 = false;
        self.field2 = 0;
        self.field3 = 0;
        self.field280 = 0;
        self.field6 = 0;
        self.field22 = 0;
        self.field4.clear();
        self.field5.clear();
        self.field59 = false;
        self.field7.clear();
        self.field16 = 0;
        self.field130 = 0;
        self.field12 = false;
        self.field17 = false;
        self.field13 = false;
        self.field14 = false;
        self.field104 = 0;
        self.field100 = 0;
        self.field101 = 0;
        self.field102.clear();
        self.field103.clear();
        self.field29 = 0;
        self.field30 = false;
        self.field60 = 0;
        self.field271 = 0;
        self.field272 = 0;
        self.field150 = 0;
        self.field23 = 0;
        self.field24 = false;
        self.field25 = 0;
        self.field15 = None;
        self.field78 = false;
        self.field67 = 0;
        self.field68 = 0;
        self.field128 = 0;
        self.field129.clear();
        self.field131 = 0;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for GoogleMessage1 {
    fn default_instance() -> &'static GoogleMessage1 {
        static DEFAULT: GoogleMessage1 = GoogleMessage1::new();
        &DEFAULT
    }
}
impl Default for GoogleMessage1 {
    #[inline]
    fn default() -> GoogleMessage1 {
        GoogleMessage1::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage1SubMessage {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
    pub field15: String,
    pub field12: bool,
    pub field13: i64,
    pub field14: i64,
    pub field16: i32,
    pub field19: i32,
    pub field20: bool,
    pub field28: bool,
    pub field21: u64,
    pub field22: i32,
    pub field23: bool,
    pub field206: bool,
    pub field203: u32,
    pub field204: i32,
    pub field205: String,
    pub field207: u64,
    pub field300: u64,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl GoogleMessage1SubMessage {
    pub const fn new() -> GoogleMessage1SubMessage {
        GoogleMessage1SubMessage {
            field1: 0,
            field2: 0,
            field3: 0,
            field15: String::new(),
            field12: false,
            field13: 0,
            field14: 0,
            field16: 0,
            field19: 0,
            field20: false,
            field28: false,
            field21: 0,
            field22: 0,
            field23: false,
            field206: false,
            field203: 0,
            field204: 0,
            field205: String::new(),
            field207: 0,
            field300: 0,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for GoogleMessage1SubMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.field1 = Varint::read_from(s)?,
                16 => self.field2 = Varint::read_from(s)?,
                24 => self.field3 = Varint::read_from(s)?,
                96 => self.field12 = Varint::read_from(s)?,
                104 => self.field13 = Varint::read_from(s)?,
                112 => self.field14 = Varint::read_from(s)?,
                122 => LengthPrefixed::merge_from(&mut self.field15, s)?,
                128 => self.field16 = Varint::read_from(s)?,
                152 => self.field19 = Varint::read_from(s)?,
                160 => self.field20 = Varint::read_from(s)?,
                169 => self.field21 = Fixed64::read_from(s)?,
                176 => self.field22 = Varint::read_from(s)?,
                184 => self.field23 = Varint::read_from(s)?,
                224 => self.field28 = Varint::read_from(s)?,
                1629 => self.field203 = Fixed32::read_from(s)?,
                1632 => self.field204 = Varint::read_from(s)?,
                1642 => LengthPrefixed::merge_from(&mut self.field205, s)?,
                1648 => self.field206 = Varint::read_from(s)?,
                1656 => self.field207 = Varint::read_from(s)?,
                2400 => self.field300 = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if self.field1 != 0 {
            s.write_tag(8)?;
            Varint::write_to(self.field1, s)?;
        }
        if self.field2 != 0 {
            s.write_tag(16)?;
            Varint::write_to(self.field2, s)?;
        }
        if self.field3 != 0 {
            s.write_tag(24)?;
            Varint::write_to(self.field3, s)?;
        }
        if self.field12 {
            s.write_tag(96)?;
            Varint::write_to(self.field12, s)?;
        }
        if self.field13 != 0 {
            s.write_tag(104)?;
            Varint::write_to(self.field13, s)?;
        }
        if self.field14 != 0 {
            s.write_tag(112)?;
            Varint::write_to(self.field14, s)?;
        }
        if !self.field15.is_empty() {
            s.write_tag(122)?;
            LengthPrefixed::write_to(&self.field15, s)?;
        }
        if self.field16 != 0 {
            s.write_tag(128)?;
            Varint::write_to(self.field16, s)?;
        }
        if self.field19 != 0 {
            s.write_tag(152)?;
            Varint::write_to(self.field19, s)?;
        }
        if self.field20 {
            s.write_tag(160)?;
            Varint::write_to(self.field20, s)?;
        }
        if self.field21 != 0 {
            s.write_tag(169)?;
            Fixed64::write_to(self.field21, s)?;
        }
        if self.field22 != 0 {
            s.write_tag(176)?;
            Varint::write_to(self.field22, s)?;
        }
        if self.field23 {
            s.write_tag(184)?;
            Varint::write_to(self.field23, s)?;
        }
        if self.field28 {
            s.write_tag(224)?;
            Varint::write_to(self.field28, s)?;
        }
        if self.field203 != 0 {
            s.write_tag(1629)?;
            Fixed32::write_to(self.field203, s)?;
        }
        if self.field204 != 0 {
            s.write_tag(1632)?;
            Varint::write_to(self.field204, s)?;
        }
        if !self.field205.is_empty() {
            s.write_tag(1642)?;
            LengthPrefixed::write_to(&self.field205, s)?;
        }
        if self.field206 {
            s.write_tag(1648)?;
            Varint::write_to(self.field206, s)?;
        }
        if self.field207 != 0 {
            s.write_tag(1656)?;
            Varint::write_to(self.field207, s)?;
        }
        if self.field300 != 0 {
            s.write_tag(2400)?;
            Varint::write_to(self.field300, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.field1 != 0 {
            l += 1 + Varint::size(self.field1);
        }
        if self.field2 != 0 {
            l += 1 + Varint::size(self.field2);
        }
        if self.field3 != 0 {
            l += 1 + Varint::size(self.field3);
        }
        if self.field12 {
            l += 1 + Varint::size(self.field12);
        }
        if self.field13 != 0 {
            l += 1 + Varint::size(self.field13);
        }
        if self.field14 != 0 {
            l += 1 + Varint::size(self.field14);
        }
        if !self.field15.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field15);
        }
        if self.field16 != 0 {
            l += 2 + Varint::size(self.field16);
        }
        if self.field19 != 0 {
            l += 2 + Varint::size(self.field19);
        }
        if self.field20 {
            l += 2 + Varint::size(self.field20);
        }
        if self.field21 != 0 {
            l += 2 + Fixed64::size(self.field21);
        }
        if self.field22 != 0 {
            l += 2 + Varint::size(self.field22);
        }
        if self.field23 {
            l += 2 + Varint::size(self.field23);
        }
        if self.field28 {
            l += 2 + Varint::size(self.field28);
        }
        if self.field203 != 0 {
            l += 2 + Fixed32::size(self.field203);
        }
        if self.field204 != 0 {
            l += 2 + Varint::size(self.field204);
        }
        if !self.field205.is_empty() {
            l += 2 + LengthPrefixed::size(&self.field205);
        }
        if self.field206 {
            l += 2 + Varint::size(self.field206);
        }
        if self.field207 != 0 {
            l += 2 + Varint::size(self.field207);
        }
        if self.field300 != 0 {
            l += 2 + Varint::size(self.field300);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self.field1 = 0;
        self.field2 = 0;
        self.field3 = 0;
        self.field15.clear();
        self.field12 = false;
        self.field13 = 0;
        self.field14 = 0;
        self.field16 = 0;
        self.field19 = 0;
        self.field20 = false;
        self.field28 = false;
        self.field21 = 0;
        self.field22 = 0;
        self.field23 = false;
        self.field206 = false;
        self.field203 = 0;
        self.field204 = 0;
        self.field205.clear();
        self.field207 = 0;
        self.field300 = 0;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for GoogleMessage1SubMessage {
    fn default_instance() -> &'static GoogleMessage1SubMessage {
        static DEFAULT: GoogleMessage1SubMessage = GoogleMessage1SubMessage::new();
        &DEFAULT
    }
}
impl Default for GoogleMessage1SubMessage {
    #[inline]
    fn default() -> GoogleMessage1SubMessage {
        GoogleMessage1SubMessage::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n?datasets/google_message1/proto3/benchmark_message1_proto3.proto\x12\x11benchmarks.proto3\"\xF9\x08\n\x0EGoogleMessage1\x12\x16\n\x06field1\x18\x01 \x01(\tR\x06field1\x12\x16\n\x06field9\x18\t \x01(\tR\x06field9\x12\x18\n\x07field18\x18\x12 \x01(\tR\x07field18\x12\x18\n\x07field80\x18P \x01(\x08R\x07field80\x12\x18\n\x07field81\x18Q \x01(\x08R\x07field81\x12\x16\n\x06field2\x18\x02 \x01(\x05R\x06field2\x12\x16\n\x06field3\x18\x03 \x01(\x05R\x06field3\x12\x1B\n\x08field280\x18\x98\x02 \x01(\x05R\x08field280\x12\x16\n\x06field6\x18\x06 \x01(\x05R\x06field6\x12\x18\n\x07field22\x18\x16 \x01(\x03R\x07field22\x12\x16\n\x06field4\x18\x04 \x01(\tR\x06field4\x12\x16\n\x06field5\x18\x05 \x03(\x06R\x06field5\x12\x18\n\x07field59\x18; \x01(\x08R\x07field59\x12\x16\n\x06field7\x18\x07 \x01(\tR\x06field7\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x1B\n\x08field130\x18\x82\x01 \x01(\x05R\x08field130\x12\x18\n\x07field12\x18\x0C \x01(\x08R\x07field12\x12\x18\n\x07field17\x18\x11 \x01(\x08R\x07field17\x12\x18\n\x07field13\x18\r \x01(\x08R\x07field13\x12\x18\n\x07field14\x18\x0E \x01(\x08R\x07field14\x12\x1A\n\x08field104\x18h \x01(\x05R\x08field104\x12\x1A\n\x08field100\x18d \x01(\x05R\x08field100\x12\x1A\n\x08field101\x18e \x01(\x05R\x08field101\x12\x1A\n\x08field102\x18f \x01(\tR\x08field102\x12\x1A\n\x08field103\x18g \x01(\tR\x08field103\x12\x18\n\x07field29\x18\x1D \x01(\x05R\x07field29\x12\x18\n\x07field30\x18\x1E \x01(\x08R\x07field30\x12\x18\n\x07field60\x18< \x01(\x05R\x07field60\x12\x1B\n\x08field271\x18\x8F\x02 \x01(\x05R\x08field271\x12\x1B\n\x08field272\x18\x90\x02 \x01(\x05R\x08field272\x12\x1B\n\x08field150\x18\x96\x01 \x01(\x05R\x08field150\x12\x18\n\x07field23\x18\x17 \x01(\x05R\x07field23\x12\x18\n\x07field24\x18\x18 \x01(\x08R\x07field24\x12\x18\n\x07field25\x18\x19 \x01(\x05R\x07field25\x12E\n\x07field15\x18\x0F \x01(\x0B2+.benchmarks.proto3.GoogleMessage1SubMessageR\x07field15\x12\x18\n\x07field78\x18N \x01(\x08R\x07field78\x12\x18\n\x07field67\x18C \x01(\x05R\x07field67\x12\x18\n\x07field68\x18D \x01(\x05R\x07field68\x12\x1B\n\x08field128\x18\x80\x01 \x01(\x05R\x08field128\x12\x1B\n\x08field129\x18\x81\x01 \x01(\tR\x08field129\x12\x1B\n\x08field131\x18\x83\x01 \x01(\x05R\x08field131\"\xAE\x04\n\x18GoogleMessage1SubMessage\x12\x16\n\x06field1\x18\x01 \x01(\x05R\x06field1\x12\x16\n\x06field2\x18\x02 \x01(\x05R\x06field2\x12\x16\n\x06field3\x18\x03 \x01(\x05R\x06field3\x12\x18\n\x07field15\x18\x0F \x01(\tR\x07field15\x12\x18\n\x07field12\x18\x0C \x01(\x08R\x07field12\x12\x18\n\x07field13\x18\r \x01(\x03R\x07field13\x12\x18\n\x07field14\x18\x0E \x01(\x03R\x07field14\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x18\n\x07field19\x18\x13 \x01(\x05R\x07field19\x12\x18\n\x07field20\x18\x14 \x01(\x08R\x07field20\x12\x18\n\x07field28\x18\x1C \x01(\x08R\x07field28\x12\x18\n\x07field21\x18\x15 \x01(\x06R\x07field21\x12\x18\n\x07field22\x18\x16 \x01(\x05R\x07field22\x12\x18\n\x07field23\x18\x17 \x01(\x08R\x07field23\x12\x1B\n\x08field206\x18\xCE\x01 \x01(\x08R\x08field206\x12\x1B\n\x08field203\x18\xCB\x01 \x01(\x07R\x08field203\x12\x1B\n\x08field204\x18\xCC\x01 \x01(\x05R\x08field204\x12\x1B\n\x08field205\x18\xCD\x01 \x01(\tR\x08field205\x12\x1B\n\x08field207\x18\xCF\x01 \x01(\x04R\x08field207\x12\x1B\n\x08field300\x18\xAC\x02 \x01(\x04R\x08field300B%\n\x1Ecom.google.protobuf.benchmarksH\x01\xF8\x01\x01J\xD6(\n\x06\x12\x04 \0k\x01\n\xEE\x0C\n\x01\x0C\x12\x03 \0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2  Benchmark messages for proto3.\n\n\x08\n\x01\x02\x12\x03\"\0\x1A\n\x08\n\x01\x08\x12\x03#\07\n\t\n\x02\x08\x01\x12\x03#\07\n\x08\n\x01\x08\x12\x03&\0\x1C\nD\n\x02\x08\t\x12\x03&\0\x1C\x1A9 This is the default, but we specify it here explicitly.\n\n\x08\n\x01\x08\x12\x03(\0\x1F\n\t\n\x02\x08\x1F\x12\x03(\0\x1F\n\n\n\x02\x04\0\x12\x04*\0T\x01\n\n\n\x03\x04\0\x01\x12\x03*\x08\x16\n\x0B\n\x04\x04\0\x02\0\x12\x03+\x02\x14\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03+\x02\x08\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03+\t\x0F\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03+\x12\x13\n\x0B\n\x04\x04\0\x02\x01\x12\x03,\x02\x14\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03,\x02\x08\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03,\t\x0F\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03,\x12\x13\n\x0B\n\x04\x04\0\x02\x02\x12\x03-\x02\x16\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03-\x02\x08\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03-\t\x10\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03-\x13\x15\n\x0B\n\x04\x04\0\x02\x03\x12\x03.\x02\x14\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03.\x02\x06\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03.\x07\x0E\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03.\x11\x13\n\x0B\n\x04\x04\0\x02\x04\x12\x03/\x02\x14\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x03/\x02\x06\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03/\x07\x0E\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03/\x11\x13\n\x0B\n\x04\x04\0\x02\x05\x12\x030\x02\x13\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x030\x02\x07\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x030\x08\x0E\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x030\x11\x12\n\x0B\n\x04\x04\0\x02\x06\x12\x031\x02\x13\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x031\x02\x07\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x031\x08\x0E\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x031\x11\x12\n\x0B\n\x04\x04\0\x02\x07\x12\x032\x02\x17\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x032\x02\x07\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x032\x08\x10\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x032\x13\x16\n\x0B\n\x04\x04\0\x02\x08\x12\x033\x02\x13\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x033\x02\x07\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x033\x08\x0E\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x033\x11\x12\n\x0B\n\x04\x04\0\x02\t\x12\x034\x02\x15\n\x0C\n\x05\x04\0\x02\t\x05\x12\x034\x02\x07\n\x0C\n\x05\x04\0\x02\t\x01\x12\x034\x08\x0F\n\x0C\n\x05\x04\0\x02\t\x03\x12\x034\x12\x14\n\x0B\n\x04\x04\0\x02\n\x12\x035\x02\x14\n\x0C\n\x05\x04\0\x02\n\x05\x12\x035\x02\x08\n\x0C\n\x05\x04\0\x02\n\x01\x12\x035\t\x0F\n\x0C\n\x05\x04\0\x02\n\x03\x12\x035\x12\x13\n\x0B\n\x04\x04\0\x02\x0B\x12\x036\x02\x1E\n\x0C\n\x05\x04\0\x02\x0B\x04\x12\x036\x02\n\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x036\x0B\x12\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x036\x13\x19\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x036\x1C\x1D\n\x0B\n\x04\x04\0\x02\x0C\x12\x037\x02\x14\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x037\x02\x06\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x037\x07\x0E\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x037\x11\x13\n\x0B\n\x04\x04\0\x02\r\x12\x038\x02\x14\n\x0C\n\x05\x04\0\x02\r\x05\x12\x038\x02\x08\n\x0C\n\x05\x04\0\x02\r\x01\x12\x038\t\x0F\n\x0C\n\x05\x04\0\x02\r\x03\x12\x038\x12\x13\n\x0B\n\x04\x04\0\x02\x0E\x12\x039\x02\x15\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x039\x02\x07\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x039\x08\x0F\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x039\x12\x14\n\x0B\n\x04\x04\0\x02\x0F\x12\x03:\x02\x17\n\x0C\n\x05\x04\0\x02\x0F\x05\x12\x03:\x02\x07\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03:\x08\x10\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03:\x13\x16\n\x0B\n\x04\x04\0\x02\x10\x12\x03;\x02\x14\n\x0C\n\x05\x04\0\x02\x10\x05\x12\x03;\x02\x06\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03;\x07\x0E\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03;\x11\x13\n\x0B\n\x04\x04\0\x02\x11\x12\x03<\x02\x14\n\x0C\n\x05\x04\0\x02\x11\x05\x12\x03<\x02\x06\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03<\x07\x0E\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03<\x11\x13\n\x0B\n\x04\x04\0\x02\x12\x12\x03=\x02\x14\n\x0C\n\x05\x04\0\x02\x12\x05\x12\x03=\x02\x06\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03=\x07\x0E\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03=\x11\x13\n\x0B\n\x04\x04\0\x02\x13\x12\x03>\x02\x14\n\x0C\n\x05\x04\0\x02\x13\x05\x12\x03>\x02\x06\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03>\x07\x0E\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03>\x11\x13\n\x0B\n\x04\x04\0\x02\x14\x12\x03?\x02\x17\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03?\x02\x07\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03?\x08\x10\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03?\x13\x16\n\x0B\n\x04\x04\0\x02\x15\x12\x03@\x02\x17\n\x0C\n\x05\x04\0\x02\x15\x05\x12\x03@\x02\x07\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03@\x08\x10\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03@\x13\x16\n\x0B\n\x04\x04\0\x02\x16\x12\x03A\x02\x17\n\x0C\n\x05\x04\0\x02\x16\x05\x12\x03A\x02\x07\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03A\x08\x10\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03A\x13\x16\n\x0B\n\x04\x04\0\x02\x17\x12\x03B\x02\x18\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03B\x02\x08\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03B\t\x11\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03B\x14\x17\n\x0B\n\x04\x04\0\x02\x18\x12\x03C\x02\x18\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03C\x02\x08\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03C\t\x11\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03C\x14\x17\n\x0B\n\x04\x04\0\x02\x19\x12\x03D\x02\x15\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03D\x02\x07\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03D\x08\x0F\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03D\x12\x14\n\x0B\n\x04\x04\0\x02\x1A\x12\x03E\x02\x14\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03E\x02\x06\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03E\x07\x0E\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03E\x11\x13\n\x0B\n\x04\x04\0\x02\x1B\x12\x03F\x02\x15\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03F\x02\x07\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03F\x08\x0F\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03F\x12\x14\n\x0B\n\x04\x04\0\x02\x1C\x12\x03G\x02\x17\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03G\x02\x07\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03G\x08\x10\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03G\x13\x16\n\x0B\n\x04\x04\0\x02\x1D\x12\x03H\x02\x17\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03H\x02\x07\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03H\x08\x10\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03H\x13\x16\n\x0B\n\x04\x04\0\x02\x1E\x12\x03I\x02\x17\n\x0C\n\x05\x04\0\x02\x1E\x05\x12\x03I\x02\x07\n\x0C\n\x05\x04\0\x02\x1E\x01\x12\x03I\x08\x10\n\x0C\n\x05\x04\0\x02\x1E\x03\x12\x03I\x13\x16\n\x0B\n\x04\x04\0\x02\x1F\x12\x03J\x02\x15\n\x0C\n\x05\x04\0\x02\x1F\x05\x12\x03J\x02\x07\n\x0C\n\x05\x04\0\x02\x1F\x01\x12\x03J\x08\x0F\n\x0C\n\x05\x04\0\x02\x1F\x03\x12\x03J\x12\x14\n\x0B\n\x04\x04\0\x02 \x12\x03K\x02\x14\n\x0C\n\x05\x04\0\x02 \x05\x12\x03K\x02\x06\n\x0C\n\x05\x04\0\x02 \x01\x12\x03K\x07\x0E\n\x0C\n\x05\x04\0\x02 \x03\x12\x03K\x11\x13\n\x0B\n\x04\x04\0\x02!\x12\x03L\x02\x15\n\x0C\n\x05\x04\0\x02!\x05\x12\x03L\x02\x07\n\x0C\n\x05\x04\0\x02!\x01\x12\x03L\x08\x0F\n\x0C\n\x05\x04\0\x02!\x03\x12\x03L\x12\x14\n\x0B\n\x04\x04\0\x02\"\x12\x03M\x02(\n\x0C\n\x05\x04\0\x02\"\x06\x12\x03M\x02\x1A\n\x0C\n\x05\x04\0\x02\"\x01\x12\x03M\x1B\"\n\x0C\n\x05\x04\0\x02\"\x03\x12\x03M%'\n\x0B\n\x04\x04\0\x02#\x12\x03N\x02\x14\n\x0C\n\x05\x04\0\x02#\x05\x12\x03N\x02\x06\n\x0C\n\x05\x04\0\x02#\x01\x12\x03N\x07\x0E\n\x0C\n\x05\x04\0\x02#\x03\x12\x03N\x11\x13\n\x0B\n\x04\x04\0\x02$\x12\x03O\x02\x15\n\x0C\n\x05\x04\0\x02$\x05\x12\x03O\x02\x07\n\x0C\n\x05\x04\0\x02$\x01\x12\x03O\x08\x0F\n\x0C\n\x05\x04\0\x02$\x03\x12\x03O\x12\x14\n\x0B\n\x04\x04\0\x02%\x12\x03P\x02\x15\n\x0C\n\x05\x04\0\x02%\x05\x12\x03P\x02\x07\n\x0C\n\x05\x04\0\x02%\x01\x12\x03P\x08\x0F\n\x0C\n\x05\x04\0\x02%\x03\x12\x03P\x12\x14\n\x0B\n\x04\x04\0\x02&\x12\x03Q\x02\x17\n\x0C\n\x05\x04\0\x02&\x05\x12\x03Q\x02\x07\n\x0C\n\x05\x04\0\x02&\x01\x12\x03Q\x08\x10\n\x0C\n\x05\x04\0\x02&\x03\x12\x03Q\x13\x16\n\x0B\n\x04\x04\0\x02'\x12\x03R\x02\x18\n\x0C\n\x05\x04\0\x02'\x05\x12\x03R\x02\x08\n\x0C\n\x05\x04\0\x02'\x01\x12\x03R\t\x11\n\x0C\n\x05\x04\0\x02'\x03\x12\x03R\x14\x17\n\x0B\n\x04\x04\0\x02(\x12\x03S\x02\x17\n\x0C\n\x05\x04\0\x02(\x05\x12\x03S\x02\x07\n\x0C\n\x05\x04\0\x02(\x01\x12\x03S\x08\x10\n\x0C\n\x05\x04\0\x02(\x03\x12\x03S\x13\x16\n\n\n\x02\x04\x01\x12\x04V\0k\x01\n\n\n\x03\x04\x01\x01\x12\x03V\x08 \n\x0B\n\x04\x04\x01\x02\0\x12\x03W\x02\x13\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03W\x02\x07\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03W\x08\x0E\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03W\x11\x12\n\x0B\n\x04\x04\x01\x02\x01\x12\x03X\x02\x13\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03X\x02\x07\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03X\x08\x0E\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03X\x11\x12\n\x0B\n\x04\x04\x01\x02\x02\x12\x03Y\x02\x13\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03Y\x02\x07\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03Y\x08\x0E\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03Y\x11\x12\n\x0B\n\x04\x04\x01\x02\x03\x12\x03Z\x02\x16\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03Z\x02\x08\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03Z\t\x10\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03Z\x13\x15\n\x0B\n\x04\x04\x01\x02\x04\x12\x03[\x02\x14\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03[\x02\x06\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03[\x07\x0E\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03[\x11\x13\n\x0B\n\x04\x04\x01\x02\x05\x12\x03\\\x02\x15\n\x0C\n\x05\x04\x01\x02\x05\x05\x12\x03\\\x02\x07\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03\\\x08\x0F\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03\\\x12\x14\n\x0B\n\x04\x04\x01\x02\x06\x12\x03]\x02\x15\n\x0C\n\x05\x04\x01\x02\x06\x05\x12\x03]\x02\x07\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03]\x08\x0F\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03]\x12\x14\n\x0B\n\x04\x04\x01\x02\x07\x12\x03^\x02\x15\n\x0C\n\x05\x04\x01\x02\x07\x05\x12\x03^\x02\x07\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03^\x08\x0F\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03^\x12\x14\n\x0B\n\x04\x04\x01\x02\x08\x12\x03_\x02\x15\n\x0C\n\x05\x04\x01\x02\x08\x05\x12\x03_\x02\x07\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03_\x08\x0F\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03_\x12\x14\n\x0B\n\x04\x04\x01\x02\t\x12\x03`\x02\x15\n\x0C\n\x05\x04\x01\x02\t\x05\x12\x03`\x02\x06\n\x0C\n\x05\x04\x01\x02\t\x01\x12\x03`\x07\x0E\n\x0C\n\x05\x04\x01\x02\t\x03\x12\x03`\x12\x14\n\x0B\n\x04\x04\x01\x02\n\x12\x03a\x02\x14\n\x0C\n\x05\x04\x01\x02\n\x05\x12\x03a\x02\x06\n\x0C\n\x05\x04\x01\x02\n\x01\x12\x03a\x07\x0E\n\x0C\n\x05\x04\x01\x02\n\x03\x12\x03a\x11\x13\n\x0B\n\x04\x04\x01\x02\x0B\x12\x03b\x02\x17\n\x0C\n\x05\x04\x01\x02\x0B\x05\x12\x03b\x02\t\n\x0C\n\x05\x04\x01\x02\x0B\x01\x12\x03b\n\x11\n\x0C\n\x05\x04\x01\x02\x0B\x03\x12\x03b\x14\x16\n\x0B\n\x04\x04\x01\x02\x0C\x12\x03c\x02\x15\n\x0C\n\x05\x04\x01\x02\x0C\x05\x12\x03c\x02\x07\n\x0C\n\x05\x04\x01\x02\x0C\x01\x12\x03c\x08\x0F\n\x0C\n\x05\x04\x01\x02\x0C\x03\x12\x03c\x12\x14\n\x0B\n\x04\x04\x01\x02\r\x12\x03d\x02\x14\n\x0C\n\x05\x04\x01\x02\r\x05\x12\x03d\x02\x06\n\x0C\n\x05\x04\x01\x02\r\x01\x12\x03d\x07\x0E\n\x0C\n\x05\x04\x01\x02\r\x03\x12\x03d\x11\x13\n\x0B\n\x04\x04\x01\x02\x0E\x12\x03e\x02\x16\n\x0C\n\x05\x04\x01\x02\x0E\x05\x12\x03e\x02\x06\n\x0C\n\x05\x04\x01\x02\x0E\x01\x12\x03e\x07\x0F\n\x0C\n\x05\x04\x01\x02\x0E\x03\x12\x03e\x12\x15\n\x0B\n\x04\x04\x01\x02\x0F\x12\x03f\x02\x19\n\x0C\n\x05\x04\x01\x02\x0F\x05\x12\x03f\x02\t\n\x0C\n\x05\x04\x01\x02\x0F\x01\x12\x03f\n\x12\n\x0C\n\x05\x04\x01\x02\x0F\x03\x12\x03f\x15\x18\n\x0B\n\x04\x04\x01\x02\x10\x12\x03g\x02\x17\n\x0C\n\x05\x04\x01\x02\x10\x05\x12\x03g\x02\x07\n\x0C\n\x05\x04\x01\x02\x10\x01\x12\x03g\x08\x10\n\x0C\n\x05\x04\x01\x02\x10\x03\x12\x03g\x13\x16\n\x0B\n\x04\x04\x01\x02\x11\x12\x03h\x02\x18\n\x0C\n\x05\x04\x01\x02\x11\x05\x12\x03h\x02\x08\n\x0C\n\x05\x04\x01\x02\x11\x01\x12\x03h\t\x11\n\x0C\n\x05\x04\x01\x02\x11\x03\x12\x03h\x14\x17\n\x0B\n\x04\x04\x01\x02\x12\x12\x03i\x02\x18\n\x0C\n\x05\x04\x01\x02\x12\x05\x12\x03i\x02\x08\n\x0C\n\x05\x04\x01\x02\x12\x01\x12\x03i\t\x11\n\x0C\n\x05\x04\x01\x02\x12\x03\x12\x03i\x14\x17\n\x0B\n\x04\x04\x01\x02\x13\x12\x03j\x02\x18\n\x0C\n\x05\x04\x01\x02\x13\x05\x12\x03j\x02\x08\n\x0C\n\x05\x04\x01\x02\x13\x01\x12\x03j\t\x11\n\x0C\n\x05\x04\x01\x02\x13\x03\x12\x03j\x14\x17b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
