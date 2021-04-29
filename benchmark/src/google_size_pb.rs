#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct SizeMessage1 {
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
    pub field15: Option<SizeMessage1SubMessage>,
    pub field78: Option<bool>,
    pub field67: Option<i32>,
    pub field68: Option<i32>,
    pub field128: Option<i32>,
    pub field129: Option<String>,
    pub field131: Option<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl SizeMessage1 {
    pub const fn new() -> SizeMessage1 {
        SizeMessage1 {
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
            _cached_size: pecan::CachedSize::new(),
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
    pub fn field15(&self) -> &SizeMessage1SubMessage {
        match &self.field15 {
            Some(v) => v,
            _ => SizeMessage1SubMessage::default_instance(),
        }
    }
    pub fn field15_mut(&mut self) -> &mut SizeMessage1SubMessage {
        self.field15.get_or_insert_with(Default::default)
    }
    pub fn set_field15(&mut self, val: SizeMessage1SubMessage) {
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
impl pecan::Message for SizeMessage1 {
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
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SizeMessage1 {
    fn default_instance() -> &'static SizeMessage1 {
        static DEFAULT: SizeMessage1 = SizeMessage1::new();
        &DEFAULT
    }
}
impl Default for SizeMessage1 {
    #[inline]
    fn default() -> SizeMessage1 {
        SizeMessage1::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SizeMessage1SubMessage {
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
    _cached_size: pecan::CachedSize,
}
impl SizeMessage1SubMessage {
    pub const fn new() -> SizeMessage1SubMessage {
        SizeMessage1SubMessage {
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
            _cached_size: pecan::CachedSize::new(),
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
impl pecan::Message for SizeMessage1SubMessage {
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
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
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
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SizeMessage1SubMessage {
    fn default_instance() -> &'static SizeMessage1SubMessage {
        static DEFAULT: SizeMessage1SubMessage = SizeMessage1SubMessage::new();
        &DEFAULT
    }
}
impl Default for SizeMessage1SubMessage {
    #[inline]
    fn default() -> SizeMessage1SubMessage {
        SizeMessage1SubMessage::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SizeMessage2_Group1 {
    pub field11: f32,
    pub field26: Option<f32>,
    pub field12: Option<String>,
    pub field13: Option<String>,
    pub field14: Vec<String>,
    pub field15: u64,
    pub field5: Option<i32>,
    pub field27: Option<String>,
    pub field28: Option<i32>,
    pub field29: Option<String>,
    pub field16: Option<String>,
    pub field22: Vec<String>,
    pub field73: Vec<i32>,
    pub field20: Option<i32>,
    pub field24: Option<String>,
    pub field31: Option<SizeMessage2GroupedMessage>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl SizeMessage2_Group1 {
    pub const fn new() -> SizeMessage2_Group1 {
        SizeMessage2_Group1 {
            field11: 0f32,
            field26: None,
            field12: None,
            field13: None,
            field14: Vec::new(),
            field15: 0,
            field5: None,
            field27: None,
            field28: None,
            field29: None,
            field16: None,
            field22: Vec::new(),
            field73: Vec::new(),
            field20: None,
            field24: None,
            field31: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field26(&self) -> f32 {
        self.field26.unwrap_or_default()
    }
    pub fn field26_mut(&mut self) -> &mut f32 {
        self.field26.get_or_insert_with(Default::default)
    }
    pub fn set_field26(&mut self, val: f32) {
        self.field26 = Some(val);
    }
    pub fn field12(&self) -> &String {
        match &self.field12 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field12_mut(&mut self) -> &mut String {
        self.field12.get_or_insert_with(Default::default)
    }
    pub fn set_field12(&mut self, val: String) {
        self.field12 = Some(val);
    }
    pub fn field13(&self) -> &String {
        match &self.field13 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field13_mut(&mut self) -> &mut String {
        self.field13.get_or_insert_with(Default::default)
    }
    pub fn set_field13(&mut self, val: String) {
        self.field13 = Some(val);
    }
    pub fn field5(&self) -> i32 {
        self.field5.unwrap_or_default()
    }
    pub fn field5_mut(&mut self) -> &mut i32 {
        self.field5.get_or_insert_with(Default::default)
    }
    pub fn set_field5(&mut self, val: i32) {
        self.field5 = Some(val);
    }
    pub fn field27(&self) -> &String {
        match &self.field27 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field27_mut(&mut self) -> &mut String {
        self.field27.get_or_insert_with(Default::default)
    }
    pub fn set_field27(&mut self, val: String) {
        self.field27 = Some(val);
    }
    pub fn field28(&self) -> i32 {
        self.field28.unwrap_or_default()
    }
    pub fn field28_mut(&mut self) -> &mut i32 {
        self.field28.get_or_insert_with(Default::default)
    }
    pub fn set_field28(&mut self, val: i32) {
        self.field28 = Some(val);
    }
    pub fn field29(&self) -> &String {
        match &self.field29 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field29_mut(&mut self) -> &mut String {
        self.field29.get_or_insert_with(Default::default)
    }
    pub fn set_field29(&mut self, val: String) {
        self.field29 = Some(val);
    }
    pub fn field16(&self) -> &String {
        match &self.field16 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field16_mut(&mut self) -> &mut String {
        self.field16.get_or_insert_with(Default::default)
    }
    pub fn set_field16(&mut self, val: String) {
        self.field16 = Some(val);
    }
    pub fn field20(&self) -> i32 {
        self.field20.unwrap_or_default()
    }
    pub fn field20_mut(&mut self) -> &mut i32 {
        self.field20.get_or_insert_with(Default::default)
    }
    pub fn set_field20(&mut self, val: i32) {
        self.field20 = Some(val);
    }
    pub fn field24(&self) -> &String {
        match &self.field24 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field24_mut(&mut self) -> &mut String {
        self.field24.get_or_insert_with(Default::default)
    }
    pub fn set_field24(&mut self, val: String) {
        self.field24 = Some(val);
    }
    pub fn field31(&self) -> &SizeMessage2GroupedMessage {
        match &self.field31 {
            Some(v) => v,
            _ => SizeMessage2GroupedMessage::default_instance(),
        }
    }
    pub fn field31_mut(&mut self) -> &mut SizeMessage2GroupedMessage {
        self.field31.get_or_insert_with(Default::default)
    }
    pub fn set_field31(&mut self, val: SizeMessage2GroupedMessage) {
        self.field31 = Some(val);
    }
}
impl pecan::Message for SizeMessage2_Group1 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                40 => self.field5 = Some(Varint::read_from(s)?),
                93 => self.field11 = Fixed32::read_from(s)?,
                98 => self.field12 = Some(LengthPrefixed::read_from(s)?),
                106 => self.field13 = Some(LengthPrefixed::read_from(s)?),
                114 => RefArray::<LengthPrefixed>::merge_from(&mut self.field14, s)?,
                120 => self.field15 = Varint::read_from(s)?,
                130 => self.field16 = Some(LengthPrefixed::read_from(s)?),
                160 => self.field20 = Some(Varint::read_from(s)?),
                178 => RefArray::<LengthPrefixed>::merge_from(&mut self.field22, s)?,
                194 => self.field24 = Some(LengthPrefixed::read_from(s)?),
                213 => self.field26 = Some(Fixed32::read_from(s)?),
                218 => self.field27 = Some(LengthPrefixed::read_from(s)?),
                224 => self.field28 = Some(Varint::read_from(s)?),
                234 => self.field29 = Some(LengthPrefixed::read_from(s)?),
                250 => LengthPrefixed::merge_from(self.field31_mut(), s)?,
                584 => CopyArray::<Varint>::merge_from(&mut self.field73, s)?,
                586 => PackedArray::<Varint>::merge_from(&mut self.field73, s)?,
                0 | 84 => {
                    s.set_last_tag(84);
                    return Ok(());
                }
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.field5 {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if self.field11 != 0f32 {
            s.write_tag(93)?;
            Fixed32::write_to(self.field11, s)?;
        }
        if let Some(v) = &self.field12 {
            s.write_tag(98)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field13 {
            s.write_tag(106)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.field14.is_empty() {
            for i in &self.field14 {
                s.write_tag(114)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if self.field15 != 0 {
            s.write_tag(120)?;
            Varint::write_to(self.field15, s)?;
        }
        if let Some(v) = &self.field16 {
            s.write_tag(130)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field20 {
            s.write_tag(160)?;
            Varint::write_to(v, s)?;
        }
        if !self.field22.is_empty() {
            for i in &self.field22 {
                s.write_tag(178)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.field24 {
            s.write_tag(194)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field26 {
            s.write_tag(213)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = &self.field27 {
            s.write_tag(218)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field28 {
            s.write_tag(224)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field29 {
            s.write_tag(234)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field31 {
            s.write_tag(250)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.field73.is_empty() {
            for i in &self.field73 {
                s.write_tag(584)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.field5 {
            l += 1 + Varint::size(v);
        }
        if self.field11 != 0f32 {
            l += 1 + Fixed32::size(self.field11);
        }
        if let Some(v) = &self.field12 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field13 {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.field14.is_empty() {
            l += self.field14.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field14);
        }
        if self.field15 != 0 {
            l += 1 + Varint::size(self.field15);
        }
        if let Some(v) = &self.field16 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field20 {
            l += 2 + Varint::size(v);
        }
        if !self.field22.is_empty() {
            l += 2 * self.field22.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field22);
        }
        if let Some(v) = &self.field24 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field26 {
            l += 2 + Fixed32::size(v);
        }
        if let Some(v) = &self.field27 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field28 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.field29 {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field31 {
            l += 2 + LengthPrefixed::size(v);
        }
        if !self.field73.is_empty() {
            l += 2 * self.field73.len() as u64 + CopyArray::<Varint>::size(&self.field73);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SizeMessage2_Group1 {
    fn default_instance() -> &'static SizeMessage2_Group1 {
        static DEFAULT: SizeMessage2_Group1 = SizeMessage2_Group1::new();
        &DEFAULT
    }
}
impl Default for SizeMessage2_Group1 {
    #[inline]
    fn default() -> SizeMessage2_Group1 {
        SizeMessage2_Group1::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SizeMessage2 {
    pub field1: Option<String>,
    pub field3: Option<i64>,
    pub field4: Option<i64>,
    pub field30: Option<i64>,
    pub field75: Option<bool>,
    pub field6: Option<String>,
    pub field2: Option<pecan::Bytes>,
    pub field21: Option<i32>,
    pub field71: Option<i32>,
    pub field25: Option<f32>,
    pub field109: Option<i32>,
    pub field210: Option<i32>,
    pub field211: Option<i32>,
    pub field212: Option<i32>,
    pub field213: Option<i32>,
    pub field216: Option<i32>,
    pub field217: Option<i32>,
    pub field218: Option<i32>,
    pub field220: Option<i32>,
    pub field221: Option<i32>,
    pub field222: Option<f32>,
    pub field63: Option<i32>,
    pub group1: Vec<SizeMessage2_Group1>,
    pub field128: Vec<String>,
    pub field131: Option<i64>,
    pub field127: Vec<String>,
    pub field129: Option<i32>,
    pub field130: Vec<i64>,
    pub field205: Option<bool>,
    pub field206: Option<bool>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl SizeMessage2 {
    pub const fn new() -> SizeMessage2 {
        SizeMessage2 {
            field1: None,
            field3: None,
            field4: None,
            field30: None,
            field75: None,
            field6: None,
            field2: None,
            field21: None,
            field71: None,
            field25: None,
            field109: None,
            field210: None,
            field211: None,
            field212: None,
            field213: None,
            field216: None,
            field217: None,
            field218: None,
            field220: None,
            field221: None,
            field222: None,
            field63: None,
            group1: Vec::new(),
            field128: Vec::new(),
            field131: None,
            field127: Vec::new(),
            field129: None,
            field130: Vec::new(),
            field205: None,
            field206: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field1(&self) -> &String {
        match &self.field1 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field1_mut(&mut self) -> &mut String {
        self.field1.get_or_insert_with(Default::default)
    }
    pub fn set_field1(&mut self, val: String) {
        self.field1 = Some(val);
    }
    pub fn field3(&self) -> i64 {
        self.field3.unwrap_or_default()
    }
    pub fn field3_mut(&mut self) -> &mut i64 {
        self.field3.get_or_insert_with(Default::default)
    }
    pub fn set_field3(&mut self, val: i64) {
        self.field3 = Some(val);
    }
    pub fn field4(&self) -> i64 {
        self.field4.unwrap_or_default()
    }
    pub fn field4_mut(&mut self) -> &mut i64 {
        self.field4.get_or_insert_with(Default::default)
    }
    pub fn set_field4(&mut self, val: i64) {
        self.field4 = Some(val);
    }
    pub fn field30(&self) -> i64 {
        self.field30.unwrap_or_default()
    }
    pub fn field30_mut(&mut self) -> &mut i64 {
        self.field30.get_or_insert_with(Default::default)
    }
    pub fn set_field30(&mut self, val: i64) {
        self.field30 = Some(val);
    }
    pub fn field75(&self) -> bool {
        self.field75.unwrap_or_default()
    }
    pub fn field75_mut(&mut self) -> &mut bool {
        self.field75.get_or_insert_with(Default::default)
    }
    pub fn set_field75(&mut self, val: bool) {
        self.field75 = Some(val);
    }
    pub fn field6(&self) -> &String {
        match &self.field6 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field6_mut(&mut self) -> &mut String {
        self.field6.get_or_insert_with(Default::default)
    }
    pub fn set_field6(&mut self, val: String) {
        self.field6 = Some(val);
    }
    pub fn field2(&self) -> &pecan::Bytes {
        match &self.field2 {
            Some(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn field2_mut(&mut self) -> &mut pecan::Bytes {
        self.field2.get_or_insert_with(Default::default)
    }
    pub fn set_field2(&mut self, val: pecan::Bytes) {
        self.field2 = Some(val);
    }
    pub fn field21(&self) -> i32 {
        self.field21.unwrap_or_default()
    }
    pub fn field21_mut(&mut self) -> &mut i32 {
        self.field21.get_or_insert_with(Default::default)
    }
    pub fn set_field21(&mut self, val: i32) {
        self.field21 = Some(val);
    }
    pub fn field71(&self) -> i32 {
        self.field71.unwrap_or_default()
    }
    pub fn field71_mut(&mut self) -> &mut i32 {
        self.field71.get_or_insert_with(Default::default)
    }
    pub fn set_field71(&mut self, val: i32) {
        self.field71 = Some(val);
    }
    pub fn field25(&self) -> f32 {
        self.field25.unwrap_or_default()
    }
    pub fn field25_mut(&mut self) -> &mut f32 {
        self.field25.get_or_insert_with(Default::default)
    }
    pub fn set_field25(&mut self, val: f32) {
        self.field25 = Some(val);
    }
    pub fn field109(&self) -> i32 {
        self.field109.unwrap_or_default()
    }
    pub fn field109_mut(&mut self) -> &mut i32 {
        self.field109.get_or_insert_with(Default::default)
    }
    pub fn set_field109(&mut self, val: i32) {
        self.field109 = Some(val);
    }
    pub fn field210(&self) -> i32 {
        self.field210.unwrap_or_default()
    }
    pub fn field210_mut(&mut self) -> &mut i32 {
        self.field210.get_or_insert_with(Default::default)
    }
    pub fn set_field210(&mut self, val: i32) {
        self.field210 = Some(val);
    }
    pub fn field211(&self) -> i32 {
        self.field211.unwrap_or_default()
    }
    pub fn field211_mut(&mut self) -> &mut i32 {
        self.field211.get_or_insert_with(Default::default)
    }
    pub fn set_field211(&mut self, val: i32) {
        self.field211 = Some(val);
    }
    pub fn field212(&self) -> i32 {
        self.field212.unwrap_or_default()
    }
    pub fn field212_mut(&mut self) -> &mut i32 {
        self.field212.get_or_insert_with(Default::default)
    }
    pub fn set_field212(&mut self, val: i32) {
        self.field212 = Some(val);
    }
    pub fn field213(&self) -> i32 {
        self.field213.unwrap_or_default()
    }
    pub fn field213_mut(&mut self) -> &mut i32 {
        self.field213.get_or_insert_with(Default::default)
    }
    pub fn set_field213(&mut self, val: i32) {
        self.field213 = Some(val);
    }
    pub fn field216(&self) -> i32 {
        self.field216.unwrap_or_default()
    }
    pub fn field216_mut(&mut self) -> &mut i32 {
        self.field216.get_or_insert_with(Default::default)
    }
    pub fn set_field216(&mut self, val: i32) {
        self.field216 = Some(val);
    }
    pub fn field217(&self) -> i32 {
        self.field217.unwrap_or_default()
    }
    pub fn field217_mut(&mut self) -> &mut i32 {
        self.field217.get_or_insert_with(Default::default)
    }
    pub fn set_field217(&mut self, val: i32) {
        self.field217 = Some(val);
    }
    pub fn field218(&self) -> i32 {
        self.field218.unwrap_or_default()
    }
    pub fn field218_mut(&mut self) -> &mut i32 {
        self.field218.get_or_insert_with(Default::default)
    }
    pub fn set_field218(&mut self, val: i32) {
        self.field218 = Some(val);
    }
    pub fn field220(&self) -> i32 {
        self.field220.unwrap_or_default()
    }
    pub fn field220_mut(&mut self) -> &mut i32 {
        self.field220.get_or_insert_with(Default::default)
    }
    pub fn set_field220(&mut self, val: i32) {
        self.field220 = Some(val);
    }
    pub fn field221(&self) -> i32 {
        self.field221.unwrap_or_default()
    }
    pub fn field221_mut(&mut self) -> &mut i32 {
        self.field221.get_or_insert_with(Default::default)
    }
    pub fn set_field221(&mut self, val: i32) {
        self.field221 = Some(val);
    }
    pub fn field222(&self) -> f32 {
        self.field222.unwrap_or_default()
    }
    pub fn field222_mut(&mut self) -> &mut f32 {
        self.field222.get_or_insert_with(Default::default)
    }
    pub fn set_field222(&mut self, val: f32) {
        self.field222 = Some(val);
    }
    pub fn field63(&self) -> i32 {
        self.field63.unwrap_or_default()
    }
    pub fn field63_mut(&mut self) -> &mut i32 {
        self.field63.get_or_insert_with(Default::default)
    }
    pub fn set_field63(&mut self, val: i32) {
        self.field63 = Some(val);
    }
    pub fn field131(&self) -> i64 {
        self.field131.unwrap_or_default()
    }
    pub fn field131_mut(&mut self) -> &mut i64 {
        self.field131.get_or_insert_with(Default::default)
    }
    pub fn set_field131(&mut self, val: i64) {
        self.field131 = Some(val);
    }
    pub fn field129(&self) -> i32 {
        self.field129.unwrap_or_default()
    }
    pub fn field129_mut(&mut self) -> &mut i32 {
        self.field129.get_or_insert_with(Default::default)
    }
    pub fn set_field129(&mut self, val: i32) {
        self.field129 = Some(val);
    }
    pub fn field205(&self) -> bool {
        self.field205.unwrap_or_default()
    }
    pub fn field205_mut(&mut self) -> &mut bool {
        self.field205.get_or_insert_with(Default::default)
    }
    pub fn set_field205(&mut self, val: bool) {
        self.field205 = Some(val);
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
}
impl pecan::Message for SizeMessage2 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field1 = Some(LengthPrefixed::read_from(s)?),
                18 => self.field2 = Some(LengthPrefixed::read_from(s)?),
                24 => self.field3 = Some(Varint::read_from(s)?),
                32 => self.field4 = Some(Varint::read_from(s)?),
                50 => self.field6 = Some(LengthPrefixed::read_from(s)?),
                83 => s.read_group(84, |s| {
                    self.group1.push(SizeMessage2_Group1::new());
                    self.group1.last_mut().unwrap().merge_from(s)
                })?,
                168 => self.field21 = Some(Varint::read_from(s)?),
                205 => self.field25 = Some(Fixed32::read_from(s)?),
                240 => self.field30 = Some(Varint::read_from(s)?),
                504 => self.field63 = Some(Varint::read_from(s)?),
                568 => self.field71 = Some(Varint::read_from(s)?),
                600 => self.field75 = Some(Varint::read_from(s)?),
                872 => self.field109 = Some(Varint::read_from(s)?),
                1018 => RefArray::<LengthPrefixed>::merge_from(&mut self.field127, s)?,
                1026 => RefArray::<LengthPrefixed>::merge_from(&mut self.field128, s)?,
                1032 => self.field129 = Some(Varint::read_from(s)?),
                1040 => CopyArray::<Varint>::merge_from(&mut self.field130, s)?,
                1042 => PackedArray::<Varint>::merge_from(&mut self.field130, s)?,
                1048 => self.field131 = Some(Varint::read_from(s)?),
                1640 => self.field205 = Some(Varint::read_from(s)?),
                1648 => self.field206 = Some(Varint::read_from(s)?),
                1680 => self.field210 = Some(Varint::read_from(s)?),
                1688 => self.field211 = Some(Varint::read_from(s)?),
                1696 => self.field212 = Some(Varint::read_from(s)?),
                1704 => self.field213 = Some(Varint::read_from(s)?),
                1728 => self.field216 = Some(Varint::read_from(s)?),
                1736 => self.field217 = Some(Varint::read_from(s)?),
                1744 => self.field218 = Some(Varint::read_from(s)?),
                1760 => self.field220 = Some(Varint::read_from(s)?),
                1768 => self.field221 = Some(Varint::read_from(s)?),
                1781 => self.field222 = Some(Fixed32::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.field1 {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field2 {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field3 {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field4 {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.field6 {
            s.write_tag(50)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.group1.is_empty() {
            for i in &self.group1 {
                s.write_tag(83)?;
                i.write_to_uncheck(s)?;
                s.write_tag(84)?;
            }
        }
        if let Some(v) = self.field21 {
            s.write_tag(168)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field25 {
            s.write_tag(205)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field30 {
            s.write_tag(240)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field63 {
            s.write_tag(504)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field71 {
            s.write_tag(568)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field75 {
            s.write_tag(600)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field109 {
            s.write_tag(872)?;
            Varint::write_to(v, s)?;
        }
        if !self.field127.is_empty() {
            for i in &self.field127 {
                s.write_tag(1018)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.field128.is_empty() {
            for i in &self.field128 {
                s.write_tag(1026)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = self.field129 {
            s.write_tag(1032)?;
            Varint::write_to(v, s)?;
        }
        if !self.field130.is_empty() {
            for i in &self.field130 {
                s.write_tag(1040)?;
                Varint::write_to(*i, s)?;
            }
        }
        if let Some(v) = self.field131 {
            s.write_tag(1048)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field205 {
            s.write_tag(1640)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field206 {
            s.write_tag(1648)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field210 {
            s.write_tag(1680)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field211 {
            s.write_tag(1688)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field212 {
            s.write_tag(1696)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field213 {
            s.write_tag(1704)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field216 {
            s.write_tag(1728)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field217 {
            s.write_tag(1736)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field218 {
            s.write_tag(1744)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field220 {
            s.write_tag(1760)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field221 {
            s.write_tag(1768)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field222 {
            s.write_tag(1781)?;
            Fixed32::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.field1 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field2 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field3 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field4 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.field6 {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.group1.is_empty() {
            l += 2 * self.group1.len() as u64;
            for i in &self.group1 {
                l += i.size();
            }
        }
        if let Some(v) = self.field21 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field25 {
            l += 2 + Fixed32::size(v);
        }
        if let Some(v) = self.field30 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field63 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field71 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field75 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field109 {
            l += 2 + Varint::size(v);
        }
        if !self.field127.is_empty() {
            l += 2 * self.field127.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field127);
        }
        if !self.field128.is_empty() {
            l += 2 * self.field128.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field128);
        }
        if let Some(v) = self.field129 {
            l += 2 + Varint::size(v);
        }
        if !self.field130.is_empty() {
            l += 2 * self.field130.len() as u64 + CopyArray::<Varint>::size(&self.field130);
        }
        if let Some(v) = self.field131 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field205 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field206 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field210 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field211 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field212 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field213 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field216 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field217 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field218 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field220 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field221 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field222 {
            l += 2 + Fixed32::size(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SizeMessage2 {
    fn default_instance() -> &'static SizeMessage2 {
        static DEFAULT: SizeMessage2 = SizeMessage2::new();
        &DEFAULT
    }
}
impl Default for SizeMessage2 {
    #[inline]
    fn default() -> SizeMessage2 {
        SizeMessage2::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SizeMessage2GroupedMessage {
    pub field1: Option<f32>,
    pub field2: Option<f32>,
    pub field3: Option<f32>,
    pub field4: Option<bool>,
    pub field5: Option<bool>,
    pub field6: Option<bool>,
    pub field7: Option<bool>,
    pub field8: Option<f32>,
    pub field9: Option<bool>,
    pub field10: Option<f32>,
    pub field11: Option<i64>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl SizeMessage2GroupedMessage {
    pub const fn new() -> SizeMessage2GroupedMessage {
        SizeMessage2GroupedMessage {
            field1: None,
            field2: None,
            field3: None,
            field4: None,
            field5: None,
            field6: None,
            field7: None,
            field8: None,
            field9: None,
            field10: None,
            field11: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field1(&self) -> f32 {
        self.field1.unwrap_or_default()
    }
    pub fn field1_mut(&mut self) -> &mut f32 {
        self.field1.get_or_insert_with(Default::default)
    }
    pub fn set_field1(&mut self, val: f32) {
        self.field1 = Some(val);
    }
    pub fn field2(&self) -> f32 {
        self.field2.unwrap_or_default()
    }
    pub fn field2_mut(&mut self) -> &mut f32 {
        self.field2.get_or_insert_with(Default::default)
    }
    pub fn set_field2(&mut self, val: f32) {
        self.field2 = Some(val);
    }
    pub fn field3(&self) -> f32 {
        self.field3.unwrap_or_default()
    }
    pub fn field3_mut(&mut self) -> &mut f32 {
        self.field3.get_or_insert_with(Default::default)
    }
    pub fn set_field3(&mut self, val: f32) {
        self.field3 = Some(val);
    }
    pub fn field4(&self) -> bool {
        self.field4.unwrap_or_default()
    }
    pub fn field4_mut(&mut self) -> &mut bool {
        self.field4.get_or_insert_with(Default::default)
    }
    pub fn set_field4(&mut self, val: bool) {
        self.field4 = Some(val);
    }
    pub fn field5(&self) -> bool {
        self.field5.unwrap_or_default()
    }
    pub fn field5_mut(&mut self) -> &mut bool {
        self.field5.get_or_insert_with(Default::default)
    }
    pub fn set_field5(&mut self, val: bool) {
        self.field5 = Some(val);
    }
    pub fn field6(&self) -> bool {
        self.field6.unwrap_or_default()
    }
    pub fn field6_mut(&mut self) -> &mut bool {
        self.field6.get_or_insert_with(Default::default)
    }
    pub fn set_field6(&mut self, val: bool) {
        self.field6 = Some(val);
    }
    pub fn field7(&self) -> bool {
        self.field7.unwrap_or_default()
    }
    pub fn field7_mut(&mut self) -> &mut bool {
        self.field7.get_or_insert_with(Default::default)
    }
    pub fn set_field7(&mut self, val: bool) {
        self.field7 = Some(val);
    }
    pub fn field8(&self) -> f32 {
        self.field8.unwrap_or_default()
    }
    pub fn field8_mut(&mut self) -> &mut f32 {
        self.field8.get_or_insert_with(Default::default)
    }
    pub fn set_field8(&mut self, val: f32) {
        self.field8 = Some(val);
    }
    pub fn field9(&self) -> bool {
        self.field9.unwrap_or_default()
    }
    pub fn field9_mut(&mut self) -> &mut bool {
        self.field9.get_or_insert_with(Default::default)
    }
    pub fn set_field9(&mut self, val: bool) {
        self.field9 = Some(val);
    }
    pub fn field10(&self) -> f32 {
        self.field10.unwrap_or_default()
    }
    pub fn field10_mut(&mut self) -> &mut f32 {
        self.field10.get_or_insert_with(Default::default)
    }
    pub fn set_field10(&mut self, val: f32) {
        self.field10 = Some(val);
    }
    pub fn field11(&self) -> i64 {
        self.field11.unwrap_or_default()
    }
    pub fn field11_mut(&mut self) -> &mut i64 {
        self.field11.get_or_insert_with(Default::default)
    }
    pub fn set_field11(&mut self, val: i64) {
        self.field11 = Some(val);
    }
}
impl pecan::Message for SizeMessage2GroupedMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                13 => self.field1 = Some(Fixed32::read_from(s)?),
                21 => self.field2 = Some(Fixed32::read_from(s)?),
                29 => self.field3 = Some(Fixed32::read_from(s)?),
                32 => self.field4 = Some(Varint::read_from(s)?),
                40 => self.field5 = Some(Varint::read_from(s)?),
                48 => self.field6 = Some(Varint::read_from(s)?),
                56 => self.field7 = Some(Varint::read_from(s)?),
                69 => self.field8 = Some(Fixed32::read_from(s)?),
                72 => self.field9 = Some(Varint::read_from(s)?),
                85 => self.field10 = Some(Fixed32::read_from(s)?),
                88 => self.field11 = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.field1 {
            s.write_tag(13)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field2 {
            s.write_tag(21)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field3 {
            s.write_tag(29)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field4 {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field5 {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field6 {
            s.write_tag(48)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field7 {
            s.write_tag(56)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field8 {
            s.write_tag(69)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field9 {
            s.write_tag(72)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field10 {
            s.write_tag(85)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.field11 {
            s.write_tag(88)?;
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
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.field2 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.field3 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.field4 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field5 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field6 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field7 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field8 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.field9 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field10 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.field11 {
            l += 1 + Varint::size(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for SizeMessage2GroupedMessage {
    fn default_instance() -> &'static SizeMessage2GroupedMessage {
        static DEFAULT: SizeMessage2GroupedMessage = SizeMessage2GroupedMessage::new();
        &DEFAULT
    }
}
impl Default for SizeMessage2GroupedMessage {
    #[inline]
    fn default() -> SizeMessage2GroupedMessage {
        SizeMessage2GroupedMessage::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x11google_size.proto\x12\nbenchmarks\"\xEC\t\n\x0CSizeMessage1\x12\x16\n\x06field1\x18\x01 \x02(\tR\x06field1\x12\x16\n\x06field9\x18\t \x01(\tR\x06field9\x12\x18\n\x07field18\x18\x12 \x01(\tR\x07field18\x12\x1F\n\x07field80\x18P \x01(\x08:\x05falseR\x07field80\x12\x1E\n\x07field81\x18Q \x01(\x08:\x04trueR\x07field81\x12\x16\n\x06field2\x18\x02 \x02(\x05R\x06field2\x12\x16\n\x06field3\x18\x03 \x02(\x05R\x06field3\x12\x1B\n\x08field280\x18\x98\x02 \x01(\x05R\x08field280\x12\x19\n\x06field6\x18\x06 \x01(\x05:\x010R\x06field6\x12\x18\n\x07field22\x18\x16 \x01(\x03R\x07field22\x12\x16\n\x06field4\x18\x04 \x01(\tR\x06field4\x12\x16\n\x06field5\x18\x05 \x03(\x06R\x06field5\x12\x1F\n\x07field59\x18; \x01(\x08:\x05falseR\x07field59\x12\x16\n\x06field7\x18\x07 \x01(\tR\x06field7\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x1E\n\x08field130\x18\x82\x01 \x01(\x05:\x010R\x08field130\x12\x1E\n\x07field12\x18\x0C \x01(\x08:\x04trueR\x07field12\x12\x1E\n\x07field17\x18\x11 \x01(\x08:\x04trueR\x07field17\x12\x1E\n\x07field13\x18\r \x01(\x08:\x04trueR\x07field13\x12\x1E\n\x07field14\x18\x0E \x01(\x08:\x04trueR\x07field14\x12\x1D\n\x08field104\x18h \x01(\x05:\x010R\x08field104\x12\x1D\n\x08field100\x18d \x01(\x05:\x010R\x08field100\x12\x1D\n\x08field101\x18e \x01(\x05:\x010R\x08field101\x12\x1A\n\x08field102\x18f \x01(\tR\x08field102\x12\x1A\n\x08field103\x18g \x01(\tR\x08field103\x12\x1B\n\x07field29\x18\x1D \x01(\x05:\x010R\x07field29\x12\x1F\n\x07field30\x18\x1E \x01(\x08:\x05falseR\x07field30\x12\x1C\n\x07field60\x18< \x01(\x05:\x02-1R\x07field60\x12\x1F\n\x08field271\x18\x8F\x02 \x01(\x05:\x02-1R\x08field271\x12\x1F\n\x08field272\x18\x90\x02 \x01(\x05:\x02-1R\x08field272\x12\x1B\n\x08field150\x18\x96\x01 \x01(\x05R\x08field150\x12\x1B\n\x07field23\x18\x17 \x01(\x05:\x010R\x07field23\x12\x1F\n\x07field24\x18\x18 \x01(\x08:\x05falseR\x07field24\x12\x1B\n\x07field25\x18\x19 \x01(\x05:\x010R\x07field25\x12<\n\x07field15\x18\x0F \x01(\x0B2\".benchmarks.SizeMessage1SubMessageR\x07field15\x12\x18\n\x07field78\x18N \x01(\x08R\x07field78\x12\x1B\n\x07field67\x18C \x01(\x05:\x010R\x07field67\x12\x18\n\x07field68\x18D \x01(\x05R\x07field68\x12\x1E\n\x08field128\x18\x80\x01 \x01(\x05:\x010R\x08field128\x122\n\x08field129\x18\x81\x01 \x01(\t:\x15xxxxxxxxxxxxxxxxxxxxxR\x08field129\x12\x1E\n\x08field131\x18\x83\x01 \x01(\x05:\x010R\x08field131\"\xD8\x04\n\x16SizeMessage1SubMessage\x12\x19\n\x06field1\x18\x01 \x01(\x05:\x010R\x06field1\x12\x19\n\x06field2\x18\x02 \x01(\x05:\x010R\x06field2\x12\x19\n\x06field3\x18\x03 \x01(\x05:\x010R\x06field3\x12\x18\n\x07field15\x18\x0F \x01(\tR\x07field15\x12\x1E\n\x07field12\x18\x0C \x01(\x08:\x04trueR\x07field12\x12\x18\n\x07field13\x18\r \x01(\x03R\x07field13\x12\x18\n\x07field14\x18\x0E \x01(\x03R\x07field14\x12\x18\n\x07field16\x18\x10 \x01(\x05R\x07field16\x12\x1B\n\x07field19\x18\x13 \x01(\x05:\x012R\x07field19\x12\x1E\n\x07field20\x18\x14 \x01(\x08:\x04trueR\x07field20\x12\x1E\n\x07field28\x18\x1C \x01(\x08:\x04trueR\x07field28\x12\x18\n\x07field21\x18\x15 \x01(\x06R\x07field21\x12\x18\n\x07field22\x18\x16 \x01(\x05R\x07field22\x12\x1F\n\x07field23\x18\x17 \x01(\x08:\x05falseR\x07field23\x12\"\n\x08field206\x18\xCE\x01 \x01(\x08:\x05falseR\x08field206\x12\x1B\n\x08field203\x18\xCB\x01 \x01(\x07R\x08field203\x12\x1B\n\x08field204\x18\xCC\x01 \x01(\x05R\x08field204\x12\x1B\n\x08field205\x18\xCD\x01 \x01(\tR\x08field205\x12\x1B\n\x08field207\x18\xCF\x01 \x01(\x04R\x08field207\x12\x1B\n\x08field300\x18\xAC\x02 \x01(\x04R\x08field300\"\xF0\n\n\x0CSizeMessage2\x12\x16\n\x06field1\x18\x01 \x01(\tR\x06field1\x12\x16\n\x06field3\x18\x03 \x01(\x03R\x06field3\x12\x16\n\x06field4\x18\x04 \x01(\x03R\x06field4\x12\x18\n\x07field30\x18\x1E \x01(\x03R\x07field30\x12\x1F\n\x07field75\x18K \x01(\x08:\x05falseR\x07field75\x12\x16\n\x06field6\x18\x06 \x01(\tR\x06field6\x12\x16\n\x06field2\x18\x02 \x01(\x0CR\x06field2\x12\x1B\n\x07field21\x18\x15 \x01(\x05:\x010R\x07field21\x12\x18\n\x07field71\x18G \x01(\x05R\x07field71\x12\x18\n\x07field25\x18\x19 \x01(\x02R\x07field25\x12\x1D\n\x08field109\x18m \x01(\x05:\x010R\x08field109\x12\x1E\n\x08field210\x18\xD2\x01 \x01(\x05:\x010R\x08field210\x12\x1E\n\x08field211\x18\xD3\x01 \x01(\x05:\x010R\x08field211\x12\x1E\n\x08field212\x18\xD4\x01 \x01(\x05:\x010R\x08field212\x12\x1E\n\x08field213\x18\xD5\x01 \x01(\x05:\x010R\x08field213\x12\x1E\n\x08field216\x18\xD8\x01 \x01(\x05:\x010R\x08field216\x12\x1E\n\x08field217\x18\xD9\x01 \x01(\x05:\x010R\x08field217\x12\x1E\n\x08field218\x18\xDA\x01 \x01(\x05:\x010R\x08field218\x12\x1E\n\x08field220\x18\xDC\x01 \x01(\x05:\x010R\x08field220\x12\x1E\n\x08field221\x18\xDD\x01 \x01(\x05:\x010R\x08field221\x12\x1E\n\x08field222\x18\xDE\x01 \x01(\x02:\x010R\x08field222\x12\x18\n\x07field63\x18? \x01(\x05R\x07field63\x127\n\x06group1\x18\n \x03(\n2\x1F.benchmarks.SizeMessage2.Group1R\x06group1\x12\x1B\n\x08field128\x18\x80\x01 \x03(\tR\x08field128\x12\x1B\n\x08field131\x18\x83\x01 \x01(\x03R\x08field131\x12\x1A\n\x08field127\x18\x7F \x03(\tR\x08field127\x12\x1B\n\x08field129\x18\x81\x01 \x01(\x05R\x08field129\x12\x1B\n\x08field130\x18\x82\x01 \x03(\x03R\x08field130\x12\"\n\x08field205\x18\xCD\x01 \x01(\x08:\x05falseR\x08field205\x12\"\n\x08field206\x18\xCE\x01 \x01(\x08:\x05falseR\x08field206\x1A\xD1\x03\n\x06Group1\x12\x18\n\x07field11\x18\x0B \x02(\x02R\x07field11\x12\x18\n\x07field26\x18\x1A \x01(\x02R\x07field26\x12\x18\n\x07field12\x18\x0C \x01(\tR\x07field12\x12\x18\n\x07field13\x18\r \x01(\tR\x07field13\x12\x18\n\x07field14\x18\x0E \x03(\tR\x07field14\x12\x18\n\x07field15\x18\x0F \x02(\x04R\x07field15\x12\x16\n\x06field5\x18\x05 \x01(\x05R\x06field5\x12\x18\n\x07field27\x18\x1B \x01(\tR\x07field27\x12\x18\n\x07field28\x18\x1C \x01(\x05R\x07field28\x12\x18\n\x07field29\x18\x1D \x01(\tR\x07field29\x12\x18\n\x07field16\x18\x10 \x01(\tR\x07field16\x12\x18\n\x07field22\x18\x16 \x03(\tR\x07field22\x12\x18\n\x07field73\x18I \x03(\x05R\x07field73\x12\x1B\n\x07field20\x18\x14 \x01(\x05:\x010R\x07field20\x12\x18\n\x07field24\x18\x18 \x01(\tR\x07field24\x12@\n\x07field31\x18\x1F \x01(\x0B2&.benchmarks.SizeMessage2GroupedMessageR\x07field31\"\xB8\x02\n\x1ASizeMessage2GroupedMessage\x12\x16\n\x06field1\x18\x01 \x01(\x02R\x06field1\x12\x16\n\x06field2\x18\x02 \x01(\x02R\x06field2\x12\x19\n\x06field3\x18\x03 \x01(\x02:\x010R\x06field3\x12\x16\n\x06field4\x18\x04 \x01(\x08R\x06field4\x12\x16\n\x06field5\x18\x05 \x01(\x08R\x06field5\x12\x1C\n\x06field6\x18\x06 \x01(\x08:\x04trueR\x06field6\x12\x1D\n\x06field7\x18\x07 \x01(\x08:\x05falseR\x06field7\x12\x16\n\x06field8\x18\x08 \x01(\x02R\x06field8\x12\x16\n\x06field9\x18\t \x01(\x08R\x06field9\x12\x18\n\x07field10\x18\n \x01(\x02R\x07field10\x12\x18\n\x07field11\x18\x0B \x01(\x03R\x07field11B\x0EB\nGoogleSizeH\x02J\xA8N\n\x07\x12\x05\0\0\x89\x01\x01\n\x08\n\x01\x0C\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x13\n\x08\n\x01\x08\x12\x03\x04\0+\n\t\n\x02\x08\x08\x12\x03\x04\0+\n\x08\n\x01\x08\x12\x03\x05\0 \n\t\n\x02\x08\t\x12\x03\x05\0 \n\n\n\x02\x04\0\x12\x04\x07\01\x01\n\n\n\x03\x04\0\x01\x12\x03\x07\x08\x14\n\x0B\n\x04\x04\0\x02\0\x12\x03\x08\x02\x1D\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03\x08\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03\x08\x0B\x11\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03\x08\x12\x18\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03\x08\x1B\x1C\n\x0B\n\x04\x04\0\x02\x01\x12\x03\t\x02\x1D\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03\t\x02\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03\t\x0B\x11\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03\t\x12\x18\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03\t\x1B\x1C\n\x0B\n\x04\x04\0\x02\x02\x12\x03\n\x02\x1F\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03\n\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03\n\x0B\x11\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03\n\x12\x19\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03\n\x1C\x1E\n\x0B\n\x04\x04\0\x02\x03\x12\x03\x0B\x02-\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x03\x0B\x02\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03\x0B\x0B\x0F\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03\x0B\x10\x17\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03\x0B\x1A\x1C\n\x0C\n\x05\x04\0\x02\x03\x08\x12\x03\x0B\x1D,\n\x0C\n\x05\x04\0\x02\x03\x07\x12\x03\x0B&+\n\x0B\n\x04\x04\0\x02\x04\x12\x03\x0C\x02,\n\x0C\n\x05\x04\0\x02\x04\x04\x12\x03\x0C\x02\n\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x03\x0C\x0B\x0F\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03\x0C\x10\x17\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03\x0C\x1A\x1C\n\x0C\n\x05\x04\0\x02\x04\x08\x12\x03\x0C\x1D+\n\x0C\n\x05\x04\0\x02\x04\x07\x12\x03\x0C&*\n\x0B\n\x04\x04\0\x02\x05\x12\x03\r\x02\x1C\n\x0C\n\x05\x04\0\x02\x05\x04\x12\x03\r\x02\n\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x03\r\x0B\x10\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x03\r\x11\x17\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x03\r\x1A\x1B\n\x0B\n\x04\x04\0\x02\x06\x12\x03\x0E\x02\x1C\n\x0C\n\x05\x04\0\x02\x06\x04\x12\x03\x0E\x02\n\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x03\x0E\x0B\x10\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x03\x0E\x11\x17\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x03\x0E\x1A\x1B\n\x0B\n\x04\x04\0\x02\x07\x12\x03\x0F\x02 \n\x0C\n\x05\x04\0\x02\x07\x04\x12\x03\x0F\x02\n\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x03\x0F\x0B\x10\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x03\x0F\x11\x19\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x03\x0F\x1C\x1F\n\x0B\n\x04\x04\0\x02\x08\x12\x03\x10\x02(\n\x0C\n\x05\x04\0\x02\x08\x04\x12\x03\x10\x02\n\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x03\x10\x0B\x10\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x03\x10\x11\x17\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x03\x10\x1A\x1B\n\x0C\n\x05\x04\0\x02\x08\x08\x12\x03\x10\x1C'\n\x0C\n\x05\x04\0\x02\x08\x07\x12\x03\x10%&\n\x0B\n\x04\x04\0\x02\t\x12\x03\x11\x02\x1E\n\x0C\n\x05\x04\0\x02\t\x04\x12\x03\x11\x02\n\n\x0C\n\x05\x04\0\x02\t\x05\x12\x03\x11\x0B\x10\n\x0C\n\x05\x04\0\x02\t\x01\x12\x03\x11\x11\x18\n\x0C\n\x05\x04\0\x02\t\x03\x12\x03\x11\x1B\x1D\n\x0B\n\x04\x04\0\x02\n\x12\x03\x12\x02\x1D\n\x0C\n\x05\x04\0\x02\n\x04\x12\x03\x12\x02\n\n\x0C\n\x05\x04\0\x02\n\x05\x12\x03\x12\x0B\x11\n\x0C\n\x05\x04\0\x02\n\x01\x12\x03\x12\x12\x18\n\x0C\n\x05\x04\0\x02\n\x03\x12\x03\x12\x1B\x1C\n\x0B\n\x04\x04\0\x02\x0B\x12\x03\x13\x02\x1E\n\x0C\n\x05\x04\0\x02\x0B\x04\x12\x03\x13\x02\n\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x03\x13\x0B\x12\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x03\x13\x13\x19\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x03\x13\x1C\x1D\n\x0B\n\x04\x04\0\x02\x0C\x12\x03\x14\x02-\n\x0C\n\x05\x04\0\x02\x0C\x04\x12\x03\x14\x02\n\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x03\x14\x0B\x0F\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x03\x14\x10\x17\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x03\x14\x1A\x1C\n\x0C\n\x05\x04\0\x02\x0C\x08\x12\x03\x14\x1D,\n\x0C\n\x05\x04\0\x02\x0C\x07\x12\x03\x14&+\n\x0B\n\x04\x04\0\x02\r\x12\x03\x15\x02\x1D\n\x0C\n\x05\x04\0\x02\r\x04\x12\x03\x15\x02\n\n\x0C\n\x05\x04\0\x02\r\x05\x12\x03\x15\x0B\x11\n\x0C\n\x05\x04\0\x02\r\x01\x12\x03\x15\x12\x18\n\x0C\n\x05\x04\0\x02\r\x03\x12\x03\x15\x1B\x1C\n\x0B\n\x04\x04\0\x02\x0E\x12\x03\x16\x02\x1E\n\x0C\n\x05\x04\0\x02\x0E\x04\x12\x03\x16\x02\n\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x03\x16\x0B\x10\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x03\x16\x11\x18\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x03\x16\x1B\x1D\n\x0B\n\x04\x04\0\x02\x0F\x12\x03\x17\x02,\n\x0C\n\x05\x04\0\x02\x0F\x04\x12\x03\x17\x02\n\n\x0C\n\x05\x04\0\x02\x0F\x05\x12\x03\x17\x0B\x10\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03\x17\x11\x19\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03\x17\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0F\x08\x12\x03\x17 +\n\x0C\n\x05\x04\0\x02\x0F\x07\x12\x03\x17)*\n\x0B\n\x04\x04\0\x02\x10\x12\x03\x18\x02,\n\x0C\n\x05\x04\0\x02\x10\x04\x12\x03\x18\x02\n\n\x0C\n\x05\x04\0\x02\x10\x05\x12\x03\x18\x0B\x0F\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03\x18\x10\x17\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03\x18\x1A\x1C\n\x0C\n\x05\x04\0\x02\x10\x08\x12\x03\x18\x1D+\n\x0C\n\x05\x04\0\x02\x10\x07\x12\x03\x18&*\n\x0B\n\x04\x04\0\x02\x11\x12\x03\x19\x02,\n\x0C\n\x05\x04\0\x02\x11\x04\x12\x03\x19\x02\n\n\x0C\n\x05\x04\0\x02\x11\x05\x12\x03\x19\x0B\x0F\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03\x19\x10\x17\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03\x19\x1A\x1C\n\x0C\n\x05\x04\0\x02\x11\x08\x12\x03\x19\x1D+\n\x0C\n\x05\x04\0\x02\x11\x07\x12\x03\x19&*\n\x0B\n\x04\x04\0\x02\x12\x12\x03\x1A\x02,\n\x0C\n\x05\x04\0\x02\x12\x04\x12\x03\x1A\x02\n\n\x0C\n\x05\x04\0\x02\x12\x05\x12\x03\x1A\x0B\x0F\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03\x1A\x10\x17\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03\x1A\x1A\x1C\n\x0C\n\x05\x04\0\x02\x12\x08\x12\x03\x1A\x1D+\n\x0C\n\x05\x04\0\x02\x12\x07\x12\x03\x1A&*\n\x0B\n\x04\x04\0\x02\x13\x12\x03\x1B\x02,\n\x0C\n\x05\x04\0\x02\x13\x04\x12\x03\x1B\x02\n\n\x0C\n\x05\x04\0\x02\x13\x05\x12\x03\x1B\x0B\x0F\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03\x1B\x10\x17\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03\x1B\x1A\x1C\n\x0C\n\x05\x04\0\x02\x13\x08\x12\x03\x1B\x1D+\n\x0C\n\x05\x04\0\x02\x13\x07\x12\x03\x1B&*\n\x0B\n\x04\x04\0\x02\x14\x12\x03\x1C\x02,\n\x0C\n\x05\x04\0\x02\x14\x04\x12\x03\x1C\x02\n\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03\x1C\x0B\x10\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03\x1C\x11\x19\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03\x1C\x1C\x1F\n\x0C\n\x05\x04\0\x02\x14\x08\x12\x03\x1C +\n\x0C\n\x05\x04\0\x02\x14\x07\x12\x03\x1C)*\n\x0B\n\x04\x04\0\x02\x15\x12\x03\x1D\x02,\n\x0C\n\x05\x04\0\x02\x15\x04\x12\x03\x1D\x02\n\n\x0C\n\x05\x04\0\x02\x15\x05\x12\x03\x1D\x0B\x10\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03\x1D\x11\x19\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03\x1D\x1C\x1F\n\x0C\n\x05\x04\0\x02\x15\x08\x12\x03\x1D +\n\x0C\n\x05\x04\0\x02\x15\x07\x12\x03\x1D)*\n\x0B\n\x04\x04\0\x02\x16\x12\x03\x1E\x02,\n\x0C\n\x05\x04\0\x02\x16\x04\x12\x03\x1E\x02\n\n\x0C\n\x05\x04\0\x02\x16\x05\x12\x03\x1E\x0B\x10\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03\x1E\x11\x19\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03\x1E\x1C\x1F\n\x0C\n\x05\x04\0\x02\x16\x08\x12\x03\x1E +\n\x0C\n\x05\x04\0\x02\x16\x07\x12\x03\x1E)*\n\x0B\n\x04\x04\0\x02\x17\x12\x03\x1F\x02!\n\x0C\n\x05\x04\0\x02\x17\x04\x12\x03\x1F\x02\n\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03\x1F\x0B\x11\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03\x1F\x12\x1A\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03\x1F\x1D \n\x0B\n\x04\x04\0\x02\x18\x12\x03 \x02!\n\x0C\n\x05\x04\0\x02\x18\x04\x12\x03 \x02\n\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03 \x0B\x11\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03 \x12\x1A\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03 \x1D \n\x0B\n\x04\x04\0\x02\x19\x12\x03!\x02*\n\x0C\n\x05\x04\0\x02\x19\x04\x12\x03!\x02\n\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03!\x0B\x10\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03!\x11\x18\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03!\x1B\x1D\n\x0C\n\x05\x04\0\x02\x19\x08\x12\x03!\x1E)\n\x0C\n\x05\x04\0\x02\x19\x07\x12\x03!'(\n\x0B\n\x04\x04\0\x02\x1A\x12\x03\"\x02-\n\x0C\n\x05\x04\0\x02\x1A\x04\x12\x03\"\x02\n\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03\"\x0B\x0F\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03\"\x10\x17\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03\"\x1A\x1C\n\x0C\n\x05\x04\0\x02\x1A\x08\x12\x03\"\x1D,\n\x0C\n\x05\x04\0\x02\x1A\x07\x12\x03\"&+\n\x0B\n\x04\x04\0\x02\x1B\x12\x03#\x02+\n\x0C\n\x05\x04\0\x02\x1B\x04\x12\x03#\x02\n\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03#\x0B\x10\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03#\x11\x18\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03#\x1B\x1D\n\x0C\n\x05\x04\0\x02\x1B\x08\x12\x03#\x1E*\n\x0C\n\x05\x04\0\x02\x1B\x07\x12\x03#')\n\x0B\n\x04\x04\0\x02\x1C\x12\x03$\x02-\n\x0C\n\x05\x04\0\x02\x1C\x04\x12\x03$\x02\n\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03$\x0B\x10\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03$\x11\x19\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03$\x1C\x1F\n\x0C\n\x05\x04\0\x02\x1C\x08\x12\x03$ ,\n\x0C\n\x05\x04\0\x02\x1C\x07\x12\x03$)+\n\x0B\n\x04\x04\0\x02\x1D\x12\x03%\x02-\n\x0C\n\x05\x04\0\x02\x1D\x04\x12\x03%\x02\n\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03%\x0B\x10\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03%\x11\x19\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03%\x1C\x1F\n\x0C\n\x05\x04\0\x02\x1D\x08\x12\x03% ,\n\x0C\n\x05\x04\0\x02\x1D\x07\x12\x03%)+\n\x0B\n\x04\x04\0\x02\x1E\x12\x03&\x02 \n\x0C\n\x05\x04\0\x02\x1E\x04\x12\x03&\x02\n\n\x0C\n\x05\x04\0\x02\x1E\x05\x12\x03&\x0B\x10\n\x0C\n\x05\x04\0\x02\x1E\x01\x12\x03&\x11\x19\n\x0C\n\x05\x04\0\x02\x1E\x03\x12\x03&\x1C\x1F\n\x0B\n\x04\x04\0\x02\x1F\x12\x03'\x02*\n\x0C\n\x05\x04\0\x02\x1F\x04\x12\x03'\x02\n\n\x0C\n\x05\x04\0\x02\x1F\x05\x12\x03'\x0B\x10\n\x0C\n\x05\x04\0\x02\x1F\x01\x12\x03'\x11\x18\n\x0C\n\x05\x04\0\x02\x1F\x03\x12\x03'\x1B\x1D\n\x0C\n\x05\x04\0\x02\x1F\x08\x12\x03'\x1E)\n\x0C\n\x05\x04\0\x02\x1F\x07\x12\x03''(\n\x0B\n\x04\x04\0\x02 \x12\x03(\x02-\n\x0C\n\x05\x04\0\x02 \x04\x12\x03(\x02\n\n\x0C\n\x05\x04\0\x02 \x05\x12\x03(\x0B\x0F\n\x0C\n\x05\x04\0\x02 \x01\x12\x03(\x10\x17\n\x0C\n\x05\x04\0\x02 \x03\x12\x03(\x1A\x1C\n\x0C\n\x05\x04\0\x02 \x08\x12\x03(\x1D,\n\x0C\n\x05\x04\0\x02 \x07\x12\x03(&+\n\x0B\n\x04\x04\0\x02!\x12\x03)\x02*\n\x0C\n\x05\x04\0\x02!\x04\x12\x03)\x02\n\n\x0C\n\x05\x04\0\x02!\x05\x12\x03)\x0B\x10\n\x0C\n\x05\x04\0\x02!\x01\x12\x03)\x11\x18\n\x0C\n\x05\x04\0\x02!\x03\x12\x03)\x1B\x1D\n\x0C\n\x05\x04\0\x02!\x08\x12\x03)\x1E)\n\x0C\n\x05\x04\0\x02!\x07\x12\x03)'(\n\x0B\n\x04\x04\0\x02\"\x12\x03*\x02/\n\x0C\n\x05\x04\0\x02\"\x04\x12\x03*\x02\n\n\x0C\n\x05\x04\0\x02\"\x06\x12\x03*\x0B!\n\x0C\n\x05\x04\0\x02\"\x01\x12\x03*\")\n\x0C\n\x05\x04\0\x02\"\x03\x12\x03*,.\n\x0B\n\x04\x04\0\x02#\x12\x03+\x02\x1D\n\x0C\n\x05\x04\0\x02#\x04\x12\x03+\x02\n\n\x0C\n\x05\x04\0\x02#\x05\x12\x03+\x0B\x0F\n\x0C\n\x05\x04\0\x02#\x01\x12\x03+\x10\x17\n\x0C\n\x05\x04\0\x02#\x03\x12\x03+\x1A\x1C\n\x0B\n\x04\x04\0\x02$\x12\x03,\x02*\n\x0C\n\x05\x04\0\x02$\x04\x12\x03,\x02\n\n\x0C\n\x05\x04\0\x02$\x05\x12\x03,\x0B\x10\n\x0C\n\x05\x04\0\x02$\x01\x12\x03,\x11\x18\n\x0C\n\x05\x04\0\x02$\x03\x12\x03,\x1B\x1D\n\x0C\n\x05\x04\0\x02$\x08\x12\x03,\x1E)\n\x0C\n\x05\x04\0\x02$\x07\x12\x03,'(\n\x0B\n\x04\x04\0\x02%\x12\x03-\x02\x1E\n\x0C\n\x05\x04\0\x02%\x04\x12\x03-\x02\n\n\x0C\n\x05\x04\0\x02%\x05\x12\x03-\x0B\x10\n\x0C\n\x05\x04\0\x02%\x01\x12\x03-\x11\x18\n\x0C\n\x05\x04\0\x02%\x03\x12\x03-\x1B\x1D\n\x0B\n\x04\x04\0\x02&\x12\x03.\x02,\n\x0C\n\x05\x04\0\x02&\x04\x12\x03.\x02\n\n\x0C\n\x05\x04\0\x02&\x05\x12\x03.\x0B\x10\n\x0C\n\x05\x04\0\x02&\x01\x12\x03.\x11\x19\n\x0C\n\x05\x04\0\x02&\x03\x12\x03.\x1C\x1F\n\x0C\n\x05\x04\0\x02&\x08\x12\x03. +\n\x0C\n\x05\x04\0\x02&\x07\x12\x03.)*\n\x0B\n\x04\x04\0\x02'\x12\x03/\x02C\n\x0C\n\x05\x04\0\x02'\x04\x12\x03/\x02\n\n\x0C\n\x05\x04\0\x02'\x05\x12\x03/\x0B\x11\n\x0C\n\x05\x04\0\x02'\x01\x12\x03/\x12\x1A\n\x0C\n\x05\x04\0\x02'\x03\x12\x03/\x1D \n\x0C\n\x05\x04\0\x02'\x08\x12\x03/!B\n\x0C\n\x05\x04\0\x02'\x07\x12\x03/*A\n\x0B\n\x04\x04\0\x02(\x12\x030\x02,\n\x0C\n\x05\x04\0\x02(\x04\x12\x030\x02\n\n\x0C\n\x05\x04\0\x02(\x05\x12\x030\x0B\x10\n\x0C\n\x05\x04\0\x02(\x01\x12\x030\x11\x19\n\x0C\n\x05\x04\0\x02(\x03\x12\x030\x1C\x1F\n\x0C\n\x05\x04\0\x02(\x08\x12\x030 +\n\x0C\n\x05\x04\0\x02(\x07\x12\x030)*\n\n\n\x02\x04\x01\x12\x043\0H\x01\n\n\n\x03\x04\x01\x01\x12\x033\x08\x1E\n\x0B\n\x04\x04\x01\x02\0\x12\x034\x02(\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x034\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x034\x0B\x10\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x034\x11\x17\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x034\x1A\x1B\n\x0C\n\x05\x04\x01\x02\0\x08\x12\x034\x1C'\n\x0C\n\x05\x04\x01\x02\0\x07\x12\x034%&\n\x0B\n\x04\x04\x01\x02\x01\x12\x035\x02(\n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x035\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x035\x0B\x10\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x035\x11\x17\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x035\x1A\x1B\n\x0C\n\x05\x04\x01\x02\x01\x08\x12\x035\x1C'\n\x0C\n\x05\x04\x01\x02\x01\x07\x12\x035%&\n\x0B\n\x04\x04\x01\x02\x02\x12\x036\x02(\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x036\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x036\x0B\x10\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x036\x11\x17\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x036\x1A\x1B\n\x0C\n\x05\x04\x01\x02\x02\x08\x12\x036\x1C'\n\x0C\n\x05\x04\x01\x02\x02\x07\x12\x036%&\n\x0B\n\x04\x04\x01\x02\x03\x12\x037\x02\x1F\n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x037\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x037\x0B\x11\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x037\x12\x19\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x037\x1C\x1E\n\x0B\n\x04\x04\x01\x02\x04\x12\x038\x02,\n\x0C\n\x05\x04\x01\x02\x04\x04\x12\x038\x02\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x038\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x038\x10\x17\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x038\x1A\x1C\n\x0C\n\x05\x04\x01\x02\x04\x08\x12\x038\x1D+\n\x0C\n\x05\x04\x01\x02\x04\x07\x12\x038&*\n\x0B\n\x04\x04\x01\x02\x05\x12\x039\x02\x1E\n\x0C\n\x05\x04\x01\x02\x05\x04\x12\x039\x02\n\n\x0C\n\x05\x04\x01\x02\x05\x05\x12\x039\x0B\x10\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x039\x11\x18\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x039\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x06\x12\x03:\x02\x1E\n\x0C\n\x05\x04\x01\x02\x06\x04\x12\x03:\x02\n\n\x0C\n\x05\x04\x01\x02\x06\x05\x12\x03:\x0B\x10\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03:\x11\x18\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03:\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x07\x12\x03;\x02\x1E\n\x0C\n\x05\x04\x01\x02\x07\x04\x12\x03;\x02\n\n\x0C\n\x05\x04\x01\x02\x07\x05\x12\x03;\x0B\x10\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03;\x11\x18\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03;\x1B\x1D\n\x0B\n\x04\x04\x01\x02\x08\x12\x03<\x02*\n\x0C\n\x05\x04\x01\x02\x08\x04\x12\x03<\x02\n\n\x0C\n\x05\x04\x01\x02\x08\x05\x12\x03<\x0B\x10\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03<\x11\x18\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03<\x1B\x1D\n\x0C\n\x05\x04\x01\x02\x08\x08\x12\x03<\x1E)\n\x0C\n\x05\x04\x01\x02\x08\x07\x12\x03<'(\n\x0B\n\x04\x04\x01\x02\t\x12\x03=\x02-\n\x0C\n\x05\x04\x01\x02\t\x04\x12\x03=\x02\n\n\x0C\n\x05\x04\x01\x02\t\x05\x12\x03=\x0B\x0F\n\x0C\n\x05\x04\x01\x02\t\x01\x12\x03=\x10\x17\n\x0C\n\x05\x04\x01\x02\t\x03\x12\x03=\x1B\x1D\n\x0C\n\x05\x04\x01\x02\t\x08\x12\x03=\x1E,\n\x0C\n\x05\x04\x01\x02\t\x07\x12\x03='+\n\x0B\n\x04\x04\x01\x02\n\x12\x03>\x02,\n\x0C\n\x05\x04\x01\x02\n\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\x01\x02\n\x05\x12\x03>\x0B\x0F\n\x0C\n\x05\x04\x01\x02\n\x01\x12\x03>\x10\x17\n\x0C\n\x05\x04\x01\x02\n\x03\x12\x03>\x1A\x1C\n\x0C\n\x05\x04\x01\x02\n\x08\x12\x03>\x1D+\n\x0C\n\x05\x04\x01\x02\n\x07\x12\x03>&*\n\x0B\n\x04\x04\x01\x02\x0B\x12\x03?\x02 \n\x0C\n\x05\x04\x01\x02\x0B\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\x01\x02\x0B\x05\x12\x03?\x0B\x12\n\x0C\n\x05\x04\x01\x02\x0B\x01\x12\x03?\x13\x1A\n\x0C\n\x05\x04\x01\x02\x0B\x03\x12\x03?\x1D\x1F\n\x0B\n\x04\x04\x01\x02\x0C\x12\x03@\x02\x1E\n\x0C\n\x05\x04\x01\x02\x0C\x04\x12\x03@\x02\n\n\x0C\n\x05\x04\x01\x02\x0C\x05\x12\x03@\x0B\x10\n\x0C\n\x05\x04\x01\x02\x0C\x01\x12\x03@\x11\x18\n\x0C\n\x05\x04\x01\x02\x0C\x03\x12\x03@\x1B\x1D\n\x0B\n\x04\x04\x01\x02\r\x12\x03A\x02/\n\x0C\n\x05\x04\x01\x02\r\x04\x12\x03A\x02\n\n\x0C\n\x05\x04\x01\x02\r\x05\x12\x03A\x0B\x0F\n\x0C\n\x05\x04\x01\x02\r\x01\x12\x03A\x10\x17\n\x0C\n\x05\x04\x01\x02\r\x03\x12\x03A\x1A\x1C\n\x0C\n\x05\x04\x01\x02\r\x08\x12\x03A\x1D.\n\x0C\n\x05\x04\x01\x02\r\x07\x12\x03A',\n\x0B\n\x04\x04\x01\x02\x0E\x12\x03B\x02/\n\x0C\n\x05\x04\x01\x02\x0E\x04\x12\x03B\x02\n\n\x0C\n\x05\x04\x01\x02\x0E\x05\x12\x03B\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x0E\x01\x12\x03B\x10\x18\n\x0C\n\x05\x04\x01\x02\x0E\x03\x12\x03B\x1B\x1E\n\x0C\n\x05\x04\x01\x02\x0E\x08\x12\x03B\x1F.\n\x0C\n\x05\x04\x01\x02\x0E\x07\x12\x03B(-\n\x0B\n\x04\x04\x01\x02\x0F\x12\x03C\x02\"\n\x0C\n\x05\x04\x01\x02\x0F\x04\x12\x03C\x02\n\n\x0C\n\x05\x04\x01\x02\x0F\x05\x12\x03C\x0B\x12\n\x0C\n\x05\x04\x01\x02\x0F\x01\x12\x03C\x13\x1B\n\x0C\n\x05\x04\x01\x02\x0F\x03\x12\x03C\x1E!\n\x0B\n\x04\x04\x01\x02\x10\x12\x03D\x02 \n\x0C\n\x05\x04\x01\x02\x10\x04\x12\x03D\x02\n\n\x0C\n\x05\x04\x01\x02\x10\x05\x12\x03D\x0B\x10\n\x0C\n\x05\x04\x01\x02\x10\x01\x12\x03D\x11\x19\n\x0C\n\x05\x04\x01\x02\x10\x03\x12\x03D\x1C\x1F\n\x0B\n\x04\x04\x01\x02\x11\x12\x03E\x02!\n\x0C\n\x05\x04\x01\x02\x11\x04\x12\x03E\x02\n\n\x0C\n\x05\x04\x01\x02\x11\x05\x12\x03E\x0B\x11\n\x0C\n\x05\x04\x01\x02\x11\x01\x12\x03E\x12\x1A\n\x0C\n\x05\x04\x01\x02\x11\x03\x12\x03E\x1D \n\x0B\n\x04\x04\x01\x02\x12\x12\x03F\x02!\n\x0C\n\x05\x04\x01\x02\x12\x04\x12\x03F\x02\n\n\x0C\n\x05\x04\x01\x02\x12\x05\x12\x03F\x0B\x11\n\x0C\n\x05\x04\x01\x02\x12\x01\x12\x03F\x12\x1A\n\x0C\n\x05\x04\x01\x02\x12\x03\x12\x03F\x1D \n\x0B\n\x04\x04\x01\x02\x13\x12\x03G\x02!\n\x0C\n\x05\x04\x01\x02\x13\x04\x12\x03G\x02\n\n\x0C\n\x05\x04\x01\x02\x13\x05\x12\x03G\x0B\x11\n\x0C\n\x05\x04\x01\x02\x13\x01\x12\x03G\x12\x1A\n\x0C\n\x05\x04\x01\x02\x13\x03\x12\x03G\x1D \n\n\n\x02\x04\x02\x12\x04J\0{\x01\n\n\n\x03\x04\x02\x01\x12\x03J\x08\x14\n\x0B\n\x04\x04\x02\x02\0\x12\x03K\x02\x1D\n\x0C\n\x05\x04\x02\x02\0\x04\x12\x03K\x02\n\n\x0C\n\x05\x04\x02\x02\0\x05\x12\x03K\x0B\x11\n\x0C\n\x05\x04\x02\x02\0\x01\x12\x03K\x12\x18\n\x0C\n\x05\x04\x02\x02\0\x03\x12\x03K\x1B\x1C\n\x0B\n\x04\x04\x02\x02\x01\x12\x03L\x02\x1C\n\x0C\n\x05\x04\x02\x02\x01\x04\x12\x03L\x02\n\n\x0C\n\x05\x04\x02\x02\x01\x05\x12\x03L\x0B\x10\n\x0C\n\x05\x04\x02\x02\x01\x01\x12\x03L\x11\x17\n\x0C\n\x05\x04\x02\x02\x01\x03\x12\x03L\x1A\x1B\n\x0B\n\x04\x04\x02\x02\x02\x12\x03M\x02\x1C\n\x0C\n\x05\x04\x02\x02\x02\x04\x12\x03M\x02\n\n\x0C\n\x05\x04\x02\x02\x02\x05\x12\x03M\x0B\x10\n\x0C\n\x05\x04\x02\x02\x02\x01\x12\x03M\x11\x17\n\x0C\n\x05\x04\x02\x02\x02\x03\x12\x03M\x1A\x1B\n\x0B\n\x04\x04\x02\x02\x03\x12\x03N\x02\x1E\n\x0C\n\x05\x04\x02\x02\x03\x04\x12\x03N\x02\n\n\x0C\n\x05\x04\x02\x02\x03\x05\x12\x03N\x0B\x10\n\x0C\n\x05\x04\x02\x02\x03\x01\x12\x03N\x11\x18\n\x0C\n\x05\x04\x02\x02\x03\x03\x12\x03N\x1B\x1D\n\x0B\n\x04\x04\x02\x02\x04\x12\x03O\x02.\n\x0C\n\x05\x04\x02\x02\x04\x04\x12\x03O\x02\n\n\x0C\n\x05\x04\x02\x02\x04\x05\x12\x03O\x0B\x0F\n\x0C\n\x05\x04\x02\x02\x04\x01\x12\x03O\x10\x17\n\x0C\n\x05\x04\x02\x02\x04\x03\x12\x03O\x1B\x1D\n\x0C\n\x05\x04\x02\x02\x04\x08\x12\x03O\x1E-\n\x0C\n\x05\x04\x02\x02\x04\x07\x12\x03O',\n\x0B\n\x04\x04\x02\x02\x05\x12\x03P\x02\x1D\n\x0C\n\x05\x04\x02\x02\x05\x04\x12\x03P\x02\n\n\x0C\n\x05\x04\x02\x02\x05\x05\x12\x03P\x0B\x11\n\x0C\n\x05\x04\x02\x02\x05\x01\x12\x03P\x12\x18\n\x0C\n\x05\x04\x02\x02\x05\x03\x12\x03P\x1B\x1C\n\x0B\n\x04\x04\x02\x02\x06\x12\x03Q\x02\x1C\n\x0C\n\x05\x04\x02\x02\x06\x04\x12\x03Q\x02\n\n\x0C\n\x05\x04\x02\x02\x06\x05\x12\x03Q\x0B\x10\n\x0C\n\x05\x04\x02\x02\x06\x01\x12\x03Q\x11\x17\n\x0C\n\x05\x04\x02\x02\x06\x03\x12\x03Q\x1A\x1B\n\x0B\n\x04\x04\x02\x02\x07\x12\x03R\x02*\n\x0C\n\x05\x04\x02\x02\x07\x04\x12\x03R\x02\n\n\x0C\n\x05\x04\x02\x02\x07\x05\x12\x03R\x0B\x10\n\x0C\n\x05\x04\x02\x02\x07\x01\x12\x03R\x11\x18\n\x0C\n\x05\x04\x02\x02\x07\x03\x12\x03R\x1B\x1D\n\x0C\n\x05\x04\x02\x02\x07\x08\x12\x03R\x1E)\n\x0C\n\x05\x04\x02\x02\x07\x07\x12\x03R'(\n\x0B\n\x04\x04\x02\x02\x08\x12\x03S\x02\x1E\n\x0C\n\x05\x04\x02\x02\x08\x04\x12\x03S\x02\n\n\x0C\n\x05\x04\x02\x02\x08\x05\x12\x03S\x0B\x10\n\x0C\n\x05\x04\x02\x02\x08\x01\x12\x03S\x11\x18\n\x0C\n\x05\x04\x02\x02\x08\x03\x12\x03S\x1B\x1D\n\x0B\n\x04\x04\x02\x02\t\x12\x03T\x02\x1E\n\x0C\n\x05\x04\x02\x02\t\x04\x12\x03T\x02\n\n\x0C\n\x05\x04\x02\x02\t\x05\x12\x03T\x0B\x10\n\x0C\n\x05\x04\x02\x02\t\x01\x12\x03T\x11\x18\n\x0C\n\x05\x04\x02\x02\t\x03\x12\x03T\x1B\x1D\n\x0B\n\x04\x04\x02\x02\n\x12\x03U\x02,\n\x0C\n\x05\x04\x02\x02\n\x04\x12\x03U\x02\n\n\x0C\n\x05\x04\x02\x02\n\x05\x12\x03U\x0B\x10\n\x0C\n\x05\x04\x02\x02\n\x01\x12\x03U\x11\x19\n\x0C\n\x05\x04\x02\x02\n\x03\x12\x03U\x1C\x1F\n\x0C\n\x05\x04\x02\x02\n\x08\x12\x03U +\n\x0C\n\x05\x04\x02\x02\n\x07\x12\x03U)*\n\x0B\n\x04\x04\x02\x02\x0B\x12\x03V\x02,\n\x0C\n\x05\x04\x02\x02\x0B\x04\x12\x03V\x02\n\n\x0C\n\x05\x04\x02\x02\x0B\x05\x12\x03V\x0B\x10\n\x0C\n\x05\x04\x02\x02\x0B\x01\x12\x03V\x11\x19\n\x0C\n\x05\x04\x02\x02\x0B\x03\x12\x03V\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x0B\x08\x12\x03V +\n\x0C\n\x05\x04\x02\x02\x0B\x07\x12\x03V)*\n\x0B\n\x04\x04\x02\x02\x0C\x12\x03W\x02,\n\x0C\n\x05\x04\x02\x02\x0C\x04\x12\x03W\x02\n\n\x0C\n\x05\x04\x02\x02\x0C\x05\x12\x03W\x0B\x10\n\x0C\n\x05\x04\x02\x02\x0C\x01\x12\x03W\x11\x19\n\x0C\n\x05\x04\x02\x02\x0C\x03\x12\x03W\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x0C\x08\x12\x03W +\n\x0C\n\x05\x04\x02\x02\x0C\x07\x12\x03W)*\n\x0B\n\x04\x04\x02\x02\r\x12\x03X\x02,\n\x0C\n\x05\x04\x02\x02\r\x04\x12\x03X\x02\n\n\x0C\n\x05\x04\x02\x02\r\x05\x12\x03X\x0B\x10\n\x0C\n\x05\x04\x02\x02\r\x01\x12\x03X\x11\x19\n\x0C\n\x05\x04\x02\x02\r\x03\x12\x03X\x1C\x1F\n\x0C\n\x05\x04\x02\x02\r\x08\x12\x03X +\n\x0C\n\x05\x04\x02\x02\r\x07\x12\x03X)*\n\x0B\n\x04\x04\x02\x02\x0E\x12\x03Y\x02,\n\x0C\n\x05\x04\x02\x02\x0E\x04\x12\x03Y\x02\n\n\x0C\n\x05\x04\x02\x02\x0E\x05\x12\x03Y\x0B\x10\n\x0C\n\x05\x04\x02\x02\x0E\x01\x12\x03Y\x11\x19\n\x0C\n\x05\x04\x02\x02\x0E\x03\x12\x03Y\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x0E\x08\x12\x03Y +\n\x0C\n\x05\x04\x02\x02\x0E\x07\x12\x03Y)*\n\x0B\n\x04\x04\x02\x02\x0F\x12\x03Z\x02,\n\x0C\n\x05\x04\x02\x02\x0F\x04\x12\x03Z\x02\n\n\x0C\n\x05\x04\x02\x02\x0F\x05\x12\x03Z\x0B\x10\n\x0C\n\x05\x04\x02\x02\x0F\x01\x12\x03Z\x11\x19\n\x0C\n\x05\x04\x02\x02\x0F\x03\x12\x03Z\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x0F\x08\x12\x03Z +\n\x0C\n\x05\x04\x02\x02\x0F\x07\x12\x03Z)*\n\x0B\n\x04\x04\x02\x02\x10\x12\x03[\x02,\n\x0C\n\x05\x04\x02\x02\x10\x04\x12\x03[\x02\n\n\x0C\n\x05\x04\x02\x02\x10\x05\x12\x03[\x0B\x10\n\x0C\n\x05\x04\x02\x02\x10\x01\x12\x03[\x11\x19\n\x0C\n\x05\x04\x02\x02\x10\x03\x12\x03[\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x10\x08\x12\x03[ +\n\x0C\n\x05\x04\x02\x02\x10\x07\x12\x03[)*\n\x0B\n\x04\x04\x02\x02\x11\x12\x03\\\x02,\n\x0C\n\x05\x04\x02\x02\x11\x04\x12\x03\\\x02\n\n\x0C\n\x05\x04\x02\x02\x11\x05\x12\x03\\\x0B\x10\n\x0C\n\x05\x04\x02\x02\x11\x01\x12\x03\\\x11\x19\n\x0C\n\x05\x04\x02\x02\x11\x03\x12\x03\\\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x11\x08\x12\x03\\ +\n\x0C\n\x05\x04\x02\x02\x11\x07\x12\x03\\)*\n\x0B\n\x04\x04\x02\x02\x12\x12\x03]\x02,\n\x0C\n\x05\x04\x02\x02\x12\x04\x12\x03]\x02\n\n\x0C\n\x05\x04\x02\x02\x12\x05\x12\x03]\x0B\x10\n\x0C\n\x05\x04\x02\x02\x12\x01\x12\x03]\x11\x19\n\x0C\n\x05\x04\x02\x02\x12\x03\x12\x03]\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x12\x08\x12\x03] +\n\x0C\n\x05\x04\x02\x02\x12\x07\x12\x03])*\n\x0B\n\x04\x04\x02\x02\x13\x12\x03^\x02,\n\x0C\n\x05\x04\x02\x02\x13\x04\x12\x03^\x02\n\n\x0C\n\x05\x04\x02\x02\x13\x05\x12\x03^\x0B\x10\n\x0C\n\x05\x04\x02\x02\x13\x01\x12\x03^\x11\x19\n\x0C\n\x05\x04\x02\x02\x13\x03\x12\x03^\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x13\x08\x12\x03^ +\n\x0C\n\x05\x04\x02\x02\x13\x07\x12\x03^)*\n\x0B\n\x04\x04\x02\x02\x14\x12\x03_\x02.\n\x0C\n\x05\x04\x02\x02\x14\x04\x12\x03_\x02\n\n\x0C\n\x05\x04\x02\x02\x14\x05\x12\x03_\x0B\x10\n\x0C\n\x05\x04\x02\x02\x14\x01\x12\x03_\x11\x19\n\x0C\n\x05\x04\x02\x02\x14\x03\x12\x03_\x1C\x1F\n\x0C\n\x05\x04\x02\x02\x14\x08\x12\x03_ -\n\x0C\n\x05\x04\x02\x02\x14\x07\x12\x03_),\n\x0B\n\x04\x04\x02\x02\x15\x12\x03`\x02\x1E\n\x0C\n\x05\x04\x02\x02\x15\x04\x12\x03`\x02\n\n\x0C\n\x05\x04\x02\x02\x15\x05\x12\x03`\x0B\x10\n\x0C\n\x05\x04\x02\x02\x15\x01\x12\x03`\x11\x18\n\x0C\n\x05\x04\x02\x02\x15\x03\x12\x03`\x1B\x1D\n\x0C\n\x04\x04\x02\x02\x16\x12\x04b\x02s\x03\n\x0C\n\x05\x04\x02\x02\x16\x04\x12\x03b\x02\n\n\x0C\n\x05\x04\x02\x02\x16\x05\x12\x03b\x0B\x10\n\x0C\n\x05\x04\x02\x02\x16\x01\x12\x03b\x11\x17\n\x0C\n\x05\x04\x02\x02\x16\x03\x12\x03b\x1A\x1C\n\x0C\n\x04\x04\x02\x03\0\x12\x04b\x02s\x03\n\x0C\n\x05\x04\x02\x03\0\x01\x12\x03b\x11\x17\n\x0C\n\x05\x04\x02\x02\x16\x06\x12\x03b\x11\x17\n\r\n\x06\x04\x02\x03\0\x02\0\x12\x03c\x04 \n\x0E\n\x07\x04\x02\x03\0\x02\0\x04\x12\x03c\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03c\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03c\x13\x1A\n\x0E\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03c\x1D\x1F\n\r\n\x06\x04\x02\x03\0\x02\x01\x12\x03d\x04 \n\x0E\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x03d\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x03d\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x03d\x13\x1A\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03d\x1D\x1F\n\r\n\x06\x04\x02\x03\0\x02\x02\x12\x03e\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x04\x12\x03e\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x05\x12\x03e\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x01\x12\x03e\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x03\x12\x03e\x1E \n\r\n\x06\x04\x02\x03\0\x02\x03\x12\x03f\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x03\x04\x12\x03f\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x03\x05\x12\x03f\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x03\x01\x12\x03f\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x03\x03\x12\x03f\x1E \n\r\n\x06\x04\x02\x03\0\x02\x04\x12\x03g\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x04\x04\x12\x03g\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x04\x05\x12\x03g\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x04\x01\x12\x03g\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x04\x03\x12\x03g\x1E \n\r\n\x06\x04\x02\x03\0\x02\x05\x12\x03h\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x05\x04\x12\x03h\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x05\x05\x12\x03h\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x05\x01\x12\x03h\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x05\x03\x12\x03h\x1E \n\r\n\x06\x04\x02\x03\0\x02\x06\x12\x03i\x04\x1E\n\x0E\n\x07\x04\x02\x03\0\x02\x06\x04\x12\x03i\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x06\x05\x12\x03i\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\x06\x01\x12\x03i\x13\x19\n\x0E\n\x07\x04\x02\x03\0\x02\x06\x03\x12\x03i\x1C\x1D\n\r\n\x06\x04\x02\x03\0\x02\x07\x12\x03j\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x07\x04\x12\x03j\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x07\x05\x12\x03j\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x07\x01\x12\x03j\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x07\x03\x12\x03j\x1E \n\r\n\x06\x04\x02\x03\0\x02\x08\x12\x03k\x04 \n\x0E\n\x07\x04\x02\x03\0\x02\x08\x04\x12\x03k\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x08\x05\x12\x03k\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\x08\x01\x12\x03k\x13\x1A\n\x0E\n\x07\x04\x02\x03\0\x02\x08\x03\x12\x03k\x1D\x1F\n\r\n\x06\x04\x02\x03\0\x02\t\x12\x03l\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\t\x04\x12\x03l\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\t\x05\x12\x03l\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\t\x01\x12\x03l\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\t\x03\x12\x03l\x1E \n\r\n\x06\x04\x02\x03\0\x02\n\x12\x03m\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\n\x04\x12\x03m\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\n\x05\x12\x03m\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\n\x01\x12\x03m\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\n\x03\x12\x03m\x1E \n\r\n\x06\x04\x02\x03\0\x02\x0B\x12\x03n\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x0B\x04\x12\x03n\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x0B\x05\x12\x03n\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x0B\x01\x12\x03n\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x0B\x03\x12\x03n\x1E \n\r\n\x06\x04\x02\x03\0\x02\x0C\x12\x03o\x04 \n\x0E\n\x07\x04\x02\x03\0\x02\x0C\x04\x12\x03o\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x0C\x05\x12\x03o\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\x0C\x01\x12\x03o\x13\x1A\n\x0E\n\x07\x04\x02\x03\0\x02\x0C\x03\x12\x03o\x1D\x1F\n\r\n\x06\x04\x02\x03\0\x02\r\x12\x03p\x04,\n\x0E\n\x07\x04\x02\x03\0\x02\r\x04\x12\x03p\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\r\x05\x12\x03p\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\r\x01\x12\x03p\x13\x1A\n\x0E\n\x07\x04\x02\x03\0\x02\r\x03\x12\x03p\x1D\x1F\n\x0E\n\x07\x04\x02\x03\0\x02\r\x08\x12\x03p +\n\x0E\n\x07\x04\x02\x03\0\x02\r\x07\x12\x03p)*\n\r\n\x06\x04\x02\x03\0\x02\x0E\x12\x03q\x04!\n\x0E\n\x07\x04\x02\x03\0\x02\x0E\x04\x12\x03q\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x0E\x05\x12\x03q\r\x13\n\x0E\n\x07\x04\x02\x03\0\x02\x0E\x01\x12\x03q\x14\x1B\n\x0E\n\x07\x04\x02\x03\0\x02\x0E\x03\x12\x03q\x1E \n\r\n\x06\x04\x02\x03\0\x02\x0F\x12\x03r\x045\n\x0E\n\x07\x04\x02\x03\0\x02\x0F\x04\x12\x03r\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x0F\x06\x12\x03r\r'\n\x0E\n\x07\x04\x02\x03\0\x02\x0F\x01\x12\x03r(/\n\x0E\n\x07\x04\x02\x03\0\x02\x0F\x03\x12\x03r24\n\x0B\n\x04\x04\x02\x02\x17\x12\x03t\x02!\n\x0C\n\x05\x04\x02\x02\x17\x04\x12\x03t\x02\n\n\x0C\n\x05\x04\x02\x02\x17\x05\x12\x03t\x0B\x11\n\x0C\n\x05\x04\x02\x02\x17\x01\x12\x03t\x12\x1A\n\x0C\n\x05\x04\x02\x02\x17\x03\x12\x03t\x1D \n\x0B\n\x04\x04\x02\x02\x18\x12\x03u\x02 \n\x0C\n\x05\x04\x02\x02\x18\x04\x12\x03u\x02\n\n\x0C\n\x05\x04\x02\x02\x18\x05\x12\x03u\x0B\x10\n\x0C\n\x05\x04\x02\x02\x18\x01\x12\x03u\x11\x19\n\x0C\n\x05\x04\x02\x02\x18\x03\x12\x03u\x1C\x1F\n\x0B\n\x04\x04\x02\x02\x19\x12\x03v\x02!\n\x0C\n\x05\x04\x02\x02\x19\x04\x12\x03v\x02\n\n\x0C\n\x05\x04\x02\x02\x19\x05\x12\x03v\x0B\x11\n\x0C\n\x05\x04\x02\x02\x19\x01\x12\x03v\x12\x1A\n\x0C\n\x05\x04\x02\x02\x19\x03\x12\x03v\x1D \n\x0B\n\x04\x04\x02\x02\x1A\x12\x03w\x02 \n\x0C\n\x05\x04\x02\x02\x1A\x04\x12\x03w\x02\n\n\x0C\n\x05\x04\x02\x02\x1A\x05\x12\x03w\x0B\x10\n\x0C\n\x05\x04\x02\x02\x1A\x01\x12\x03w\x11\x19\n\x0C\n\x05\x04\x02\x02\x1A\x03\x12\x03w\x1C\x1F\n\x0B\n\x04\x04\x02\x02\x1B\x12\x03x\x02 \n\x0C\n\x05\x04\x02\x02\x1B\x04\x12\x03x\x02\n\n\x0C\n\x05\x04\x02\x02\x1B\x05\x12\x03x\x0B\x10\n\x0C\n\x05\x04\x02\x02\x1B\x01\x12\x03x\x11\x19\n\x0C\n\x05\x04\x02\x02\x1B\x03\x12\x03x\x1C\x1F\n\x0B\n\x04\x04\x02\x02\x1C\x12\x03y\x02/\n\x0C\n\x05\x04\x02\x02\x1C\x04\x12\x03y\x02\n\n\x0C\n\x05\x04\x02\x02\x1C\x05\x12\x03y\x0B\x0F\n\x0C\n\x05\x04\x02\x02\x1C\x01\x12\x03y\x10\x18\n\x0C\n\x05\x04\x02\x02\x1C\x03\x12\x03y\x1B\x1E\n\x0C\n\x05\x04\x02\x02\x1C\x08\x12\x03y\x1F.\n\x0C\n\x05\x04\x02\x02\x1C\x07\x12\x03y(-\n\x0B\n\x04\x04\x02\x02\x1D\x12\x03z\x02/\n\x0C\n\x05\x04\x02\x02\x1D\x04\x12\x03z\x02\n\n\x0C\n\x05\x04\x02\x02\x1D\x05\x12\x03z\x0B\x0F\n\x0C\n\x05\x04\x02\x02\x1D\x01\x12\x03z\x10\x18\n\x0C\n\x05\x04\x02\x02\x1D\x03\x12\x03z\x1B\x1E\n\x0C\n\x05\x04\x02\x02\x1D\x08\x12\x03z\x1F.\n\x0C\n\x05\x04\x02\x02\x1D\x07\x12\x03z(-\n\x0B\n\x02\x04\x03\x12\x05}\0\x89\x01\x01\n\n\n\x03\x04\x03\x01\x12\x03}\x08\"\n\x0B\n\x04\x04\x03\x02\0\x12\x03~\x02\x1C\n\x0C\n\x05\x04\x03\x02\0\x04\x12\x03~\x02\n\n\x0C\n\x05\x04\x03\x02\0\x05\x12\x03~\x0B\x10\n\x0C\n\x05\x04\x03\x02\0\x01\x12\x03~\x11\x17\n\x0C\n\x05\x04\x03\x02\0\x03\x12\x03~\x1A\x1B\n\x0B\n\x04\x04\x03\x02\x01\x12\x03\x7F\x02\x1C\n\x0C\n\x05\x04\x03\x02\x01\x04\x12\x03\x7F\x02\n\n\x0C\n\x05\x04\x03\x02\x01\x05\x12\x03\x7F\x0B\x10\n\x0C\n\x05\x04\x03\x02\x01\x01\x12\x03\x7F\x11\x17\n\x0C\n\x05\x04\x03\x02\x01\x03\x12\x03\x7F\x1A\x1B\n\x0C\n\x04\x04\x03\x02\x02\x12\x04\x80\x01\x02*\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04\x80\x01\x02\n\n\r\n\x05\x04\x03\x02\x02\x05\x12\x04\x80\x01\x0B\x10\n\r\n\x05\x04\x03\x02\x02\x01\x12\x04\x80\x01\x11\x17\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\x80\x01\x1A\x1B\n\r\n\x05\x04\x03\x02\x02\x08\x12\x04\x80\x01\x1C)\n\r\n\x05\x04\x03\x02\x02\x07\x12\x04\x80\x01%(\n\x0C\n\x04\x04\x03\x02\x03\x12\x04\x81\x01\x02\x1B\n\r\n\x05\x04\x03\x02\x03\x04\x12\x04\x81\x01\x02\n\n\r\n\x05\x04\x03\x02\x03\x05\x12\x04\x81\x01\x0B\x0F\n\r\n\x05\x04\x03\x02\x03\x01\x12\x04\x81\x01\x10\x16\n\r\n\x05\x04\x03\x02\x03\x03\x12\x04\x81\x01\x19\x1A\n\x0C\n\x04\x04\x03\x02\x04\x12\x04\x82\x01\x02\x1B\n\r\n\x05\x04\x03\x02\x04\x04\x12\x04\x82\x01\x02\n\n\r\n\x05\x04\x03\x02\x04\x05\x12\x04\x82\x01\x0B\x0F\n\r\n\x05\x04\x03\x02\x04\x01\x12\x04\x82\x01\x10\x16\n\r\n\x05\x04\x03\x02\x04\x03\x12\x04\x82\x01\x19\x1A\n\x0C\n\x04\x04\x03\x02\x05\x12\x04\x83\x01\x02*\n\r\n\x05\x04\x03\x02\x05\x04\x12\x04\x83\x01\x02\n\n\r\n\x05\x04\x03\x02\x05\x05\x12\x04\x83\x01\x0B\x0F\n\r\n\x05\x04\x03\x02\x05\x01\x12\x04\x83\x01\x10\x16\n\r\n\x05\x04\x03\x02\x05\x03\x12\x04\x83\x01\x19\x1A\n\r\n\x05\x04\x03\x02\x05\x08\x12\x04\x83\x01\x1B)\n\r\n\x05\x04\x03\x02\x05\x07\x12\x04\x83\x01$(\n\x0C\n\x04\x04\x03\x02\x06\x12\x04\x84\x01\x02+\n\r\n\x05\x04\x03\x02\x06\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\x04\x03\x02\x06\x05\x12\x04\x84\x01\x0B\x0F\n\r\n\x05\x04\x03\x02\x06\x01\x12\x04\x84\x01\x10\x16\n\r\n\x05\x04\x03\x02\x06\x03\x12\x04\x84\x01\x19\x1A\n\r\n\x05\x04\x03\x02\x06\x08\x12\x04\x84\x01\x1B*\n\r\n\x05\x04\x03\x02\x06\x07\x12\x04\x84\x01$)\n\x0C\n\x04\x04\x03\x02\x07\x12\x04\x85\x01\x02\x1C\n\r\n\x05\x04\x03\x02\x07\x04\x12\x04\x85\x01\x02\n\n\r\n\x05\x04\x03\x02\x07\x05\x12\x04\x85\x01\x0B\x10\n\r\n\x05\x04\x03\x02\x07\x01\x12\x04\x85\x01\x11\x17\n\r\n\x05\x04\x03\x02\x07\x03\x12\x04\x85\x01\x1A\x1B\n\x0C\n\x04\x04\x03\x02\x08\x12\x04\x86\x01\x02\x1B\n\r\n\x05\x04\x03\x02\x08\x04\x12\x04\x86\x01\x02\n\n\r\n\x05\x04\x03\x02\x08\x05\x12\x04\x86\x01\x0B\x0F\n\r\n\x05\x04\x03\x02\x08\x01\x12\x04\x86\x01\x10\x16\n\r\n\x05\x04\x03\x02\x08\x03\x12\x04\x86\x01\x19\x1A\n\x0C\n\x04\x04\x03\x02\t\x12\x04\x87\x01\x02\x1E\n\r\n\x05\x04\x03\x02\t\x04\x12\x04\x87\x01\x02\n\n\r\n\x05\x04\x03\x02\t\x05\x12\x04\x87\x01\x0B\x10\n\r\n\x05\x04\x03\x02\t\x01\x12\x04\x87\x01\x11\x18\n\r\n\x05\x04\x03\x02\t\x03\x12\x04\x87\x01\x1B\x1D\n\x0C\n\x04\x04\x03\x02\n\x12\x04\x88\x01\x02\x1E\n\r\n\x05\x04\x03\x02\n\x04\x12\x04\x88\x01\x02\n\n\r\n\x05\x04\x03\x02\n\x05\x12\x04\x88\x01\x0B\x10\n\r\n\x05\x04\x03\x02\n\x01\x12\x04\x88\x01\x11\x18\n\r\n\x05\x04\x03\x02\n\x03\x12\x04\x88\x01\x1B\x1D" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
