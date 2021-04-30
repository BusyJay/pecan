#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage2_Group1 {
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
    pub field31: Option<GoogleMessage2GroupedMessage>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl GoogleMessage2_Group1 {
    pub const fn new() -> GoogleMessage2_Group1 {
        GoogleMessage2_Group1 {
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
    pub fn field31(&self) -> &GoogleMessage2GroupedMessage {
        match &self.field31 {
            Some(v) => v,
            _ => GoogleMessage2GroupedMessage::default_instance(),
        }
    }
    pub fn field31_mut(&mut self) -> &mut GoogleMessage2GroupedMessage {
        self.field31.get_or_insert_with(Default::default)
    }
    pub fn set_field31(&mut self, val: GoogleMessage2GroupedMessage) {
        self.field31 = Some(val);
    }
}
impl pecan::Message for GoogleMessage2_Group1 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                40 => self.field5 = Some(Varint::read_from(s)?),
                93 => self.field11 = Fixed32::read_from(s)?,
                98 => LengthPrefixed::merge_from(self.field12_mut(), s)?,
                106 => LengthPrefixed::merge_from(self.field13_mut(), s)?,
                114 => RefArray::<LengthPrefixed>::merge_from(&mut self.field14, s)?,
                120 => self.field15 = Varint::read_from(s)?,
                130 => LengthPrefixed::merge_from(self.field16_mut(), s)?,
                160 => self.field20 = Some(Varint::read_from(s)?),
                178 => RefArray::<LengthPrefixed>::merge_from(&mut self.field22, s)?,
                194 => LengthPrefixed::merge_from(self.field24_mut(), s)?,
                213 => self.field26 = Some(Fixed32::read_from(s)?),
                218 => LengthPrefixed::merge_from(self.field27_mut(), s)?,
                224 => self.field28 = Some(Varint::read_from(s)?),
                234 => LengthPrefixed::merge_from(self.field29_mut(), s)?,
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
    fn clear(&mut self) {
        self.field11 = 0f32;
        self.field26 = None;
        self.field12 = None;
        self.field13 = None;
        self.field14.clear();
        self.field15 = 0;
        self.field5 = None;
        self.field27 = None;
        self.field28 = None;
        self.field29 = None;
        self.field16 = None;
        self.field22.clear();
        self.field73.clear();
        self.field20 = None;
        self.field24 = None;
        self.field31 = None;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for GoogleMessage2_Group1 {
    fn default_instance() -> &'static GoogleMessage2_Group1 {
        static DEFAULT: GoogleMessage2_Group1 = GoogleMessage2_Group1::new();
        &DEFAULT
    }
}
impl Default for GoogleMessage2_Group1 {
    #[inline]
    fn default() -> GoogleMessage2_Group1 {
        GoogleMessage2_Group1::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage2 {
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
    pub group1: Vec<GoogleMessage2_Group1>,
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
impl GoogleMessage2 {
    pub const fn new() -> GoogleMessage2 {
        GoogleMessage2 {
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
impl pecan::Message for GoogleMessage2 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixed::merge_from(self.field1_mut(), s)?,
                18 => LengthPrefixed::merge_from(self.field2_mut(), s)?,
                24 => self.field3 = Some(Varint::read_from(s)?),
                32 => self.field4 = Some(Varint::read_from(s)?),
                50 => LengthPrefixed::merge_from(self.field6_mut(), s)?,
                83 => s.read_group(84, |s| {
                    self.group1.push(GoogleMessage2_Group1::new());
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
    fn clear(&mut self) {
        self.field1 = None;
        self.field3 = None;
        self.field4 = None;
        self.field30 = None;
        self.field75 = None;
        self.field6 = None;
        self.field2 = None;
        self.field21 = None;
        self.field71 = None;
        self.field25 = None;
        self.field109 = None;
        self.field210 = None;
        self.field211 = None;
        self.field212 = None;
        self.field213 = None;
        self.field216 = None;
        self.field217 = None;
        self.field218 = None;
        self.field220 = None;
        self.field221 = None;
        self.field222 = None;
        self.field63 = None;
        self.group1.clear();
        self.field128.clear();
        self.field131 = None;
        self.field127.clear();
        self.field129 = None;
        self.field130.clear();
        self.field205 = None;
        self.field206 = None;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for GoogleMessage2 {
    fn default_instance() -> &'static GoogleMessage2 {
        static DEFAULT: GoogleMessage2 = GoogleMessage2::new();
        &DEFAULT
    }
}
impl Default for GoogleMessage2 {
    #[inline]
    fn default() -> GoogleMessage2 {
        GoogleMessage2::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoogleMessage2GroupedMessage {
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
impl GoogleMessage2GroupedMessage {
    pub const fn new() -> GoogleMessage2GroupedMessage {
        GoogleMessage2GroupedMessage {
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
impl pecan::Message for GoogleMessage2GroupedMessage {
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
    fn clear(&mut self) {
        self.field1 = None;
        self.field2 = None;
        self.field3 = None;
        self.field4 = None;
        self.field5 = None;
        self.field6 = None;
        self.field7 = None;
        self.field8 = None;
        self.field9 = None;
        self.field10 = None;
        self.field11 = None;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for GoogleMessage2GroupedMessage {
    fn default_instance() -> &'static GoogleMessage2GroupedMessage {
        static DEFAULT: GoogleMessage2GroupedMessage = GoogleMessage2GroupedMessage::new();
        &DEFAULT
    }
}
impl Default for GoogleMessage2GroupedMessage {
    #[inline]
    fn default() -> GoogleMessage2GroupedMessage {
        GoogleMessage2GroupedMessage::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n1datasets/google_message2/benchmark_message2.proto\x12\x11benchmarks.proto2\"\x84\x0B\n\x0EGoogleMessage2\x12\x16\n\x06field1\x18\x01 \x01(\tR\x06field1\x12\x16\n\x06field3\x18\x03 \x01(\x03R\x06field3\x12\x16\n\x06field4\x18\x04 \x01(\x03R\x06field4\x12\x18\n\x07field30\x18\x1E \x01(\x03R\x07field30\x12\x1F\n\x07field75\x18K \x01(\x08:\x05falseR\x07field75\x12\x16\n\x06field6\x18\x06 \x01(\tR\x06field6\x12\x16\n\x06field2\x18\x02 \x01(\x0CR\x06field2\x12\x1B\n\x07field21\x18\x15 \x01(\x05:\x010R\x07field21\x12\x18\n\x07field71\x18G \x01(\x05R\x07field71\x12\x18\n\x07field25\x18\x19 \x01(\x02R\x07field25\x12\x1D\n\x08field109\x18m \x01(\x05:\x010R\x08field109\x12\x1E\n\x08field210\x18\xD2\x01 \x01(\x05:\x010R\x08field210\x12\x1E\n\x08field211\x18\xD3\x01 \x01(\x05:\x010R\x08field211\x12\x1E\n\x08field212\x18\xD4\x01 \x01(\x05:\x010R\x08field212\x12\x1E\n\x08field213\x18\xD5\x01 \x01(\x05:\x010R\x08field213\x12\x1E\n\x08field216\x18\xD8\x01 \x01(\x05:\x010R\x08field216\x12\x1E\n\x08field217\x18\xD9\x01 \x01(\x05:\x010R\x08field217\x12\x1E\n\x08field218\x18\xDA\x01 \x01(\x05:\x010R\x08field218\x12\x1E\n\x08field220\x18\xDC\x01 \x01(\x05:\x010R\x08field220\x12\x1E\n\x08field221\x18\xDD\x01 \x01(\x05:\x010R\x08field221\x12\x1E\n\x08field222\x18\xDE\x01 \x01(\x02:\x010R\x08field222\x12\x18\n\x07field63\x18? \x01(\x05R\x07field63\x12@\n\x06group1\x18\n \x03(\n2(.benchmarks.proto2.GoogleMessage2.Group1R\x06group1\x12\x1B\n\x08field128\x18\x80\x01 \x03(\tR\x08field128\x12\x1B\n\x08field131\x18\x83\x01 \x01(\x03R\x08field131\x12\x1A\n\x08field127\x18\x7F \x03(\tR\x08field127\x12\x1B\n\x08field129\x18\x81\x01 \x01(\x05R\x08field129\x12\x1B\n\x08field130\x18\x82\x01 \x03(\x03R\x08field130\x12\"\n\x08field205\x18\xCD\x01 \x01(\x08:\x05falseR\x08field205\x12\"\n\x08field206\x18\xCE\x01 \x01(\x08:\x05falseR\x08field206\x1A\xDA\x03\n\x06Group1\x12\x18\n\x07field11\x18\x0B \x02(\x02R\x07field11\x12\x18\n\x07field26\x18\x1A \x01(\x02R\x07field26\x12\x18\n\x07field12\x18\x0C \x01(\tR\x07field12\x12\x18\n\x07field13\x18\r \x01(\tR\x07field13\x12\x18\n\x07field14\x18\x0E \x03(\tR\x07field14\x12\x18\n\x07field15\x18\x0F \x02(\x04R\x07field15\x12\x16\n\x06field5\x18\x05 \x01(\x05R\x06field5\x12\x18\n\x07field27\x18\x1B \x01(\tR\x07field27\x12\x18\n\x07field28\x18\x1C \x01(\x05R\x07field28\x12\x18\n\x07field29\x18\x1D \x01(\tR\x07field29\x12\x18\n\x07field16\x18\x10 \x01(\tR\x07field16\x12\x18\n\x07field22\x18\x16 \x03(\tR\x07field22\x12\x18\n\x07field73\x18I \x03(\x05R\x07field73\x12\x1B\n\x07field20\x18\x14 \x01(\x05:\x010R\x07field20\x12\x18\n\x07field24\x18\x18 \x01(\tR\x07field24\x12I\n\x07field31\x18\x1F \x01(\x0B2/.benchmarks.proto2.GoogleMessage2GroupedMessageR\x07field31\"\xBA\x02\n\x1CGoogleMessage2GroupedMessage\x12\x16\n\x06field1\x18\x01 \x01(\x02R\x06field1\x12\x16\n\x06field2\x18\x02 \x01(\x02R\x06field2\x12\x19\n\x06field3\x18\x03 \x01(\x02:\x010R\x06field3\x12\x16\n\x06field4\x18\x04 \x01(\x08R\x06field4\x12\x16\n\x06field5\x18\x05 \x01(\x08R\x06field5\x12\x1C\n\x06field6\x18\x06 \x01(\x08:\x04trueR\x06field6\x12\x1D\n\x06field7\x18\x07 \x01(\x08:\x05falseR\x06field7\x12\x16\n\x06field8\x18\x08 \x01(\x02R\x06field8\x12\x16\n\x06field9\x18\t \x01(\x08R\x06field9\x12\x18\n\x07field10\x18\n \x01(\x02R\x07field10\x12\x18\n\x07field11\x18\x0B \x01(\x03R\x07field11B%\n\x1Ecom.google.protobuf.benchmarksH\x01\xF8\x01\x01J\x833\n\x06\x12\x04\"\0k\x01\n\x84\r\n\x01\x0C\x12\x03\"\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2\x14 LINT: ALLOW_GROUPS\n2  Benchmark messages for proto2.\n\n\x08\n\x01\x02\x12\x03$\0\x1A\n\x08\n\x01\x08\x12\x03%\07\n\t\n\x02\x08\x01\x12\x03%\07\n\x08\n\x01\x08\x12\x03(\0\x1C\nD\n\x02\x08\t\x12\x03(\0\x1C\x1A9 This is the default, but we specify it here explicitly.\n\n\x08\n\x01\x08\x12\x03*\0\x1F\n\t\n\x02\x08\x1F\x12\x03*\0\x1F\n\n\n\x02\x04\0\x12\x04,\0]\x01\n\n\n\x03\x04\0\x01\x12\x03,\x08\x16\n\x0B\n\x04\x04\0\x02\0\x12\x03-\x02\x1D\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03-\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03-\x0B\x11\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03-\x12\x18\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03-\x1B\x1C\n\x0B\n\x04\x04\0\x02\x01\x12\x03.\x02\x1C\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03.\x02\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03.\x0B\x10\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03.\x11\x17\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03.\x1A\x1B\n\x0B\n\x04\x04\0\x02\x02\x12\x03/\x02\x1C\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03/\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03/\x0B\x10\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03/\x11\x17\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03/\x1A\x1B\n\x0B\n\x04\x04\0\x02\x03\x12\x030\x02\x1E\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x030\x02\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x030\x0B\x10\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x030\x11\x18\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x030\x1B\x1D\n\x0B\n\x04\x04\0\x02\x04\x12\x031\x02/\n\x0C\n\x05\x04\0\x02\x04\x04\x12\x031\x02\n\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x031\x0B\x0F\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x031\x10\x17\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x031\x1A\x1C\n\x0C\n\x05\x04\0\x02\x04\x08\x12\x031\x1D.\n\x0C\n\x05\x04\0\x02\x04\x07\x12\x031(-\n\x0B\n\x04\x04\0\x02\x05\x12\x032\x02\x1D\n\x0C\n\x05\x04\0\x02\x05\x04\x12\x032\x02\n\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x032\x0B\x11\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x032\x12\x18\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x032\x1B\x1C\n\x0B\n\x04\x04\0\x02\x06\x12\x033\x02\x1C\n\x0C\n\x05\x04\0\x02\x06\x04\x12\x033\x02\n\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x033\x0B\x10\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x033\x11\x17\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x033\x1A\x1B\n\x0B\n\x04\x04\0\x02\x07\x12\x034\x02,\n\x0C\n\x05\x04\0\x02\x07\x04\x12\x034\x02\n\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x034\x0B\x10\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x034\x11\x18\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x034\x1B\x1D\n\x0C\n\x05\x04\0\x02\x07\x08\x12\x034\x1E+\n\x0C\n\x05\x04\0\x02\x07\x07\x12\x034)*\n\x0B\n\x04\x04\0\x02\x08\x12\x035\x02\x1E\n\x0C\n\x05\x04\0\x02\x08\x04\x12\x035\x02\n\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x035\x0B\x10\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x035\x11\x18\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x035\x1B\x1D\n\x0B\n\x04\x04\0\x02\t\x12\x036\x02\x1E\n\x0C\n\x05\x04\0\x02\t\x04\x12\x036\x02\n\n\x0C\n\x05\x04\0\x02\t\x05\x12\x036\x0B\x10\n\x0C\n\x05\x04\0\x02\t\x01\x12\x036\x11\x18\n\x0C\n\x05\x04\0\x02\t\x03\x12\x036\x1B\x1D\n\x0B\n\x04\x04\0\x02\n\x12\x037\x02.\n\x0C\n\x05\x04\0\x02\n\x04\x12\x037\x02\n\n\x0C\n\x05\x04\0\x02\n\x05\x12\x037\x0B\x10\n\x0C\n\x05\x04\0\x02\n\x01\x12\x037\x11\x19\n\x0C\n\x05\x04\0\x02\n\x03\x12\x037\x1C\x1F\n\x0C\n\x05\x04\0\x02\n\x08\x12\x037 -\n\x0C\n\x05\x04\0\x02\n\x07\x12\x037+,\n\x0B\n\x04\x04\0\x02\x0B\x12\x038\x02.\n\x0C\n\x05\x04\0\x02\x0B\x04\x12\x038\x02\n\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x038\x0B\x10\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x038\x11\x19\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x038\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0B\x08\x12\x038 -\n\x0C\n\x05\x04\0\x02\x0B\x07\x12\x038+,\n\x0B\n\x04\x04\0\x02\x0C\x12\x039\x02.\n\x0C\n\x05\x04\0\x02\x0C\x04\x12\x039\x02\n\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x039\x0B\x10\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x039\x11\x19\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x039\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0C\x08\x12\x039 -\n\x0C\n\x05\x04\0\x02\x0C\x07\x12\x039+,\n\x0B\n\x04\x04\0\x02\r\x12\x03:\x02.\n\x0C\n\x05\x04\0\x02\r\x04\x12\x03:\x02\n\n\x0C\n\x05\x04\0\x02\r\x05\x12\x03:\x0B\x10\n\x0C\n\x05\x04\0\x02\r\x01\x12\x03:\x11\x19\n\x0C\n\x05\x04\0\x02\r\x03\x12\x03:\x1C\x1F\n\x0C\n\x05\x04\0\x02\r\x08\x12\x03: -\n\x0C\n\x05\x04\0\x02\r\x07\x12\x03:+,\n\x0B\n\x04\x04\0\x02\x0E\x12\x03;\x02.\n\x0C\n\x05\x04\0\x02\x0E\x04\x12\x03;\x02\n\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x03;\x0B\x10\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x03;\x11\x19\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x03;\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0E\x08\x12\x03; -\n\x0C\n\x05\x04\0\x02\x0E\x07\x12\x03;+,\n\x0B\n\x04\x04\0\x02\x0F\x12\x03<\x02.\n\x0C\n\x05\x04\0\x02\x0F\x04\x12\x03<\x02\n\n\x0C\n\x05\x04\0\x02\x0F\x05\x12\x03<\x0B\x10\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03<\x11\x19\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03<\x1C\x1F\n\x0C\n\x05\x04\0\x02\x0F\x08\x12\x03< -\n\x0C\n\x05\x04\0\x02\x0F\x07\x12\x03<+,\n\x0B\n\x04\x04\0\x02\x10\x12\x03=\x02.\n\x0C\n\x05\x04\0\x02\x10\x04\x12\x03=\x02\n\n\x0C\n\x05\x04\0\x02\x10\x05\x12\x03=\x0B\x10\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03=\x11\x19\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03=\x1C\x1F\n\x0C\n\x05\x04\0\x02\x10\x08\x12\x03= -\n\x0C\n\x05\x04\0\x02\x10\x07\x12\x03=+,\n\x0B\n\x04\x04\0\x02\x11\x12\x03>\x02.\n\x0C\n\x05\x04\0\x02\x11\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\0\x02\x11\x05\x12\x03>\x0B\x10\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03>\x11\x19\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03>\x1C\x1F\n\x0C\n\x05\x04\0\x02\x11\x08\x12\x03> -\n\x0C\n\x05\x04\0\x02\x11\x07\x12\x03>+,\n\x0B\n\x04\x04\0\x02\x12\x12\x03?\x02.\n\x0C\n\x05\x04\0\x02\x12\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\0\x02\x12\x05\x12\x03?\x0B\x10\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03?\x11\x19\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03?\x1C\x1F\n\x0C\n\x05\x04\0\x02\x12\x08\x12\x03? -\n\x0C\n\x05\x04\0\x02\x12\x07\x12\x03?+,\n\x0B\n\x04\x04\0\x02\x13\x12\x03@\x02.\n\x0C\n\x05\x04\0\x02\x13\x04\x12\x03@\x02\n\n\x0C\n\x05\x04\0\x02\x13\x05\x12\x03@\x0B\x10\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03@\x11\x19\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03@\x1C\x1F\n\x0C\n\x05\x04\0\x02\x13\x08\x12\x03@ -\n\x0C\n\x05\x04\0\x02\x13\x07\x12\x03@+,\n\x0B\n\x04\x04\0\x02\x14\x12\x03A\x020\n\x0C\n\x05\x04\0\x02\x14\x04\x12\x03A\x02\n\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03A\x0B\x10\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03A\x11\x19\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03A\x1C\x1F\n\x0C\n\x05\x04\0\x02\x14\x08\x12\x03A /\n\x0C\n\x05\x04\0\x02\x14\x07\x12\x03A+.\n\x0B\n\x04\x04\0\x02\x15\x12\x03B\x02\x1E\n\x0C\n\x05\x04\0\x02\x15\x04\x12\x03B\x02\n\n\x0C\n\x05\x04\0\x02\x15\x05\x12\x03B\x0B\x10\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03B\x11\x18\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03B\x1B\x1D\n\x0C\n\x04\x04\0\x02\x16\x12\x04D\x02U\x03\n\x0C\n\x05\x04\0\x02\x16\x04\x12\x03D\x02\n\n\x0C\n\x05\x04\0\x02\x16\x05\x12\x03D\x0B\x10\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03D\x11\x17\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03D\x1A\x1C\n\x0C\n\x04\x04\0\x03\0\x12\x04D\x02U\x03\n\x0C\n\x05\x04\0\x03\0\x01\x12\x03D\x11\x17\n\x0C\n\x05\x04\0\x02\x16\x06\x12\x03D\x11\x17\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03E\x04 \n\x0E\n\x07\x04\0\x03\0\x02\0\x04\x12\x03E\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\0\x05\x12\x03E\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\0\x01\x12\x03E\x13\x1A\n\x0E\n\x07\x04\0\x03\0\x02\0\x03\x12\x03E\x1D\x1F\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03F\x04 \n\x0E\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03F\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x01\x05\x12\x03F\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03F\x13\x1A\n\x0E\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03F\x1D\x1F\n\r\n\x06\x04\0\x03\0\x02\x02\x12\x03G\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x02\x04\x12\x03G\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x02\x05\x12\x03G\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x02\x01\x12\x03G\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x02\x03\x12\x03G\x1E \n\r\n\x06\x04\0\x03\0\x02\x03\x12\x03H\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x03\x04\x12\x03H\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x03\x05\x12\x03H\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x03\x01\x12\x03H\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x03\x03\x12\x03H\x1E \n\r\n\x06\x04\0\x03\0\x02\x04\x12\x03I\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x04\x04\x12\x03I\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x04\x05\x12\x03I\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x04\x01\x12\x03I\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x04\x03\x12\x03I\x1E \n\r\n\x06\x04\0\x03\0\x02\x05\x12\x03J\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x05\x04\x12\x03J\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x05\x05\x12\x03J\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x05\x01\x12\x03J\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x05\x03\x12\x03J\x1E \n\r\n\x06\x04\0\x03\0\x02\x06\x12\x03K\x04\x1E\n\x0E\n\x07\x04\0\x03\0\x02\x06\x04\x12\x03K\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x06\x05\x12\x03K\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\x06\x01\x12\x03K\x13\x19\n\x0E\n\x07\x04\0\x03\0\x02\x06\x03\x12\x03K\x1C\x1D\n\r\n\x06\x04\0\x03\0\x02\x07\x12\x03L\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x07\x04\x12\x03L\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x07\x05\x12\x03L\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x07\x01\x12\x03L\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x07\x03\x12\x03L\x1E \n\r\n\x06\x04\0\x03\0\x02\x08\x12\x03M\x04 \n\x0E\n\x07\x04\0\x03\0\x02\x08\x04\x12\x03M\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x08\x05\x12\x03M\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\x08\x01\x12\x03M\x13\x1A\n\x0E\n\x07\x04\0\x03\0\x02\x08\x03\x12\x03M\x1D\x1F\n\r\n\x06\x04\0\x03\0\x02\t\x12\x03N\x04!\n\x0E\n\x07\x04\0\x03\0\x02\t\x04\x12\x03N\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\t\x05\x12\x03N\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\t\x01\x12\x03N\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\t\x03\x12\x03N\x1E \n\r\n\x06\x04\0\x03\0\x02\n\x12\x03O\x04!\n\x0E\n\x07\x04\0\x03\0\x02\n\x04\x12\x03O\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\n\x05\x12\x03O\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\n\x01\x12\x03O\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\n\x03\x12\x03O\x1E \n\r\n\x06\x04\0\x03\0\x02\x0B\x12\x03P\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x0B\x04\x12\x03P\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x0B\x05\x12\x03P\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x0B\x01\x12\x03P\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x0B\x03\x12\x03P\x1E \n\r\n\x06\x04\0\x03\0\x02\x0C\x12\x03Q\x04 \n\x0E\n\x07\x04\0\x03\0\x02\x0C\x04\x12\x03Q\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x0C\x05\x12\x03Q\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\x0C\x01\x12\x03Q\x13\x1A\n\x0E\n\x07\x04\0\x03\0\x02\x0C\x03\x12\x03Q\x1D\x1F\n\r\n\x06\x04\0\x03\0\x02\r\x12\x03R\x04.\n\x0E\n\x07\x04\0\x03\0\x02\r\x04\x12\x03R\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\r\x05\x12\x03R\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\r\x01\x12\x03R\x13\x1A\n\x0E\n\x07\x04\0\x03\0\x02\r\x03\x12\x03R\x1D\x1F\n\x0E\n\x07\x04\0\x03\0\x02\r\x08\x12\x03R -\n\x0E\n\x07\x04\0\x03\0\x02\r\x07\x12\x03R+,\n\r\n\x06\x04\0\x03\0\x02\x0E\x12\x03S\x04!\n\x0E\n\x07\x04\0\x03\0\x02\x0E\x04\x12\x03S\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x0E\x05\x12\x03S\r\x13\n\x0E\n\x07\x04\0\x03\0\x02\x0E\x01\x12\x03S\x14\x1B\n\x0E\n\x07\x04\0\x03\0\x02\x0E\x03\x12\x03S\x1E \n\r\n\x06\x04\0\x03\0\x02\x0F\x12\x03T\x047\n\x0E\n\x07\x04\0\x03\0\x02\x0F\x04\x12\x03T\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x0F\x06\x12\x03T\r)\n\x0E\n\x07\x04\0\x03\0\x02\x0F\x01\x12\x03T*1\n\x0E\n\x07\x04\0\x03\0\x02\x0F\x03\x12\x03T46\n\x0B\n\x04\x04\0\x02\x17\x12\x03V\x02!\n\x0C\n\x05\x04\0\x02\x17\x04\x12\x03V\x02\n\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03V\x0B\x11\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03V\x12\x1A\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03V\x1D \n\x0B\n\x04\x04\0\x02\x18\x12\x03W\x02 \n\x0C\n\x05\x04\0\x02\x18\x04\x12\x03W\x02\n\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03W\x0B\x10\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03W\x11\x19\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03W\x1C\x1F\n\x0B\n\x04\x04\0\x02\x19\x12\x03X\x02!\n\x0C\n\x05\x04\0\x02\x19\x04\x12\x03X\x02\n\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03X\x0B\x11\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03X\x12\x1A\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03X\x1D \n\x0B\n\x04\x04\0\x02\x1A\x12\x03Y\x02 \n\x0C\n\x05\x04\0\x02\x1A\x04\x12\x03Y\x02\n\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03Y\x0B\x10\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03Y\x11\x19\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03Y\x1C\x1F\n\x0B\n\x04\x04\0\x02\x1B\x12\x03Z\x02 \n\x0C\n\x05\x04\0\x02\x1B\x04\x12\x03Z\x02\n\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03Z\x0B\x10\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03Z\x11\x19\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03Z\x1C\x1F\n\x0B\n\x04\x04\0\x02\x1C\x12\x03[\x021\n\x0C\n\x05\x04\0\x02\x1C\x04\x12\x03[\x02\n\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03[\x0B\x0F\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03[\x10\x18\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03[\x1B\x1E\n\x0C\n\x05\x04\0\x02\x1C\x08\x12\x03[\x1F0\n\x0C\n\x05\x04\0\x02\x1C\x07\x12\x03[*/\n\x0B\n\x04\x04\0\x02\x1D\x12\x03\\\x021\n\x0C\n\x05\x04\0\x02\x1D\x04\x12\x03\\\x02\n\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03\\\x0B\x0F\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03\\\x10\x18\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03\\\x1B\x1E\n\x0C\n\x05\x04\0\x02\x1D\x08\x12\x03\\\x1F0\n\x0C\n\x05\x04\0\x02\x1D\x07\x12\x03\\*/\n\n\n\x02\x04\x01\x12\x04_\0k\x01\n\n\n\x03\x04\x01\x01\x12\x03_\x08$\n\x0B\n\x04\x04\x01\x02\0\x12\x03`\x02\x1C\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x03`\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03`\x0B\x10\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03`\x11\x17\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03`\x1A\x1B\n\x0B\n\x04\x04\x01\x02\x01\x12\x03a\x02\x1C\n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x03a\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03a\x0B\x10\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03a\x11\x17\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03a\x1A\x1B\n\x0B\n\x04\x04\x01\x02\x02\x12\x03b\x02,\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x03b\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03b\x0B\x10\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03b\x11\x17\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03b\x1A\x1B\n\x0C\n\x05\x04\x01\x02\x02\x08\x12\x03b\x1C+\n\x0C\n\x05\x04\x01\x02\x02\x07\x12\x03b'*\n\x0B\n\x04\x04\x01\x02\x03\x12\x03c\x02\x1B\n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x03c\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03c\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03c\x10\x16\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03c\x19\x1A\n\x0B\n\x04\x04\x01\x02\x04\x12\x03d\x02\x1B\n\x0C\n\x05\x04\x01\x02\x04\x04\x12\x03d\x02\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03d\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03d\x10\x16\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03d\x19\x1A\n\x0B\n\x04\x04\x01\x02\x05\x12\x03e\x02,\n\x0C\n\x05\x04\x01\x02\x05\x04\x12\x03e\x02\n\n\x0C\n\x05\x04\x01\x02\x05\x05\x12\x03e\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03e\x10\x16\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03e\x19\x1A\n\x0C\n\x05\x04\x01\x02\x05\x08\x12\x03e\x1B+\n\x0C\n\x05\x04\x01\x02\x05\x07\x12\x03e&*\n\x0B\n\x04\x04\x01\x02\x06\x12\x03f\x02-\n\x0C\n\x05\x04\x01\x02\x06\x04\x12\x03f\x02\n\n\x0C\n\x05\x04\x01\x02\x06\x05\x12\x03f\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03f\x10\x16\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03f\x19\x1A\n\x0C\n\x05\x04\x01\x02\x06\x08\x12\x03f\x1B,\n\x0C\n\x05\x04\x01\x02\x06\x07\x12\x03f&+\n\x0B\n\x04\x04\x01\x02\x07\x12\x03g\x02\x1C\n\x0C\n\x05\x04\x01\x02\x07\x04\x12\x03g\x02\n\n\x0C\n\x05\x04\x01\x02\x07\x05\x12\x03g\x0B\x10\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03g\x11\x17\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03g\x1A\x1B\n\x0B\n\x04\x04\x01\x02\x08\x12\x03h\x02\x1B\n\x0C\n\x05\x04\x01\x02\x08\x04\x12\x03h\x02\n\n\x0C\n\x05\x04\x01\x02\x08\x05\x12\x03h\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03h\x10\x16\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03h\x19\x1A\n\x0B\n\x04\x04\x01\x02\t\x12\x03i\x02\x1E\n\x0C\n\x05\x04\x01\x02\t\x04\x12\x03i\x02\n\n\x0C\n\x05\x04\x01\x02\t\x05\x12\x03i\x0B\x10\n\x0C\n\x05\x04\x01\x02\t\x01\x12\x03i\x11\x18\n\x0C\n\x05\x04\x01\x02\t\x03\x12\x03i\x1B\x1D\n\x0B\n\x04\x04\x01\x02\n\x12\x03j\x02\x1E\n\x0C\n\x05\x04\x01\x02\n\x04\x12\x03j\x02\n\n\x0C\n\x05\x04\x01\x02\n\x05\x12\x03j\x0B\x10\n\x0C\n\x05\x04\x01\x02\n\x01\x12\x03j\x11\x18\n\x0C\n\x05\x04\x01\x02\n\x03\x12\x03j\x1B\x1D" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
