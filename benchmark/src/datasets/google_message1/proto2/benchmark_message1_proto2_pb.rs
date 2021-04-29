#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage1 {
    pub field1: String,
    pub field9: Option<String>,
    pub field18: Option<String>,
    pub field80: Option<bool>,
    pub field81: Option<bool>,
    pub field2: i32,
    pub field3: i32,
    pub field280: Option<i32>,
    pub field6: Option<i32>,
    pub field22: Option<i64>,
    pub field4: Option<String>,
    pub field5: Vec<u64>,
    pub field59: Option<bool>,
    pub field7: Option<String>,
    pub field16: Option<i32>,
    pub field130: Option<i32>,
    pub field12: Option<bool>,
    pub field17: Option<bool>,
    pub field13: Option<bool>,
    pub field14: Option<bool>,
    pub field104: Option<i32>,
    pub field100: Option<i32>,
    pub field101: Option<i32>,
    pub field102: Option<String>,
    pub field103: Option<String>,
    pub field29: Option<i32>,
    pub field30: Option<bool>,
    pub field60: Option<i32>,
    pub field271: Option<i32>,
    pub field272: Option<i32>,
    pub field150: Option<i32>,
    pub field23: Option<i32>,
    pub field24: Option<bool>,
    pub field25: Option<i32>,
    pub field15: Option<GoogleMessage1SubMessage>,
    pub field78: Option<bool>,
    pub field67: Option<i32>,
    pub field68: Option<i32>,
    pub field128: Option<i32>,
    pub field129: Option<String>,
    pub field131: Option<i32>,
    _unknown: Vec<u8>,
}
impl GoogleMessage1 {
    pub const fn new() -> GoogleMessage1 {
        GoogleMessage1 {
            field1: String::new(),
            field9: None,
            field18: None,
            field80: None,
            field81: None,
            field2: 0,
            field3: 0,
            field280: None,
            field6: None,
            field22: None,
            field4: None,
            field5: Vec::new(),
            field59: None,
            field7: None,
            field16: None,
            field130: None,
            field12: None,
            field17: None,
            field13: None,
            field14: None,
            field104: None,
            field100: None,
            field101: None,
            field102: None,
            field103: None,
            field29: None,
            field30: None,
            field60: None,
            field271: None,
            field272: None,
            field150: None,
            field23: None,
            field24: None,
            field25: None,
            field15: None,
            field78: None,
            field67: None,
            field68: None,
            field128: None,
            field129: None,
            field131: None,
            _unknown: Vec::new(),
        }
    }
    pub fn field9(&self) -> &String {
        match &self.field9 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field9_mut(&mut self) -> &mut String {
        self.field9.get_or_insert_with(Default::default)
    }
    pub fn set_field9(&mut self, val: String) {
        self.field9 = Some(val);
    }
    pub fn field18(&self) -> &String {
        match &self.field18 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field18_mut(&mut self) -> &mut String {
        self.field18.get_or_insert_with(Default::default)
    }
    pub fn set_field18(&mut self, val: String) {
        self.field18 = Some(val);
    }
    pub fn field80(&self) -> bool {
        self.field80.unwrap_or_default()
    }
    pub fn field80_mut(&mut self) -> &mut bool {
        self.field80.get_or_insert_with(Default::default)
    }
    pub fn set_field80(&mut self, val: bool) {
        self.field80 = Some(val);
    }
    pub fn field81(&self) -> bool {
        self.field81.unwrap_or_default()
    }
    pub fn field81_mut(&mut self) -> &mut bool {
        self.field81.get_or_insert_with(Default::default)
    }
    pub fn set_field81(&mut self, val: bool) {
        self.field81 = Some(val);
    }
    pub fn field280(&self) -> i32 {
        self.field280.unwrap_or_default()
    }
    pub fn field280_mut(&mut self) -> &mut i32 {
        self.field280.get_or_insert_with(Default::default)
    }
    pub fn set_field280(&mut self, val: i32) {
        self.field280 = Some(val);
    }
    pub fn field6(&self) -> i32 {
        self.field6.unwrap_or_default()
    }
    pub fn field6_mut(&mut self) -> &mut i32 {
        self.field6.get_or_insert_with(Default::default)
    }
    pub fn set_field6(&mut self, val: i32) {
        self.field6 = Some(val);
    }
    pub fn field22(&self) -> i64 {
        self.field22.unwrap_or_default()
    }
    pub fn field22_mut(&mut self) -> &mut i64 {
        self.field22.get_or_insert_with(Default::default)
    }
    pub fn set_field22(&mut self, val: i64) {
        self.field22 = Some(val);
    }
    pub fn field4(&self) -> &String {
        match &self.field4 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field4_mut(&mut self) -> &mut String {
        self.field4.get_or_insert_with(Default::default)
    }
    pub fn set_field4(&mut self, val: String) {
        self.field4 = Some(val);
    }
    pub fn field59(&self) -> bool {
        self.field59.unwrap_or_default()
    }
    pub fn field59_mut(&mut self) -> &mut bool {
        self.field59.get_or_insert_with(Default::default)
    }
    pub fn set_field59(&mut self, val: bool) {
        self.field59 = Some(val);
    }
    pub fn field7(&self) -> &String {
        match &self.field7 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field7_mut(&mut self) -> &mut String {
        self.field7.get_or_insert_with(Default::default)
    }
    pub fn set_field7(&mut self, val: String) {
        self.field7 = Some(val);
    }
    pub fn field16(&self) -> i32 {
        self.field16.unwrap_or_default()
    }
    pub fn field16_mut(&mut self) -> &mut i32 {
        self.field16.get_or_insert_with(Default::default)
    }
    pub fn set_field16(&mut self, val: i32) {
        self.field16 = Some(val);
    }
    pub fn field130(&self) -> i32 {
        self.field130.unwrap_or_default()
    }
    pub fn field130_mut(&mut self) -> &mut i32 {
        self.field130.get_or_insert_with(Default::default)
    }
    pub fn set_field130(&mut self, val: i32) {
        self.field130 = Some(val);
    }
    pub fn field12(&self) -> bool {
        self.field12.unwrap_or_default()
    }
    pub fn field12_mut(&mut self) -> &mut bool {
        self.field12.get_or_insert_with(Default::default)
    }
    pub fn set_field12(&mut self, val: bool) {
        self.field12 = Some(val);
    }
    pub fn field17(&self) -> bool {
        self.field17.unwrap_or_default()
    }
    pub fn field17_mut(&mut self) -> &mut bool {
        self.field17.get_or_insert_with(Default::default)
    }
    pub fn set_field17(&mut self, val: bool) {
        self.field17 = Some(val);
    }
    pub fn field13(&self) -> bool {
        self.field13.unwrap_or_default()
    }
    pub fn field13_mut(&mut self) -> &mut bool {
        self.field13.get_or_insert_with(Default::default)
    }
    pub fn set_field13(&mut self, val: bool) {
        self.field13 = Some(val);
    }
    pub fn field14(&self) -> bool {
        self.field14.unwrap_or_default()
    }
    pub fn field14_mut(&mut self) -> &mut bool {
        self.field14.get_or_insert_with(Default::default)
    }
    pub fn set_field14(&mut self, val: bool) {
        self.field14 = Some(val);
    }
    pub fn field104(&self) -> i32 {
        self.field104.unwrap_or_default()
    }
    pub fn field104_mut(&mut self) -> &mut i32 {
        self.field104.get_or_insert_with(Default::default)
    }
    pub fn set_field104(&mut self, val: i32) {
        self.field104 = Some(val);
    }
    pub fn field100(&self) -> i32 {
        self.field100.unwrap_or_default()
    }
    pub fn field100_mut(&mut self) -> &mut i32 {
        self.field100.get_or_insert_with(Default::default)
    }
    pub fn set_field100(&mut self, val: i32) {
        self.field100 = Some(val);
    }
    pub fn field101(&self) -> i32 {
        self.field101.unwrap_or_default()
    }
    pub fn field101_mut(&mut self) -> &mut i32 {
        self.field101.get_or_insert_with(Default::default)
    }
    pub fn set_field101(&mut self, val: i32) {
        self.field101 = Some(val);
    }
    pub fn field102(&self) -> &String {
        match &self.field102 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field102_mut(&mut self) -> &mut String {
        self.field102.get_or_insert_with(Default::default)
    }
    pub fn set_field102(&mut self, val: String) {
        self.field102 = Some(val);
    }
    pub fn field103(&self) -> &String {
        match &self.field103 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field103_mut(&mut self) -> &mut String {
        self.field103.get_or_insert_with(Default::default)
    }
    pub fn set_field103(&mut self, val: String) {
        self.field103 = Some(val);
    }
    pub fn field29(&self) -> i32 {
        self.field29.unwrap_or_default()
    }
    pub fn field29_mut(&mut self) -> &mut i32 {
        self.field29.get_or_insert_with(Default::default)
    }
    pub fn set_field29(&mut self, val: i32) {
        self.field29 = Some(val);
    }
    pub fn field30(&self) -> bool {
        self.field30.unwrap_or_default()
    }
    pub fn field30_mut(&mut self) -> &mut bool {
        self.field30.get_or_insert_with(Default::default)
    }
    pub fn set_field30(&mut self, val: bool) {
        self.field30 = Some(val);
    }
    pub fn field60(&self) -> i32 {
        self.field60.unwrap_or_default()
    }
    pub fn field60_mut(&mut self) -> &mut i32 {
        self.field60.get_or_insert_with(Default::default)
    }
    pub fn set_field60(&mut self, val: i32) {
        self.field60 = Some(val);
    }
    pub fn field271(&self) -> i32 {
        self.field271.unwrap_or_default()
    }
    pub fn field271_mut(&mut self) -> &mut i32 {
        self.field271.get_or_insert_with(Default::default)
    }
    pub fn set_field271(&mut self, val: i32) {
        self.field271 = Some(val);
    }
    pub fn field272(&self) -> i32 {
        self.field272.unwrap_or_default()
    }
    pub fn field272_mut(&mut self) -> &mut i32 {
        self.field272.get_or_insert_with(Default::default)
    }
    pub fn set_field272(&mut self, val: i32) {
        self.field272 = Some(val);
    }
    pub fn field150(&self) -> i32 {
        self.field150.unwrap_or_default()
    }
    pub fn field150_mut(&mut self) -> &mut i32 {
        self.field150.get_or_insert_with(Default::default)
    }
    pub fn set_field150(&mut self, val: i32) {
        self.field150 = Some(val);
    }
    pub fn field23(&self) -> i32 {
        self.field23.unwrap_or_default()
    }
    pub fn field23_mut(&mut self) -> &mut i32 {
        self.field23.get_or_insert_with(Default::default)
    }
    pub fn set_field23(&mut self, val: i32) {
        self.field23 = Some(val);
    }
    pub fn field24(&self) -> bool {
        self.field24.unwrap_or_default()
    }
    pub fn field24_mut(&mut self) -> &mut bool {
        self.field24.get_or_insert_with(Default::default)
    }
    pub fn set_field24(&mut self, val: bool) {
        self.field24 = Some(val);
    }
    pub fn field25(&self) -> i32 {
        self.field25.unwrap_or_default()
    }
    pub fn field25_mut(&mut self) -> &mut i32 {
        self.field25.get_or_insert_with(Default::default)
    }
    pub fn set_field25(&mut self, val: i32) {
        self.field25 = Some(val);
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
    pub fn field78(&self) -> bool {
        self.field78.unwrap_or_default()
    }
    pub fn field78_mut(&mut self) -> &mut bool {
        self.field78.get_or_insert_with(Default::default)
    }
    pub fn set_field78(&mut self, val: bool) {
        self.field78 = Some(val);
    }
    pub fn field67(&self) -> i32 {
        self.field67.unwrap_or_default()
    }
    pub fn field67_mut(&mut self) -> &mut i32 {
        self.field67.get_or_insert_with(Default::default)
    }
    pub fn set_field67(&mut self, val: i32) {
        self.field67 = Some(val);
    }
    pub fn field68(&self) -> i32 {
        self.field68.unwrap_or_default()
    }
    pub fn field68_mut(&mut self) -> &mut i32 {
        self.field68.get_or_insert_with(Default::default)
    }
    pub fn set_field68(&mut self, val: i32) {
        self.field68 = Some(val);
    }
    pub fn field128(&self) -> i32 {
        self.field128.unwrap_or_default()
    }
    pub fn field128_mut(&mut self) -> &mut i32 {
        self.field128.get_or_insert_with(Default::default)
    }
    pub fn set_field128(&mut self, val: i32) {
        self.field128 = Some(val);
    }
    pub fn field129(&self) -> &String {
        match &self.field129 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field129_mut(&mut self) -> &mut String {
        self.field129.get_or_insert_with(Default::default)
    }
    pub fn set_field129(&mut self, val: String) {
        self.field129 = Some(val);
    }
    pub fn field131(&self) -> i32 {
        self.field131.unwrap_or_default()
    }
    pub fn field131_mut(&mut self) -> &mut i32 {
        self.field131.get_or_insert_with(Default::default)
    }
    pub fn set_field131(&mut self, val: i32) {
        self.field131 = Some(val);
    }
}
impl pecan::Message for GoogleMessage1 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field1 = LengthPrefixed::read_from(s)?,
                16 => self.field2 = Varint::read_from(s)?,
                24 => self.field3 = Varint::read_from(s)?,
                34 => self.field4 = Some(LengthPrefixed::read_from(s)?),
                41 => CopyArray::<Fixed64>::merge_from(&mut self.field5, s)?,
                42 => PackedArray::<Fixed64>::merge_from(&mut self.field5, s)?,
                48 => self.field6 = Some(Varint::read_from(s)?),
                58 => self.field7 = Some(LengthPrefixed::read_from(s)?),
                74 => self.field9 = Some(LengthPrefixed::read_from(s)?),
                96 => self.field12 = Some(Varint::read_from(s)?),
                104 => self.field13 = Some(Varint::read_from(s)?),
                112 => self.field14 = Some(Varint::read_from(s)?),
                122 => LengthPrefixed::merge_from(self.field15_mut(), s)?,
                128 => self.field16 = Some(Varint::read_from(s)?),
                136 => self.field17 = Some(Varint::read_from(s)?),
                146 => self.field18 = Some(LengthPrefixed::read_from(s)?),
                176 => self.field22 = Some(Varint::read_from(s)?),
                184 => self.field23 = Some(Varint::read_from(s)?),
                192 => self.field24 = Some(Varint::read_from(s)?),
                200 => self.field25 = Some(Varint::read_from(s)?),
                232 => self.field29 = Some(Varint::read_from(s)?),
                240 => self.field30 = Some(Varint::read_from(s)?),
                472 => self.field59 = Some(Varint::read_from(s)?),
                480 => self.field60 = Some(Varint::read_from(s)?),
                536 => self.field67 = Some(Varint::read_from(s)?),
                544 => self.field68 = Some(Varint::read_from(s)?),
                624 => self.field78 = Some(Varint::read_from(s)?),
                640 => self.field80 = Some(Varint::read_from(s)?),
                648 => self.field81 = Some(Varint::read_from(s)?),
                800 => self.field100 = Some(Varint::read_from(s)?),
                808 => self.field101 = Some(Varint::read_from(s)?),
                818 => self.field102 = Some(LengthPrefixed::read_from(s)?),
                826 => self.field103 = Some(LengthPrefixed::read_from(s)?),
                832 => self.field104 = Some(Varint::read_from(s)?),
                1024 => self.field128 = Some(Varint::read_from(s)?),
                1034 => self.field129 = Some(LengthPrefixed::read_from(s)?),
                1040 => self.field130 = Some(Varint::read_from(s)?),
                1048 => self.field131 = Some(Varint::read_from(s)?),
                1200 => self.field150 = Some(Varint::read_from(s)?),
                2168 => self.field271 = Some(Varint::read_from(s)?),
                2176 => self.field272 = Some(Varint::read_from(s)?),
                2240 => self.field280 = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
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
        if let Some(v) = &self.field4 {
            s.write_tag(34)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.field5.is_empty() {
            for i in &self.field5 {
                s.write_tag(41)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if let Some(v) = self.field6 {
            s.write_tag(48)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field7 {
            s.write_tag(58)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field9 {
            s.write_tag(74)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field12 {
            s.write_tag(96)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field13 {
            s.write_tag(104)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field14 {
            s.write_tag(112)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field15 {
            s.write_tag(122)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field16 {
            s.write_tag(128)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field17 {
            s.write_tag(136)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field18 {
            s.write_tag(146)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field22 {
            s.write_tag(176)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field23 {
            s.write_tag(184)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field24 {
            s.write_tag(192)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field25 {
            s.write_tag(200)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field29 {
            s.write_tag(232)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field30 {
            s.write_tag(240)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field59 {
            s.write_tag(472)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field60 {
            s.write_tag(480)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field67 {
            s.write_tag(536)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field68 {
            s.write_tag(544)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field78 {
            s.write_tag(624)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field80 {
            s.write_tag(640)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field81 {
            s.write_tag(648)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field100 {
            s.write_tag(800)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field101 {
            s.write_tag(808)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field102 {
            s.write_tag(818)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field103 {
            s.write_tag(826)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field104 {
            s.write_tag(832)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field128 {
            s.write_tag(1024)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field129 {
            s.write_tag(1034)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field130 {
            s.write_tag(1040)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field131 {
            s.write_tag(1048)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field150 {
            s.write_tag(1200)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field271 {
            s.write_tag(2168)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field272 {
            s.write_tag(2176)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field280 {
            s.write_tag(2240)?;
            Varint::write_to(v, s)?;
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
        if let Some(v) = &self.field4 {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.field5.is_empty() {
            l += self.field5.len() as u64 + CopyArray::<Fixed64>::size(&self.field5);
        }
        if let Some(v) = self.field6 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.field7 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field9 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field12 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field13 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field14 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.field15 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field16 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field17 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.field18 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field22 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field23 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field24 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field25 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field29 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field30 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field59 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field60 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field67 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field68 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field78 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field80 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field81 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field100 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field101 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.field102 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field103 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field104 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field128 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.field129 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field130 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field131 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field150 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field271 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field272 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field280 {
            l += 2 + Varint::size(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
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
    pub field1: Option<i32>,
    pub field2: Option<i32>,
    pub field3: Option<i32>,
    pub field15: Option<String>,
    pub field12: Option<bool>,
    pub field13: Option<i64>,
    pub field14: Option<i64>,
    pub field16: Option<i32>,
    pub field19: Option<i32>,
    pub field20: Option<bool>,
    pub field28: Option<bool>,
    pub field21: Option<u64>,
    pub field22: Option<i32>,
    pub field23: Option<bool>,
    pub field206: Option<bool>,
    pub field203: Option<u32>,
    pub field204: Option<i32>,
    pub field205: Option<String>,
    pub field207: Option<u64>,
    pub field300: Option<u64>,
    _unknown: Vec<u8>,
}
impl GoogleMessage1SubMessage {
    pub const fn new() -> GoogleMessage1SubMessage {
        GoogleMessage1SubMessage {
            field1: None,
            field2: None,
            field3: None,
            field15: None,
            field12: None,
            field13: None,
            field14: None,
            field16: None,
            field19: None,
            field20: None,
            field28: None,
            field21: None,
            field22: None,
            field23: None,
            field206: None,
            field203: None,
            field204: None,
            field205: None,
            field207: None,
            field300: None,
            _unknown: Vec::new(),
        }
    }
    pub fn field1(&self) -> i32 {
        self.field1.unwrap_or_default()
    }
    pub fn field1_mut(&mut self) -> &mut i32 {
        self.field1.get_or_insert_with(Default::default)
    }
    pub fn set_field1(&mut self, val: i32) {
        self.field1 = Some(val);
    }
    pub fn field2(&self) -> i32 {
        self.field2.unwrap_or_default()
    }
    pub fn field2_mut(&mut self) -> &mut i32 {
        self.field2.get_or_insert_with(Default::default)
    }
    pub fn set_field2(&mut self, val: i32) {
        self.field2 = Some(val);
    }
    pub fn field3(&self) -> i32 {
        self.field3.unwrap_or_default()
    }
    pub fn field3_mut(&mut self) -> &mut i32 {
        self.field3.get_or_insert_with(Default::default)
    }
    pub fn set_field3(&mut self, val: i32) {
        self.field3 = Some(val);
    }
    pub fn field15(&self) -> &String {
        match &self.field15 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field15_mut(&mut self) -> &mut String {
        self.field15.get_or_insert_with(Default::default)
    }
    pub fn set_field15(&mut self, val: String) {
        self.field15 = Some(val);
    }
    pub fn field12(&self) -> bool {
        self.field12.unwrap_or_default()
    }
    pub fn field12_mut(&mut self) -> &mut bool {
        self.field12.get_or_insert_with(Default::default)
    }
    pub fn set_field12(&mut self, val: bool) {
        self.field12 = Some(val);
    }
    pub fn field13(&self) -> i64 {
        self.field13.unwrap_or_default()
    }
    pub fn field13_mut(&mut self) -> &mut i64 {
        self.field13.get_or_insert_with(Default::default)
    }
    pub fn set_field13(&mut self, val: i64) {
        self.field13 = Some(val);
    }
    pub fn field14(&self) -> i64 {
        self.field14.unwrap_or_default()
    }
    pub fn field14_mut(&mut self) -> &mut i64 {
        self.field14.get_or_insert_with(Default::default)
    }
    pub fn set_field14(&mut self, val: i64) {
        self.field14 = Some(val);
    }
    pub fn field16(&self) -> i32 {
        self.field16.unwrap_or_default()
    }
    pub fn field16_mut(&mut self) -> &mut i32 {
        self.field16.get_or_insert_with(Default::default)
    }
    pub fn set_field16(&mut self, val: i32) {
        self.field16 = Some(val);
    }
    pub fn field19(&self) -> i32 {
        self.field19.unwrap_or_default()
    }
    pub fn field19_mut(&mut self) -> &mut i32 {
        self.field19.get_or_insert_with(Default::default)
    }
    pub fn set_field19(&mut self, val: i32) {
        self.field19 = Some(val);
    }
    pub fn field20(&self) -> bool {
        self.field20.unwrap_or_default()
    }
    pub fn field20_mut(&mut self) -> &mut bool {
        self.field20.get_or_insert_with(Default::default)
    }
    pub fn set_field20(&mut self, val: bool) {
        self.field20 = Some(val);
    }
    pub fn field28(&self) -> bool {
        self.field28.unwrap_or_default()
    }
    pub fn field28_mut(&mut self) -> &mut bool {
        self.field28.get_or_insert_with(Default::default)
    }
    pub fn set_field28(&mut self, val: bool) {
        self.field28 = Some(val);
    }
    pub fn field21(&self) -> u64 {
        self.field21.unwrap_or_default()
    }
    pub fn field21_mut(&mut self) -> &mut u64 {
        self.field21.get_or_insert_with(Default::default)
    }
    pub fn set_field21(&mut self, val: u64) {
        self.field21 = Some(val);
    }
    pub fn field22(&self) -> i32 {
        self.field22.unwrap_or_default()
    }
    pub fn field22_mut(&mut self) -> &mut i32 {
        self.field22.get_or_insert_with(Default::default)
    }
    pub fn set_field22(&mut self, val: i32) {
        self.field22 = Some(val);
    }
    pub fn field23(&self) -> bool {
        self.field23.unwrap_or_default()
    }
    pub fn field23_mut(&mut self) -> &mut bool {
        self.field23.get_or_insert_with(Default::default)
    }
    pub fn set_field23(&mut self, val: bool) {
        self.field23 = Some(val);
    }
    pub fn field206(&self) -> bool {
        self.field206.unwrap_or_default()
    }
    pub fn field206_mut(&mut self) -> &mut bool {
        self.field206.get_or_insert_with(Default::default)
    }
    pub fn set_field206(&mut self, val: bool) {
        self.field206 = Some(val);
    }
    pub fn field203(&self) -> u32 {
        self.field203.unwrap_or_default()
    }
    pub fn field203_mut(&mut self) -> &mut u32 {
        self.field203.get_or_insert_with(Default::default)
    }
    pub fn set_field203(&mut self, val: u32) {
        self.field203 = Some(val);
    }
    pub fn field204(&self) -> i32 {
        self.field204.unwrap_or_default()
    }
    pub fn field204_mut(&mut self) -> &mut i32 {
        self.field204.get_or_insert_with(Default::default)
    }
    pub fn set_field204(&mut self, val: i32) {
        self.field204 = Some(val);
    }
    pub fn field205(&self) -> &String {
        match &self.field205 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field205_mut(&mut self) -> &mut String {
        self.field205.get_or_insert_with(Default::default)
    }
    pub fn set_field205(&mut self, val: String) {
        self.field205 = Some(val);
    }
    pub fn field207(&self) -> u64 {
        self.field207.unwrap_or_default()
    }
    pub fn field207_mut(&mut self) -> &mut u64 {
        self.field207.get_or_insert_with(Default::default)
    }
    pub fn set_field207(&mut self, val: u64) {
        self.field207 = Some(val);
    }
    pub fn field300(&self) -> u64 {
        self.field300.unwrap_or_default()
    }
    pub fn field300_mut(&mut self) -> &mut u64 {
        self.field300.get_or_insert_with(Default::default)
    }
    pub fn set_field300(&mut self, val: u64) {
        self.field300 = Some(val);
    }
}
impl pecan::Message for GoogleMessage1SubMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.field1 = Some(Varint::read_from(s)?),
                16 => self.field2 = Some(Varint::read_from(s)?),
                24 => self.field3 = Some(Varint::read_from(s)?),
                96 => self.field12 = Some(Varint::read_from(s)?),
                104 => self.field13 = Some(Varint::read_from(s)?),
                112 => self.field14 = Some(Varint::read_from(s)?),
                122 => self.field15 = Some(LengthPrefixed::read_from(s)?),
                128 => self.field16 = Some(Varint::read_from(s)?),
                152 => self.field19 = Some(Varint::read_from(s)?),
                160 => self.field20 = Some(Varint::read_from(s)?),
                169 => self.field21 = Some(Fixed64::read_from(s)?),
                176 => self.field22 = Some(Varint::read_from(s)?),
                184 => self.field23 = Some(Varint::read_from(s)?),
                224 => self.field28 = Some(Varint::read_from(s)?),
                1629 => self.field203 = Some(Fixed32::read_from(s)?),
                1632 => self.field204 = Some(Varint::read_from(s)?),
                1642 => self.field205 = Some(LengthPrefixed::read_from(s)?),
                1648 => self.field206 = Some(Varint::read_from(s)?),
                1656 => self.field207 = Some(Varint::read_from(s)?),
                2400 => self.field300 = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.field1 {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field2 {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field3 {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field12 {
            s.write_tag(96)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field13 {
            s.write_tag(104)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field14 {
            s.write_tag(112)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field15 {
            s.write_tag(122)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field16 {
            s.write_tag(128)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field19 {
            s.write_tag(152)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field20 {
            s.write_tag(160)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field21 {
            s.write_tag(169)?;
            Fixed64::write_to(v, s)?;
        }
        if let Some(v) = self.field22 {
            s.write_tag(176)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field23 {
            s.write_tag(184)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field28 {
            s.write_tag(224)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field203 {
            s.write_tag(1629)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field204 {
            s.write_tag(1632)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field205 {
            s.write_tag(1642)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field206 {
            s.write_tag(1648)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field207 {
            s.write_tag(1656)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field300 {
            s.write_tag(2400)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.field1 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field2 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field3 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field12 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field13 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field14 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.field15 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field16 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field19 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field20 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field21 {
            l += 2 + Fixed64::size(v);
        }
        if let Some(v) = self.field22 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field23 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field28 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field203 {
            l += 2 + Fixed32::size(v);
        }
        if let Some(v) = self.field204 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.field205 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field206 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field207 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field300 {
            l += 2 + Varint::size(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
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
static DESCRIPTOR_RAW : & [u8] = b"\n?datasets/google_message1/proto2/benchmark_message1_proto2.proto\x12\x11benchmarks.proto2\"\xF7\t\n\x0EGoogleMessage1\x12\x16\n\x06field1\x18\x01 \x02(\tR\x06field1\x12\x16\n\x06field9\x18\t \x01(\tR\x06field9\x12\x18\n\x07field18\x18\x12 \x01(\tR\x07field18\x12\x1F\n\x07field80\x18P \x01(\x08:\x05falseR\x07field80\x12\x1E\n\x07field81\x18Q \x01(\x08:\x04trueR\x07field81\x12\x16\n\x06field2\x18\x02 \x02(\x05R\x06field2\x12\x16\n\x06field3\x18\x03 \x02(\x05R\x06field3\x12\x1B\n\x08field280\x18\x98\x02 \x01(\x05R\x08field280\x12\x19\n\x06field6\x18\x06 \x01(\x05:\x010R\x06field6\x12\x18\n\x07field22\x18\x16 \x01(\x03R\x07field22\x12\x16\n\x06field4\x18\x04 \x01(\tR\x06field4\x12\x16\n\x06field5\x18\x05 \x03(\x06R\x06field5\x12\x1F\n\x07field59\x18; \x01(\x08:\x05falseR\x07field59\x12\x16\n\x06field7\x18\x07 \x01(\tR\x06field7\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x1E\n\x08field130\x18\x82\x01 \x01(\x05:\x010R\x08field130\x12\x1E\n\x07field12\x18\x0C \x01(\x08:\x04trueR\x07field12\x12\x1E\n\x07field17\x18\x11 \x01(\x08:\x04trueR\x07field17\x12\x1E\n\x07field13\x18\r \x01(\x08:\x04trueR\x07field13\x12\x1E\n\x07field14\x18\x0E \x01(\x08:\x04trueR\x07field14\x12\x1D\n\x08field104\x18h \x01(\x05:\x010R\x08field104\x12\x1D\n\x08field100\x18d \x01(\x05:\x010R\x08field100\x12\x1D\n\x08field101\x18e \x01(\x05:\x010R\x08field101\x12\x1A\n\x08field102\x18f \x01(\tR\x08field102\x12\x1A\n\x08field103\x18g \x01(\tR\x08field103\x12\x1B\n\x07field29\x18\x1D \x01(\x05:\x010R\x07field29\x12\x1F\n\x07field30\x18\x1E \x01(\x08:\x05falseR\x07field30\x12\x1C\n\x07field60\x18< \x01(\x05:\x02-1R\x07field60\x12\x1F\n\x08field271\x18\x8F\x02 \x01(\x05:\x02-1R\x08field271\x12\x1F\n\x08field272\x18\x90\x02 \x01(\x05:\x02-1R\x08field272\x12\x1B\n\x08field150\x18\x96\x01 \x01(\x05R\x08field150\x12\x1B\n\x07field23\x18\x17 \x01(\x05:\x010R\x07field23\x12\x1F\n\x07field24\x18\x18 \x01(\x08:\x05falseR\x07field24\x12\x1B\n\x07field25\x18\x19 \x01(\x05:\x010R\x07field25\x12E\n\x07field15\x18\x0F \x01(\x0B2+.benchmarks.proto2.GoogleMessage1SubMessageR\x07field15\x12\x18\n\x07field78\x18N \x01(\x08R\x07field78\x12\x1B\n\x07field67\x18C \x01(\x05:\x010R\x07field67\x12\x18\n\x07field68\x18D \x01(\x05R\x07field68\x12\x1E\n\x08field128\x18\x80\x01 \x01(\x05:\x010R\x08field128\x122\n\x08field129\x18\x81\x01 \x01(\t:\x15xxxxxxxxxxxxxxxxxxxxxR\x08field129\x12\x1E\n\x08field131\x18\x83\x01 \x01(\x05:\x010R\x08field131\"\xDA\x04\n\x18GoogleMessage1SubMessage\x12\x19\n\x06field1\x18\x01 \x01(\x05:\x010R\x06field1\x12\x19\n\x06field2\x18\x02 \x01(\x05:\x010R\x06field2\x12\x19\n\x06field3\x18\x03 \x01(\x05:\x010R\x06field3\x12\x18\n\x07field15\x18\x0F \x01(\tR\x07field15\x12\x1E\n\x07field12\x18\x0C \x01(\x08:\x04trueR\x07field12\x12\x18\n\x07field13\x18\r \x01(\x03R\x07field13\x12\x18\n\x07field14\x18\x0E \x01(\x03R\x07field14\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x1B\n\x07field19\x18\x13 \x01(\x05:\x012R\x07field19\x12\x1E\n\x07field20\x18\x14 \x01(\x08:\x04trueR\x07field20\x12\x1E\n\x07field28\x18\x1C \x01(\x08:\x04trueR\x07field28\x12\x18\n\x07field21\x18\x15 \x01(\x06R\x07field21\x12\x18\n\x07field22\x18\x16 \x01(\x05R\x07field22\x12\x1F\n\x07field23\x18\x17 \x01(\x08:\x05falseR\x07field23\x12\"\n\x08field206\x18\xCE\x01 \x01(\x08:\x05falseR\x08field206\x12\x1B\n\x08field203\x18\xCB\x01 \x01(\x07R\x08field203\x12\x1B\n\x08field204\x18\xCC\x01 \x01(\x05R\x08field204\x12\x1B\n\x08field205\x18\xCD\x01 \x01(\tR\x08field205\x12\x1B\n\x08field207\x18\xCF\x01 \x01(\x04R\x08field207\x12\x1B\n\x08field300\x18\xAC\x02 \x01(\x04R\x08field300B%\n\x1Ecom.google.protobuf.benchmarksH\x01\xF8\x01\x01J\xBA6\n\x06\x12\x04 \0k\x01\n\xEE\x0C\n\x01\x0C\x12\x03 \0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2  Benchmark messages for proto2.\n\n\x08\n\x01\x02\x12\x03\"\0\x1A\n\x08\n\x01\x08\x12\x03#\07\n\t\n\x02\x08\x01\x12\x03#\07\n\x08\n\x01\x08\x12\x03&\0\x1C\nD\n\x02\x08\t\x12\x03&\0\x1C\x1A9 This is the default, but we specify it here explicitly.\n\n\x08\n\x01\x08\x12\x03(\0\x1F\n\t\n\x02\x08\x1F\x12\x03(\0\x1F\n\n\n\x02\x04\0\x12\x04*\0T\x01\n\n\n\x03\x04\0\x01\x12\x03*\x08\x16\n\x0B\n\x04\x04\0\x02\0\x12\x03+\x02\x1D\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03+\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03+\x0B\x11\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03+\x12\x18\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03+\x1B\x1C\n\x0B\n\x04\x04\0\x02\x01\x12\x03,\x02\x1D\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03,\x02\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03,\x0B\x11\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03,\x12\x18\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03,\x1B\x1C\n\x0B\n\x04\x04\0\x02\x02\x12\x03-\x02\x1F\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03-\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03-\x0B\x11\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03-\x12\x19\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03-\x1C\x1E\n\x0B\n\x04\x04\0\x02\x03\x12\x03.\x02/\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x03.\x02\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03.\x0B\x0F\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03.\x10\x17\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03.\x1A\x1C\n\x0C\n\x05\x04\0\x02\x03\x08\x12\x03.\x1D.\n\x0C\n\x05\x04\0\x02\x03\x07\x12\x03.(-\n\x0B\n\x04\x04\0\x02\x04\x12\x03/\x02.\n\x0C\n\x05\x04\0\x02\x04\x04\x12\x03/\x02\n\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x03/\x0B\x0F\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03/\x10\x17\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03/\x1A\x1C\n\x0C\n\x05\x04\0\x02\x04\x08\x12\x03/\x1D-\n\x0C\n\x05\x04\0\x02\x04\x07\x12\x03/(,\n\x0B\n\x04\x04\0\x02\x05\x12\x030\x02\x1C\n\x0C\n\x05\x04\0\x02\x05\x04\x12\x030\x02\n\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x030\x0B\x10\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x030\x11\x17\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x030\x1A\x1B\n\x0B\n\x04\x04\0\x02\x06\x12\x031\x02\x1C\n\x0C\n\x05\x04\0\x02\x06\x04\x12\x031\x02\n\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x031\x0B\x10\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x031\x11\x17\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x031\x1A\x1B\n\x0B\n\x04\x04\0\x02\x07\x12\x032\x02 \n\x0C\n\x05\x04\0\x02\x07\x04\x12\x032\x02\n\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x032\x0B\x10\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x032\x11\x19\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x032\x1C\x1F\n\x0B\n\x04\x04\0\x02\x08\x12\x033\x02*\n\x0C\n\x05\x04\0\x02\x08\x04\x12\x033\x02\n\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x033\x0B\x10\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x033\x11\x17\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x033\x1A\x1B\n\x0C\n\x05\x04\0\x02\x08\x08\x12\x033\x1C)\n\x0C\n\x05\x04\0\x02\x08\x07\x12\x033'(\n\x0B\n\x04\x04\0\x02\t\x12\x034\x02\x1E\n\x0C\n\x05\x04\0\x02\t\x04\x12\x034\x02\n\n\x0C\n\x05\x04\0\x02\t\x05\x12\x034\x0B\x10\n\x0C\n\x05\x04\0\x02\t\x01\x12\x034\x11\x18\n\x0C\n\x05\x04\0\x02\t\x03\x12\x034\x1B\x1D\n\x0B\n\x04\x04\0\x02\n\x12\x035\x02\x1D\n\x0C\n\x05\x04\0\x02\n\x04\x12\x035\x02\n\n\x0C\n\x05\x04\0\x02\n\x05\x12\x035\x0B\x11\n\x0C\n\x05\x04\0\x02\n\x01\x12\x035\x12\x18\n\x0C\n\x05\x04\0\x02\n\x03\x12\x035\x1B\x1C\n\x0B\n\x04\x04\0\x02\x0B\x12\x036\x02\x1E\n\x0C\n\x05\x04\0\x02\x0B\x04\x12\x036\x02\n\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x036\x0B\x12\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x036\x13\x19\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x036\x1C\x1D\n\x0B\n\x04\x04\0\x02\x0C\x12\x037\x02/\n\x0C\n\x05\x04\0\x02\x0C\x04\x12\x037\x02\n\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x037\x0B\x0F\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x037\x10\x17\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x037\x1A\x1C\n\x0C\n\x05\x04\0\x02\x0C\x08\x12\x037\x1D.\n\x0C\n\x05\x04\0\x02\x0C\x07\x12\x037(-\n\x0B\n\x04\x04\0\x02\r\x12\x038\x02\x1D\n\x0C\n\x05\x04\0\x02\r\x04\x12\x038\x02\n\n\x0C\n\x05\x04\0\x02\r\x05\x12\x038\x0B\x11\n\x0C\n\x05\x04\0\x02\r\x01\x12\x038\x12\x18\n\x0C\n\x05\x04\0\x02\r\x03\x12\x038\x1B\x1C\n\x0B\n\x04\x04\0\x02\x0E\x12\x039\x02\x1E\n\x0C\n\x05\x04\0\x02\x0E\x04\x12\x039\x02\n\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x039\x0B\x10\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x039\x11\x18\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x039\x1B\x1D\n\x0B\n\x04\x04\0\x02\x0F\x12\x03:\x02.\n\x0C\n\x05\x04\0\x02\x0F\x04\x12\x03:\x02\n\n\x0C\n\x05\x04\0\x02\x0F\x05\x12\x03:\x0B\x10\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03:\x11\x19\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03:\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0F\x08\x12\x03: -\n\x0C\n\x05\x04\0\x02\x0F\x07\x12\x03:+,\n\x0B\n\x04\x04\0\x02\x10\x12\x03;\x02.\n\x0C\n\x05\x04\0\x02\x10\x04\x12\x03;\x02\n\n\x0C\n\x05\x04\0\x02\x10\x05\x12\x03;\x0B\x0F\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03;\x10\x17\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03;\x1A\x1C\n\x0C\n\x05\x04\0\x02\x10\x08\x12\x03;\x1D-\n\x0C\n\x05\x04\0\x02\x10\x07\x12\x03;(,\n\x0B\n\x04\x04\0\x02\x11\x12\x03<\x02.\n\x0C\n\x05\x04\0\x02\x11\x04\x12\x03<\x02\n\n\x0C\n\x05\x04\0\x02\x11\x05\x12\x03<\x0B\x0F\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03<\x10\x17\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03<\x1A\x1C\n\x0C\n\x05\x04\0\x02\x11\x08\x12\x03<\x1D-\n\x0C\n\x05\x04\0\x02\x11\x07\x12\x03<(,\n\x0B\n\x04\x04\0\x02\x12\x12\x03=\x02.\n\x0C\n\x05\x04\0\x02\x12\x04\x12\x03=\x02\n\n\x0C\n\x05\x04\0\x02\x12\x05\x12\x03=\x0B\x0F\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03=\x10\x17\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03=\x1A\x1C\n\x0C\n\x05\x04\0\x02\x12\x08\x12\x03=\x1D-\n\x0C\n\x05\x04\0\x02\x12\x07\x12\x03=(,\n\x0B\n\x04\x04\0\x02\x13\x12\x03>\x02.\n\x0C\n\x05\x04\0\x02\x13\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\0\x02\x13\x05\x12\x03>\x0B\x0F\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03>\x10\x17\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03>\x1A\x1C\n\x0C\n\x05\x04\0\x02\x13\x08\x12\x03>\x1D-\n\x0C\n\x05\x04\0\x02\x13\x07\x12\x03>(,\n\x0B\n\x04\x04\0\x02\x14\x12\x03?\x02.\n\x0C\n\x05\x04\0\x02\x14\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03?\x0B\x10\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03?\x11\x19\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03?\x1C\x1F\n\x0C\n\x05\x04\0\x02\x14\x08\x12\x03? -\n\x0C\n\x05\x04\0\x02\x14\x07\x12\x03?+,\n\x0B\n\x04\x04\0\x02\x15\x12\x03@\x02.\n\x0C\n\x05\x04\0\x02\x15\x04\x12\x03@\x02\n\n\x0C\n\x05\x04\0\x02\x15\x05\x12\x03@\x0B\x10\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03@\x11\x19\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03@\x1C\x1F\n\x0C\n\x05\x04\0\x02\x15\x08\x12\x03@ -\n\x0C\n\x05\x04\0\x02\x15\x07\x12\x03@+,\n\x0B\n\x04\x04\0\x02\x16\x12\x03A\x02.\n\x0C\n\x05\x04\0\x02\x16\x04\x12\x03A\x02\n\n\x0C\n\x05\x04\0\x02\x16\x05\x12\x03A\x0B\x10\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03A\x11\x19\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03A\x1C\x1F\n\x0C\n\x05\x04\0\x02\x16\x08\x12\x03A -\n\x0C\n\x05\x04\0\x02\x16\x07\x12\x03A+,\n\x0B\n\x04\x04\0\x02\x17\x12\x03B\x02!\n\x0C\n\x05\x04\0\x02\x17\x04\x12\x03B\x02\n\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03B\x0B\x11\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03B\x12\x1A\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03B\x1D \n\x0B\n\x04\x04\0\x02\x18\x12\x03C\x02!\n\x0C\n\x05\x04\0\x02\x18\x04\x12\x03C\x02\n\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03C\x0B\x11\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03C\x12\x1A\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03C\x1D \n\x0B\n\x04\x04\0\x02\x19\x12\x03D\x02,\n\x0C\n\x05\x04\0\x02\x19\x04\x12\x03D\x02\n\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03D\x0B\x10\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03D\x11\x18\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03D\x1B\x1D\n\x0C\n\x05\x04\0\x02\x19\x08\x12\x03D\x1E+\n\x0C\n\x05\x04\0\x02\x19\x07\x12\x03D)*\n\x0B\n\x04\x04\0\x02\x1A\x12\x03E\x02/\n\x0C\n\x05\x04\0\x02\x1A\x04\x12\x03E\x02\n\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03E\x0B\x0F\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03E\x10\x17\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03E\x1A\x1C\n\x0C\n\x05\x04\0\x02\x1A\x08\x12\x03E\x1D.\n\x0C\n\x05\x04\0\x02\x1A\x07\x12\x03E(-\n\x0B\n\x04\x04\0\x02\x1B\x12\x03F\x02-\n\x0C\n\x05\x04\0\x02\x1B\x04\x12\x03F\x02\n\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03F\x0B\x10\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03F\x11\x18\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03F\x1B\x1D\n\x0C\n\x05\x04\0\x02\x1B\x08\x12\x03F\x1E,\n\x0C\n\x05\x04\0\x02\x1B\x07\x12\x03F)+\n\x0B\n\x04\x04\0\x02\x1C\x12\x03G\x02/\n\x0C\n\x05\x04\0\x02\x1C\x04\x12\x03G\x02\n\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03G\x0B\x10\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03G\x11\x19\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03G\x1C\x1F\n\x0C\n\x05\x04\0\x02\x1C\x08\x12\x03G .\n\x0C\n\x05\x04\0\x02\x1C\x07\x12\x03G+-\n\x0B\n\x04\x04\0\x02\x1D\x12\x03H\x02/\n\x0C\n\x05\x04\0\x02\x1D\x04\x12\x03H\x02\n\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03H\x0B\x10\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03H\x11\x19\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03H\x1C\x1F\n\x0C\n\x05\x04\0\x02\x1D\x08\x12\x03H .\n\x0C\n\x05\x04\0\x02\x1D\x07\x12\x03H+-\n\x0B\n\x04\x04\0\x02\x1E\x12\x03I\x02 \n\x0C\n\x05\x04\0\x02\x1E\x04\x12\x03I\x02\n\n\x0C\n\x05\x04\0\x02\x1E\x05\x12\x03I\x0B\x10\n\x0C\n\x05\x04\0\x02\x1E\x01\x12\x03I\x11\x19\n\x0C\n\x05\x04\0\x02\x1E\x03\x12\x03I\x1C\x1F\n\x0B\n\x04\x04\0\x02\x1F\x12\x03J\x02,\n\x0C\n\x05\x04\0\x02\x1F\x04\x12\x03J\x02\n\n\x0C\n\x05\x04\0\x02\x1F\x05\x12\x03J\x0B\x10\n\x0C\n\x05\x04\0\x02\x1F\x01\x12\x03J\x11\x18\n\x0C\n\x05\x04\0\x02\x1F\x03\x12\x03J\x1B\x1D\n\x0C\n\x05\x04\0\x02\x1F\x08\x12\x03J\x1E+\n\x0C\n\x05\x04\0\x02\x1F\x07\x12\x03J)*\n\x0B\n\x04\x04\0\x02 \x12\x03K\x02/\n\x0C\n\x05\x04\0\x02 \x04\x12\x03K\x02\n\n\x0C\n\x05\x04\0\x02 \x05\x12\x03K\x0B\x0F\n\x0C\n\x05\x04\0\x02 \x01\x12\x03K\x10\x17\n\x0C\n\x05\x04\0\x02 \x03\x12\x03K\x1A\x1C\n\x0C\n\x05\x04\0\x02 \x08\x12\x03K\x1D.\n\x0C\n\x05\x04\0\x02 \x07\x12\x03K(-\n\x0B\n\x04\x04\0\x02!\x12\x03L\x02,\n\x0C\n\x05\x04\0\x02!\x04\x12\x03L\x02\n\n\x0C\n\x05\x04\0\x02!\x05\x12\x03L\x0B\x10\n\x0C\n\x05\x04\0\x02!\x01\x12\x03L\x11\x18\n\x0C\n\x05\x04\0\x02!\x03\x12\x03L\x1B\x1D\n\x0C\n\x05\x04\0\x02!\x08\x12\x03L\x1E+\n\x0C\n\x05\x04\0\x02!\x07\x12\x03L)*\n\x0B\n\x04\x04\0\x02\"\x12\x03M\x021\n\x0C\n\x05\x04\0\x02\"\x04\x12\x03M\x02\n\n\x0C\n\x05\x04\0\x02\"\x06\x12\x03M\x0B#\n\x0C\n\x05\x04\0\x02\"\x01\x12\x03M$+\n\x0C\n\x05\x04\0\x02\"\x03\x12\x03M.0\n\x0B\n\x04\x04\0\x02#\x12\x03N\x02\x1D\n\x0C\n\x05\x04\0\x02#\x04\x12\x03N\x02\n\n\x0C\n\x05\x04\0\x02#\x05\x12\x03N\x0B\x0F\n\x0C\n\x05\x04\0\x02#\x01\x12\x03N\x10\x17\n\x0C\n\x05\x04\0\x02#\x03\x12\x03N\x1A\x1C\n\x0B\n\x04\x04\0\x02$\x12\x03O\x02,\n\x0C\n\x05\x04\0\x02$\x04\x12\x03O\x02\n\n\x0C\n\x05\x04\0\x02$\x05\x12\x03O\x0B\x10\n\x0C\n\x05\x04\0\x02$\x01\x12\x03O\x11\x18\n\x0C\n\x05\x04\0\x02$\x03\x12\x03O\x1B\x1D\n\x0C\n\x05\x04\0\x02$\x08\x12\x03O\x1E+\n\x0C\n\x05\x04\0\x02$\x07\x12\x03O)*\n\x0B\n\x04\x04\0\x02%\x12\x03P\x02\x1E\n\x0C\n\x05\x04\0\x02%\x04\x12\x03P\x02\n\n\x0C\n\x05\x04\0\x02%\x05\x12\x03P\x0B\x10\n\x0C\n\x05\x04\0\x02%\x01\x12\x03P\x11\x18\n\x0C\n\x05\x04\0\x02%\x03\x12\x03P\x1B\x1D\n\x0B\n\x04\x04\0\x02&\x12\x03Q\x02.\n\x0C\n\x05\x04\0\x02&\x04\x12\x03Q\x02\n\n\x0C\n\x05\x04\0\x02&\x05\x12\x03Q\x0B\x10\n\x0C\n\x05\x04\0\x02&\x01\x12\x03Q\x11\x19\n\x0C\n\x05\x04\0\x02&\x03\x12\x03Q\x1C\x1F\n\x0C\n\x05\x04\0\x02&\x08\x12\x03Q -\n\x0C\n\x05\x04\0\x02&\x07\x12\x03Q+,\n\x0B\n\x04\x04\0\x02'\x12\x03R\x02E\n\x0C\n\x05\x04\0\x02'\x04\x12\x03R\x02\n\n\x0C\n\x05\x04\0\x02'\x05\x12\x03R\x0B\x11\n\x0C\n\x05\x04\0\x02'\x01\x12\x03R\x12\x1A\n\x0C\n\x05\x04\0\x02'\x03\x12\x03R\x1D \n\x0C\n\x05\x04\0\x02'\x08\x12\x03R!D\n\x0C\n\x05\x04\0\x02'\x07\x12\x03R,C\n\x0B\n\x04\x04\0\x02(\x12\x03S\x02.\n\x0C\n\x05\x04\0\x02(\x04\x12\x03S\x02\n\n\x0C\n\x05\x04\0\x02(\x05\x12\x03S\x0B\x10\n\x0C\n\x05\x04\0\x02(\x01\x12\x03S\x11\x19\n\x0C\n\x05\x04\0\x02(\x03\x12\x03S\x1C\x1F\n\x0C\n\x05\x04\0\x02(\x08\x12\x03S -\n\x0C\n\x05\x04\0\x02(\x07\x12\x03S+,\n\n\n\x02\x04\x01\x12\x04V\0k\x01\n\n\n\x03\x04\x01\x01\x12\x03V\x08 \n\x0B\n\x04\x04\x01\x02\0\x12\x03W\x02*\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x03W\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03W\x0B\x10\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03W\x11\x17\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03W\x1A\x1B\n\x0C\n\x05\x04\x01\x02\0\x08\x12\x03W\x1C)\n\x0C\n\x05\x04\x01\x02\0\x07\x12\x03W'(\n\x0B\n\x04\x04\x01\x02\x01\x12\x03X\x02*\n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x03X\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03X\x0B\x10\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03X\x11\x17\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03X\x1A\x1B\n\x0C\n\x05\x04\x01\x02\x01\x08\x12\x03X\x1C)\n\x0C\n\x05\x04\x01\x02\x01\x07\x12\x03X'(\n\x0B\n\x04\x04\x01\x02\x02\x12\x03Y\x02*\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x03Y\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03Y\x0B\x10\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03Y\x11\x17\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03Y\x1A\x1B\n\x0C\n\x05\x04\x01\x02\x02\x08\x12\x03Y\x1C)\n\x0C\n\x05\x04\x01\x02\x02\x07\x12\x03Y'(\n\x0B\n\x04\x04\x01\x02\x03\x12\x03Z\x02\x1F\n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x03Z\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03Z\x0B\x11\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03Z\x12\x19\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03Z\x1C\x1E\n\x0B\n\x04\x04\x01\x02\x04\x12\x03[\x02.\n\x0C\n\x05\x04\x01\x02\x04\x04\x12\x03[\x02\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03[\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03[\x10\x17\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03[\x1A\x1C\n\x0C\n\x05\x04\x01\x02\x04\x08\x12\x03[\x1D-\n\x0C\n\x05\x04\x01\x02\x04\x07\x12\x03[(,\n\x0B\n\x04\x04\x01\x02\x05\x12\x03\\\x02\x1E\n\x0C\n\x05\x04\x01\x02\x05\x04\x12\x03\\\x02\n\n\x0C\n\x05\x04\x01\x02\x05\x05\x12\x03\\\x0B\x10\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03\\\x11\x18\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03\\\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x06\x12\x03]\x02\x1E\n\x0C\n\x05\x04\x01\x02\x06\x04\x12\x03]\x02\n\n\x0C\n\x05\x04\x01\x02\x06\x05\x12\x03]\x0B\x10\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03]\x11\x18\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03]\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x07\x12\x03^\x02\x1E\n\x0C\n\x05\x04\x01\x02\x07\x04\x12\x03^\x02\n\n\x0C\n\x05\x04\x01\x02\x07\x05\x12\x03^\x0B\x10\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03^\x11\x18\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03^\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x08\x12\x03_\x02,\n\x0C\n\x05\x04\x01\x02\x08\x04\x12\x03_\x02\n\n\x0C\n\x05\x04\x01\x02\x08\x05\x12\x03_\x0B\x10\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03_\x11\x18\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03_\x1B\x1D\n\x0C\n\x05\x04\x01\x02\x08\x08\x12\x03_\x1E+\n\x0C\n\x05\x04\x01\x02\x08\x07\x12\x03_)*\n\x0B\n\x04\x04\x01\x02\t\x12\x03`\x02.\n\x0C\n\x05\x04\x01\x02\t\x04\x12\x03`\x02\n\n\x0C\n\x05\x04\x01\x02\t\x05\x12\x03`\x0B\x0F\n\x0C\n\x05\x04\x01\x02\t\x01\x12\x03`\x10\x17\n\x0C\n\x05\x04\x01\x02\t\x03\x12\x03`\x1A\x1C\n\x0C\n\x05\x04\x01\x02\t\x08\x12\x03`\x1D-\n\x0C\n\x05\x04\x01\x02\t\x07\x12\x03`(,\n\x0B\n\x04\x04\x01\x02\n\x12\x03a\x02.\n\x0C\n\x05\x04\x01\x02\n\x04\x12\x03a\x02\n\n\x0C\n\x05\x04\x01\x02\n\x05\x12\x03a\x0B\x0F\n\x0C\n\x05\x04\x01\x02\n\x01\x12\x03a\x10\x17\n\x0C\n\x05\x04\x01\x02\n\x03\x12\x03a\x1A\x1C\n\x0C\n\x05\x04\x01\x02\n\x08\x12\x03a\x1D-\n\x0C\n\x05\x04\x01\x02\n\x07\x12\x03a(,\n\x0B\n\x04\x04\x01\x02\x0B\x12\x03b\x02 \n\x0C\n\x05\x04\x01\x02\x0B\x04\x12\x03b\x02\n\n\x0C\n\x05\x04\x01\x02\x0B\x05\x12\x03b\x0B\x12\n\x0C\n\x05\x04\x01\x02\x0B\x01\x12\x03b\x13\x1A\n\x0C\n\x05\x04\x01\x02\x0B\x03\x12\x03b\x1D\x1F\n\x0B\n\x04\x04\x01\x02\x0C\x12\x03c\x02\x1E\n\x0C\n\x05\x04\x01\x02\x0C\x04\x12\x03c\x02\n\n\x0C\n\x05\x04\x01\x02\x0C\x05\x12\x03c\x0B\x10\n\x0C\n\x05\x04\x01\x02\x0C\x01\x12\x03c\x11\x18\n\x0C\n\x05\x04\x01\x02\x0C\x03\x12\x03c\x1B\x1D\n\x0B\n\x04\x04\x01\x02\r\x12\x03d\x02/\n\x0C\n\x05\x04\x01\x02\r\x04\x12\x03d\x02\n\n\x0C\n\x05\x04\x01\x02\r\x05\x12\x03d\x0B\x0F\n\x0C\n\x05\x04\x01\x02\r\x01\x12\x03d\x10\x17\n\x0C\n\x05\x04\x01\x02\r\x03\x12\x03d\x1A\x1C\n\x0C\n\x05\x04\x01\x02\r\x08\x12\x03d\x1D.\n\x0C\n\x05\x04\x01\x02\r\x07\x12\x03d(-\n\x0B\n\x04\x04\x01\x02\x0E\x12\x03e\x021\n\x0C\n\x05\x04\x01\x02\x0E\x04\x12\x03e\x02\n\n\x0C\n\x05\x04\x01\x02\x0E\x05\x12\x03e\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x0E\x01\x12\x03e\x10\x18\n\x0C\n\x05\x04\x01\x02\x0E\x03\x12\x03e\x1B\x1E\n\x0C\n\x05\x04\x01\x02\x0E\x08\x12\x03e\x1F0\n\x0C\n\x05\x04\x01\x02\x0E\x07\x12\x03e*/\n\x0B\n\x04\x04\x01\x02\x0F\x12\x03f\x02\"\n\x0C\n\x05\x04\x01\x02\x0F\x04\x12\x03f\x02\n\n\x0C\n\x05\x04\x01\x02\x0F\x05\x12\x03f\x0B\x12\n\x0C\n\x05\x04\x01\x02\x0F\x01\x12\x03f\x13\x1B\n\x0C\n\x05\x04\x01\x02\x0F\x03\x12\x03f\x1E!\n\x0B\n\x04\x04\x01\x02\x10\x12\x03g\x02 \n\x0C\n\x05\x04\x01\x02\x10\x04\x12\x03g\x02\n\n\x0C\n\x05\x04\x01\x02\x10\x05\x12\x03g\x0B\x10\n\x0C\n\x05\x04\x01\x02\x10\x01\x12\x03g\x11\x19\n\x0C\n\x05\x04\x01\x02\x10\x03\x12\x03g\x1C\x1F\n\x0B\n\x04\x04\x01\x02\x11\x12\x03h\x02!\n\x0C\n\x05\x04\x01\x02\x11\x04\x12\x03h\x02\n\n\x0C\n\x05\x04\x01\x02\x11\x05\x12\x03h\x0B\x11\n\x0C\n\x05\x04\x01\x02\x11\x01\x12\x03h\x12\x1A\n\x0C\n\x05\x04\x01\x02\x11\x03\x12\x03h\x1D \n\x0B\n\x04\x04\x01\x02\x12\x12\x03i\x02!\n\x0C\n\x05\x04\x01\x02\x12\x04\x12\x03i\x02\n\n\x0C\n\x05\x04\x01\x02\x12\x05\x12\x03i\x0B\x11\n\x0C\n\x05\x04\x01\x02\x12\x01\x12\x03i\x12\x1A\n\x0C\n\x05\x04\x01\x02\x12\x03\x12\x03i\x1D \n\x0B\n\x04\x04\x01\x02\x13\x12\x03j\x02!\n\x0C\n\x05\x04\x01\x02\x13\x04\x12\x03j\x02\n\n\x0C\n\x05\x04\x01\x02\x13\x05\x12\x03j\x0B\x11\n\x0C\n\x05\x04\x01\x02\x13\x01\x12\x03j\x12\x1A\n\x0C\n\x05\x04\x01\x02\x13\x03\x12\x03j\x1D " ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
