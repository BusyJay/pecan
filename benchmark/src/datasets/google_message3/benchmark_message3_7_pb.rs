#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Message11018 {
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message11018 {
    pub const fn new() -> Message11018 {
        Message11018 {
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Message11018 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
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
impl pecan::DefaultInstance for Message11018 {
    fn default_instance() -> &'static Message11018 {
        static DEFAULT: Message11018 = Message11018::new();
        &DEFAULT
    }
}
impl Default for Message11018 {
    #[inline]
    fn default() -> Message11018 {
        Message11018::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message10800 {
    pub field10808: Option<String>,
    pub field10809: Option<i64>,
    pub field10810: Option<bool>,
    pub field10811: Option<f32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message10800 {
    pub const fn new() -> Message10800 {
        Message10800 {
            field10808: None,
            field10809: None,
            field10810: None,
            field10811: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field10808(&self) -> &String {
        match &self.field10808 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field10808_mut(&mut self) -> &mut String {
        self.field10808.get_or_insert_with(Default::default)
    }
    pub fn set_field10808(&mut self, val: String) {
        self.field10808 = Some(val);
    }
    pub fn field10809(&self) -> i64 {
        self.field10809.unwrap_or_default()
    }
    pub fn field10809_mut(&mut self) -> &mut i64 {
        self.field10809.get_or_insert_with(Default::default)
    }
    pub fn set_field10809(&mut self, val: i64) {
        self.field10809 = Some(val);
    }
    pub fn field10810(&self) -> bool {
        self.field10810.unwrap_or_default()
    }
    pub fn field10810_mut(&mut self) -> &mut bool {
        self.field10810.get_or_insert_with(Default::default)
    }
    pub fn set_field10810(&mut self, val: bool) {
        self.field10810 = Some(val);
    }
    pub fn field10811(&self) -> f32 {
        self.field10811.unwrap_or_default()
    }
    pub fn field10811_mut(&mut self) -> &mut f32 {
        self.field10811.get_or_insert_with(Default::default)
    }
    pub fn set_field10811(&mut self, val: f32) {
        self.field10811 = Some(val);
    }
}
impl pecan::Message for Message10800 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field10808 = Some(LengthPrefixed::read_from(s)?),
                16 => self.field10809 = Some(Varint::read_from(s)?),
                24 => self.field10810 = Some(Varint::read_from(s)?),
                37 => self.field10811 = Some(Fixed32::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.field10808 {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field10809 {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field10810 {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field10811 {
            s.write_tag(37)?;
            Fixed32::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.field10808 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field10809 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field10810 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field10811 {
            l += 1 + Fixed32::size(v);
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
impl pecan::DefaultInstance for Message10800 {
    fn default_instance() -> &'static Message10800 {
        static DEFAULT: Message10800 = Message10800::new();
        &DEFAULT
    }
}
impl Default for Message10800 {
    #[inline]
    fn default() -> Message10800 {
        Message10800::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message10802 {
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message10802 {
    pub const fn new() -> Message10802 {
        Message10802 {
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Message10802 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
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
impl pecan::DefaultInstance for Message10802 {
    fn default_instance() -> &'static Message10802 {
        static DEFAULT: Message10802 = Message10802::new();
        &DEFAULT
    }
}
impl Default for Message10802 {
    #[inline]
    fn default() -> Message10802 {
        Message10802::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message10748 {
    pub field10750: Option<String>,
    pub field10751: Option<i32>,
    pub field10752: Option<i32>,
    pub field10753: Option<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message10748 {
    pub const fn new() -> Message10748 {
        Message10748 {
            field10750: None,
            field10751: None,
            field10752: None,
            field10753: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field10750(&self) -> &String {
        match &self.field10750 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field10750_mut(&mut self) -> &mut String {
        self.field10750.get_or_insert_with(Default::default)
    }
    pub fn set_field10750(&mut self, val: String) {
        self.field10750 = Some(val);
    }
    pub fn field10751(&self) -> i32 {
        self.field10751.unwrap_or_default()
    }
    pub fn field10751_mut(&mut self) -> &mut i32 {
        self.field10751.get_or_insert_with(Default::default)
    }
    pub fn set_field10751(&mut self, val: i32) {
        self.field10751 = Some(val);
    }
    pub fn field10752(&self) -> i32 {
        self.field10752.unwrap_or_default()
    }
    pub fn field10752_mut(&mut self) -> &mut i32 {
        self.field10752.get_or_insert_with(Default::default)
    }
    pub fn set_field10752(&mut self, val: i32) {
        self.field10752 = Some(val);
    }
    pub fn field10753(&self) -> i32 {
        self.field10753.unwrap_or_default()
    }
    pub fn field10753_mut(&mut self) -> &mut i32 {
        self.field10753.get_or_insert_with(Default::default)
    }
    pub fn set_field10753(&mut self, val: i32) {
        self.field10753 = Some(val);
    }
}
impl pecan::Message for Message10748 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field10750 = Some(LengthPrefixed::read_from(s)?),
                16 => self.field10751 = Some(Varint::read_from(s)?),
                24 => self.field10752 = Some(Varint::read_from(s)?),
                32 => self.field10753 = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.field10750 {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field10751 {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field10752 {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field10753 {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.field10750 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field10751 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field10752 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.field10753 {
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
impl pecan::DefaultInstance for Message10748 {
    fn default_instance() -> &'static Message10748 {
        static DEFAULT: Message10748 = Message10748::new();
        &DEFAULT
    }
}
impl Default for Message10748 {
    #[inline]
    fn default() -> Message10748 {
        Message10748::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message7966 {
    pub field7969: Option<String>,
    pub field7970: Option<bool>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message7966 {
    pub const fn new() -> Message7966 {
        Message7966 {
            field7969: None,
            field7970: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field7969(&self) -> &String {
        match &self.field7969 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field7969_mut(&mut self) -> &mut String {
        self.field7969.get_or_insert_with(Default::default)
    }
    pub fn set_field7969(&mut self, val: String) {
        self.field7969 = Some(val);
    }
    pub fn field7970(&self) -> bool {
        self.field7970.unwrap_or_default()
    }
    pub fn field7970_mut(&mut self) -> &mut bool {
        self.field7970.get_or_insert_with(Default::default)
    }
    pub fn set_field7970(&mut self, val: bool) {
        self.field7970 = Some(val);
    }
}
impl pecan::Message for Message7966 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field7969 = Some(LengthPrefixed::read_from(s)?),
                16 => self.field7970 = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.field7969 {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.field7970 {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.field7969 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.field7970 {
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
impl pecan::DefaultInstance for Message7966 {
    fn default_instance() -> &'static Message7966 {
        static DEFAULT: Message7966 = Message7966::new();
        &DEFAULT
    }
}
impl Default for Message7966 {
    #[inline]
    fn default() -> Message7966 {
        Message7966::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message708 {
    pub field823: Option<Message741>,
    pub field824: Vec<String>,
    pub field825: Option<String>,
    pub field826: Option<String>,
    pub field827: Vec<String>,
    pub field828: Vec<String>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message708 {
    pub const fn new() -> Message708 {
        Message708 {
            field823: None,
            field824: Vec::new(),
            field825: None,
            field826: None,
            field827: Vec::new(),
            field828: Vec::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn field823(&self) -> &Message741 {
        match &self.field823 {
            Some(v) => v,
            _ => Message741::default_instance(),
        }
    }
    pub fn field823_mut(&mut self) -> &mut Message741 {
        self.field823.get_or_insert_with(Default::default)
    }
    pub fn set_field823(&mut self, val: Message741) {
        self.field823 = Some(val);
    }
    pub fn field825(&self) -> &String {
        match &self.field825 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field825_mut(&mut self) -> &mut String {
        self.field825.get_or_insert_with(Default::default)
    }
    pub fn set_field825(&mut self, val: String) {
        self.field825 = Some(val);
    }
    pub fn field826(&self) -> &String {
        match &self.field826 {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn field826_mut(&mut self) -> &mut String {
        self.field826.get_or_insert_with(Default::default)
    }
    pub fn set_field826(&mut self, val: String) {
        self.field826 = Some(val);
    }
}
impl pecan::Message for Message708 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixed::merge_from(self.field823_mut(), s)?,
                18 => self.field825 = Some(LengthPrefixed::read_from(s)?),
                26 => self.field826 = Some(LengthPrefixed::read_from(s)?),
                34 => RefArray::<LengthPrefixed>::merge_from(&mut self.field827, s)?,
                42 => RefArray::<LengthPrefixed>::merge_from(&mut self.field828, s)?,
                50 => RefArray::<LengthPrefixed>::merge_from(&mut self.field824, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.field823 {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field825 {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.field826 {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.field827.is_empty() {
            for i in &self.field827 {
                s.write_tag(34)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.field828.is_empty() {
            for i in &self.field828 {
                s.write_tag(42)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.field824.is_empty() {
            for i in &self.field824 {
                s.write_tag(50)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.field823 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field825 {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.field826 {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.field827.is_empty() {
            l += self.field827.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field827);
        }
        if !self.field828.is_empty() {
            l += self.field828.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field828);
        }
        if !self.field824.is_empty() {
            l += self.field824.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field824);
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
impl pecan::DefaultInstance for Message708 {
    fn default_instance() -> &'static Message708 {
        static DEFAULT: Message708 = Message708::new();
        &DEFAULT
    }
}
impl Default for Message708 {
    #[inline]
    fn default() -> Message708 {
        Message708::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message8942 {
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message8942 {
    pub const fn new() -> Message8942 {
        Message8942 {
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Message8942 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
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
impl pecan::DefaultInstance for Message8942 {
    fn default_instance() -> &'static Message8942 {
        static DEFAULT: Message8942 = Message8942::new();
        &DEFAULT
    }
}
impl Default for Message8942 {
    #[inline]
    fn default() -> Message8942 {
        Message8942::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message11011 {
    pub field11752: pecan::Bytes,
    pub field11753: pecan::Bytes,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message11011 {
    pub const fn new() -> Message11011 {
        Message11011 {
            field11752: pecan::Bytes::new(),
            field11753: pecan::Bytes::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Message11011 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.field11752 = LengthPrefixed::read_from(s)?,
                18 => self.field11753 = LengthPrefixed::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.field11752.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.field11752, s)?;
        }
        if !self.field11753.is_empty() {
            s.write_tag(18)?;
            LengthPrefixed::write_to(&self.field11753, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.field11752.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field11752);
        }
        if !self.field11753.is_empty() {
            l += 1 + LengthPrefixed::size(&self.field11753);
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
impl pecan::DefaultInstance for Message11011 {
    fn default_instance() -> &'static Message11011 {
        static DEFAULT: Message11011 = Message11011::new();
        &DEFAULT
    }
}
impl Default for Message11011 {
    #[inline]
    fn default() -> Message11011 {
        Message11011::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnusedEmptyMessage {
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl UnusedEmptyMessage {
    pub const fn new() -> UnusedEmptyMessage {
        UnusedEmptyMessage {
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for UnusedEmptyMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
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
impl pecan::DefaultInstance for UnusedEmptyMessage {
    fn default_instance() -> &'static UnusedEmptyMessage {
        static DEFAULT: UnusedEmptyMessage = UnusedEmptyMessage::new();
        &DEFAULT
    }
}
impl Default for UnusedEmptyMessage {
    #[inline]
    fn default() -> UnusedEmptyMessage {
        UnusedEmptyMessage::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Message741 {
    pub field936: Vec<String>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl Message741 {
    pub const fn new() -> Message741 {
        Message741 {
            field936: Vec::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for Message741 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => RefArray::<LengthPrefixed>::merge_from(&mut self.field936, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.field936.is_empty() {
            for i in &self.field936 {
                s.write_tag(10)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.field936.is_empty() {
            l += self.field936.len() as u64 + RefArray::<LengthPrefixed>::size(&self.field936);
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
impl pecan::DefaultInstance for Message741 {
    fn default_instance() -> &'static Message741 {
        static DEFAULT: Message741 = Message741::new();
        &DEFAULT
    }
}
impl Default for Message741 {
    #[inline]
    fn default() -> Message741 {
        Message741::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n3datasets/google_message3/benchmark_message3_7.proto\x12\x1Abenchmarks.google_message3\"\x0E\n\x0CMessage11018\"\x8E\x01\n\x0CMessage10800\x12\x1E\n\nfield10808\x18\x01 \x01(\tR\nfield10808\x12\x1E\n\nfield10809\x18\x02 \x01(\x03R\nfield10809\x12\x1E\n\nfield10810\x18\x03 \x01(\x08R\nfield10810\x12\x1E\n\nfield10811\x18\x04 \x01(\x02R\nfield10811\"\x0E\n\x0CMessage10802\"\x8E\x01\n\x0CMessage10748\x12\x1E\n\nfield10750\x18\x01 \x01(\tR\nfield10750\x12\x1E\n\nfield10751\x18\x02 \x01(\x05R\nfield10751\x12\x1E\n\nfield10752\x18\x03 \x01(\x05R\nfield10752\x12\x1E\n\nfield10753\x18\x04 \x01(\x05R\nfield10753\"I\n\x0BMessage7966\x12\x1C\n\tfield7969\x18\x01 \x01(\tR\tfield7969\x12\x1C\n\tfield7970\x18\x02 \x01(\x08R\tfield7970\"\xDC\x01\n\nMessage708\x12B\n\x08field823\x18\x01 \x01(\x0B2&.benchmarks.google_message3.Message741R\x08field823\x12\x1A\n\x08field824\x18\x06 \x03(\tR\x08field824\x12\x1A\n\x08field825\x18\x02 \x01(\tR\x08field825\x12\x1A\n\x08field826\x18\x03 \x01(\tR\x08field826\x12\x1A\n\x08field827\x18\x04 \x03(\tR\x08field827\x12\x1A\n\x08field828\x18\x05 \x03(\tR\x08field828\"\r\n\x0BMessage8942\"N\n\x0CMessage11011\x12\x1E\n\nfield11752\x18\x01 \x02(\x0CR\nfield11752\x12\x1E\n\nfield11753\x18\x02 \x02(\x0CR\nfield11753\"\x14\n\x12UnusedEmptyMessage\"(\n\nMessage741\x12\x1A\n\x08field936\x18\x01 \x03(\tR\x08field936B#\n\x1Ecom.google.protobuf.benchmarks\xF8\x01\x01J\x96\x19\n\x06\x12\x04\x1E\0P\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03 \0#\n\x08\n\x01\x08\x12\x03\"\0\x1F\n\t\n\x02\x08\x1F\x12\x03\"\0\x1F\n\x08\n\x01\x08\x12\x03#\07\n\t\n\x02\x08\x01\x12\x03#\07\n\t\n\x02\x04\0\x12\x03%\0\x17\n\n\n\x03\x04\0\x01\x12\x03%\x08\x14\n\n\n\x02\x04\x01\x12\x04'\0,\x01\n\n\n\x03\x04\x01\x01\x12\x03'\x08\x14\n\x0B\n\x04\x04\x01\x02\0\x12\x03(\x02!\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x03(\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03(\x0B\x11\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03(\x12\x1C\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03(\x1F \n\x0B\n\x04\x04\x01\x02\x01\x12\x03)\x02 \n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x03)\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03)\x0B\x10\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03)\x11\x1B\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03)\x1E\x1F\n\x0B\n\x04\x04\x01\x02\x02\x12\x03*\x02\x1F\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x03*\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03*\x0B\x0F\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03*\x10\x1A\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03*\x1D\x1E\n\x0B\n\x04\x04\x01\x02\x03\x12\x03+\x02 \n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x03+\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03+\x0B\x10\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03+\x11\x1B\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03+\x1E\x1F\n\t\n\x02\x04\x02\x12\x03.\0\x17\n\n\n\x03\x04\x02\x01\x12\x03.\x08\x14\n\n\n\x02\x04\x03\x12\x040\05\x01\n\n\n\x03\x04\x03\x01\x12\x030\x08\x14\n\x0B\n\x04\x04\x03\x02\0\x12\x031\x02!\n\x0C\n\x05\x04\x03\x02\0\x04\x12\x031\x02\n\n\x0C\n\x05\x04\x03\x02\0\x05\x12\x031\x0B\x11\n\x0C\n\x05\x04\x03\x02\0\x01\x12\x031\x12\x1C\n\x0C\n\x05\x04\x03\x02\0\x03\x12\x031\x1F \n\x0B\n\x04\x04\x03\x02\x01\x12\x032\x02 \n\x0C\n\x05\x04\x03\x02\x01\x04\x12\x032\x02\n\n\x0C\n\x05\x04\x03\x02\x01\x05\x12\x032\x0B\x10\n\x0C\n\x05\x04\x03\x02\x01\x01\x12\x032\x11\x1B\n\x0C\n\x05\x04\x03\x02\x01\x03\x12\x032\x1E\x1F\n\x0B\n\x04\x04\x03\x02\x02\x12\x033\x02 \n\x0C\n\x05\x04\x03\x02\x02\x04\x12\x033\x02\n\n\x0C\n\x05\x04\x03\x02\x02\x05\x12\x033\x0B\x10\n\x0C\n\x05\x04\x03\x02\x02\x01\x12\x033\x11\x1B\n\x0C\n\x05\x04\x03\x02\x02\x03\x12\x033\x1E\x1F\n\x0B\n\x04\x04\x03\x02\x03\x12\x034\x02 \n\x0C\n\x05\x04\x03\x02\x03\x04\x12\x034\x02\n\n\x0C\n\x05\x04\x03\x02\x03\x05\x12\x034\x0B\x10\n\x0C\n\x05\x04\x03\x02\x03\x01\x12\x034\x11\x1B\n\x0C\n\x05\x04\x03\x02\x03\x03\x12\x034\x1E\x1F\n\n\n\x02\x04\x04\x12\x047\0:\x01\n\n\n\x03\x04\x04\x01\x12\x037\x08\x13\n\x0B\n\x04\x04\x04\x02\0\x12\x038\x02 \n\x0C\n\x05\x04\x04\x02\0\x04\x12\x038\x02\n\n\x0C\n\x05\x04\x04\x02\0\x05\x12\x038\x0B\x11\n\x0C\n\x05\x04\x04\x02\0\x01\x12\x038\x12\x1B\n\x0C\n\x05\x04\x04\x02\0\x03\x12\x038\x1E\x1F\n\x0B\n\x04\x04\x04\x02\x01\x12\x039\x02\x1E\n\x0C\n\x05\x04\x04\x02\x01\x04\x12\x039\x02\n\n\x0C\n\x05\x04\x04\x02\x01\x05\x12\x039\x0B\x0F\n\x0C\n\x05\x04\x04\x02\x01\x01\x12\x039\x10\x19\n\x0C\n\x05\x04\x04\x02\x01\x03\x12\x039\x1C\x1D\n\n\n\x02\x04\x05\x12\x04<\0C\x01\n\n\n\x03\x04\x05\x01\x12\x03<\x08\x12\n\x0B\n\x04\x04\x05\x02\0\x12\x03=\x02?\n\x0C\n\x05\x04\x05\x02\0\x04\x12\x03=\x02\n\n\x0C\n\x05\x04\x05\x02\0\x06\x12\x03=\x0B1\n\x0C\n\x05\x04\x05\x02\0\x01\x12\x03=2:\n\x0C\n\x05\x04\x05\x02\0\x03\x12\x03==>\n\x0B\n\x04\x04\x05\x02\x01\x12\x03>\x02\x1F\n\x0C\n\x05\x04\x05\x02\x01\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\x05\x02\x01\x05\x12\x03>\x0B\x11\n\x0C\n\x05\x04\x05\x02\x01\x01\x12\x03>\x12\x1A\n\x0C\n\x05\x04\x05\x02\x01\x03\x12\x03>\x1D\x1E\n\x0B\n\x04\x04\x05\x02\x02\x12\x03?\x02\x1F\n\x0C\n\x05\x04\x05\x02\x02\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\x05\x02\x02\x05\x12\x03?\x0B\x11\n\x0C\n\x05\x04\x05\x02\x02\x01\x12\x03?\x12\x1A\n\x0C\n\x05\x04\x05\x02\x02\x03\x12\x03?\x1D\x1E\n\x0B\n\x04\x04\x05\x02\x03\x12\x03@\x02\x1F\n\x0C\n\x05\x04\x05\x02\x03\x04\x12\x03@\x02\n\n\x0C\n\x05\x04\x05\x02\x03\x05\x12\x03@\x0B\x11\n\x0C\n\x05\x04\x05\x02\x03\x01\x12\x03@\x12\x1A\n\x0C\n\x05\x04\x05\x02\x03\x03\x12\x03@\x1D\x1E\n\x0B\n\x04\x04\x05\x02\x04\x12\x03A\x02\x1F\n\x0C\n\x05\x04\x05\x02\x04\x04\x12\x03A\x02\n\n\x0C\n\x05\x04\x05\x02\x04\x05\x12\x03A\x0B\x11\n\x0C\n\x05\x04\x05\x02\x04\x01\x12\x03A\x12\x1A\n\x0C\n\x05\x04\x05\x02\x04\x03\x12\x03A\x1D\x1E\n\x0B\n\x04\x04\x05\x02\x05\x12\x03B\x02\x1F\n\x0C\n\x05\x04\x05\x02\x05\x04\x12\x03B\x02\n\n\x0C\n\x05\x04\x05\x02\x05\x05\x12\x03B\x0B\x11\n\x0C\n\x05\x04\x05\x02\x05\x01\x12\x03B\x12\x1A\n\x0C\n\x05\x04\x05\x02\x05\x03\x12\x03B\x1D\x1E\n\t\n\x02\x04\x06\x12\x03E\0\x16\n\n\n\x03\x04\x06\x01\x12\x03E\x08\x13\n\n\n\x02\x04\x07\x12\x04G\0J\x01\n\n\n\x03\x04\x07\x01\x12\x03G\x08\x14\n\x0B\n\x04\x04\x07\x02\0\x12\x03H\x02 \n\x0C\n\x05\x04\x07\x02\0\x04\x12\x03H\x02\n\n\x0C\n\x05\x04\x07\x02\0\x05\x12\x03H\x0B\x10\n\x0C\n\x05\x04\x07\x02\0\x01\x12\x03H\x11\x1B\n\x0C\n\x05\x04\x07\x02\0\x03\x12\x03H\x1E\x1F\n\x0B\n\x04\x04\x07\x02\x01\x12\x03I\x02 \n\x0C\n\x05\x04\x07\x02\x01\x04\x12\x03I\x02\n\n\x0C\n\x05\x04\x07\x02\x01\x05\x12\x03I\x0B\x10\n\x0C\n\x05\x04\x07\x02\x01\x01\x12\x03I\x11\x1B\n\x0C\n\x05\x04\x07\x02\x01\x03\x12\x03I\x1E\x1F\n\t\n\x02\x04\x08\x12\x03L\0\x1D\n\n\n\x03\x04\x08\x01\x12\x03L\x08\x1A\n\n\n\x02\x04\t\x12\x04N\0P\x01\n\n\n\x03\x04\t\x01\x12\x03N\x08\x12\n\x0B\n\x04\x04\t\x02\0\x12\x03O\x02\x1F\n\x0C\n\x05\x04\t\x02\0\x04\x12\x03O\x02\n\n\x0C\n\x05\x04\t\x02\0\x05\x12\x03O\x0B\x11\n\x0C\n\x05\x04\t\x02\0\x01\x12\x03O\x12\x1A\n\x0C\n\x05\x04\t\x02\0\x03\x12\x03O\x1D\x1E" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
