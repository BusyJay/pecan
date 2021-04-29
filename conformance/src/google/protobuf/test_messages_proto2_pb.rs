#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
pub const EXTENSION_INT32: pecan::Extension<i32, Varint> = pecan::Extension::new(960);
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ForeignEnumProto2(i32);
impl pecan::Enumerate for ForeignEnumProto2 {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> ForeignEnumProto2 {
        ForeignEnumProto2(v)
    }
}
impl ForeignEnumProto2 {
    pub const FOREIGN_FOO: ForeignEnumProto2 = ForeignEnumProto2(0);
    pub const FOREIGN_BAR: ForeignEnumProto2 = ForeignEnumProto2(1);
    pub const FOREIGN_BAZ: ForeignEnumProto2 = ForeignEnumProto2(2);
    pub const fn new() -> ForeignEnumProto2 {
        ForeignEnumProto2(0)
    }
}
impl std::fmt::Debug for ForeignEnumProto2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FOREIGN_FOO"),
            1 => write!(f, "FOREIGN_BAR"),
            2 => write!(f, "FOREIGN_BAZ"),
            v => write!(f, "ForeignEnumProto2({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TestAllTypesProto2_NestedEnum(i32);
impl pecan::Enumerate for TestAllTypesProto2_NestedEnum {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> TestAllTypesProto2_NestedEnum {
        TestAllTypesProto2_NestedEnum(v)
    }
}
impl TestAllTypesProto2_NestedEnum {
    pub const FOO: TestAllTypesProto2_NestedEnum = TestAllTypesProto2_NestedEnum(0);
    pub const BAR: TestAllTypesProto2_NestedEnum = TestAllTypesProto2_NestedEnum(1);
    pub const BAZ: TestAllTypesProto2_NestedEnum = TestAllTypesProto2_NestedEnum(2);
    pub const NEG: TestAllTypesProto2_NestedEnum = TestAllTypesProto2_NestedEnum(-1);
    pub const fn new() -> TestAllTypesProto2_NestedEnum {
        TestAllTypesProto2_NestedEnum(0)
    }
}
impl std::fmt::Debug for TestAllTypesProto2_NestedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FOO"),
            1 => write!(f, "BAR"),
            2 => write!(f, "BAZ"),
            -1 => write!(f, "NEG"),
            v => write!(f, "TestAllTypesProto2_NestedEnum({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2_NestedMessage {
    pub a: Option<i32>,
    pub corecursive: Option<Box<TestAllTypesProto2>>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2_NestedMessage {
    pub const fn new() -> TestAllTypesProto2_NestedMessage {
        TestAllTypesProto2_NestedMessage {
            a: None,
            corecursive: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn a(&self) -> i32 {
        self.a.unwrap_or_default()
    }
    pub fn a_mut(&mut self) -> &mut i32 {
        self.a.get_or_insert_with(Default::default)
    }
    pub fn set_a(&mut self, val: i32) {
        self.a = Some(val);
    }
    pub fn corecursive(&self) -> &TestAllTypesProto2 {
        match &self.corecursive {
            Some(v) => v,
            _ => TestAllTypesProto2::default_instance(),
        }
    }
    pub fn corecursive_mut(&mut self) -> &mut TestAllTypesProto2 {
        self.corecursive.get_or_insert_with(Default::default)
    }
    pub fn set_corecursive(&mut self, val: Box<TestAllTypesProto2>) {
        self.corecursive = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto2_NestedMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.a = Some(Varint::read_from(s)?),
                18 => LengthPrefixed::merge_from(self.corecursive_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.a {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.corecursive {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v.as_ref(), s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.a {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.corecursive {
            l += 1 + LengthPrefixed::size(v.as_ref());
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
impl pecan::DefaultInstance for TestAllTypesProto2_NestedMessage {
    fn default_instance() -> &'static TestAllTypesProto2_NestedMessage {
        static DEFAULT: TestAllTypesProto2_NestedMessage = TestAllTypesProto2_NestedMessage::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2_NestedMessage {
    #[inline]
    fn default() -> TestAllTypesProto2_NestedMessage {
        TestAllTypesProto2_NestedMessage::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2_Data {
    pub group_int32: Option<i32>,
    pub group_uint32: Option<u32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2_Data {
    pub const fn new() -> TestAllTypesProto2_Data {
        TestAllTypesProto2_Data {
            group_int32: None,
            group_uint32: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn group_int32(&self) -> i32 {
        self.group_int32.unwrap_or_default()
    }
    pub fn group_int32_mut(&mut self) -> &mut i32 {
        self.group_int32.get_or_insert_with(Default::default)
    }
    pub fn set_group_int32(&mut self, val: i32) {
        self.group_int32 = Some(val);
    }
    pub fn group_uint32(&self) -> u32 {
        self.group_uint32.unwrap_or_default()
    }
    pub fn group_uint32_mut(&mut self) -> &mut u32 {
        self.group_uint32.get_or_insert_with(Default::default)
    }
    pub fn set_group_uint32(&mut self, val: u32) {
        self.group_uint32 = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto2_Data {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                1616 => self.group_int32 = Some(Varint::read_from(s)?),
                1624 => self.group_uint32 = Some(Varint::read_from(s)?),
                0 | 1612 => {
                    s.set_last_tag(1612);
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
        if let Some(v) = self.group_int32 {
            s.write_tag(1616)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.group_uint32 {
            s.write_tag(1624)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.group_int32 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.group_uint32 {
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
impl pecan::DefaultInstance for TestAllTypesProto2_Data {
    fn default_instance() -> &'static TestAllTypesProto2_Data {
        static DEFAULT: TestAllTypesProto2_Data = TestAllTypesProto2_Data::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2_Data {
    #[inline]
    fn default() -> TestAllTypesProto2_Data {
        TestAllTypesProto2_Data::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2_MessageSetCorrect {
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2_MessageSetCorrect {
    pub const fn new() -> TestAllTypesProto2_MessageSetCorrect {
        TestAllTypesProto2_MessageSetCorrect {
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for TestAllTypesProto2_MessageSetCorrect {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                0 => return Ok(()),
                tag => {
                    if (32..=17179869183).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.extensions.is_empty() {
            l += self.extensions.size();
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
impl pecan::DefaultInstance for TestAllTypesProto2_MessageSetCorrect {
    fn default_instance() -> &'static TestAllTypesProto2_MessageSetCorrect {
        static DEFAULT: TestAllTypesProto2_MessageSetCorrect =
            TestAllTypesProto2_MessageSetCorrect::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2_MessageSetCorrect {
    #[inline]
    fn default() -> TestAllTypesProto2_MessageSetCorrect {
        TestAllTypesProto2_MessageSetCorrect::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2_MessageSetCorrectExtension1 {
    pub str: Option<String>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2_MessageSetCorrectExtension1 {
    pub const fn new() -> TestAllTypesProto2_MessageSetCorrectExtension1 {
        TestAllTypesProto2_MessageSetCorrectExtension1 {
            str: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn str(&self) -> &String {
        match &self.str {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn str_mut(&mut self) -> &mut String {
        self.str.get_or_insert_with(Default::default)
    }
    pub fn set_str(&mut self, val: String) {
        self.str = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto2_MessageSetCorrectExtension1 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                202 => self.str = Some(LengthPrefixed::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = &self.str {
            s.write_tag(202)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.str {
            l += 2 + LengthPrefixed::size(v);
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
impl pecan::DefaultInstance for TestAllTypesProto2_MessageSetCorrectExtension1 {
    fn default_instance() -> &'static TestAllTypesProto2_MessageSetCorrectExtension1 {
        static DEFAULT: TestAllTypesProto2_MessageSetCorrectExtension1 =
            TestAllTypesProto2_MessageSetCorrectExtension1::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2_MessageSetCorrectExtension1 {
    #[inline]
    fn default() -> TestAllTypesProto2_MessageSetCorrectExtension1 {
        TestAllTypesProto2_MessageSetCorrectExtension1::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2_MessageSetCorrectExtension2 {
    pub i: Option<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2_MessageSetCorrectExtension2 {
    pub const fn new() -> TestAllTypesProto2_MessageSetCorrectExtension2 {
        TestAllTypesProto2_MessageSetCorrectExtension2 {
            i: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn i(&self) -> i32 {
        self.i.unwrap_or_default()
    }
    pub fn i_mut(&mut self) -> &mut i32 {
        self.i.get_or_insert_with(Default::default)
    }
    pub fn set_i(&mut self, val: i32) {
        self.i = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto2_MessageSetCorrectExtension2 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                72 => self.i = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.i {
            s.write_tag(72)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.i {
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
impl pecan::DefaultInstance for TestAllTypesProto2_MessageSetCorrectExtension2 {
    fn default_instance() -> &'static TestAllTypesProto2_MessageSetCorrectExtension2 {
        static DEFAULT: TestAllTypesProto2_MessageSetCorrectExtension2 =
            TestAllTypesProto2_MessageSetCorrectExtension2::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2_MessageSetCorrectExtension2 {
    #[inline]
    fn default() -> TestAllTypesProto2_MessageSetCorrectExtension2 {
        TestAllTypesProto2_MessageSetCorrectExtension2::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum TestAllTypesProto2_Oneof_Field {
    OneofUint32(u32),
    OneofNestedMessage(TestAllTypesProto2_NestedMessage),
    OneofString(String),
    OneofBytes(pecan::Bytes),
    OneofBool(bool),
    OneofUint64(u64),
    OneofFloat(f32),
    OneofDouble(f64),
    OneofEnum(TestAllTypesProto2_NestedEnum),
    None,
}
impl Default for TestAllTypesProto2_Oneof_Field {
    #[inline]
    fn default() -> TestAllTypesProto2_Oneof_Field {
        TestAllTypesProto2_Oneof_Field::None
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto2 {
    pub optional_int32: Option<i32>,
    pub optional_int64: Option<i64>,
    pub optional_uint32: Option<u32>,
    pub optional_uint64: Option<u64>,
    pub optional_sint32: Option<i32>,
    pub optional_sint64: Option<i64>,
    pub optional_fixed32: Option<u32>,
    pub optional_fixed64: Option<u64>,
    pub optional_sfixed32: Option<i32>,
    pub optional_sfixed64: Option<i64>,
    pub optional_float: Option<f32>,
    pub optional_double: Option<f64>,
    pub optional_bool: Option<bool>,
    pub optional_string: Option<String>,
    pub optional_bytes: Option<pecan::Bytes>,
    pub optional_nested_message: Option<TestAllTypesProto2_NestedMessage>,
    pub optional_foreign_message: Option<ForeignMessageProto2>,
    pub optional_nested_enum: Option<TestAllTypesProto2_NestedEnum>,
    pub optional_foreign_enum: Option<ForeignEnumProto2>,
    pub optional_string_piece: Option<String>,
    pub optional_cord: Option<String>,
    pub recursive_message: Option<Box<TestAllTypesProto2>>,
    pub repeated_int32: Vec<i32>,
    pub repeated_int64: Vec<i64>,
    pub repeated_uint32: Vec<u32>,
    pub repeated_uint64: Vec<u64>,
    pub repeated_sint32: Vec<i32>,
    pub repeated_sint64: Vec<i64>,
    pub repeated_fixed32: Vec<u32>,
    pub repeated_fixed64: Vec<u64>,
    pub repeated_sfixed32: Vec<i32>,
    pub repeated_sfixed64: Vec<i64>,
    pub repeated_float: Vec<f32>,
    pub repeated_double: Vec<f64>,
    pub repeated_bool: Vec<bool>,
    pub repeated_string: Vec<String>,
    pub repeated_bytes: Vec<pecan::Bytes>,
    pub repeated_nested_message: Vec<TestAllTypesProto2_NestedMessage>,
    pub repeated_foreign_message: Vec<ForeignMessageProto2>,
    pub repeated_nested_enum: Vec<TestAllTypesProto2_NestedEnum>,
    pub repeated_foreign_enum: Vec<ForeignEnumProto2>,
    pub repeated_string_piece: Vec<String>,
    pub repeated_cord: Vec<String>,
    pub packed_int32: Vec<i32>,
    pub packed_int64: Vec<i64>,
    pub packed_uint32: Vec<u32>,
    pub packed_uint64: Vec<u64>,
    pub packed_sint32: Vec<i32>,
    pub packed_sint64: Vec<i64>,
    pub packed_fixed32: Vec<u32>,
    pub packed_fixed64: Vec<u64>,
    pub packed_sfixed32: Vec<i32>,
    pub packed_sfixed64: Vec<i64>,
    pub packed_float: Vec<f32>,
    pub packed_double: Vec<f64>,
    pub packed_bool: Vec<bool>,
    pub packed_nested_enum: Vec<TestAllTypesProto2_NestedEnum>,
    pub unpacked_int32: Vec<i32>,
    pub unpacked_int64: Vec<i64>,
    pub unpacked_uint32: Vec<u32>,
    pub unpacked_uint64: Vec<u64>,
    pub unpacked_sint32: Vec<i32>,
    pub unpacked_sint64: Vec<i64>,
    pub unpacked_fixed32: Vec<u32>,
    pub unpacked_fixed64: Vec<u64>,
    pub unpacked_sfixed32: Vec<i32>,
    pub unpacked_sfixed64: Vec<i64>,
    pub unpacked_float: Vec<f32>,
    pub unpacked_double: Vec<f64>,
    pub unpacked_bool: Vec<bool>,
    pub unpacked_nested_enum: Vec<TestAllTypesProto2_NestedEnum>,
    pub map_int32_int32: Option<pecan::HashMap<i32, i32>>,
    pub map_int64_int64: Option<pecan::HashMap<i64, i64>>,
    pub map_uint32_uint32: Option<pecan::HashMap<u32, u32>>,
    pub map_uint64_uint64: Option<pecan::HashMap<u64, u64>>,
    pub map_sint32_sint32: Option<pecan::HashMap<i32, i32>>,
    pub map_sint64_sint64: Option<pecan::HashMap<i64, i64>>,
    pub map_fixed32_fixed32: Option<pecan::HashMap<u32, u32>>,
    pub map_fixed64_fixed64: Option<pecan::HashMap<u64, u64>>,
    pub map_sfixed32_sfixed32: Option<pecan::HashMap<i32, i32>>,
    pub map_sfixed64_sfixed64: Option<pecan::HashMap<i64, i64>>,
    pub map_int32_float: Option<pecan::HashMap<i32, f32>>,
    pub map_int32_double: Option<pecan::HashMap<i32, f64>>,
    pub map_bool_bool: Option<pecan::HashMap<bool, bool>>,
    pub map_string_string: Option<pecan::HashMap<String, String>>,
    pub map_string_bytes: Option<pecan::HashMap<String, pecan::Bytes>>,
    pub map_string_nested_message: Option<pecan::HashMap<String, TestAllTypesProto2_NestedMessage>>,
    pub map_string_foreign_message: Option<pecan::HashMap<String, ForeignMessageProto2>>,
    pub map_string_nested_enum: Option<pecan::HashMap<String, TestAllTypesProto2_NestedEnum>>,
    pub map_string_foreign_enum: Option<pecan::HashMap<String, ForeignEnumProto2>>,
    pub oneof_field: TestAllTypesProto2_Oneof_Field,
    pub data: Option<TestAllTypesProto2_Data>,
    pub fieldname1: Option<i32>,
    pub field_name2: Option<i32>,
    pub _field_name3: Option<i32>,
    pub field__name4_: Option<i32>,
    pub field0name5: Option<i32>,
    pub field_0_name6: Option<i32>,
    pub field_name7: Option<i32>,
    pub field_name8: Option<i32>,
    pub field_name9: Option<i32>,
    pub field_name10: Option<i32>,
    pub field_name11: Option<i32>,
    pub field_name12: Option<i32>,
    pub __field_name13: Option<i32>,
    pub __field_name14: Option<i32>,
    pub field__name15: Option<i32>,
    pub field__name16: Option<i32>,
    pub field_name17__: Option<i32>,
    pub field_name18__: Option<i32>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto2 {
    pub const fn new() -> TestAllTypesProto2 {
        TestAllTypesProto2 {
            optional_int32: None,
            optional_int64: None,
            optional_uint32: None,
            optional_uint64: None,
            optional_sint32: None,
            optional_sint64: None,
            optional_fixed32: None,
            optional_fixed64: None,
            optional_sfixed32: None,
            optional_sfixed64: None,
            optional_float: None,
            optional_double: None,
            optional_bool: None,
            optional_string: None,
            optional_bytes: None,
            optional_nested_message: None,
            optional_foreign_message: None,
            optional_nested_enum: None,
            optional_foreign_enum: None,
            optional_string_piece: None,
            optional_cord: None,
            recursive_message: None,
            repeated_int32: Vec::new(),
            repeated_int64: Vec::new(),
            repeated_uint32: Vec::new(),
            repeated_uint64: Vec::new(),
            repeated_sint32: Vec::new(),
            repeated_sint64: Vec::new(),
            repeated_fixed32: Vec::new(),
            repeated_fixed64: Vec::new(),
            repeated_sfixed32: Vec::new(),
            repeated_sfixed64: Vec::new(),
            repeated_float: Vec::new(),
            repeated_double: Vec::new(),
            repeated_bool: Vec::new(),
            repeated_string: Vec::new(),
            repeated_bytes: Vec::new(),
            repeated_nested_message: Vec::new(),
            repeated_foreign_message: Vec::new(),
            repeated_nested_enum: Vec::new(),
            repeated_foreign_enum: Vec::new(),
            repeated_string_piece: Vec::new(),
            repeated_cord: Vec::new(),
            packed_int32: Vec::new(),
            packed_int64: Vec::new(),
            packed_uint32: Vec::new(),
            packed_uint64: Vec::new(),
            packed_sint32: Vec::new(),
            packed_sint64: Vec::new(),
            packed_fixed32: Vec::new(),
            packed_fixed64: Vec::new(),
            packed_sfixed32: Vec::new(),
            packed_sfixed64: Vec::new(),
            packed_float: Vec::new(),
            packed_double: Vec::new(),
            packed_bool: Vec::new(),
            packed_nested_enum: Vec::new(),
            unpacked_int32: Vec::new(),
            unpacked_int64: Vec::new(),
            unpacked_uint32: Vec::new(),
            unpacked_uint64: Vec::new(),
            unpacked_sint32: Vec::new(),
            unpacked_sint64: Vec::new(),
            unpacked_fixed32: Vec::new(),
            unpacked_fixed64: Vec::new(),
            unpacked_sfixed32: Vec::new(),
            unpacked_sfixed64: Vec::new(),
            unpacked_float: Vec::new(),
            unpacked_double: Vec::new(),
            unpacked_bool: Vec::new(),
            unpacked_nested_enum: Vec::new(),
            map_int32_int32: None,
            map_int64_int64: None,
            map_uint32_uint32: None,
            map_uint64_uint64: None,
            map_sint32_sint32: None,
            map_sint64_sint64: None,
            map_fixed32_fixed32: None,
            map_fixed64_fixed64: None,
            map_sfixed32_sfixed32: None,
            map_sfixed64_sfixed64: None,
            map_int32_float: None,
            map_int32_double: None,
            map_bool_bool: None,
            map_string_string: None,
            map_string_bytes: None,
            map_string_nested_message: None,
            map_string_foreign_message: None,
            map_string_nested_enum: None,
            map_string_foreign_enum: None,
            oneof_field: TestAllTypesProto2_Oneof_Field::None,
            data: None,
            fieldname1: None,
            field_name2: None,
            _field_name3: None,
            field__name4_: None,
            field0name5: None,
            field_0_name6: None,
            field_name7: None,
            field_name8: None,
            field_name9: None,
            field_name10: None,
            field_name11: None,
            field_name12: None,
            __field_name13: None,
            __field_name14: None,
            field__name15: None,
            field__name16: None,
            field_name17__: None,
            field_name18__: None,
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn optional_int32(&self) -> i32 {
        self.optional_int32.unwrap_or_default()
    }
    pub fn optional_int32_mut(&mut self) -> &mut i32 {
        self.optional_int32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_int32(&mut self, val: i32) {
        self.optional_int32 = Some(val);
    }
    pub fn optional_int64(&self) -> i64 {
        self.optional_int64.unwrap_or_default()
    }
    pub fn optional_int64_mut(&mut self) -> &mut i64 {
        self.optional_int64.get_or_insert_with(Default::default)
    }
    pub fn set_optional_int64(&mut self, val: i64) {
        self.optional_int64 = Some(val);
    }
    pub fn optional_uint32(&self) -> u32 {
        self.optional_uint32.unwrap_or_default()
    }
    pub fn optional_uint32_mut(&mut self) -> &mut u32 {
        self.optional_uint32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_uint32(&mut self, val: u32) {
        self.optional_uint32 = Some(val);
    }
    pub fn optional_uint64(&self) -> u64 {
        self.optional_uint64.unwrap_or_default()
    }
    pub fn optional_uint64_mut(&mut self) -> &mut u64 {
        self.optional_uint64.get_or_insert_with(Default::default)
    }
    pub fn set_optional_uint64(&mut self, val: u64) {
        self.optional_uint64 = Some(val);
    }
    pub fn optional_sint32(&self) -> i32 {
        self.optional_sint32.unwrap_or_default()
    }
    pub fn optional_sint32_mut(&mut self) -> &mut i32 {
        self.optional_sint32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_sint32(&mut self, val: i32) {
        self.optional_sint32 = Some(val);
    }
    pub fn optional_sint64(&self) -> i64 {
        self.optional_sint64.unwrap_or_default()
    }
    pub fn optional_sint64_mut(&mut self) -> &mut i64 {
        self.optional_sint64.get_or_insert_with(Default::default)
    }
    pub fn set_optional_sint64(&mut self, val: i64) {
        self.optional_sint64 = Some(val);
    }
    pub fn optional_fixed32(&self) -> u32 {
        self.optional_fixed32.unwrap_or_default()
    }
    pub fn optional_fixed32_mut(&mut self) -> &mut u32 {
        self.optional_fixed32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_fixed32(&mut self, val: u32) {
        self.optional_fixed32 = Some(val);
    }
    pub fn optional_fixed64(&self) -> u64 {
        self.optional_fixed64.unwrap_or_default()
    }
    pub fn optional_fixed64_mut(&mut self) -> &mut u64 {
        self.optional_fixed64.get_or_insert_with(Default::default)
    }
    pub fn set_optional_fixed64(&mut self, val: u64) {
        self.optional_fixed64 = Some(val);
    }
    pub fn optional_sfixed32(&self) -> i32 {
        self.optional_sfixed32.unwrap_or_default()
    }
    pub fn optional_sfixed32_mut(&mut self) -> &mut i32 {
        self.optional_sfixed32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_sfixed32(&mut self, val: i32) {
        self.optional_sfixed32 = Some(val);
    }
    pub fn optional_sfixed64(&self) -> i64 {
        self.optional_sfixed64.unwrap_or_default()
    }
    pub fn optional_sfixed64_mut(&mut self) -> &mut i64 {
        self.optional_sfixed64.get_or_insert_with(Default::default)
    }
    pub fn set_optional_sfixed64(&mut self, val: i64) {
        self.optional_sfixed64 = Some(val);
    }
    pub fn optional_float(&self) -> f32 {
        self.optional_float.unwrap_or_default()
    }
    pub fn optional_float_mut(&mut self) -> &mut f32 {
        self.optional_float.get_or_insert_with(Default::default)
    }
    pub fn set_optional_float(&mut self, val: f32) {
        self.optional_float = Some(val);
    }
    pub fn optional_double(&self) -> f64 {
        self.optional_double.unwrap_or_default()
    }
    pub fn optional_double_mut(&mut self) -> &mut f64 {
        self.optional_double.get_or_insert_with(Default::default)
    }
    pub fn set_optional_double(&mut self, val: f64) {
        self.optional_double = Some(val);
    }
    pub fn optional_bool(&self) -> bool {
        self.optional_bool.unwrap_or_default()
    }
    pub fn optional_bool_mut(&mut self) -> &mut bool {
        self.optional_bool.get_or_insert_with(Default::default)
    }
    pub fn set_optional_bool(&mut self, val: bool) {
        self.optional_bool = Some(val);
    }
    pub fn optional_string(&self) -> &String {
        match &self.optional_string {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn optional_string_mut(&mut self) -> &mut String {
        self.optional_string.get_or_insert_with(Default::default)
    }
    pub fn set_optional_string(&mut self, val: String) {
        self.optional_string = Some(val);
    }
    pub fn optional_bytes(&self) -> &pecan::Bytes {
        match &self.optional_bytes {
            Some(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn optional_bytes_mut(&mut self) -> &mut pecan::Bytes {
        self.optional_bytes.get_or_insert_with(Default::default)
    }
    pub fn set_optional_bytes(&mut self, val: pecan::Bytes) {
        self.optional_bytes = Some(val);
    }
    pub fn optional_nested_message(&self) -> &TestAllTypesProto2_NestedMessage {
        match &self.optional_nested_message {
            Some(v) => v,
            _ => TestAllTypesProto2_NestedMessage::default_instance(),
        }
    }
    pub fn optional_nested_message_mut(&mut self) -> &mut TestAllTypesProto2_NestedMessage {
        self.optional_nested_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_nested_message(&mut self, val: TestAllTypesProto2_NestedMessage) {
        self.optional_nested_message = Some(val);
    }
    pub fn optional_foreign_message(&self) -> &ForeignMessageProto2 {
        match &self.optional_foreign_message {
            Some(v) => v,
            _ => ForeignMessageProto2::default_instance(),
        }
    }
    pub fn optional_foreign_message_mut(&mut self) -> &mut ForeignMessageProto2 {
        self.optional_foreign_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_foreign_message(&mut self, val: ForeignMessageProto2) {
        self.optional_foreign_message = Some(val);
    }
    pub fn optional_nested_enum(&self) -> TestAllTypesProto2_NestedEnum {
        self.optional_nested_enum.unwrap_or_default()
    }
    pub fn optional_nested_enum_mut(&mut self) -> &mut TestAllTypesProto2_NestedEnum {
        self.optional_nested_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_nested_enum(&mut self, val: TestAllTypesProto2_NestedEnum) {
        self.optional_nested_enum = Some(val);
    }
    pub fn optional_foreign_enum(&self) -> ForeignEnumProto2 {
        self.optional_foreign_enum.unwrap_or_default()
    }
    pub fn optional_foreign_enum_mut(&mut self) -> &mut ForeignEnumProto2 {
        self.optional_foreign_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_foreign_enum(&mut self, val: ForeignEnumProto2) {
        self.optional_foreign_enum = Some(val);
    }
    pub fn optional_string_piece(&self) -> &String {
        match &self.optional_string_piece {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn optional_string_piece_mut(&mut self) -> &mut String {
        self.optional_string_piece
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_string_piece(&mut self, val: String) {
        self.optional_string_piece = Some(val);
    }
    pub fn optional_cord(&self) -> &String {
        match &self.optional_cord {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn optional_cord_mut(&mut self) -> &mut String {
        self.optional_cord.get_or_insert_with(Default::default)
    }
    pub fn set_optional_cord(&mut self, val: String) {
        self.optional_cord = Some(val);
    }
    pub fn recursive_message(&self) -> &TestAllTypesProto2 {
        match &self.recursive_message {
            Some(v) => v,
            _ => TestAllTypesProto2::default_instance(),
        }
    }
    pub fn recursive_message_mut(&mut self) -> &mut TestAllTypesProto2 {
        self.recursive_message.get_or_insert_with(Default::default)
    }
    pub fn set_recursive_message(&mut self, val: Box<TestAllTypesProto2>) {
        self.recursive_message = Some(val);
    }
    pub fn map_int32_int32(&self) -> &pecan::HashMap<i32, i32> {
        match &self.map_int32_int32 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i32 , i32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_int32_int32_mut(&mut self) -> &mut pecan::HashMap<i32, i32> {
        self.map_int32_int32.get_or_insert_with(Default::default)
    }
    pub fn set_map_int32_int32(&mut self, val: pecan::HashMap<i32, i32>) {
        self.map_int32_int32 = Some(val);
    }
    pub fn map_int64_int64(&self) -> &pecan::HashMap<i64, i64> {
        match &self.map_int64_int64 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i64 , i64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_int64_int64_mut(&mut self) -> &mut pecan::HashMap<i64, i64> {
        self.map_int64_int64.get_or_insert_with(Default::default)
    }
    pub fn set_map_int64_int64(&mut self, val: pecan::HashMap<i64, i64>) {
        self.map_int64_int64 = Some(val);
    }
    pub fn map_uint32_uint32(&self) -> &pecan::HashMap<u32, u32> {
        match &self.map_uint32_uint32 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < u32 , u32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_uint32_uint32_mut(&mut self) -> &mut pecan::HashMap<u32, u32> {
        self.map_uint32_uint32.get_or_insert_with(Default::default)
    }
    pub fn set_map_uint32_uint32(&mut self, val: pecan::HashMap<u32, u32>) {
        self.map_uint32_uint32 = Some(val);
    }
    pub fn map_uint64_uint64(&self) -> &pecan::HashMap<u64, u64> {
        match &self.map_uint64_uint64 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < u64 , u64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_uint64_uint64_mut(&mut self) -> &mut pecan::HashMap<u64, u64> {
        self.map_uint64_uint64.get_or_insert_with(Default::default)
    }
    pub fn set_map_uint64_uint64(&mut self, val: pecan::HashMap<u64, u64>) {
        self.map_uint64_uint64 = Some(val);
    }
    pub fn map_sint32_sint32(&self) -> &pecan::HashMap<i32, i32> {
        match &self.map_sint32_sint32 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i32 , i32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_sint32_sint32_mut(&mut self) -> &mut pecan::HashMap<i32, i32> {
        self.map_sint32_sint32.get_or_insert_with(Default::default)
    }
    pub fn set_map_sint32_sint32(&mut self, val: pecan::HashMap<i32, i32>) {
        self.map_sint32_sint32 = Some(val);
    }
    pub fn map_sint64_sint64(&self) -> &pecan::HashMap<i64, i64> {
        match &self.map_sint64_sint64 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i64 , i64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_sint64_sint64_mut(&mut self) -> &mut pecan::HashMap<i64, i64> {
        self.map_sint64_sint64.get_or_insert_with(Default::default)
    }
    pub fn set_map_sint64_sint64(&mut self, val: pecan::HashMap<i64, i64>) {
        self.map_sint64_sint64 = Some(val);
    }
    pub fn map_fixed32_fixed32(&self) -> &pecan::HashMap<u32, u32> {
        match &self.map_fixed32_fixed32 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < u32 , u32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_fixed32_fixed32_mut(&mut self) -> &mut pecan::HashMap<u32, u32> {
        self.map_fixed32_fixed32
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_fixed32_fixed32(&mut self, val: pecan::HashMap<u32, u32>) {
        self.map_fixed32_fixed32 = Some(val);
    }
    pub fn map_fixed64_fixed64(&self) -> &pecan::HashMap<u64, u64> {
        match &self.map_fixed64_fixed64 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < u64 , u64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_fixed64_fixed64_mut(&mut self) -> &mut pecan::HashMap<u64, u64> {
        self.map_fixed64_fixed64
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_fixed64_fixed64(&mut self, val: pecan::HashMap<u64, u64>) {
        self.map_fixed64_fixed64 = Some(val);
    }
    pub fn map_sfixed32_sfixed32(&self) -> &pecan::HashMap<i32, i32> {
        match &self.map_sfixed32_sfixed32 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i32 , i32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_sfixed32_sfixed32_mut(&mut self) -> &mut pecan::HashMap<i32, i32> {
        self.map_sfixed32_sfixed32
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_sfixed32_sfixed32(&mut self, val: pecan::HashMap<i32, i32>) {
        self.map_sfixed32_sfixed32 = Some(val);
    }
    pub fn map_sfixed64_sfixed64(&self) -> &pecan::HashMap<i64, i64> {
        match &self.map_sfixed64_sfixed64 {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i64 , i64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_sfixed64_sfixed64_mut(&mut self) -> &mut pecan::HashMap<i64, i64> {
        self.map_sfixed64_sfixed64
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_sfixed64_sfixed64(&mut self, val: pecan::HashMap<i64, i64>) {
        self.map_sfixed64_sfixed64 = Some(val);
    }
    pub fn map_int32_float(&self) -> &pecan::HashMap<i32, f32> {
        match &self.map_int32_float {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i32 , f32 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_int32_float_mut(&mut self) -> &mut pecan::HashMap<i32, f32> {
        self.map_int32_float.get_or_insert_with(Default::default)
    }
    pub fn set_map_int32_float(&mut self, val: pecan::HashMap<i32, f32>) {
        self.map_int32_float = Some(val);
    }
    pub fn map_int32_double(&self) -> &pecan::HashMap<i32, f64> {
        match &self.map_int32_double {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < i32 , f64 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_int32_double_mut(&mut self) -> &mut pecan::HashMap<i32, f64> {
        self.map_int32_double.get_or_insert_with(Default::default)
    }
    pub fn set_map_int32_double(&mut self, val: pecan::HashMap<i32, f64>) {
        self.map_int32_double = Some(val);
    }
    pub fn map_bool_bool(&self) -> &pecan::HashMap<bool, bool> {
        match &self.map_bool_bool {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < bool , bool > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_bool_bool_mut(&mut self) -> &mut pecan::HashMap<bool, bool> {
        self.map_bool_bool.get_or_insert_with(Default::default)
    }
    pub fn set_map_bool_bool(&mut self, val: pecan::HashMap<bool, bool>) {
        self.map_bool_bool = Some(val);
    }
    pub fn map_string_string(&self) -> &pecan::HashMap<String, String> {
        match &self.map_string_string {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , String > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_string_mut(&mut self) -> &mut pecan::HashMap<String, String> {
        self.map_string_string.get_or_insert_with(Default::default)
    }
    pub fn set_map_string_string(&mut self, val: pecan::HashMap<String, String>) {
        self.map_string_string = Some(val);
    }
    pub fn map_string_bytes(&self) -> &pecan::HashMap<String, pecan::Bytes> {
        match &self.map_string_bytes {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , pecan :: Bytes > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_bytes_mut(&mut self) -> &mut pecan::HashMap<String, pecan::Bytes> {
        self.map_string_bytes.get_or_insert_with(Default::default)
    }
    pub fn set_map_string_bytes(&mut self, val: pecan::HashMap<String, pecan::Bytes>) {
        self.map_string_bytes = Some(val);
    }
    pub fn map_string_nested_message(
        &self,
    ) -> &pecan::HashMap<String, TestAllTypesProto2_NestedMessage> {
        match &self.map_string_nested_message {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , TestAllTypesProto2_NestedMessage > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_nested_message_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, TestAllTypesProto2_NestedMessage> {
        self.map_string_nested_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_nested_message(
        &mut self,
        val: pecan::HashMap<String, TestAllTypesProto2_NestedMessage>,
    ) {
        self.map_string_nested_message = Some(val);
    }
    pub fn map_string_foreign_message(&self) -> &pecan::HashMap<String, ForeignMessageProto2> {
        match &self.map_string_foreign_message {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , ForeignMessageProto2 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_foreign_message_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, ForeignMessageProto2> {
        self.map_string_foreign_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_foreign_message(
        &mut self,
        val: pecan::HashMap<String, ForeignMessageProto2>,
    ) {
        self.map_string_foreign_message = Some(val);
    }
    pub fn map_string_nested_enum(&self) -> &pecan::HashMap<String, TestAllTypesProto2_NestedEnum> {
        match &self.map_string_nested_enum {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , TestAllTypesProto2_NestedEnum > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_nested_enum_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, TestAllTypesProto2_NestedEnum> {
        self.map_string_nested_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_nested_enum(
        &mut self,
        val: pecan::HashMap<String, TestAllTypesProto2_NestedEnum>,
    ) {
        self.map_string_nested_enum = Some(val);
    }
    pub fn map_string_foreign_enum(&self) -> &pecan::HashMap<String, ForeignEnumProto2> {
        match &self.map_string_foreign_enum {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , ForeignEnumProto2 > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_foreign_enum_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, ForeignEnumProto2> {
        self.map_string_foreign_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_foreign_enum(&mut self, val: pecan::HashMap<String, ForeignEnumProto2>) {
        self.map_string_foreign_enum = Some(val);
    }
    pub fn oneof_uint32(&self) -> u32 {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofUint32(v) => *v,
            _ => 0,
        }
    }
    pub fn oneof_uint32_mut(&mut self) -> &mut u32 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofUint32(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofUint32(0);
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofUint32(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_uint32(&mut self, val: u32) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofUint32(val);
    }
    pub fn oneof_nested_message(&self) -> &TestAllTypesProto2_NestedMessage {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofNestedMessage(v) => v,
            _ => TestAllTypesProto2_NestedMessage::default_instance(),
        }
    }
    pub fn oneof_nested_message_mut(&mut self) -> &mut TestAllTypesProto2_NestedMessage {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofNestedMessage(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofNestedMessage(
                TestAllTypesProto2_NestedMessage::new(),
            );
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofNestedMessage(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_nested_message(&mut self, val: TestAllTypesProto2_NestedMessage) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofNestedMessage(val);
    }
    pub fn oneof_string(&self) -> &String {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofString(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn oneof_string_mut(&mut self) -> &mut String {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofString(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofString(String::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofString(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_string(&mut self, val: String) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofString(val);
    }
    pub fn oneof_bytes(&self) -> &pecan::Bytes {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofBytes(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn oneof_bytes_mut(&mut self) -> &mut pecan::Bytes {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofBytes(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofBytes(pecan::Bytes::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofBytes(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_bytes(&mut self, val: pecan::Bytes) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofBytes(val);
    }
    pub fn oneof_bool(&self) -> bool {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofBool(v) => *v,
            _ => false,
        }
    }
    pub fn oneof_bool_mut(&mut self) -> &mut bool {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofBool(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofBool(false);
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofBool(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_bool(&mut self, val: bool) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofBool(val);
    }
    pub fn oneof_uint64(&self) -> u64 {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofUint64(v) => *v,
            _ => 0,
        }
    }
    pub fn oneof_uint64_mut(&mut self) -> &mut u64 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofUint64(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofUint64(0);
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofUint64(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_uint64(&mut self, val: u64) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofUint64(val);
    }
    pub fn oneof_float(&self) -> f32 {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofFloat(v) => *v,
            _ => 0f32,
        }
    }
    pub fn oneof_float_mut(&mut self) -> &mut f32 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofFloat(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofFloat(0f32);
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofFloat(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_float(&mut self, val: f32) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofFloat(val);
    }
    pub fn oneof_double(&self) -> f64 {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofDouble(v) => *v,
            _ => 0f64,
        }
    }
    pub fn oneof_double_mut(&mut self) -> &mut f64 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofDouble(_)
        ) {
            self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofDouble(0f64);
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofDouble(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_double(&mut self, val: f64) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofDouble(val);
    }
    pub fn oneof_enum(&self) -> TestAllTypesProto2_NestedEnum {
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofEnum(v) => *v,
            _ => TestAllTypesProto2_NestedEnum::new(),
        }
    }
    pub fn oneof_enum_mut(&mut self) -> &mut TestAllTypesProto2_NestedEnum {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto2_Oneof_Field::OneofEnum(_)
        ) {
            self.oneof_field =
                TestAllTypesProto2_Oneof_Field::OneofEnum(TestAllTypesProto2_NestedEnum::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto2_Oneof_Field::OneofEnum(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_enum(&mut self, val: TestAllTypesProto2_NestedEnum) {
        self.oneof_field = TestAllTypesProto2_Oneof_Field::OneofEnum(val);
    }
    pub fn data(&self) -> &TestAllTypesProto2_Data {
        match &self.data {
            Some(v) => v,
            _ => TestAllTypesProto2_Data::default_instance(),
        }
    }
    pub fn data_mut(&mut self) -> &mut TestAllTypesProto2_Data {
        self.data.get_or_insert_with(Default::default)
    }
    pub fn set_data(&mut self, val: TestAllTypesProto2_Data) {
        self.data = Some(val);
    }
    pub fn fieldname1(&self) -> i32 {
        self.fieldname1.unwrap_or_default()
    }
    pub fn fieldname1_mut(&mut self) -> &mut i32 {
        self.fieldname1.get_or_insert_with(Default::default)
    }
    pub fn set_fieldname1(&mut self, val: i32) {
        self.fieldname1 = Some(val);
    }
    pub fn field_name2(&self) -> i32 {
        self.field_name2.unwrap_or_default()
    }
    pub fn field_name2_mut(&mut self) -> &mut i32 {
        self.field_name2.get_or_insert_with(Default::default)
    }
    pub fn set_field_name2(&mut self, val: i32) {
        self.field_name2 = Some(val);
    }
    pub fn _field_name3(&self) -> i32 {
        self._field_name3.unwrap_or_default()
    }
    pub fn _field_name3_mut(&mut self) -> &mut i32 {
        self._field_name3.get_or_insert_with(Default::default)
    }
    pub fn set__field_name3(&mut self, val: i32) {
        self._field_name3 = Some(val);
    }
    pub fn field__name4_(&self) -> i32 {
        self.field__name4_.unwrap_or_default()
    }
    pub fn field__name4__mut(&mut self) -> &mut i32 {
        self.field__name4_.get_or_insert_with(Default::default)
    }
    pub fn set_field__name4_(&mut self, val: i32) {
        self.field__name4_ = Some(val);
    }
    pub fn field0name5(&self) -> i32 {
        self.field0name5.unwrap_or_default()
    }
    pub fn field0name5_mut(&mut self) -> &mut i32 {
        self.field0name5.get_or_insert_with(Default::default)
    }
    pub fn set_field0name5(&mut self, val: i32) {
        self.field0name5 = Some(val);
    }
    pub fn field_0_name6(&self) -> i32 {
        self.field_0_name6.unwrap_or_default()
    }
    pub fn field_0_name6_mut(&mut self) -> &mut i32 {
        self.field_0_name6.get_or_insert_with(Default::default)
    }
    pub fn set_field_0_name6(&mut self, val: i32) {
        self.field_0_name6 = Some(val);
    }
    pub fn field_name7(&self) -> i32 {
        self.field_name7.unwrap_or_default()
    }
    pub fn field_name7_mut(&mut self) -> &mut i32 {
        self.field_name7.get_or_insert_with(Default::default)
    }
    pub fn set_field_name7(&mut self, val: i32) {
        self.field_name7 = Some(val);
    }
    pub fn field_name8(&self) -> i32 {
        self.field_name8.unwrap_or_default()
    }
    pub fn field_name8_mut(&mut self) -> &mut i32 {
        self.field_name8.get_or_insert_with(Default::default)
    }
    pub fn set_field_name8(&mut self, val: i32) {
        self.field_name8 = Some(val);
    }
    pub fn field_name9(&self) -> i32 {
        self.field_name9.unwrap_or_default()
    }
    pub fn field_name9_mut(&mut self) -> &mut i32 {
        self.field_name9.get_or_insert_with(Default::default)
    }
    pub fn set_field_name9(&mut self, val: i32) {
        self.field_name9 = Some(val);
    }
    pub fn field_name10(&self) -> i32 {
        self.field_name10.unwrap_or_default()
    }
    pub fn field_name10_mut(&mut self) -> &mut i32 {
        self.field_name10.get_or_insert_with(Default::default)
    }
    pub fn set_field_name10(&mut self, val: i32) {
        self.field_name10 = Some(val);
    }
    pub fn field_name11(&self) -> i32 {
        self.field_name11.unwrap_or_default()
    }
    pub fn field_name11_mut(&mut self) -> &mut i32 {
        self.field_name11.get_or_insert_with(Default::default)
    }
    pub fn set_field_name11(&mut self, val: i32) {
        self.field_name11 = Some(val);
    }
    pub fn field_name12(&self) -> i32 {
        self.field_name12.unwrap_or_default()
    }
    pub fn field_name12_mut(&mut self) -> &mut i32 {
        self.field_name12.get_or_insert_with(Default::default)
    }
    pub fn set_field_name12(&mut self, val: i32) {
        self.field_name12 = Some(val);
    }
    pub fn __field_name13(&self) -> i32 {
        self.__field_name13.unwrap_or_default()
    }
    pub fn __field_name13_mut(&mut self) -> &mut i32 {
        self.__field_name13.get_or_insert_with(Default::default)
    }
    pub fn set___field_name13(&mut self, val: i32) {
        self.__field_name13 = Some(val);
    }
    pub fn __field_name14(&self) -> i32 {
        self.__field_name14.unwrap_or_default()
    }
    pub fn __field_name14_mut(&mut self) -> &mut i32 {
        self.__field_name14.get_or_insert_with(Default::default)
    }
    pub fn set___field_name14(&mut self, val: i32) {
        self.__field_name14 = Some(val);
    }
    pub fn field__name15(&self) -> i32 {
        self.field__name15.unwrap_or_default()
    }
    pub fn field__name15_mut(&mut self) -> &mut i32 {
        self.field__name15.get_or_insert_with(Default::default)
    }
    pub fn set_field__name15(&mut self, val: i32) {
        self.field__name15 = Some(val);
    }
    pub fn field__name16(&self) -> i32 {
        self.field__name16.unwrap_or_default()
    }
    pub fn field__name16_mut(&mut self) -> &mut i32 {
        self.field__name16.get_or_insert_with(Default::default)
    }
    pub fn set_field__name16(&mut self, val: i32) {
        self.field__name16 = Some(val);
    }
    pub fn field_name17__(&self) -> i32 {
        self.field_name17__.unwrap_or_default()
    }
    pub fn field_name17___mut(&mut self) -> &mut i32 {
        self.field_name17__.get_or_insert_with(Default::default)
    }
    pub fn set_field_name17__(&mut self, val: i32) {
        self.field_name17__ = Some(val);
    }
    pub fn field_name18__(&self) -> i32 {
        self.field_name18__.unwrap_or_default()
    }
    pub fn field_name18___mut(&mut self) -> &mut i32 {
        self.field_name18__.get_or_insert_with(Default::default)
    }
    pub fn set_field_name18__(&mut self, val: i32) {
        self.field_name18__ = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto2 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.optional_int32 = Some(Varint::read_from(s)?),
                16 => self.optional_int64 = Some(Varint::read_from(s)?),
                24 => self.optional_uint32 = Some(Varint::read_from(s)?),
                32 => self.optional_uint64 = Some(Varint::read_from(s)?),
                40 => self.optional_sint32 = Some(ZigZag::read_from(s)?),
                48 => self.optional_sint64 = Some(ZigZag::read_from(s)?),
                61 => self.optional_fixed32 = Some(Fixed32::read_from(s)?),
                65 => self.optional_fixed64 = Some(Fixed64::read_from(s)?),
                77 => self.optional_sfixed32 = Some(Fixed32::read_from(s)?),
                81 => self.optional_sfixed64 = Some(Fixed64::read_from(s)?),
                93 => self.optional_float = Some(Fixed32::read_from(s)?),
                97 => self.optional_double = Some(Fixed64::read_from(s)?),
                104 => self.optional_bool = Some(Varint::read_from(s)?),
                114 => self.optional_string = Some(LengthPrefixed::read_from(s)?),
                122 => self.optional_bytes = Some(LengthPrefixed::read_from(s)?),
                146 => LengthPrefixed::merge_from(self.optional_nested_message_mut(), s)?,
                154 => LengthPrefixed::merge_from(self.optional_foreign_message_mut(), s)?,
                168 => self.optional_nested_enum = Some(Varint::read_from(s)?),
                176 => self.optional_foreign_enum = Some(Varint::read_from(s)?),
                194 => self.optional_string_piece = Some(LengthPrefixed::read_from(s)?),
                202 => self.optional_cord = Some(LengthPrefixed::read_from(s)?),
                218 => LengthPrefixed::merge_from(self.recursive_message_mut(), s)?,
                248 => CopyArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                250 => PackedArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                256 => CopyArray::<Varint>::merge_from(&mut self.repeated_int64, s)?,
                258 => PackedArray::<Varint>::merge_from(&mut self.repeated_int64, s)?,
                264 => CopyArray::<Varint>::merge_from(&mut self.repeated_uint32, s)?,
                266 => PackedArray::<Varint>::merge_from(&mut self.repeated_uint32, s)?,
                272 => CopyArray::<Varint>::merge_from(&mut self.repeated_uint64, s)?,
                274 => PackedArray::<Varint>::merge_from(&mut self.repeated_uint64, s)?,
                280 => CopyArray::<ZigZag>::merge_from(&mut self.repeated_sint32, s)?,
                282 => PackedArray::<ZigZag>::merge_from(&mut self.repeated_sint32, s)?,
                288 => CopyArray::<ZigZag>::merge_from(&mut self.repeated_sint64, s)?,
                290 => PackedArray::<ZigZag>::merge_from(&mut self.repeated_sint64, s)?,
                301 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_fixed32, s)?,
                298 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_fixed32, s)?,
                305 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_fixed64, s)?,
                306 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_fixed64, s)?,
                317 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_sfixed32, s)?,
                314 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_sfixed32, s)?,
                321 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_sfixed64, s)?,
                322 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_sfixed64, s)?,
                333 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_float, s)?,
                330 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_float, s)?,
                337 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_double, s)?,
                338 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_double, s)?,
                344 => CopyArray::<Varint>::merge_from(&mut self.repeated_bool, s)?,
                346 => PackedArray::<Varint>::merge_from(&mut self.repeated_bool, s)?,
                354 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_string, s)?,
                362 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_bytes, s)?,
                386 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_nested_message, s)?
                }
                394 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_foreign_message, s)?
                }
                408 => CopyArray::<Varint>::merge_from(&mut self.repeated_nested_enum, s)?,
                410 => PackedArray::<Varint>::merge_from(&mut self.repeated_nested_enum, s)?,
                416 => CopyArray::<Varint>::merge_from(&mut self.repeated_foreign_enum, s)?,
                418 => PackedArray::<Varint>::merge_from(&mut self.repeated_foreign_enum, s)?,
                434 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_string_piece, s)?,
                442 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_cord, s)?,
                450 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_int32_int32_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                458 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_int64_int64_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                466 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_uint32_uint32_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                474 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_uint64_uint64_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                482 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = ZigZag::read_from(s)?,
                                16 => val = ZigZag::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_sint32_sint32_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                490 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                8 => key = ZigZag::read_from(s)?,
                                16 => val = ZigZag::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_sint64_sint64_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                498 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                13 => key = Fixed32::read_from(s)?,
                                21 => val = Fixed32::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_fixed32_fixed32_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                506 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                9 => key = Fixed64::read_from(s)?,
                                17 => val = Fixed64::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_fixed64_fixed64_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                514 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                13 => key = Fixed32::read_from(s)?,
                                21 => val = Fixed32::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_sfixed32_sfixed32_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                522 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0;
                        loop {
                            match s.read_tag()? {
                                9 => key = Fixed64::read_from(s)?,
                                17 => val = Fixed64::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_sfixed64_sfixed64_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                530 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0f32;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                21 => val = Fixed32::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_int32_float_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                538 => {
                    s.read_length_limited(|s| {
                        let mut key = 0;
                        let mut val = 0f64;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                17 => val = Fixed64::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_int32_double_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                546 => {
                    s.read_length_limited(|s| {
                        let mut key = false;
                        let mut val = false;
                        loop {
                            match s.read_tag()? {
                                8 => key = Varint::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_bool_bool_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                554 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = String::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                18 => val = LengthPrefixed::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_string_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                562 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = pecan::Bytes::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                18 => val = LengthPrefixed::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_bytes_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                570 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = TestAllTypesProto2_NestedMessage::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                18 => LengthPrefixed::merge_from(&mut val, s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_nested_message_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                578 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = ForeignMessageProto2::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                18 => LengthPrefixed::merge_from(&mut val, s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_foreign_message_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                586 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = TestAllTypesProto2_NestedEnum::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_nested_enum_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                594 => {
                    s.read_length_limited(|s| {
                        let mut key = String::new();
                        let mut val = ForeignEnumProto2::new();
                        loop {
                            match s.read_tag()? {
                                10 => key = LengthPrefixed::read_from(s)?,
                                16 => val = Varint::read_from(s)?,
                                0 => break,
                                _ => (),
                            }
                        }
                        self.map_string_foreign_enum_mut().insert(key, val);
                        Ok(())
                    })?;
                }
                602 => PackedArray::<Varint>::merge_from(&mut self.packed_int32, s)?,
                600 => CopyArray::<Varint>::merge_from(&mut self.packed_int32, s)?,
                610 => PackedArray::<Varint>::merge_from(&mut self.packed_int64, s)?,
                608 => CopyArray::<Varint>::merge_from(&mut self.packed_int64, s)?,
                618 => PackedArray::<Varint>::merge_from(&mut self.packed_uint32, s)?,
                616 => CopyArray::<Varint>::merge_from(&mut self.packed_uint32, s)?,
                626 => PackedArray::<Varint>::merge_from(&mut self.packed_uint64, s)?,
                624 => CopyArray::<Varint>::merge_from(&mut self.packed_uint64, s)?,
                634 => PackedArray::<ZigZag>::merge_from(&mut self.packed_sint32, s)?,
                632 => CopyArray::<ZigZag>::merge_from(&mut self.packed_sint32, s)?,
                642 => PackedArray::<ZigZag>::merge_from(&mut self.packed_sint64, s)?,
                640 => CopyArray::<ZigZag>::merge_from(&mut self.packed_sint64, s)?,
                650 => PackedArray::<Fixed32>::merge_from(&mut self.packed_fixed32, s)?,
                653 => CopyArray::<Fixed32>::merge_from(&mut self.packed_fixed32, s)?,
                658 => PackedArray::<Fixed64>::merge_from(&mut self.packed_fixed64, s)?,
                657 => CopyArray::<Fixed64>::merge_from(&mut self.packed_fixed64, s)?,
                666 => PackedArray::<Fixed32>::merge_from(&mut self.packed_sfixed32, s)?,
                669 => CopyArray::<Fixed32>::merge_from(&mut self.packed_sfixed32, s)?,
                674 => PackedArray::<Fixed64>::merge_from(&mut self.packed_sfixed64, s)?,
                673 => CopyArray::<Fixed64>::merge_from(&mut self.packed_sfixed64, s)?,
                682 => PackedArray::<Fixed32>::merge_from(&mut self.packed_float, s)?,
                685 => CopyArray::<Fixed32>::merge_from(&mut self.packed_float, s)?,
                690 => PackedArray::<Fixed64>::merge_from(&mut self.packed_double, s)?,
                689 => CopyArray::<Fixed64>::merge_from(&mut self.packed_double, s)?,
                698 => PackedArray::<Varint>::merge_from(&mut self.packed_bool, s)?,
                696 => CopyArray::<Varint>::merge_from(&mut self.packed_bool, s)?,
                706 => PackedArray::<Varint>::merge_from(&mut self.packed_nested_enum, s)?,
                704 => CopyArray::<Varint>::merge_from(&mut self.packed_nested_enum, s)?,
                712 => CopyArray::<Varint>::merge_from(&mut self.unpacked_int32, s)?,
                714 => PackedArray::<Varint>::merge_from(&mut self.unpacked_int32, s)?,
                720 => CopyArray::<Varint>::merge_from(&mut self.unpacked_int64, s)?,
                722 => PackedArray::<Varint>::merge_from(&mut self.unpacked_int64, s)?,
                728 => CopyArray::<Varint>::merge_from(&mut self.unpacked_uint32, s)?,
                730 => PackedArray::<Varint>::merge_from(&mut self.unpacked_uint32, s)?,
                736 => CopyArray::<Varint>::merge_from(&mut self.unpacked_uint64, s)?,
                738 => PackedArray::<Varint>::merge_from(&mut self.unpacked_uint64, s)?,
                744 => CopyArray::<ZigZag>::merge_from(&mut self.unpacked_sint32, s)?,
                746 => PackedArray::<ZigZag>::merge_from(&mut self.unpacked_sint32, s)?,
                752 => CopyArray::<ZigZag>::merge_from(&mut self.unpacked_sint64, s)?,
                754 => PackedArray::<ZigZag>::merge_from(&mut self.unpacked_sint64, s)?,
                765 => CopyArray::<Fixed32>::merge_from(&mut self.unpacked_fixed32, s)?,
                762 => PackedArray::<Fixed32>::merge_from(&mut self.unpacked_fixed32, s)?,
                769 => CopyArray::<Fixed64>::merge_from(&mut self.unpacked_fixed64, s)?,
                770 => PackedArray::<Fixed64>::merge_from(&mut self.unpacked_fixed64, s)?,
                781 => CopyArray::<Fixed32>::merge_from(&mut self.unpacked_sfixed32, s)?,
                778 => PackedArray::<Fixed32>::merge_from(&mut self.unpacked_sfixed32, s)?,
                785 => CopyArray::<Fixed64>::merge_from(&mut self.unpacked_sfixed64, s)?,
                786 => PackedArray::<Fixed64>::merge_from(&mut self.unpacked_sfixed64, s)?,
                797 => CopyArray::<Fixed32>::merge_from(&mut self.unpacked_float, s)?,
                794 => PackedArray::<Fixed32>::merge_from(&mut self.unpacked_float, s)?,
                801 => CopyArray::<Fixed64>::merge_from(&mut self.unpacked_double, s)?,
                802 => PackedArray::<Fixed64>::merge_from(&mut self.unpacked_double, s)?,
                808 => CopyArray::<Varint>::merge_from(&mut self.unpacked_bool, s)?,
                810 => PackedArray::<Varint>::merge_from(&mut self.unpacked_bool, s)?,
                816 => CopyArray::<Varint>::merge_from(&mut self.unpacked_nested_enum, s)?,
                818 => PackedArray::<Varint>::merge_from(&mut self.unpacked_nested_enum, s)?,
                888 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofUint32(Varint::read_from(s)?)
                }
                898 => LengthPrefixed::merge_from(self.oneof_nested_message_mut(), s)?,
                906 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofString(LengthPrefixed::read_from(s)?)
                }
                914 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofBytes(LengthPrefixed::read_from(s)?)
                }
                920 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofBool(Varint::read_from(s)?)
                }
                928 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofUint64(Varint::read_from(s)?)
                }
                941 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofFloat(Fixed32::read_from(s)?)
                }
                945 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofDouble(Fixed64::read_from(s)?)
                }
                952 => {
                    self.oneof_field =
                        TestAllTypesProto2_Oneof_Field::OneofEnum(Varint::read_from(s)?)
                }
                1611 => s.read_group(1612, |s| self.data_mut().merge_from(s))?,
                3208 => self.fieldname1 = Some(Varint::read_from(s)?),
                3216 => self.field_name2 = Some(Varint::read_from(s)?),
                3224 => self._field_name3 = Some(Varint::read_from(s)?),
                3232 => self.field__name4_ = Some(Varint::read_from(s)?),
                3240 => self.field0name5 = Some(Varint::read_from(s)?),
                3248 => self.field_0_name6 = Some(Varint::read_from(s)?),
                3256 => self.field_name7 = Some(Varint::read_from(s)?),
                3264 => self.field_name8 = Some(Varint::read_from(s)?),
                3272 => self.field_name9 = Some(Varint::read_from(s)?),
                3280 => self.field_name10 = Some(Varint::read_from(s)?),
                3288 => self.field_name11 = Some(Varint::read_from(s)?),
                3296 => self.field_name12 = Some(Varint::read_from(s)?),
                3304 => self.__field_name13 = Some(Varint::read_from(s)?),
                3312 => self.__field_name14 = Some(Varint::read_from(s)?),
                3320 => self.field__name15 = Some(Varint::read_from(s)?),
                3328 => self.field__name16 = Some(Varint::read_from(s)?),
                3336 => self.field_name17__ = Some(Varint::read_from(s)?),
                3344 => self.field_name18__ = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => {
                    if (960..=1615).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.optional_int32 {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.optional_int64 {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.optional_uint32 {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.optional_uint64 {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.optional_sint32 {
            s.write_tag(40)?;
            ZigZag::write_to(v, s)?;
        }
        if let Some(v) = self.optional_sint64 {
            s.write_tag(48)?;
            ZigZag::write_to(v, s)?;
        }
        if let Some(v) = self.optional_fixed32 {
            s.write_tag(61)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.optional_fixed64 {
            s.write_tag(65)?;
            Fixed64::write_to(v, s)?;
        }
        if let Some(v) = self.optional_sfixed32 {
            s.write_tag(77)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.optional_sfixed64 {
            s.write_tag(81)?;
            Fixed64::write_to(v, s)?;
        }
        if let Some(v) = self.optional_float {
            s.write_tag(93)?;
            Fixed32::write_to(v, s)?;
        }
        if let Some(v) = self.optional_double {
            s.write_tag(97)?;
            Fixed64::write_to(v, s)?;
        }
        if let Some(v) = self.optional_bool {
            s.write_tag(104)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_string {
            s.write_tag(114)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_bytes {
            s.write_tag(122)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_nested_message {
            s.write_tag(146)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_foreign_message {
            s.write_tag(154)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.optional_nested_enum {
            s.write_tag(168)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.optional_foreign_enum {
            s.write_tag(176)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_string_piece {
            s.write_tag(194)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_cord {
            s.write_tag(202)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.recursive_message {
            s.write_tag(218)?;
            LengthPrefixed::write_to(v.as_ref(), s)?;
        }
        if !self.repeated_int32.is_empty() {
            for i in &self.repeated_int32 {
                s.write_tag(248)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_int64.is_empty() {
            for i in &self.repeated_int64 {
                s.write_tag(256)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_uint32.is_empty() {
            for i in &self.repeated_uint32 {
                s.write_tag(264)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_uint64.is_empty() {
            for i in &self.repeated_uint64 {
                s.write_tag(272)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_sint32.is_empty() {
            for i in &self.repeated_sint32 {
                s.write_tag(280)?;
                ZigZag::write_to(*i, s)?;
            }
        }
        if !self.repeated_sint64.is_empty() {
            for i in &self.repeated_sint64 {
                s.write_tag(288)?;
                ZigZag::write_to(*i, s)?;
            }
        }
        if !self.repeated_fixed32.is_empty() {
            for i in &self.repeated_fixed32 {
                s.write_tag(301)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.repeated_fixed64.is_empty() {
            for i in &self.repeated_fixed64 {
                s.write_tag(305)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.repeated_sfixed32.is_empty() {
            for i in &self.repeated_sfixed32 {
                s.write_tag(317)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.repeated_sfixed64.is_empty() {
            for i in &self.repeated_sfixed64 {
                s.write_tag(321)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.repeated_float.is_empty() {
            for i in &self.repeated_float {
                s.write_tag(333)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.repeated_double.is_empty() {
            for i in &self.repeated_double {
                s.write_tag(337)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.repeated_bool.is_empty() {
            for i in &self.repeated_bool {
                s.write_tag(344)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_string.is_empty() {
            for i in &self.repeated_string {
                s.write_tag(354)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_bytes.is_empty() {
            for i in &self.repeated_bytes {
                s.write_tag(362)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_nested_message.is_empty() {
            for i in &self.repeated_nested_message {
                s.write_tag(386)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_foreign_message.is_empty() {
            for i in &self.repeated_foreign_message {
                s.write_tag(394)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_nested_enum.is_empty() {
            for i in &self.repeated_nested_enum {
                s.write_tag(408)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_foreign_enum.is_empty() {
            for i in &self.repeated_foreign_enum {
                s.write_tag(416)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.repeated_string_piece.is_empty() {
            for i in &self.repeated_string_piece {
                s.write_tag(434)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_cord.is_empty() {
            for i in &self.repeated_cord {
                s.write_tag(442)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.map_int32_int32 {
            for (key, val) in v {
                s.write_tag(450)?;
                let l = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_int64_int64 {
            for (key, val) in v {
                s.write_tag(458)?;
                let l = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_uint32_uint32 {
            for (key, val) in v {
                s.write_tag(466)?;
                let l = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_uint64_uint64 {
            for (key, val) in v {
                s.write_tag(474)?;
                let l = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_sint32_sint32 {
            for (key, val) in v {
                s.write_tag(482)?;
                let l = 1 + ZigZag::size(*key) + 1 + ZigZag::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                ZigZag::write_to(*key, s)?;
                s.write_tag(16)?;
                ZigZag::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_sint64_sint64 {
            for (key, val) in v {
                s.write_tag(490)?;
                let l = 1 + ZigZag::size(*key) + 1 + ZigZag::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                ZigZag::write_to(*key, s)?;
                s.write_tag(16)?;
                ZigZag::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_fixed32_fixed32 {
            for (key, val) in v {
                s.write_tag(498)?;
                let l = 1 + Fixed32::size(*key) + 1 + Fixed32::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(13)?;
                Fixed32::write_to(*key, s)?;
                s.write_tag(21)?;
                Fixed32::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_fixed64_fixed64 {
            for (key, val) in v {
                s.write_tag(506)?;
                let l = 1 + Fixed64::size(*key) + 1 + Fixed64::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(9)?;
                Fixed64::write_to(*key, s)?;
                s.write_tag(17)?;
                Fixed64::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_sfixed32_sfixed32 {
            for (key, val) in v {
                s.write_tag(514)?;
                let l = 1 + Fixed32::size(*key) + 1 + Fixed32::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(13)?;
                Fixed32::write_to(*key, s)?;
                s.write_tag(21)?;
                Fixed32::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_sfixed64_sfixed64 {
            for (key, val) in v {
                s.write_tag(522)?;
                let l = 1 + Fixed64::size(*key) + 1 + Fixed64::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(9)?;
                Fixed64::write_to(*key, s)?;
                s.write_tag(17)?;
                Fixed64::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_int32_float {
            for (key, val) in v {
                s.write_tag(530)?;
                let l = 1 + Varint::size(*key) + 1 + Fixed32::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(21)?;
                Fixed32::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_int32_double {
            for (key, val) in v {
                s.write_tag(538)?;
                let l = 1 + Varint::size(*key) + 1 + Fixed64::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(17)?;
                Fixed64::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_bool_bool {
            for (key, val) in v {
                s.write_tag(546)?;
                let l = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(8)?;
                Varint::write_to(*key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_string_string {
            for (key, val) in v {
                s.write_tag(554)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(18)?;
                LengthPrefixed::write_to(val, s)?;
            }
        }
        if let Some(v) = &self.map_string_bytes {
            for (key, val) in v {
                s.write_tag(562)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(18)?;
                LengthPrefixed::write_to(val, s)?;
            }
        }
        if let Some(v) = &self.map_string_nested_message {
            for (key, val) in v {
                s.write_tag(570)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(18)?;
                LengthPrefixed::write_to(val, s)?;
            }
        }
        if let Some(v) = &self.map_string_foreign_message {
            for (key, val) in v {
                s.write_tag(578)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(18)?;
                LengthPrefixed::write_to(val, s)?;
            }
        }
        if let Some(v) = &self.map_string_nested_enum {
            for (key, val) in v {
                s.write_tag(586)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if let Some(v) = &self.map_string_foreign_enum {
            for (key, val) in v {
                s.write_tag(594)?;
                let l = 1 + LengthPrefixed::size(key) + 1 + Varint::size(*val);
                Varint::write_to(l, s)?;
                s.write_tag(10)?;
                LengthPrefixed::write_to(key, s)?;
                s.write_tag(16)?;
                Varint::write_to(*val, s)?;
            }
        }
        if !self.packed_int32.is_empty() {
            s.write_tag(602)?;
            PackedArray::<Varint>::write_to(&self.packed_int32, s)?
        }
        if !self.packed_int64.is_empty() {
            s.write_tag(610)?;
            PackedArray::<Varint>::write_to(&self.packed_int64, s)?
        }
        if !self.packed_uint32.is_empty() {
            s.write_tag(618)?;
            PackedArray::<Varint>::write_to(&self.packed_uint32, s)?
        }
        if !self.packed_uint64.is_empty() {
            s.write_tag(626)?;
            PackedArray::<Varint>::write_to(&self.packed_uint64, s)?
        }
        if !self.packed_sint32.is_empty() {
            s.write_tag(634)?;
            PackedArray::<ZigZag>::write_to(&self.packed_sint32, s)?
        }
        if !self.packed_sint64.is_empty() {
            s.write_tag(642)?;
            PackedArray::<ZigZag>::write_to(&self.packed_sint64, s)?
        }
        if !self.packed_fixed32.is_empty() {
            s.write_tag(650)?;
            PackedArray::<Fixed32>::write_to(&self.packed_fixed32, s)?
        }
        if !self.packed_fixed64.is_empty() {
            s.write_tag(658)?;
            PackedArray::<Fixed64>::write_to(&self.packed_fixed64, s)?
        }
        if !self.packed_sfixed32.is_empty() {
            s.write_tag(666)?;
            PackedArray::<Fixed32>::write_to(&self.packed_sfixed32, s)?
        }
        if !self.packed_sfixed64.is_empty() {
            s.write_tag(674)?;
            PackedArray::<Fixed64>::write_to(&self.packed_sfixed64, s)?
        }
        if !self.packed_float.is_empty() {
            s.write_tag(682)?;
            PackedArray::<Fixed32>::write_to(&self.packed_float, s)?
        }
        if !self.packed_double.is_empty() {
            s.write_tag(690)?;
            PackedArray::<Fixed64>::write_to(&self.packed_double, s)?
        }
        if !self.packed_bool.is_empty() {
            s.write_tag(698)?;
            PackedArray::<Varint>::write_to(&self.packed_bool, s)?
        }
        if !self.packed_nested_enum.is_empty() {
            s.write_tag(706)?;
            PackedArray::<Varint>::write_to(&self.packed_nested_enum, s)?
        }
        if !self.unpacked_int32.is_empty() {
            for i in &self.unpacked_int32 {
                s.write_tag(712)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.unpacked_int64.is_empty() {
            for i in &self.unpacked_int64 {
                s.write_tag(720)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.unpacked_uint32.is_empty() {
            for i in &self.unpacked_uint32 {
                s.write_tag(728)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.unpacked_uint64.is_empty() {
            for i in &self.unpacked_uint64 {
                s.write_tag(736)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.unpacked_sint32.is_empty() {
            for i in &self.unpacked_sint32 {
                s.write_tag(744)?;
                ZigZag::write_to(*i, s)?;
            }
        }
        if !self.unpacked_sint64.is_empty() {
            for i in &self.unpacked_sint64 {
                s.write_tag(752)?;
                ZigZag::write_to(*i, s)?;
            }
        }
        if !self.unpacked_fixed32.is_empty() {
            for i in &self.unpacked_fixed32 {
                s.write_tag(765)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.unpacked_fixed64.is_empty() {
            for i in &self.unpacked_fixed64 {
                s.write_tag(769)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.unpacked_sfixed32.is_empty() {
            for i in &self.unpacked_sfixed32 {
                s.write_tag(781)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.unpacked_sfixed64.is_empty() {
            for i in &self.unpacked_sfixed64 {
                s.write_tag(785)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.unpacked_float.is_empty() {
            for i in &self.unpacked_float {
                s.write_tag(797)?;
                Fixed32::write_to(*i, s)?;
            }
        }
        if !self.unpacked_double.is_empty() {
            for i in &self.unpacked_double {
                s.write_tag(801)?;
                Fixed64::write_to(*i, s)?;
            }
        }
        if !self.unpacked_bool.is_empty() {
            for i in &self.unpacked_bool {
                s.write_tag(808)?;
                Varint::write_to(*i, s)?;
            }
        }
        if !self.unpacked_nested_enum.is_empty() {
            for i in &self.unpacked_nested_enum {
                s.write_tag(816)?;
                Varint::write_to(*i, s)?;
            }
        }
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::None => (),
            TestAllTypesProto2_Oneof_Field::OneofUint32(v) => {
                s.write_tag(888)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofNestedMessage(v) => {
                s.write_tag(898)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofString(v) => {
                s.write_tag(906)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofBytes(v) => {
                s.write_tag(914)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofBool(v) => {
                s.write_tag(920)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofUint64(v) => {
                s.write_tag(928)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofFloat(v) => {
                s.write_tag(941)?;
                Fixed32::write_to(*v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofDouble(v) => {
                s.write_tag(945)?;
                Fixed64::write_to(*v, s)?;
            }
            TestAllTypesProto2_Oneof_Field::OneofEnum(v) => {
                s.write_tag(952)?;
                Varint::write_to(*v, s)?;
            }
        }
        if let Some(v) = &self.data {
            s.write_tag(1611)?;
            v.write_to_uncheck(s)?;
            s.write_tag(1612)?;
        }
        if let Some(v) = self.fieldname1 {
            s.write_tag(3208)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name2 {
            s.write_tag(3216)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self._field_name3 {
            s.write_tag(3224)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field__name4_ {
            s.write_tag(3232)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field0name5 {
            s.write_tag(3240)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_0_name6 {
            s.write_tag(3248)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name7 {
            s.write_tag(3256)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name8 {
            s.write_tag(3264)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name9 {
            s.write_tag(3272)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name10 {
            s.write_tag(3280)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name11 {
            s.write_tag(3288)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name12 {
            s.write_tag(3296)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.__field_name13 {
            s.write_tag(3304)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.__field_name14 {
            s.write_tag(3312)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field__name15 {
            s.write_tag(3320)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field__name16 {
            s.write_tag(3328)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name17__ {
            s.write_tag(3336)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.field_name18__ {
            s.write_tag(3344)?;
            Varint::write_to(v, s)?;
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.optional_int32 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.optional_int64 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.optional_uint32 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.optional_uint64 {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.optional_sint32 {
            l += 1 + ZigZag::size(v);
        }
        if let Some(v) = self.optional_sint64 {
            l += 1 + ZigZag::size(v);
        }
        if let Some(v) = self.optional_fixed32 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.optional_fixed64 {
            l += 1 + Fixed64::size(v);
        }
        if let Some(v) = self.optional_sfixed32 {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.optional_sfixed64 {
            l += 1 + Fixed64::size(v);
        }
        if let Some(v) = self.optional_float {
            l += 1 + Fixed32::size(v);
        }
        if let Some(v) = self.optional_double {
            l += 1 + Fixed64::size(v);
        }
        if let Some(v) = self.optional_bool {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.optional_string {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_bytes {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_nested_message {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_foreign_message {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.optional_nested_enum {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.optional_foreign_enum {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.optional_string_piece {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_cord {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.recursive_message {
            l += 2 + LengthPrefixed::size(v.as_ref());
        }
        if !self.repeated_int32.is_empty() {
            l += 2 * self.repeated_int32.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_int32);
        }
        if !self.repeated_int64.is_empty() {
            l += 2 * self.repeated_int64.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_int64);
        }
        if !self.repeated_uint32.is_empty() {
            l += 2 * self.repeated_uint32.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_uint32);
        }
        if !self.repeated_uint64.is_empty() {
            l += 2 * self.repeated_uint64.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_uint64);
        }
        if !self.repeated_sint32.is_empty() {
            l += 2 * self.repeated_sint32.len() as u64
                + CopyArray::<ZigZag>::size(&self.repeated_sint32);
        }
        if !self.repeated_sint64.is_empty() {
            l += 2 * self.repeated_sint64.len() as u64
                + CopyArray::<ZigZag>::size(&self.repeated_sint64);
        }
        if !self.repeated_fixed32.is_empty() {
            l += 2 * self.repeated_fixed32.len() as u64
                + CopyArray::<Fixed32>::size(&self.repeated_fixed32);
        }
        if !self.repeated_fixed64.is_empty() {
            l += 2 * self.repeated_fixed64.len() as u64
                + CopyArray::<Fixed64>::size(&self.repeated_fixed64);
        }
        if !self.repeated_sfixed32.is_empty() {
            l += 2 * self.repeated_sfixed32.len() as u64
                + CopyArray::<Fixed32>::size(&self.repeated_sfixed32);
        }
        if !self.repeated_sfixed64.is_empty() {
            l += 2 * self.repeated_sfixed64.len() as u64
                + CopyArray::<Fixed64>::size(&self.repeated_sfixed64);
        }
        if !self.repeated_float.is_empty() {
            l += 2 * self.repeated_float.len() as u64
                + CopyArray::<Fixed32>::size(&self.repeated_float);
        }
        if !self.repeated_double.is_empty() {
            l += 2 * self.repeated_double.len() as u64
                + CopyArray::<Fixed64>::size(&self.repeated_double);
        }
        if !self.repeated_bool.is_empty() {
            l += 2 * self.repeated_bool.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_bool);
        }
        if !self.repeated_string.is_empty() {
            l += 2 * self.repeated_string.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_string);
        }
        if !self.repeated_bytes.is_empty() {
            l += 2 * self.repeated_bytes.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_bytes);
        }
        if !self.repeated_nested_message.is_empty() {
            l += 2 * self.repeated_nested_message.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_nested_message);
        }
        if !self.repeated_foreign_message.is_empty() {
            l += 2 * self.repeated_foreign_message.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_foreign_message);
        }
        if !self.repeated_nested_enum.is_empty() {
            l += 2 * self.repeated_nested_enum.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_nested_enum);
        }
        if !self.repeated_foreign_enum.is_empty() {
            l += 2 * self.repeated_foreign_enum.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_foreign_enum);
        }
        if !self.repeated_string_piece.is_empty() {
            l += 2 * self.repeated_string_piece.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_string_piece);
        }
        if !self.repeated_cord.is_empty() {
            l += 2 * self.repeated_cord.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_cord);
        }
        if let Some(v) = &self.map_int32_int32 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_int64_int64 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_uint32_uint32 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_uint64_uint64 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_sint32_sint32 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + ZigZag::size(*key) + 1 + ZigZag::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_sint64_sint64 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + ZigZag::size(*key) + 1 + ZigZag::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_fixed32_fixed32 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Fixed32::size(*key) + 1 + Fixed32::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_fixed64_fixed64 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Fixed64::size(*key) + 1 + Fixed64::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_sfixed32_sfixed32 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Fixed32::size(*key) + 1 + Fixed32::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_sfixed64_sfixed64 {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Fixed64::size(*key) + 1 + Fixed64::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_int32_float {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Fixed32::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_int32_double {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Fixed64::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_bool_bool {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + Varint::size(*key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_string {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_bytes {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_nested_message {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_foreign_message {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + LengthPrefixed::size(val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_nested_enum {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if let Some(v) = &self.map_string_foreign_enum {
            l += 2 * v.len() as u64;
            for (key, val) in v {
                let el = 1 + LengthPrefixed::size(key) + 1 + Varint::size(*val);
                l += Varint::size(el) + el;
            }
        }
        if !self.packed_int32.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_int32);
        }
        if !self.packed_int64.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_int64);
        }
        if !self.packed_uint32.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_uint32);
        }
        if !self.packed_uint64.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_uint64);
        }
        if !self.packed_sint32.is_empty() {
            l += 2 + PackedArray::<ZigZag>::size(&self.packed_sint32);
        }
        if !self.packed_sint64.is_empty() {
            l += 2 + PackedArray::<ZigZag>::size(&self.packed_sint64);
        }
        if !self.packed_fixed32.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.packed_fixed32);
        }
        if !self.packed_fixed64.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.packed_fixed64);
        }
        if !self.packed_sfixed32.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.packed_sfixed32);
        }
        if !self.packed_sfixed64.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.packed_sfixed64);
        }
        if !self.packed_float.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.packed_float);
        }
        if !self.packed_double.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.packed_double);
        }
        if !self.packed_bool.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_bool);
        }
        if !self.packed_nested_enum.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.packed_nested_enum);
        }
        if !self.unpacked_int32.is_empty() {
            l += 2 * self.unpacked_int32.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_int32);
        }
        if !self.unpacked_int64.is_empty() {
            l += 2 * self.unpacked_int64.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_int64);
        }
        if !self.unpacked_uint32.is_empty() {
            l += 2 * self.unpacked_uint32.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_uint32);
        }
        if !self.unpacked_uint64.is_empty() {
            l += 2 * self.unpacked_uint64.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_uint64);
        }
        if !self.unpacked_sint32.is_empty() {
            l += 2 * self.unpacked_sint32.len() as u64
                + CopyArray::<ZigZag>::size(&self.unpacked_sint32);
        }
        if !self.unpacked_sint64.is_empty() {
            l += 2 * self.unpacked_sint64.len() as u64
                + CopyArray::<ZigZag>::size(&self.unpacked_sint64);
        }
        if !self.unpacked_fixed32.is_empty() {
            l += 2 * self.unpacked_fixed32.len() as u64
                + CopyArray::<Fixed32>::size(&self.unpacked_fixed32);
        }
        if !self.unpacked_fixed64.is_empty() {
            l += 2 * self.unpacked_fixed64.len() as u64
                + CopyArray::<Fixed64>::size(&self.unpacked_fixed64);
        }
        if !self.unpacked_sfixed32.is_empty() {
            l += 2 * self.unpacked_sfixed32.len() as u64
                + CopyArray::<Fixed32>::size(&self.unpacked_sfixed32);
        }
        if !self.unpacked_sfixed64.is_empty() {
            l += 2 * self.unpacked_sfixed64.len() as u64
                + CopyArray::<Fixed64>::size(&self.unpacked_sfixed64);
        }
        if !self.unpacked_float.is_empty() {
            l += 2 * self.unpacked_float.len() as u64
                + CopyArray::<Fixed32>::size(&self.unpacked_float);
        }
        if !self.unpacked_double.is_empty() {
            l += 2 * self.unpacked_double.len() as u64
                + CopyArray::<Fixed64>::size(&self.unpacked_double);
        }
        if !self.unpacked_bool.is_empty() {
            l += 2 * self.unpacked_bool.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_bool);
        }
        if !self.unpacked_nested_enum.is_empty() {
            l += 2 * self.unpacked_nested_enum.len() as u64
                + CopyArray::<Varint>::size(&self.unpacked_nested_enum);
        }
        match &self.oneof_field {
            TestAllTypesProto2_Oneof_Field::None => (),
            TestAllTypesProto2_Oneof_Field::OneofUint32(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto2_Oneof_Field::OneofNestedMessage(v) => {
                l += 2 + LengthPrefixed::size(v)
            }
            TestAllTypesProto2_Oneof_Field::OneofString(v) => l += 2 + LengthPrefixed::size(v),
            TestAllTypesProto2_Oneof_Field::OneofBytes(v) => l += 2 + LengthPrefixed::size(v),
            TestAllTypesProto2_Oneof_Field::OneofBool(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto2_Oneof_Field::OneofUint64(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto2_Oneof_Field::OneofFloat(v) => l += 2 + Fixed32::size(*v),
            TestAllTypesProto2_Oneof_Field::OneofDouble(v) => l += 2 + Fixed64::size(*v),
            TestAllTypesProto2_Oneof_Field::OneofEnum(v) => l += 2 + Varint::size(*v),
        }
        if let Some(v) = &self.data {
            l += 4 + v.size();
        }
        if let Some(v) = self.fieldname1 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name2 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self._field_name3 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field__name4_ {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field0name5 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_0_name6 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name7 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name8 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name9 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name10 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name11 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name12 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.__field_name13 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.__field_name14 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field__name15 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field__name16 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name17__ {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = self.field_name18__ {
            l += 2 + Varint::size(v);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.size();
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
impl pecan::DefaultInstance for TestAllTypesProto2 {
    fn default_instance() -> &'static TestAllTypesProto2 {
        static DEFAULT: TestAllTypesProto2 = TestAllTypesProto2::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto2 {
    #[inline]
    fn default() -> TestAllTypesProto2 {
        TestAllTypesProto2::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForeignMessageProto2 {
    pub c: Option<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl ForeignMessageProto2 {
    pub const fn new() -> ForeignMessageProto2 {
        ForeignMessageProto2 {
            c: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn c(&self) -> i32 {
        self.c.unwrap_or_default()
    }
    pub fn c_mut(&mut self) -> &mut i32 {
        self.c.get_or_insert_with(Default::default)
    }
    pub fn set_c(&mut self, val: i32) {
        self.c = Some(val);
    }
}
impl pecan::Message for ForeignMessageProto2 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.c = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.c {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.c {
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
impl pecan::DefaultInstance for ForeignMessageProto2 {
    fn default_instance() -> &'static ForeignMessageProto2 {
        static DEFAULT: ForeignMessageProto2 = ForeignMessageProto2::new();
        &DEFAULT
    }
}
impl Default for ForeignMessageProto2 {
    #[inline]
    fn default() -> ForeignMessageProto2 {
        ForeignMessageProto2::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnknownToTestAllTypes_OptionalGroup {
    pub a: Option<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl UnknownToTestAllTypes_OptionalGroup {
    pub const fn new() -> UnknownToTestAllTypes_OptionalGroup {
        UnknownToTestAllTypes_OptionalGroup {
            a: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn a(&self) -> i32 {
        self.a.unwrap_or_default()
    }
    pub fn a_mut(&mut self) -> &mut i32 {
        self.a.get_or_insert_with(Default::default)
    }
    pub fn set_a(&mut self, val: i32) {
        self.a = Some(val);
    }
}
impl pecan::Message for UnknownToTestAllTypes_OptionalGroup {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.a = Some(Varint::read_from(s)?),
                0 | 8036 => {
                    s.set_last_tag(8036);
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
        if let Some(v) = self.a {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.a {
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
impl pecan::DefaultInstance for UnknownToTestAllTypes_OptionalGroup {
    fn default_instance() -> &'static UnknownToTestAllTypes_OptionalGroup {
        static DEFAULT: UnknownToTestAllTypes_OptionalGroup =
            UnknownToTestAllTypes_OptionalGroup::new();
        &DEFAULT
    }
}
impl Default for UnknownToTestAllTypes_OptionalGroup {
    #[inline]
    fn default() -> UnknownToTestAllTypes_OptionalGroup {
        UnknownToTestAllTypes_OptionalGroup::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnknownToTestAllTypes {
    pub optional_int32: Option<i32>,
    pub optional_string: Option<String>,
    pub nested_message: Option<ForeignMessageProto2>,
    pub optionalgroup: Option<UnknownToTestAllTypes_OptionalGroup>,
    pub optional_bool: Option<bool>,
    pub repeated_int32: Vec<i32>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl UnknownToTestAllTypes {
    pub const fn new() -> UnknownToTestAllTypes {
        UnknownToTestAllTypes {
            optional_int32: None,
            optional_string: None,
            nested_message: None,
            optionalgroup: None,
            optional_bool: None,
            repeated_int32: Vec::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn optional_int32(&self) -> i32 {
        self.optional_int32.unwrap_or_default()
    }
    pub fn optional_int32_mut(&mut self) -> &mut i32 {
        self.optional_int32.get_or_insert_with(Default::default)
    }
    pub fn set_optional_int32(&mut self, val: i32) {
        self.optional_int32 = Some(val);
    }
    pub fn optional_string(&self) -> &String {
        match &self.optional_string {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn optional_string_mut(&mut self) -> &mut String {
        self.optional_string.get_or_insert_with(Default::default)
    }
    pub fn set_optional_string(&mut self, val: String) {
        self.optional_string = Some(val);
    }
    pub fn nested_message(&self) -> &ForeignMessageProto2 {
        match &self.nested_message {
            Some(v) => v,
            _ => ForeignMessageProto2::default_instance(),
        }
    }
    pub fn nested_message_mut(&mut self) -> &mut ForeignMessageProto2 {
        self.nested_message.get_or_insert_with(Default::default)
    }
    pub fn set_nested_message(&mut self, val: ForeignMessageProto2) {
        self.nested_message = Some(val);
    }
    pub fn optionalgroup(&self) -> &UnknownToTestAllTypes_OptionalGroup {
        match &self.optionalgroup {
            Some(v) => v,
            _ => UnknownToTestAllTypes_OptionalGroup::default_instance(),
        }
    }
    pub fn optionalgroup_mut(&mut self) -> &mut UnknownToTestAllTypes_OptionalGroup {
        self.optionalgroup.get_or_insert_with(Default::default)
    }
    pub fn set_optionalgroup(&mut self, val: UnknownToTestAllTypes_OptionalGroup) {
        self.optionalgroup = Some(val);
    }
    pub fn optional_bool(&self) -> bool {
        self.optional_bool.unwrap_or_default()
    }
    pub fn optional_bool_mut(&mut self) -> &mut bool {
        self.optional_bool.get_or_insert_with(Default::default)
    }
    pub fn set_optional_bool(&mut self, val: bool) {
        self.optional_bool = Some(val);
    }
}
impl pecan::Message for UnknownToTestAllTypes {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8008 => self.optional_int32 = Some(Varint::read_from(s)?),
                8018 => self.optional_string = Some(LengthPrefixed::read_from(s)?),
                8026 => LengthPrefixed::merge_from(self.nested_message_mut(), s)?,
                8035 => s.read_group(8036, |s| self.optionalgroup_mut().merge_from(s))?,
                8048 => self.optional_bool = Some(Varint::read_from(s)?),
                8088 => CopyArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                8090 => PackedArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if let Some(v) = self.optional_int32 {
            s.write_tag(8008)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_string {
            s.write_tag(8018)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.nested_message {
            s.write_tag(8026)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optionalgroup {
            s.write_tag(8035)?;
            v.write_to_uncheck(s)?;
            s.write_tag(8036)?;
        }
        if let Some(v) = self.optional_bool {
            s.write_tag(8048)?;
            Varint::write_to(v, s)?;
        }
        if !self.repeated_int32.is_empty() {
            for i in &self.repeated_int32 {
                s.write_tag(8088)?;
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
        if let Some(v) = self.optional_int32 {
            l += 2 + Varint::size(v);
        }
        if let Some(v) = &self.optional_string {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.nested_message {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optionalgroup {
            l += 4 + v.size();
        }
        if let Some(v) = self.optional_bool {
            l += 2 + Varint::size(v);
        }
        if !self.repeated_int32.is_empty() {
            l += 2 * self.repeated_int32.len() as u64
                + CopyArray::<Varint>::size(&self.repeated_int32);
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
impl pecan::DefaultInstance for UnknownToTestAllTypes {
    fn default_instance() -> &'static UnknownToTestAllTypes {
        static DEFAULT: UnknownToTestAllTypes = UnknownToTestAllTypes::new();
        &DEFAULT
    }
}
impl Default for UnknownToTestAllTypes {
    #[inline]
    fn default() -> UnknownToTestAllTypes {
        UnknownToTestAllTypes::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n*google/protobuf/test_messages_proto2.proto\x12\x1Dprotobuf_test_messages.proto2\x1A\x13pecan/options.proto\"\xA2K\n\x12TestAllTypesProto2\x12%\n\x0Eoptional_int32\x18\x01 \x01(\x05R\roptionalInt32\x12%\n\x0Eoptional_int64\x18\x02 \x01(\x03R\roptionalInt64\x12'\n\x0Foptional_uint32\x18\x03 \x01(\rR\x0EoptionalUint32\x12'\n\x0Foptional_uint64\x18\x04 \x01(\x04R\x0EoptionalUint64\x12'\n\x0Foptional_sint32\x18\x05 \x01(\x11R\x0EoptionalSint32\x12'\n\x0Foptional_sint64\x18\x06 \x01(\x12R\x0EoptionalSint64\x12)\n\x10optional_fixed32\x18\x07 \x01(\x07R\x0FoptionalFixed32\x12)\n\x10optional_fixed64\x18\x08 \x01(\x06R\x0FoptionalFixed64\x12+\n\x11optional_sfixed32\x18\t \x01(\x0FR\x10optionalSfixed32\x12+\n\x11optional_sfixed64\x18\n \x01(\x10R\x10optionalSfixed64\x12%\n\x0Eoptional_float\x18\x0B \x01(\x02R\roptionalFloat\x12'\n\x0Foptional_double\x18\x0C \x01(\x01R\x0EoptionalDouble\x12#\n\roptional_bool\x18\r \x01(\x08R\x0CoptionalBool\x12'\n\x0Foptional_string\x18\x0E \x01(\tR\x0EoptionalString\x12%\n\x0Eoptional_bytes\x18\x0F \x01(\x0CR\roptionalBytes\x12w\n\x17optional_nested_message\x18\x12 \x01(\x0B2?.protobuf_test_messages.proto2.TestAllTypesProto2.NestedMessageR\x15optionalNestedMessage\x12m\n\x18optional_foreign_message\x18\x13 \x01(\x0B23.protobuf_test_messages.proto2.ForeignMessageProto2R\x16optionalForeignMessage\x12n\n\x14optional_nested_enum\x18\x15 \x01(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumR\x12optionalNestedEnum\x12d\n\x15optional_foreign_enum\x18\x16 \x01(\x0E20.protobuf_test_messages.proto2.ForeignEnumProto2R\x13optionalForeignEnum\x126\n\x15optional_string_piece\x18\x18 \x01(\tB\x02\x08\x02R\x13optionalStringPiece\x12'\n\roptional_cord\x18\x19 \x01(\tB\x02\x08\x01R\x0CoptionalCord\x12f\n\x11recursive_message\x18\x1B \x01(\x0B21.protobuf_test_messages.proto2.TestAllTypesProto2B\x06\xEA\x91\x01\x02\x08\x01R\x10recursiveMessage\x12%\n\x0Erepeated_int32\x18\x1F \x03(\x05R\rrepeatedInt32\x12%\n\x0Erepeated_int64\x18  \x03(\x03R\rrepeatedInt64\x12'\n\x0Frepeated_uint32\x18! \x03(\rR\x0ErepeatedUint32\x12'\n\x0Frepeated_uint64\x18\" \x03(\x04R\x0ErepeatedUint64\x12'\n\x0Frepeated_sint32\x18# \x03(\x11R\x0ErepeatedSint32\x12'\n\x0Frepeated_sint64\x18$ \x03(\x12R\x0ErepeatedSint64\x12)\n\x10repeated_fixed32\x18% \x03(\x07R\x0FrepeatedFixed32\x12)\n\x10repeated_fixed64\x18& \x03(\x06R\x0FrepeatedFixed64\x12+\n\x11repeated_sfixed32\x18' \x03(\x0FR\x10repeatedSfixed32\x12+\n\x11repeated_sfixed64\x18( \x03(\x10R\x10repeatedSfixed64\x12%\n\x0Erepeated_float\x18) \x03(\x02R\rrepeatedFloat\x12'\n\x0Frepeated_double\x18* \x03(\x01R\x0ErepeatedDouble\x12#\n\rrepeated_bool\x18+ \x03(\x08R\x0CrepeatedBool\x12'\n\x0Frepeated_string\x18, \x03(\tR\x0ErepeatedString\x12%\n\x0Erepeated_bytes\x18- \x03(\x0CR\rrepeatedBytes\x12w\n\x17repeated_nested_message\x180 \x03(\x0B2?.protobuf_test_messages.proto2.TestAllTypesProto2.NestedMessageR\x15repeatedNestedMessage\x12m\n\x18repeated_foreign_message\x181 \x03(\x0B23.protobuf_test_messages.proto2.ForeignMessageProto2R\x16repeatedForeignMessage\x12n\n\x14repeated_nested_enum\x183 \x03(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumR\x12repeatedNestedEnum\x12d\n\x15repeated_foreign_enum\x184 \x03(\x0E20.protobuf_test_messages.proto2.ForeignEnumProto2R\x13repeatedForeignEnum\x126\n\x15repeated_string_piece\x186 \x03(\tB\x02\x08\x02R\x13repeatedStringPiece\x12'\n\rrepeated_cord\x187 \x03(\tB\x02\x08\x01R\x0CrepeatedCord\x12%\n\x0Cpacked_int32\x18K \x03(\x05B\x02\x10\x01R\x0BpackedInt32\x12%\n\x0Cpacked_int64\x18L \x03(\x03B\x02\x10\x01R\x0BpackedInt64\x12'\n\rpacked_uint32\x18M \x03(\rB\x02\x10\x01R\x0CpackedUint32\x12'\n\rpacked_uint64\x18N \x03(\x04B\x02\x10\x01R\x0CpackedUint64\x12'\n\rpacked_sint32\x18O \x03(\x11B\x02\x10\x01R\x0CpackedSint32\x12'\n\rpacked_sint64\x18P \x03(\x12B\x02\x10\x01R\x0CpackedSint64\x12)\n\x0Epacked_fixed32\x18Q \x03(\x07B\x02\x10\x01R\rpackedFixed32\x12)\n\x0Epacked_fixed64\x18R \x03(\x06B\x02\x10\x01R\rpackedFixed64\x12+\n\x0Fpacked_sfixed32\x18S \x03(\x0FB\x02\x10\x01R\x0EpackedSfixed32\x12+\n\x0Fpacked_sfixed64\x18T \x03(\x10B\x02\x10\x01R\x0EpackedSfixed64\x12%\n\x0Cpacked_float\x18U \x03(\x02B\x02\x10\x01R\x0BpackedFloat\x12'\n\rpacked_double\x18V \x03(\x01B\x02\x10\x01R\x0CpackedDouble\x12#\n\x0Bpacked_bool\x18W \x03(\x08B\x02\x10\x01R\npackedBool\x12n\n\x12packed_nested_enum\x18X \x03(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumB\x02\x10\x01R\x10packedNestedEnum\x12)\n\x0Eunpacked_int32\x18Y \x03(\x05B\x02\x10\0R\runpackedInt32\x12)\n\x0Eunpacked_int64\x18Z \x03(\x03B\x02\x10\0R\runpackedInt64\x12+\n\x0Funpacked_uint32\x18[ \x03(\rB\x02\x10\0R\x0EunpackedUint32\x12+\n\x0Funpacked_uint64\x18\\ \x03(\x04B\x02\x10\0R\x0EunpackedUint64\x12+\n\x0Funpacked_sint32\x18] \x03(\x11B\x02\x10\0R\x0EunpackedSint32\x12+\n\x0Funpacked_sint64\x18^ \x03(\x12B\x02\x10\0R\x0EunpackedSint64\x12-\n\x10unpacked_fixed32\x18_ \x03(\x07B\x02\x10\0R\x0FunpackedFixed32\x12-\n\x10unpacked_fixed64\x18` \x03(\x06B\x02\x10\0R\x0FunpackedFixed64\x12/\n\x11unpacked_sfixed32\x18a \x03(\x0FB\x02\x10\0R\x10unpackedSfixed32\x12/\n\x11unpacked_sfixed64\x18b \x03(\x10B\x02\x10\0R\x10unpackedSfixed64\x12)\n\x0Eunpacked_float\x18c \x03(\x02B\x02\x10\0R\runpackedFloat\x12+\n\x0Funpacked_double\x18d \x03(\x01B\x02\x10\0R\x0EunpackedDouble\x12'\n\runpacked_bool\x18e \x03(\x08B\x02\x10\0R\x0CunpackedBool\x12r\n\x14unpacked_nested_enum\x18f \x03(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumB\x02\x10\0R\x12unpackedNestedEnum\x12l\n\x0Fmap_int32_int32\x188 \x03(\x0B2D.protobuf_test_messages.proto2.TestAllTypesProto2.MapInt32Int32EntryR\rmapInt32Int32\x12l\n\x0Fmap_int64_int64\x189 \x03(\x0B2D.protobuf_test_messages.proto2.TestAllTypesProto2.MapInt64Int64EntryR\rmapInt64Int64\x12r\n\x11map_uint32_uint32\x18: \x03(\x0B2F.protobuf_test_messages.proto2.TestAllTypesProto2.MapUint32Uint32EntryR\x0FmapUint32Uint32\x12r\n\x11map_uint64_uint64\x18; \x03(\x0B2F.protobuf_test_messages.proto2.TestAllTypesProto2.MapUint64Uint64EntryR\x0FmapUint64Uint64\x12r\n\x11map_sint32_sint32\x18< \x03(\x0B2F.protobuf_test_messages.proto2.TestAllTypesProto2.MapSint32Sint32EntryR\x0FmapSint32Sint32\x12r\n\x11map_sint64_sint64\x18= \x03(\x0B2F.protobuf_test_messages.proto2.TestAllTypesProto2.MapSint64Sint64EntryR\x0FmapSint64Sint64\x12x\n\x13map_fixed32_fixed32\x18> \x03(\x0B2H.protobuf_test_messages.proto2.TestAllTypesProto2.MapFixed32Fixed32EntryR\x11mapFixed32Fixed32\x12x\n\x13map_fixed64_fixed64\x18? \x03(\x0B2H.protobuf_test_messages.proto2.TestAllTypesProto2.MapFixed64Fixed64EntryR\x11mapFixed64Fixed64\x12~\n\x15map_sfixed32_sfixed32\x18@ \x03(\x0B2J.protobuf_test_messages.proto2.TestAllTypesProto2.MapSfixed32Sfixed32EntryR\x13mapSfixed32Sfixed32\x12~\n\x15map_sfixed64_sfixed64\x18A \x03(\x0B2J.protobuf_test_messages.proto2.TestAllTypesProto2.MapSfixed64Sfixed64EntryR\x13mapSfixed64Sfixed64\x12l\n\x0Fmap_int32_float\x18B \x03(\x0B2D.protobuf_test_messages.proto2.TestAllTypesProto2.MapInt32FloatEntryR\rmapInt32Float\x12o\n\x10map_int32_double\x18C \x03(\x0B2E.protobuf_test_messages.proto2.TestAllTypesProto2.MapInt32DoubleEntryR\x0EmapInt32Double\x12f\n\rmap_bool_bool\x18D \x03(\x0B2B.protobuf_test_messages.proto2.TestAllTypesProto2.MapBoolBoolEntryR\x0BmapBoolBool\x12r\n\x11map_string_string\x18E \x03(\x0B2F.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringStringEntryR\x0FmapStringString\x12o\n\x10map_string_bytes\x18F \x03(\x0B2E.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringBytesEntryR\x0EmapStringBytes\x12\x88\x01\n\x19map_string_nested_message\x18G \x03(\x0B2M.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringNestedMessageEntryR\x16mapStringNestedMessage\x12\x8B\x01\n\x1Amap_string_foreign_message\x18H \x03(\x0B2N.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringForeignMessageEntryR\x17mapStringForeignMessage\x12\x7F\n\x16map_string_nested_enum\x18I \x03(\x0B2J.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringNestedEnumEntryR\x13mapStringNestedEnum\x12\x82\x01\n\x17map_string_foreign_enum\x18J \x03(\x0B2K.protobuf_test_messages.proto2.TestAllTypesProto2.MapStringForeignEnumEntryR\x14mapStringForeignEnum\x12#\n\x0Coneof_uint32\x18o \x01(\rH\0R\x0BoneofUint32\x12s\n\x14oneof_nested_message\x18p \x01(\x0B2?.protobuf_test_messages.proto2.TestAllTypesProto2.NestedMessageH\0R\x12oneofNestedMessage\x12#\n\x0Coneof_string\x18q \x01(\tH\0R\x0BoneofString\x12!\n\x0Boneof_bytes\x18r \x01(\x0CH\0R\noneofBytes\x12\x1F\n\noneof_bool\x18s \x01(\x08H\0R\toneofBool\x12#\n\x0Coneof_uint64\x18t \x01(\x04H\0R\x0BoneofUint64\x12!\n\x0Boneof_float\x18u \x01(\x02H\0R\noneofFloat\x12#\n\x0Coneof_double\x18v \x01(\x01H\0R\x0BoneofDouble\x12]\n\noneof_enum\x18w \x01(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumH\0R\toneofEnum\x12K\n\x04data\x18\xC9\x01 \x01(\n26.protobuf_test_messages.proto2.TestAllTypesProto2.DataR\x04data\x12\x1F\n\nfieldname1\x18\x91\x03 \x01(\x05R\nfieldname1\x12 \n\x0Bfield_name2\x18\x92\x03 \x01(\x05R\nfieldName2\x12!\n\x0C_field_name3\x18\x93\x03 \x01(\x05R\nFieldName3\x12\"\n\rfield__name4_\x18\x94\x03 \x01(\x05R\nfieldName4\x12!\n\x0Bfield0name5\x18\x95\x03 \x01(\x05R\x0Bfield0name5\x12#\n\rfield_0_name6\x18\x96\x03 \x01(\x05R\x0Bfield0Name6\x12\x1F\n\nfieldName7\x18\x97\x03 \x01(\x05R\nfieldName7\x12\x1F\n\nFieldName8\x18\x98\x03 \x01(\x05R\nFieldName8\x12 \n\x0Bfield_Name9\x18\x99\x03 \x01(\x05R\nfieldName9\x12\"\n\x0CField_Name10\x18\x9A\x03 \x01(\x05R\x0BFieldName10\x12\"\n\x0CFIELD_NAME11\x18\x9B\x03 \x01(\x05R\x0BFIELDNAME11\x12\"\n\x0CFIELD_name12\x18\x9C\x03 \x01(\x05R\x0BFIELDName12\x12$\n\x0E__field_name13\x18\x9D\x03 \x01(\x05R\x0BFieldName13\x12$\n\x0E__Field_name14\x18\x9E\x03 \x01(\x05R\x0BFieldName14\x12#\n\rfield__name15\x18\x9F\x03 \x01(\x05R\x0BfieldName15\x12#\n\rfield__Name16\x18\xA0\x03 \x01(\x05R\x0BfieldName16\x12$\n\x0Efield_name17__\x18\xA1\x03 \x01(\x05R\x0BfieldName17\x12$\n\x0EField_name18__\x18\xA2\x03 \x01(\x05R\x0BFieldName18\x1Az\n\rNestedMessage\x12\x0C\n\x01a\x18\x01 \x01(\x05R\x01a\x12[\n\x0Bcorecursive\x18\x02 \x01(\x0B21.protobuf_test_messages.proto2.TestAllTypesProto2B\x06\xEA\x91\x01\x02\x08\x01R\x0Bcorecursive\x1A@\n\x12MapInt32Int32Entry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x05R\x05value:\x028\x01\x1A@\n\x12MapInt64Int64Entry\x12\x10\n\x03key\x18\x01 \x01(\x03R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x03R\x05value:\x028\x01\x1AB\n\x14MapUint32Uint32Entry\x12\x10\n\x03key\x18\x01 \x01(\rR\x03key\x12\x14\n\x05value\x18\x02 \x01(\rR\x05value:\x028\x01\x1AB\n\x14MapUint64Uint64Entry\x12\x10\n\x03key\x18\x01 \x01(\x04R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x04R\x05value:\x028\x01\x1AB\n\x14MapSint32Sint32Entry\x12\x10\n\x03key\x18\x01 \x01(\x11R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x11R\x05value:\x028\x01\x1AB\n\x14MapSint64Sint64Entry\x12\x10\n\x03key\x18\x01 \x01(\x12R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x12R\x05value:\x028\x01\x1AD\n\x16MapFixed32Fixed32Entry\x12\x10\n\x03key\x18\x01 \x01(\x07R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x07R\x05value:\x028\x01\x1AD\n\x16MapFixed64Fixed64Entry\x12\x10\n\x03key\x18\x01 \x01(\x06R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x06R\x05value:\x028\x01\x1AF\n\x18MapSfixed32Sfixed32Entry\x12\x10\n\x03key\x18\x01 \x01(\x0FR\x03key\x12\x14\n\x05value\x18\x02 \x01(\x0FR\x05value:\x028\x01\x1AF\n\x18MapSfixed64Sfixed64Entry\x12\x10\n\x03key\x18\x01 \x01(\x10R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x10R\x05value:\x028\x01\x1A@\n\x12MapInt32FloatEntry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x02R\x05value:\x028\x01\x1AA\n\x13MapInt32DoubleEntry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x01R\x05value:\x028\x01\x1A>\n\x10MapBoolBoolEntry\x12\x10\n\x03key\x18\x01 \x01(\x08R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x08R\x05value:\x028\x01\x1AB\n\x14MapStringStringEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x14\n\x05value\x18\x02 \x01(\tR\x05value:\x028\x01\x1AA\n\x13MapStringBytesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x14\n\x05value\x18\x02 \x01(\x0CR\x05value:\x028\x01\x1A\x8A\x01\n\x1BMapStringNestedMessageEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12U\n\x05value\x18\x02 \x01(\x0B2?.protobuf_test_messages.proto2.TestAllTypesProto2.NestedMessageR\x05value:\x028\x01\x1A\x7F\n\x1CMapStringForeignMessageEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12I\n\x05value\x18\x02 \x01(\x0B23.protobuf_test_messages.proto2.ForeignMessageProto2R\x05value:\x028\x01\x1A\x84\x01\n\x18MapStringNestedEnumEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12R\n\x05value\x18\x02 \x01(\x0E2<.protobuf_test_messages.proto2.TestAllTypesProto2.NestedEnumR\x05value:\x028\x01\x1Ay\n\x19MapStringForeignEnumEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12F\n\x05value\x18\x02 \x01(\x0E20.protobuf_test_messages.proto2.ForeignEnumProto2R\x05value:\x028\x01\x1AL\n\x04Data\x12 \n\x0Bgroup_int32\x18\xCA\x01 \x01(\x05R\ngroupInt32\x12\"\n\x0Cgroup_uint32\x18\xCB\x01 \x01(\rR\x0BgroupUint32\x1A!\n\x11MessageSetCorrect*\x08\x08\x04\x10\xFF\xFF\xFF\xFF\x07:\x02\x08\x01\x1A\xFA\x01\n\x1BMessageSetCorrectExtension1\x12\x10\n\x03str\x18\x19 \x01(\tR\x03str2\xC8\x01\n\x15message_set_extension\x12C.protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrect\x18\xF9\xBB^ \x01(\x0B2M.protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrectExtension1R\x13messageSetExtension\x1A\xF7\x01\n\x1BMessageSetCorrectExtension2\x12\x0C\n\x01i\x18\t \x01(\x05R\x01i2\xC9\x01\n\x15message_set_extension\x12C.protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrect\x18\x90\xB3\xFC\x01 \x01(\x0B2M.protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrectExtension2R\x13messageSetExtension\"9\n\nNestedEnum\x12\x07\n\x03FOO\x10\0\x12\x07\n\x03BAR\x10\x01\x12\x07\n\x03BAZ\x10\x02\x12\x10\n\x03NEG\x10\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01*\x05\x08x\x10\xC9\x01B\r\n\x0Boneof_fieldJ\x06\x08\xE8\x07\x10\x90N\"$\n\x14ForeignMessageProto2\x12\x0C\n\x01c\x18\x01 \x01(\x05R\x01c\"\x9E\x03\n\x15UnknownToTestAllTypes\x12&\n\x0Eoptional_int32\x18\xE9\x07 \x01(\x05R\roptionalInt32\x12(\n\x0Foptional_string\x18\xEA\x07 \x01(\tR\x0EoptionalString\x12[\n\x0Enested_message\x18\xEB\x07 \x01(\x0B23.protobuf_test_messages.proto2.ForeignMessageProto2R\rnestedMessage\x12i\n\roptionalgroup\x18\xEC\x07 \x01(\n2B.protobuf_test_messages.proto2.UnknownToTestAllTypes.OptionalGroupR\roptionalgroup\x12$\n\roptional_bool\x18\xEE\x07 \x01(\x08R\x0CoptionalBool\x12&\n\x0Erepeated_int32\x18\xF3\x07 \x03(\x05R\rrepeatedInt32\x1A\x1D\n\rOptionalGroup\x12\x0C\n\x01a\x18\x01 \x01(\x05R\x01a*F\n\x11ForeignEnumProto2\x12\x0F\n\x0BFOREIGN_FOO\x10\0\x12\x0F\n\x0BFOREIGN_BAR\x10\x01\x12\x0F\n\x0BFOREIGN_BAZ\x10\x02:Z\n\x0Fextension_int32\x121.protobuf_test_messages.proto2.TestAllTypesProto2\x18x \x01(\x05R\x0EextensionInt32B/\n(com.google.protobuf_test_messages.proto2H\x01\xF8\x01\x01J\xF7l\n\x07\x12\x05%\0\x8B\x02\x01\n\xBA\r\n\x01\x0C\x12\x03%\0\x122\x99\r Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n Test schema for proto2 messages.  This test schema is used by:\n\n - conformance tests\n\n2\x14 LINT: ALLOW_GROUPS\n\n\x08\n\x01\x02\x12\x03'\0&\n\t\n\x02\x03\0\x12\x03)\0\x1D\n\x08\n\x01\x08\x12\x03+\0A\n\t\n\x02\x08\x01\x12\x03+\0A\n\x08\n\x01\x08\x12\x03.\0\x1C\nD\n\x02\x08\t\x12\x03.\0\x1C\x1A9 This is the default, but we specify it here explicitly.\n\n\x08\n\x01\x08\x12\x030\0\x1F\n\t\n\x02\x08\x1F\x12\x030\0\x1F\n\xDB\x02\n\x02\x04\0\x12\x059\0\xF2\x01\x01\x1A\xCD\x02 This proto includes every type of field in both singular and repeated\n forms.\n\n Also, crucially, all messages and enums in this file are eventually\n submessages of this message.  So for example, a fuzz test of TestAllTypes\n could trigger bugs that occur in any message type in this file.  We verify\n this stays true in a unit test.\n\n\n\n\x03\x04\0\x01\x12\x039\x08\x1A\n\x0C\n\x04\x04\0\x03\0\x12\x04:\x02=\x03\n\x0C\n\x05\x04\0\x03\0\x01\x12\x03:\n\x17\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03;\x04\x19\n\x0E\n\x07\x04\0\x03\0\x02\0\x04\x12\x03;\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\0\x05\x12\x03;\r\x12\n\x0E\n\x07\x04\0\x03\0\x02\0\x01\x12\x03;\x13\x14\n\x0E\n\x07\x04\0\x03\0\x02\0\x03\x12\x03;\x17\x18\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03<\x04U\n\x0E\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03<\x04\x0C\n\x0E\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03<\r\x1F\n\x0E\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03< +\n\x0E\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03<./\n\x0E\n\x07\x04\0\x03\0\x02\x01\x08\x12\x03<0T\n\x11\n\n\x04\0\x03\0\x02\x01\x08\x9D\x12\x01\x12\x03<1S\n\x0C\n\x04\x04\0\x04\0\x12\x04?\x02D\x03\n\x0C\n\x05\x04\0\x04\0\x01\x12\x03?\x07\x11\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03@\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\0\x01\x12\x03@\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\0\x02\x12\x03@\n\x0B\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03A\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03A\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03A\n\x0B\n\r\n\x06\x04\0\x04\0\x02\x02\x12\x03B\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03B\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03B\n\x0B\n(\n\x06\x04\0\x04\0\x02\x03\x12\x03C\x04\r\"\x19 Intentionally negative.\n\n\x0E\n\x07\x04\0\x04\0\x02\x03\x01\x12\x03C\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x03\x02\x12\x03C\n\x0C\n\x17\n\x04\x04\0\x02\0\x12\x03G\x02$\x1A\n Singular\n\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03G\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03G\x0B\x10\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03G\x11\x1F\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03G\"#\n\x0B\n\x04\x04\0\x02\x01\x12\x03H\x02$\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03H\x02\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03H\x0B\x10\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03H\x11\x1F\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03H\"#\n\x0B\n\x04\x04\0\x02\x02\x12\x03I\x02&\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03I\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03I\x0B\x11\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03I\x12!\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03I$%\n\x0B\n\x04\x04\0\x02\x03\x12\x03J\x02&\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x03J\x02\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03J\x0B\x11\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03J\x12!\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03J$%\n\x0B\n\x04\x04\0\x02\x04\x12\x03K\x02&\n\x0C\n\x05\x04\0\x02\x04\x04\x12\x03K\x02\n\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x03K\x0B\x11\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03K\x12!\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03K$%\n\x0B\n\x04\x04\0\x02\x05\x12\x03L\x02&\n\x0C\n\x05\x04\0\x02\x05\x04\x12\x03L\x02\n\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x03L\x0B\x11\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x03L\x12!\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x03L$%\n\x0B\n\x04\x04\0\x02\x06\x12\x03M\x02(\n\x0C\n\x05\x04\0\x02\x06\x04\x12\x03M\x02\n\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x03M\x0B\x12\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x03M\x13#\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x03M&'\n\x0B\n\x04\x04\0\x02\x07\x12\x03N\x02(\n\x0C\n\x05\x04\0\x02\x07\x04\x12\x03N\x02\n\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x03N\x0B\x12\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x03N\x13#\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x03N&'\n\x0B\n\x04\x04\0\x02\x08\x12\x03O\x02*\n\x0C\n\x05\x04\0\x02\x08\x04\x12\x03O\x02\n\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x03O\x0B\x13\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x03O\x14%\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x03O()\n\x0B\n\x04\x04\0\x02\t\x12\x03P\x02+\n\x0C\n\x05\x04\0\x02\t\x04\x12\x03P\x02\n\n\x0C\n\x05\x04\0\x02\t\x05\x12\x03P\x0B\x13\n\x0C\n\x05\x04\0\x02\t\x01\x12\x03P\x14%\n\x0C\n\x05\x04\0\x02\t\x03\x12\x03P(*\n\x0B\n\x04\x04\0\x02\n\x12\x03Q\x02%\n\x0C\n\x05\x04\0\x02\n\x04\x12\x03Q\x02\n\n\x0C\n\x05\x04\0\x02\n\x05\x12\x03Q\x0B\x10\n\x0C\n\x05\x04\0\x02\n\x01\x12\x03Q\x11\x1F\n\x0C\n\x05\x04\0\x02\n\x03\x12\x03Q\"$\n\x0B\n\x04\x04\0\x02\x0B\x12\x03R\x02'\n\x0C\n\x05\x04\0\x02\x0B\x04\x12\x03R\x02\n\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x03R\x0B\x11\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x03R\x12!\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x03R$&\n\x0B\n\x04\x04\0\x02\x0C\x12\x03S\x02#\n\x0C\n\x05\x04\0\x02\x0C\x04\x12\x03S\x02\n\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x03S\x0B\x0F\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x03S\x10\x1D\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x03S \"\n\x0B\n\x04\x04\0\x02\r\x12\x03T\x02'\n\x0C\n\x05\x04\0\x02\r\x04\x12\x03T\x02\n\n\x0C\n\x05\x04\0\x02\r\x05\x12\x03T\x0B\x11\n\x0C\n\x05\x04\0\x02\r\x01\x12\x03T\x12!\n\x0C\n\x05\x04\0\x02\r\x03\x12\x03T$&\n\x0B\n\x04\x04\0\x02\x0E\x12\x03U\x02%\n\x0C\n\x05\x04\0\x02\x0E\x04\x12\x03U\x02\n\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x03U\x0B\x10\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x03U\x11\x1F\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x03U\"$\n\x0B\n\x04\x04\0\x02\x0F\x12\x03W\x026\n\x0C\n\x05\x04\0\x02\x0F\x04\x12\x03W\x02\n\n\x0C\n\x05\x04\0\x02\x0F\x06\x12\x03W\x0B\x18\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03W\x190\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03W35\n\x0B\n\x04\x04\0\x02\x10\x12\x03X\x02>\n\x0C\n\x05\x04\0\x02\x10\x04\x12\x03X\x02\n\n\x0C\n\x05\x04\0\x02\x10\x06\x12\x03X\x0B\x1F\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03X 8\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03X;=\n\x0B\n\x04\x04\0\x02\x11\x12\x03Z\x020\n\x0C\n\x05\x04\0\x02\x11\x04\x12\x03Z\x02\n\n\x0C\n\x05\x04\0\x02\x11\x06\x12\x03Z\x0B\x15\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03Z\x16*\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03Z-/\n\x0B\n\x04\x04\0\x02\x12\x12\x03[\x028\n\x0C\n\x05\x04\0\x02\x12\x04\x12\x03[\x02\n\n\x0C\n\x05\x04\0\x02\x12\x06\x12\x03[\x0B\x1C\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03[\x1D2\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03[57\n\x0B\n\x04\x04\0\x02\x13\x12\x03]\x02D\n\x0C\n\x05\x04\0\x02\x13\x04\x12\x03]\x02\n\n\x0C\n\x05\x04\0\x02\x13\x05\x12\x03]\x0B\x11\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03]\x12'\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03]*,\n\x0C\n\x05\x04\0\x02\x13\x08\x12\x03]-C\n\r\n\x06\x04\0\x02\x13\x08\x01\x12\x03].B\n\x0B\n\x04\x04\0\x02\x14\x12\x03^\x024\n\x0C\n\x05\x04\0\x02\x14\x04\x12\x03^\x02\n\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03^\x0B\x11\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03^\x12\x1F\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03^\"$\n\x0C\n\x05\x04\0\x02\x14\x08\x12\x03^%3\n\r\n\x06\x04\0\x02\x14\x08\x01\x12\x03^&2\n\x0B\n\x04\x04\0\x02\x15\x12\x03`\x02Z\n\x0C\n\x05\x04\0\x02\x15\x04\x12\x03`\x02\n\n\x0C\n\x05\x04\0\x02\x15\x06\x12\x03`\x0B\x1D\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03`\x1E/\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03`24\n\x0C\n\x05\x04\0\x02\x15\x08\x12\x03`5Y\n\x0F\n\x08\x04\0\x02\x15\x08\x9D\x12\x01\x12\x03`6X\n\x17\n\x04\x04\0\x02\x16\x12\x03c\x02%\x1A\n Repeated\n\n\x0C\n\x05\x04\0\x02\x16\x04\x12\x03c\x02\n\n\x0C\n\x05\x04\0\x02\x16\x05\x12\x03c\x0B\x10\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03c\x11\x1F\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03c\"$\n\x0B\n\x04\x04\0\x02\x17\x12\x03d\x02%\n\x0C\n\x05\x04\0\x02\x17\x04\x12\x03d\x02\n\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03d\x0B\x10\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03d\x11\x1F\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03d\"$\n\x0B\n\x04\x04\0\x02\x18\x12\x03e\x02'\n\x0C\n\x05\x04\0\x02\x18\x04\x12\x03e\x02\n\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03e\x0B\x11\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03e\x12!\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03e$&\n\x0B\n\x04\x04\0\x02\x19\x12\x03f\x02'\n\x0C\n\x05\x04\0\x02\x19\x04\x12\x03f\x02\n\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03f\x0B\x11\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03f\x12!\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03f$&\n\x0B\n\x04\x04\0\x02\x1A\x12\x03g\x02'\n\x0C\n\x05\x04\0\x02\x1A\x04\x12\x03g\x02\n\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03g\x0B\x11\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03g\x12!\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03g$&\n\x0B\n\x04\x04\0\x02\x1B\x12\x03h\x02'\n\x0C\n\x05\x04\0\x02\x1B\x04\x12\x03h\x02\n\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03h\x0B\x11\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03h\x12!\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03h$&\n\x0B\n\x04\x04\0\x02\x1C\x12\x03i\x02)\n\x0C\n\x05\x04\0\x02\x1C\x04\x12\x03i\x02\n\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03i\x0B\x12\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03i\x13#\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03i&(\n\x0B\n\x04\x04\0\x02\x1D\x12\x03j\x02)\n\x0C\n\x05\x04\0\x02\x1D\x04\x12\x03j\x02\n\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03j\x0B\x12\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03j\x13#\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03j&(\n\x0B\n\x04\x04\0\x02\x1E\x12\x03k\x02+\n\x0C\n\x05\x04\0\x02\x1E\x04\x12\x03k\x02\n\n\x0C\n\x05\x04\0\x02\x1E\x05\x12\x03k\x0B\x13\n\x0C\n\x05\x04\0\x02\x1E\x01\x12\x03k\x14%\n\x0C\n\x05\x04\0\x02\x1E\x03\x12\x03k(*\n\x0B\n\x04\x04\0\x02\x1F\x12\x03l\x02+\n\x0C\n\x05\x04\0\x02\x1F\x04\x12\x03l\x02\n\n\x0C\n\x05\x04\0\x02\x1F\x05\x12\x03l\x0B\x13\n\x0C\n\x05\x04\0\x02\x1F\x01\x12\x03l\x14%\n\x0C\n\x05\x04\0\x02\x1F\x03\x12\x03l(*\n\x0B\n\x04\x04\0\x02 \x12\x03m\x02%\n\x0C\n\x05\x04\0\x02 \x04\x12\x03m\x02\n\n\x0C\n\x05\x04\0\x02 \x05\x12\x03m\x0B\x10\n\x0C\n\x05\x04\0\x02 \x01\x12\x03m\x11\x1F\n\x0C\n\x05\x04\0\x02 \x03\x12\x03m\"$\n\x0B\n\x04\x04\0\x02!\x12\x03n\x02'\n\x0C\n\x05\x04\0\x02!\x04\x12\x03n\x02\n\n\x0C\n\x05\x04\0\x02!\x05\x12\x03n\x0B\x11\n\x0C\n\x05\x04\0\x02!\x01\x12\x03n\x12!\n\x0C\n\x05\x04\0\x02!\x03\x12\x03n$&\n\x0B\n\x04\x04\0\x02\"\x12\x03o\x02#\n\x0C\n\x05\x04\0\x02\"\x04\x12\x03o\x02\n\n\x0C\n\x05\x04\0\x02\"\x05\x12\x03o\x0B\x0F\n\x0C\n\x05\x04\0\x02\"\x01\x12\x03o\x10\x1D\n\x0C\n\x05\x04\0\x02\"\x03\x12\x03o \"\n\x0B\n\x04\x04\0\x02#\x12\x03p\x02'\n\x0C\n\x05\x04\0\x02#\x04\x12\x03p\x02\n\n\x0C\n\x05\x04\0\x02#\x05\x12\x03p\x0B\x11\n\x0C\n\x05\x04\0\x02#\x01\x12\x03p\x12!\n\x0C\n\x05\x04\0\x02#\x03\x12\x03p$&\n\x0B\n\x04\x04\0\x02$\x12\x03q\x02%\n\x0C\n\x05\x04\0\x02$\x04\x12\x03q\x02\n\n\x0C\n\x05\x04\0\x02$\x05\x12\x03q\x0B\x10\n\x0C\n\x05\x04\0\x02$\x01\x12\x03q\x11\x1F\n\x0C\n\x05\x04\0\x02$\x03\x12\x03q\"$\n\x0B\n\x04\x04\0\x02%\x12\x03s\x026\n\x0C\n\x05\x04\0\x02%\x04\x12\x03s\x02\n\n\x0C\n\x05\x04\0\x02%\x06\x12\x03s\x0B\x18\n\x0C\n\x05\x04\0\x02%\x01\x12\x03s\x190\n\x0C\n\x05\x04\0\x02%\x03\x12\x03s35\n\x0B\n\x04\x04\0\x02&\x12\x03t\x02>\n\x0C\n\x05\x04\0\x02&\x04\x12\x03t\x02\n\n\x0C\n\x05\x04\0\x02&\x06\x12\x03t\x0B\x1F\n\x0C\n\x05\x04\0\x02&\x01\x12\x03t 8\n\x0C\n\x05\x04\0\x02&\x03\x12\x03t;=\n\x0B\n\x04\x04\0\x02'\x12\x03v\x020\n\x0C\n\x05\x04\0\x02'\x04\x12\x03v\x02\n\n\x0C\n\x05\x04\0\x02'\x06\x12\x03v\x0B\x15\n\x0C\n\x05\x04\0\x02'\x01\x12\x03v\x16*\n\x0C\n\x05\x04\0\x02'\x03\x12\x03v-/\n\x0B\n\x04\x04\0\x02(\x12\x03w\x028\n\x0C\n\x05\x04\0\x02(\x04\x12\x03w\x02\n\n\x0C\n\x05\x04\0\x02(\x06\x12\x03w\x0B\x1C\n\x0C\n\x05\x04\0\x02(\x01\x12\x03w\x1D2\n\x0C\n\x05\x04\0\x02(\x03\x12\x03w57\n\x0B\n\x04\x04\0\x02)\x12\x03y\x02D\n\x0C\n\x05\x04\0\x02)\x04\x12\x03y\x02\n\n\x0C\n\x05\x04\0\x02)\x05\x12\x03y\x0B\x11\n\x0C\n\x05\x04\0\x02)\x01\x12\x03y\x12'\n\x0C\n\x05\x04\0\x02)\x03\x12\x03y*,\n\x0C\n\x05\x04\0\x02)\x08\x12\x03y-C\n\r\n\x06\x04\0\x02)\x08\x01\x12\x03y.B\n\x0B\n\x04\x04\0\x02*\x12\x03z\x024\n\x0C\n\x05\x04\0\x02*\x04\x12\x03z\x02\n\n\x0C\n\x05\x04\0\x02*\x05\x12\x03z\x0B\x11\n\x0C\n\x05\x04\0\x02*\x01\x12\x03z\x12\x1F\n\x0C\n\x05\x04\0\x02*\x03\x12\x03z\"$\n\x0C\n\x05\x04\0\x02*\x08\x12\x03z%3\n\r\n\x06\x04\0\x02*\x08\x01\x12\x03z&2\n\x15\n\x04\x04\0\x02+\x12\x03}\x023\x1A\x08 Packed\n\n\x0C\n\x05\x04\0\x02+\x04\x12\x03}\x02\n\n\x0C\n\x05\x04\0\x02+\x05\x12\x03}\x0B\x10\n\x0C\n\x05\x04\0\x02+\x01\x12\x03}\x11\x1D\n\x0C\n\x05\x04\0\x02+\x03\x12\x03} \"\n\x0C\n\x05\x04\0\x02+\x08\x12\x03}#2\n\r\n\x06\x04\0\x02+\x08\x02\x12\x03}$1\n\x0B\n\x04\x04\0\x02,\x12\x03~\x023\n\x0C\n\x05\x04\0\x02,\x04\x12\x03~\x02\n\n\x0C\n\x05\x04\0\x02,\x05\x12\x03~\x0B\x10\n\x0C\n\x05\x04\0\x02,\x01\x12\x03~\x11\x1D\n\x0C\n\x05\x04\0\x02,\x03\x12\x03~ \"\n\x0C\n\x05\x04\0\x02,\x08\x12\x03~#2\n\r\n\x06\x04\0\x02,\x08\x02\x12\x03~$1\n\x0B\n\x04\x04\0\x02-\x12\x03\x7F\x025\n\x0C\n\x05\x04\0\x02-\x04\x12\x03\x7F\x02\n\n\x0C\n\x05\x04\0\x02-\x05\x12\x03\x7F\x0B\x11\n\x0C\n\x05\x04\0\x02-\x01\x12\x03\x7F\x12\x1F\n\x0C\n\x05\x04\0\x02-\x03\x12\x03\x7F\"$\n\x0C\n\x05\x04\0\x02-\x08\x12\x03\x7F%4\n\r\n\x06\x04\0\x02-\x08\x02\x12\x03\x7F&3\n\x0C\n\x04\x04\0\x02.\x12\x04\x80\x01\x025\n\r\n\x05\x04\0\x02.\x04\x12\x04\x80\x01\x02\n\n\r\n\x05\x04\0\x02.\x05\x12\x04\x80\x01\x0B\x11\n\r\n\x05\x04\0\x02.\x01\x12\x04\x80\x01\x12\x1F\n\r\n\x05\x04\0\x02.\x03\x12\x04\x80\x01\"$\n\r\n\x05\x04\0\x02.\x08\x12\x04\x80\x01%4\n\x0E\n\x06\x04\0\x02.\x08\x02\x12\x04\x80\x01&3\n\x0C\n\x04\x04\0\x02/\x12\x04\x81\x01\x025\n\r\n\x05\x04\0\x02/\x04\x12\x04\x81\x01\x02\n\n\r\n\x05\x04\0\x02/\x05\x12\x04\x81\x01\x0B\x11\n\r\n\x05\x04\0\x02/\x01\x12\x04\x81\x01\x12\x1F\n\r\n\x05\x04\0\x02/\x03\x12\x04\x81\x01\"$\n\r\n\x05\x04\0\x02/\x08\x12\x04\x81\x01%4\n\x0E\n\x06\x04\0\x02/\x08\x02\x12\x04\x81\x01&3\n\x0C\n\x04\x04\0\x020\x12\x04\x82\x01\x025\n\r\n\x05\x04\0\x020\x04\x12\x04\x82\x01\x02\n\n\r\n\x05\x04\0\x020\x05\x12\x04\x82\x01\x0B\x11\n\r\n\x05\x04\0\x020\x01\x12\x04\x82\x01\x12\x1F\n\r\n\x05\x04\0\x020\x03\x12\x04\x82\x01\"$\n\r\n\x05\x04\0\x020\x08\x12\x04\x82\x01%4\n\x0E\n\x06\x04\0\x020\x08\x02\x12\x04\x82\x01&3\n\x0C\n\x04\x04\0\x021\x12\x04\x83\x01\x027\n\r\n\x05\x04\0\x021\x04\x12\x04\x83\x01\x02\n\n\r\n\x05\x04\0\x021\x05\x12\x04\x83\x01\x0B\x12\n\r\n\x05\x04\0\x021\x01\x12\x04\x83\x01\x13!\n\r\n\x05\x04\0\x021\x03\x12\x04\x83\x01$&\n\r\n\x05\x04\0\x021\x08\x12\x04\x83\x01'6\n\x0E\n\x06\x04\0\x021\x08\x02\x12\x04\x83\x01(5\n\x0C\n\x04\x04\0\x022\x12\x04\x84\x01\x027\n\r\n\x05\x04\0\x022\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\x04\0\x022\x05\x12\x04\x84\x01\x0B\x12\n\r\n\x05\x04\0\x022\x01\x12\x04\x84\x01\x13!\n\r\n\x05\x04\0\x022\x03\x12\x04\x84\x01$&\n\r\n\x05\x04\0\x022\x08\x12\x04\x84\x01'6\n\x0E\n\x06\x04\0\x022\x08\x02\x12\x04\x84\x01(5\n\x0C\n\x04\x04\0\x023\x12\x04\x85\x01\x029\n\r\n\x05\x04\0\x023\x04\x12\x04\x85\x01\x02\n\n\r\n\x05\x04\0\x023\x05\x12\x04\x85\x01\x0B\x13\n\r\n\x05\x04\0\x023\x01\x12\x04\x85\x01\x14#\n\r\n\x05\x04\0\x023\x03\x12\x04\x85\x01&(\n\r\n\x05\x04\0\x023\x08\x12\x04\x85\x01)8\n\x0E\n\x06\x04\0\x023\x08\x02\x12\x04\x85\x01*7\n\x0C\n\x04\x04\0\x024\x12\x04\x86\x01\x029\n\r\n\x05\x04\0\x024\x04\x12\x04\x86\x01\x02\n\n\r\n\x05\x04\0\x024\x05\x12\x04\x86\x01\x0B\x13\n\r\n\x05\x04\0\x024\x01\x12\x04\x86\x01\x14#\n\r\n\x05\x04\0\x024\x03\x12\x04\x86\x01&(\n\r\n\x05\x04\0\x024\x08\x12\x04\x86\x01)8\n\x0E\n\x06\x04\0\x024\x08\x02\x12\x04\x86\x01*7\n\x0C\n\x04\x04\0\x025\x12\x04\x87\x01\x023\n\r\n\x05\x04\0\x025\x04\x12\x04\x87\x01\x02\n\n\r\n\x05\x04\0\x025\x05\x12\x04\x87\x01\x0B\x10\n\r\n\x05\x04\0\x025\x01\x12\x04\x87\x01\x11\x1D\n\r\n\x05\x04\0\x025\x03\x12\x04\x87\x01 \"\n\r\n\x05\x04\0\x025\x08\x12\x04\x87\x01#2\n\x0E\n\x06\x04\0\x025\x08\x02\x12\x04\x87\x01$1\n\x0C\n\x04\x04\0\x026\x12\x04\x88\x01\x025\n\r\n\x05\x04\0\x026\x04\x12\x04\x88\x01\x02\n\n\r\n\x05\x04\0\x026\x05\x12\x04\x88\x01\x0B\x11\n\r\n\x05\x04\0\x026\x01\x12\x04\x88\x01\x12\x1F\n\r\n\x05\x04\0\x026\x03\x12\x04\x88\x01\"$\n\r\n\x05\x04\0\x026\x08\x12\x04\x88\x01%4\n\x0E\n\x06\x04\0\x026\x08\x02\x12\x04\x88\x01&3\n\x0C\n\x04\x04\0\x027\x12\x04\x89\x01\x021\n\r\n\x05\x04\0\x027\x04\x12\x04\x89\x01\x02\n\n\r\n\x05\x04\0\x027\x05\x12\x04\x89\x01\x0B\x0F\n\r\n\x05\x04\0\x027\x01\x12\x04\x89\x01\x10\x1B\n\r\n\x05\x04\0\x027\x03\x12\x04\x89\x01\x1E \n\r\n\x05\x04\0\x027\x08\x12\x04\x89\x01!0\n\x0E\n\x06\x04\0\x027\x08\x02\x12\x04\x89\x01\"/\n\x0C\n\x04\x04\0\x028\x12\x04\x8A\x01\x02>\n\r\n\x05\x04\0\x028\x04\x12\x04\x8A\x01\x02\n\n\r\n\x05\x04\0\x028\x06\x12\x04\x8A\x01\x0B\x15\n\r\n\x05\x04\0\x028\x01\x12\x04\x8A\x01\x16(\n\r\n\x05\x04\0\x028\x03\x12\x04\x8A\x01+-\n\r\n\x05\x04\0\x028\x08\x12\x04\x8A\x01.=\n\x0E\n\x06\x04\0\x028\x08\x02\x12\x04\x8A\x01/<\n\x18\n\x04\x04\0\x029\x12\x04\x8D\x01\x026\x1A\n Unpacked\n\n\r\n\x05\x04\0\x029\x04\x12\x04\x8D\x01\x02\n\n\r\n\x05\x04\0\x029\x05\x12\x04\x8D\x01\x0B\x10\n\r\n\x05\x04\0\x029\x01\x12\x04\x8D\x01\x11\x1F\n\r\n\x05\x04\0\x029\x03\x12\x04\x8D\x01\"$\n\r\n\x05\x04\0\x029\x08\x12\x04\x8D\x01%5\n\x0E\n\x06\x04\0\x029\x08\x02\x12\x04\x8D\x01&4\n\x0C\n\x04\x04\0\x02:\x12\x04\x8E\x01\x026\n\r\n\x05\x04\0\x02:\x04\x12\x04\x8E\x01\x02\n\n\r\n\x05\x04\0\x02:\x05\x12\x04\x8E\x01\x0B\x10\n\r\n\x05\x04\0\x02:\x01\x12\x04\x8E\x01\x11\x1F\n\r\n\x05\x04\0\x02:\x03\x12\x04\x8E\x01\"$\n\r\n\x05\x04\0\x02:\x08\x12\x04\x8E\x01%5\n\x0E\n\x06\x04\0\x02:\x08\x02\x12\x04\x8E\x01&4\n\x0C\n\x04\x04\0\x02;\x12\x04\x8F\x01\x028\n\r\n\x05\x04\0\x02;\x04\x12\x04\x8F\x01\x02\n\n\r\n\x05\x04\0\x02;\x05\x12\x04\x8F\x01\x0B\x11\n\r\n\x05\x04\0\x02;\x01\x12\x04\x8F\x01\x12!\n\r\n\x05\x04\0\x02;\x03\x12\x04\x8F\x01$&\n\r\n\x05\x04\0\x02;\x08\x12\x04\x8F\x01'7\n\x0E\n\x06\x04\0\x02;\x08\x02\x12\x04\x8F\x01(6\n\x0C\n\x04\x04\0\x02<\x12\x04\x90\x01\x028\n\r\n\x05\x04\0\x02<\x04\x12\x04\x90\x01\x02\n\n\r\n\x05\x04\0\x02<\x05\x12\x04\x90\x01\x0B\x11\n\r\n\x05\x04\0\x02<\x01\x12\x04\x90\x01\x12!\n\r\n\x05\x04\0\x02<\x03\x12\x04\x90\x01$&\n\r\n\x05\x04\0\x02<\x08\x12\x04\x90\x01'7\n\x0E\n\x06\x04\0\x02<\x08\x02\x12\x04\x90\x01(6\n\x0C\n\x04\x04\0\x02=\x12\x04\x91\x01\x028\n\r\n\x05\x04\0\x02=\x04\x12\x04\x91\x01\x02\n\n\r\n\x05\x04\0\x02=\x05\x12\x04\x91\x01\x0B\x11\n\r\n\x05\x04\0\x02=\x01\x12\x04\x91\x01\x12!\n\r\n\x05\x04\0\x02=\x03\x12\x04\x91\x01$&\n\r\n\x05\x04\0\x02=\x08\x12\x04\x91\x01'7\n\x0E\n\x06\x04\0\x02=\x08\x02\x12\x04\x91\x01(6\n\x0C\n\x04\x04\0\x02>\x12\x04\x92\x01\x028\n\r\n\x05\x04\0\x02>\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\0\x02>\x05\x12\x04\x92\x01\x0B\x11\n\r\n\x05\x04\0\x02>\x01\x12\x04\x92\x01\x12!\n\r\n\x05\x04\0\x02>\x03\x12\x04\x92\x01$&\n\r\n\x05\x04\0\x02>\x08\x12\x04\x92\x01'7\n\x0E\n\x06\x04\0\x02>\x08\x02\x12\x04\x92\x01(6\n\x0C\n\x04\x04\0\x02?\x12\x04\x93\x01\x02:\n\r\n\x05\x04\0\x02?\x04\x12\x04\x93\x01\x02\n\n\r\n\x05\x04\0\x02?\x05\x12\x04\x93\x01\x0B\x12\n\r\n\x05\x04\0\x02?\x01\x12\x04\x93\x01\x13#\n\r\n\x05\x04\0\x02?\x03\x12\x04\x93\x01&(\n\r\n\x05\x04\0\x02?\x08\x12\x04\x93\x01)9\n\x0E\n\x06\x04\0\x02?\x08\x02\x12\x04\x93\x01*8\n\x0C\n\x04\x04\0\x02@\x12\x04\x94\x01\x02:\n\r\n\x05\x04\0\x02@\x04\x12\x04\x94\x01\x02\n\n\r\n\x05\x04\0\x02@\x05\x12\x04\x94\x01\x0B\x12\n\r\n\x05\x04\0\x02@\x01\x12\x04\x94\x01\x13#\n\r\n\x05\x04\0\x02@\x03\x12\x04\x94\x01&(\n\r\n\x05\x04\0\x02@\x08\x12\x04\x94\x01)9\n\x0E\n\x06\x04\0\x02@\x08\x02\x12\x04\x94\x01*8\n\x0C\n\x04\x04\0\x02A\x12\x04\x95\x01\x02<\n\r\n\x05\x04\0\x02A\x04\x12\x04\x95\x01\x02\n\n\r\n\x05\x04\0\x02A\x05\x12\x04\x95\x01\x0B\x13\n\r\n\x05\x04\0\x02A\x01\x12\x04\x95\x01\x14%\n\r\n\x05\x04\0\x02A\x03\x12\x04\x95\x01(*\n\r\n\x05\x04\0\x02A\x08\x12\x04\x95\x01+;\n\x0E\n\x06\x04\0\x02A\x08\x02\x12\x04\x95\x01,:\n\x0C\n\x04\x04\0\x02B\x12\x04\x96\x01\x02<\n\r\n\x05\x04\0\x02B\x04\x12\x04\x96\x01\x02\n\n\r\n\x05\x04\0\x02B\x05\x12\x04\x96\x01\x0B\x13\n\r\n\x05\x04\0\x02B\x01\x12\x04\x96\x01\x14%\n\r\n\x05\x04\0\x02B\x03\x12\x04\x96\x01(*\n\r\n\x05\x04\0\x02B\x08\x12\x04\x96\x01+;\n\x0E\n\x06\x04\0\x02B\x08\x02\x12\x04\x96\x01,:\n\x0C\n\x04\x04\0\x02C\x12\x04\x97\x01\x026\n\r\n\x05\x04\0\x02C\x04\x12\x04\x97\x01\x02\n\n\r\n\x05\x04\0\x02C\x05\x12\x04\x97\x01\x0B\x10\n\r\n\x05\x04\0\x02C\x01\x12\x04\x97\x01\x11\x1F\n\r\n\x05\x04\0\x02C\x03\x12\x04\x97\x01\"$\n\r\n\x05\x04\0\x02C\x08\x12\x04\x97\x01%5\n\x0E\n\x06\x04\0\x02C\x08\x02\x12\x04\x97\x01&4\n\x0C\n\x04\x04\0\x02D\x12\x04\x98\x01\x029\n\r\n\x05\x04\0\x02D\x04\x12\x04\x98\x01\x02\n\n\r\n\x05\x04\0\x02D\x05\x12\x04\x98\x01\x0B\x11\n\r\n\x05\x04\0\x02D\x01\x12\x04\x98\x01\x12!\n\r\n\x05\x04\0\x02D\x03\x12\x04\x98\x01$'\n\r\n\x05\x04\0\x02D\x08\x12\x04\x98\x01(8\n\x0E\n\x06\x04\0\x02D\x08\x02\x12\x04\x98\x01)7\n\x0C\n\x04\x04\0\x02E\x12\x04\x99\x01\x025\n\r\n\x05\x04\0\x02E\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\x04\0\x02E\x05\x12\x04\x99\x01\x0B\x0F\n\r\n\x05\x04\0\x02E\x01\x12\x04\x99\x01\x10\x1D\n\r\n\x05\x04\0\x02E\x03\x12\x04\x99\x01 #\n\r\n\x05\x04\0\x02E\x08\x12\x04\x99\x01$4\n\x0E\n\x06\x04\0\x02E\x08\x02\x12\x04\x99\x01%3\n\x0C\n\x04\x04\0\x02F\x12\x04\x9A\x01\x02B\n\r\n\x05\x04\0\x02F\x04\x12\x04\x9A\x01\x02\n\n\r\n\x05\x04\0\x02F\x06\x12\x04\x9A\x01\x0B\x15\n\r\n\x05\x04\0\x02F\x01\x12\x04\x9A\x01\x16*\n\r\n\x05\x04\0\x02F\x03\x12\x04\x9A\x01-0\n\r\n\x05\x04\0\x02F\x08\x12\x04\x9A\x011A\n\x0E\n\x06\x04\0\x02F\x08\x02\x12\x04\x9A\x012@\n\x13\n\x04\x04\0\x02G\x12\x04\x9D\x01\x02)\x1A\x05 Map\n\n\r\n\x05\x04\0\x02G\x06\x12\x04\x9D\x01\x02\x13\n\r\n\x05\x04\0\x02G\x01\x12\x04\x9D\x01\x14#\n\r\n\x05\x04\0\x02G\x03\x12\x04\x9D\x01&(\n\x0C\n\x04\x04\0\x02H\x12\x04\x9E\x01\x02)\n\r\n\x05\x04\0\x02H\x06\x12\x04\x9E\x01\x02\x13\n\r\n\x05\x04\0\x02H\x01\x12\x04\x9E\x01\x14#\n\r\n\x05\x04\0\x02H\x03\x12\x04\x9E\x01&(\n\x0C\n\x04\x04\0\x02I\x12\x04\x9F\x01\x02-\n\r\n\x05\x04\0\x02I\x06\x12\x04\x9F\x01\x02\x15\n\r\n\x05\x04\0\x02I\x01\x12\x04\x9F\x01\x16'\n\r\n\x05\x04\0\x02I\x03\x12\x04\x9F\x01*,\n\x0C\n\x04\x04\0\x02J\x12\x04\xA0\x01\x02-\n\r\n\x05\x04\0\x02J\x06\x12\x04\xA0\x01\x02\x15\n\r\n\x05\x04\0\x02J\x01\x12\x04\xA0\x01\x16'\n\r\n\x05\x04\0\x02J\x03\x12\x04\xA0\x01*,\n\x0C\n\x04\x04\0\x02K\x12\x04\xA1\x01\x02-\n\r\n\x05\x04\0\x02K\x06\x12\x04\xA1\x01\x02\x15\n\r\n\x05\x04\0\x02K\x01\x12\x04\xA1\x01\x16'\n\r\n\x05\x04\0\x02K\x03\x12\x04\xA1\x01*,\n\x0C\n\x04\x04\0\x02L\x12\x04\xA2\x01\x02-\n\r\n\x05\x04\0\x02L\x06\x12\x04\xA2\x01\x02\x15\n\r\n\x05\x04\0\x02L\x01\x12\x04\xA2\x01\x16'\n\r\n\x05\x04\0\x02L\x03\x12\x04\xA2\x01*,\n\x0C\n\x04\x04\0\x02M\x12\x04\xA3\x01\x021\n\r\n\x05\x04\0\x02M\x06\x12\x04\xA3\x01\x02\x17\n\r\n\x05\x04\0\x02M\x01\x12\x04\xA3\x01\x18+\n\r\n\x05\x04\0\x02M\x03\x12\x04\xA3\x01.0\n\x0C\n\x04\x04\0\x02N\x12\x04\xA4\x01\x021\n\r\n\x05\x04\0\x02N\x06\x12\x04\xA4\x01\x02\x17\n\r\n\x05\x04\0\x02N\x01\x12\x04\xA4\x01\x18+\n\r\n\x05\x04\0\x02N\x03\x12\x04\xA4\x01.0\n\x0C\n\x04\x04\0\x02O\x12\x04\xA5\x01\x025\n\r\n\x05\x04\0\x02O\x06\x12\x04\xA5\x01\x02\x19\n\r\n\x05\x04\0\x02O\x01\x12\x04\xA5\x01\x1A/\n\r\n\x05\x04\0\x02O\x03\x12\x04\xA5\x0124\n\x0C\n\x04\x04\0\x02P\x12\x04\xA6\x01\x025\n\r\n\x05\x04\0\x02P\x06\x12\x04\xA6\x01\x02\x19\n\r\n\x05\x04\0\x02P\x01\x12\x04\xA6\x01\x1A/\n\r\n\x05\x04\0\x02P\x03\x12\x04\xA6\x0124\n\x0C\n\x04\x04\0\x02Q\x12\x04\xA7\x01\x02)\n\r\n\x05\x04\0\x02Q\x06\x12\x04\xA7\x01\x02\x13\n\r\n\x05\x04\0\x02Q\x01\x12\x04\xA7\x01\x14#\n\r\n\x05\x04\0\x02Q\x03\x12\x04\xA7\x01&(\n\x0C\n\x04\x04\0\x02R\x12\x04\xA8\x01\x02+\n\r\n\x05\x04\0\x02R\x06\x12\x04\xA8\x01\x02\x14\n\r\n\x05\x04\0\x02R\x01\x12\x04\xA8\x01\x15%\n\r\n\x05\x04\0\x02R\x03\x12\x04\xA8\x01(*\n\x0C\n\x04\x04\0\x02S\x12\x04\xA9\x01\x02%\n\r\n\x05\x04\0\x02S\x06\x12\x04\xA9\x01\x02\x11\n\r\n\x05\x04\0\x02S\x01\x12\x04\xA9\x01\x12\x1F\n\r\n\x05\x04\0\x02S\x03\x12\x04\xA9\x01\"$\n\x0C\n\x04\x04\0\x02T\x12\x04\xAA\x01\x02-\n\r\n\x05\x04\0\x02T\x06\x12\x04\xAA\x01\x02\x15\n\r\n\x05\x04\0\x02T\x01\x12\x04\xAA\x01\x16'\n\r\n\x05\x04\0\x02T\x03\x12\x04\xAA\x01*,\n\x0C\n\x04\x04\0\x02U\x12\x04\xAB\x01\x02+\n\r\n\x05\x04\0\x02U\x06\x12\x04\xAB\x01\x02\x14\n\r\n\x05\x04\0\x02U\x01\x12\x04\xAB\x01\x15%\n\r\n\x05\x04\0\x02U\x03\x12\x04\xAB\x01(*\n\x0C\n\x04\x04\0\x02V\x12\x04\xAC\x01\x02<\n\r\n\x05\x04\0\x02V\x06\x12\x04\xAC\x01\x02\x1C\n\r\n\x05\x04\0\x02V\x01\x12\x04\xAC\x01\x1D6\n\r\n\x05\x04\0\x02V\x03\x12\x04\xAC\x019;\n\x0C\n\x04\x04\0\x02W\x12\x04\xAD\x01\x02D\n\r\n\x05\x04\0\x02W\x06\x12\x04\xAD\x01\x02#\n\r\n\x05\x04\0\x02W\x01\x12\x04\xAD\x01$>\n\r\n\x05\x04\0\x02W\x03\x12\x04\xAD\x01AC\n\x0C\n\x04\x04\0\x02X\x12\x04\xAE\x01\x026\n\r\n\x05\x04\0\x02X\x06\x12\x04\xAE\x01\x02\x19\n\r\n\x05\x04\0\x02X\x01\x12\x04\xAE\x01\x1A0\n\r\n\x05\x04\0\x02X\x03\x12\x04\xAE\x0135\n\x0C\n\x04\x04\0\x02Y\x12\x04\xAF\x01\x02>\n\r\n\x05\x04\0\x02Y\x06\x12\x04\xAF\x01\x02 \n\r\n\x05\x04\0\x02Y\x01\x12\x04\xAF\x01!8\n\r\n\x05\x04\0\x02Y\x03\x12\x04\xAF\x01;=\n\x0E\n\x04\x04\0\x08\0\x12\x06\xB1\x01\x02\xBB\x01\x03\n\r\n\x05\x04\0\x08\0\x01\x12\x04\xB1\x01\x08\x13\n\x0C\n\x04\x04\0\x02Z\x12\x04\xB2\x01\x04\x1E\n\r\n\x05\x04\0\x02Z\x05\x12\x04\xB2\x01\x04\n\n\r\n\x05\x04\0\x02Z\x01\x12\x04\xB2\x01\x0B\x17\n\r\n\x05\x04\0\x02Z\x03\x12\x04\xB2\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02[\x12\x04\xB3\x01\x04-\n\r\n\x05\x04\0\x02[\x06\x12\x04\xB3\x01\x04\x11\n\r\n\x05\x04\0\x02[\x01\x12\x04\xB3\x01\x12&\n\r\n\x05\x04\0\x02[\x03\x12\x04\xB3\x01),\n\x0C\n\x04\x04\0\x02\\\x12\x04\xB4\x01\x04\x1E\n\r\n\x05\x04\0\x02\\\x05\x12\x04\xB4\x01\x04\n\n\r\n\x05\x04\0\x02\\\x01\x12\x04\xB4\x01\x0B\x17\n\r\n\x05\x04\0\x02\\\x03\x12\x04\xB4\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02]\x12\x04\xB5\x01\x04\x1C\n\r\n\x05\x04\0\x02]\x05\x12\x04\xB5\x01\x04\t\n\r\n\x05\x04\0\x02]\x01\x12\x04\xB5\x01\n\x15\n\r\n\x05\x04\0\x02]\x03\x12\x04\xB5\x01\x18\x1B\n\x0C\n\x04\x04\0\x02^\x12\x04\xB6\x01\x04\x1A\n\r\n\x05\x04\0\x02^\x05\x12\x04\xB6\x01\x04\x08\n\r\n\x05\x04\0\x02^\x01\x12\x04\xB6\x01\t\x13\n\r\n\x05\x04\0\x02^\x03\x12\x04\xB6\x01\x16\x19\n\x0C\n\x04\x04\0\x02_\x12\x04\xB7\x01\x04\x1E\n\r\n\x05\x04\0\x02_\x05\x12\x04\xB7\x01\x04\n\n\r\n\x05\x04\0\x02_\x01\x12\x04\xB7\x01\x0B\x17\n\r\n\x05\x04\0\x02_\x03\x12\x04\xB7\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02`\x12\x04\xB8\x01\x04\x1C\n\r\n\x05\x04\0\x02`\x05\x12\x04\xB8\x01\x04\t\n\r\n\x05\x04\0\x02`\x01\x12\x04\xB8\x01\n\x15\n\r\n\x05\x04\0\x02`\x03\x12\x04\xB8\x01\x18\x1B\n\x0C\n\x04\x04\0\x02a\x12\x04\xB9\x01\x04\x1E\n\r\n\x05\x04\0\x02a\x05\x12\x04\xB9\x01\x04\n\n\r\n\x05\x04\0\x02a\x01\x12\x04\xB9\x01\x0B\x17\n\r\n\x05\x04\0\x02a\x03\x12\x04\xB9\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02b\x12\x04\xBA\x01\x04 \n\r\n\x05\x04\0\x02b\x06\x12\x04\xBA\x01\x04\x0E\n\r\n\x05\x04\0\x02b\x01\x12\x04\xBA\x01\x0F\x19\n\r\n\x05\x04\0\x02b\x03\x12\x04\xBA\x01\x1C\x1F\n\x19\n\x03\x04\0\x05\x12\x04\xBE\x01\x02\x18\x1A\x0C extensions\n\n\x0C\n\x04\x04\0\x05\0\x12\x04\xBE\x01\r\x17\n\r\n\x05\x04\0\x05\0\x01\x12\x04\xBE\x01\r\x10\n\r\n\x05\x04\0\x05\0\x02\x12\x04\xBE\x01\x14\x17\n\x0E\n\x04\x04\0\x02c\x12\x06\xC1\x01\x02\xC4\x01\x03\n\r\n\x05\x04\0\x02c\x04\x12\x04\xC1\x01\x02\n\n\r\n\x05\x04\0\x02c\x05\x12\x04\xC1\x01\x0B\x10\n\r\n\x05\x04\0\x02c\x01\x12\x04\xC1\x01\x11\x15\n\r\n\x05\x04\0\x02c\x03\x12\x04\xC1\x01\x18\x1B\n\x18\n\x04\x04\0\x03\x14\x12\x06\xC1\x01\x02\xC4\x01\x03\x1A\x08 groups\n\n\r\n\x05\x04\0\x03\x14\x01\x12\x04\xC1\x01\x11\x15\n\r\n\x05\x04\0\x02c\x06\x12\x04\xC1\x01\x11\x15\n\x0E\n\x06\x04\0\x03\x14\x02\0\x12\x04\xC2\x01\x04%\n\x0F\n\x07\x04\0\x03\x14\x02\0\x04\x12\x04\xC2\x01\x04\x0C\n\x0F\n\x07\x04\0\x03\x14\x02\0\x05\x12\x04\xC2\x01\r\x12\n\x0F\n\x07\x04\0\x03\x14\x02\0\x01\x12\x04\xC2\x01\x13\x1E\n\x0F\n\x07\x04\0\x03\x14\x02\0\x03\x12\x04\xC2\x01!$\n\x0E\n\x06\x04\0\x03\x14\x02\x01\x12\x04\xC3\x01\x04'\n\x0F\n\x07\x04\0\x03\x14\x02\x01\x04\x12\x04\xC3\x01\x04\x0C\n\x0F\n\x07\x04\0\x03\x14\x02\x01\x05\x12\x04\xC3\x01\r\x13\n\x0F\n\x07\x04\0\x03\x14\x02\x01\x01\x12\x04\xC3\x01\x14 \n\x0F\n\x07\x04\0\x03\x14\x02\x01\x03\x12\x04\xC3\x01#&\nr\n\x04\x04\0\x02d\x12\x04\xC8\x01\x02\"\x1Ad Test field-name-to-JSON-name convention.\n (protobuf says names can be any valid C/C++ identifier.)\n\n\r\n\x05\x04\0\x02d\x04\x12\x04\xC8\x01\x02\n\n\r\n\x05\x04\0\x02d\x05\x12\x04\xC8\x01\x0B\x10\n\r\n\x05\x04\0\x02d\x01\x12\x04\xC8\x01\x11\x1B\n\r\n\x05\x04\0\x02d\x03\x12\x04\xC8\x01\x1E!\n\x0C\n\x04\x04\0\x02e\x12\x04\xC9\x01\x02#\n\r\n\x05\x04\0\x02e\x04\x12\x04\xC9\x01\x02\n\n\r\n\x05\x04\0\x02e\x05\x12\x04\xC9\x01\x0B\x10\n\r\n\x05\x04\0\x02e\x01\x12\x04\xC9\x01\x11\x1C\n\r\n\x05\x04\0\x02e\x03\x12\x04\xC9\x01\x1F\"\n\x0C\n\x04\x04\0\x02f\x12\x04\xCA\x01\x02$\n\r\n\x05\x04\0\x02f\x04\x12\x04\xCA\x01\x02\n\n\r\n\x05\x04\0\x02f\x05\x12\x04\xCA\x01\x0B\x10\n\r\n\x05\x04\0\x02f\x01\x12\x04\xCA\x01\x11\x1D\n\r\n\x05\x04\0\x02f\x03\x12\x04\xCA\x01 #\n\x0C\n\x04\x04\0\x02g\x12\x04\xCB\x01\x02%\n\r\n\x05\x04\0\x02g\x04\x12\x04\xCB\x01\x02\n\n\r\n\x05\x04\0\x02g\x05\x12\x04\xCB\x01\x0B\x10\n\r\n\x05\x04\0\x02g\x01\x12\x04\xCB\x01\x11\x1E\n\r\n\x05\x04\0\x02g\x03\x12\x04\xCB\x01!$\n\x0C\n\x04\x04\0\x02h\x12\x04\xCC\x01\x02#\n\r\n\x05\x04\0\x02h\x04\x12\x04\xCC\x01\x02\n\n\r\n\x05\x04\0\x02h\x05\x12\x04\xCC\x01\x0B\x10\n\r\n\x05\x04\0\x02h\x01\x12\x04\xCC\x01\x11\x1C\n\r\n\x05\x04\0\x02h\x03\x12\x04\xCC\x01\x1F\"\n\x0C\n\x04\x04\0\x02i\x12\x04\xCD\x01\x02%\n\r\n\x05\x04\0\x02i\x04\x12\x04\xCD\x01\x02\n\n\r\n\x05\x04\0\x02i\x05\x12\x04\xCD\x01\x0B\x10\n\r\n\x05\x04\0\x02i\x01\x12\x04\xCD\x01\x11\x1E\n\r\n\x05\x04\0\x02i\x03\x12\x04\xCD\x01!$\n\x0C\n\x04\x04\0\x02j\x12\x04\xCE\x01\x02\"\n\r\n\x05\x04\0\x02j\x04\x12\x04\xCE\x01\x02\n\n\r\n\x05\x04\0\x02j\x05\x12\x04\xCE\x01\x0B\x10\n\r\n\x05\x04\0\x02j\x01\x12\x04\xCE\x01\x11\x1B\n\r\n\x05\x04\0\x02j\x03\x12\x04\xCE\x01\x1E!\n\x0C\n\x04\x04\0\x02k\x12\x04\xCF\x01\x02\"\n\r\n\x05\x04\0\x02k\x04\x12\x04\xCF\x01\x02\n\n\r\n\x05\x04\0\x02k\x05\x12\x04\xCF\x01\x0B\x10\n\r\n\x05\x04\0\x02k\x01\x12\x04\xCF\x01\x11\x1B\n\r\n\x05\x04\0\x02k\x03\x12\x04\xCF\x01\x1E!\n\x0C\n\x04\x04\0\x02l\x12\x04\xD0\x01\x02#\n\r\n\x05\x04\0\x02l\x04\x12\x04\xD0\x01\x02\n\n\r\n\x05\x04\0\x02l\x05\x12\x04\xD0\x01\x0B\x10\n\r\n\x05\x04\0\x02l\x01\x12\x04\xD0\x01\x11\x1C\n\r\n\x05\x04\0\x02l\x03\x12\x04\xD0\x01\x1F\"\n\x0C\n\x04\x04\0\x02m\x12\x04\xD1\x01\x02$\n\r\n\x05\x04\0\x02m\x04\x12\x04\xD1\x01\x02\n\n\r\n\x05\x04\0\x02m\x05\x12\x04\xD1\x01\x0B\x10\n\r\n\x05\x04\0\x02m\x01\x12\x04\xD1\x01\x11\x1D\n\r\n\x05\x04\0\x02m\x03\x12\x04\xD1\x01 #\n\x0C\n\x04\x04\0\x02n\x12\x04\xD2\x01\x02$\n\r\n\x05\x04\0\x02n\x04\x12\x04\xD2\x01\x02\n\n\r\n\x05\x04\0\x02n\x05\x12\x04\xD2\x01\x0B\x10\n\r\n\x05\x04\0\x02n\x01\x12\x04\xD2\x01\x11\x1D\n\r\n\x05\x04\0\x02n\x03\x12\x04\xD2\x01 #\n\x0C\n\x04\x04\0\x02o\x12\x04\xD3\x01\x02$\n\r\n\x05\x04\0\x02o\x04\x12\x04\xD3\x01\x02\n\n\r\n\x05\x04\0\x02o\x05\x12\x04\xD3\x01\x0B\x10\n\r\n\x05\x04\0\x02o\x01\x12\x04\xD3\x01\x11\x1D\n\r\n\x05\x04\0\x02o\x03\x12\x04\xD3\x01 #\n\x0C\n\x04\x04\0\x02p\x12\x04\xD4\x01\x02&\n\r\n\x05\x04\0\x02p\x04\x12\x04\xD4\x01\x02\n\n\r\n\x05\x04\0\x02p\x05\x12\x04\xD4\x01\x0B\x10\n\r\n\x05\x04\0\x02p\x01\x12\x04\xD4\x01\x11\x1F\n\r\n\x05\x04\0\x02p\x03\x12\x04\xD4\x01\"%\n\x0C\n\x04\x04\0\x02q\x12\x04\xD5\x01\x02&\n\r\n\x05\x04\0\x02q\x04\x12\x04\xD5\x01\x02\n\n\r\n\x05\x04\0\x02q\x05\x12\x04\xD5\x01\x0B\x10\n\r\n\x05\x04\0\x02q\x01\x12\x04\xD5\x01\x11\x1F\n\r\n\x05\x04\0\x02q\x03\x12\x04\xD5\x01\"%\n\x0C\n\x04\x04\0\x02r\x12\x04\xD6\x01\x02%\n\r\n\x05\x04\0\x02r\x04\x12\x04\xD6\x01\x02\n\n\r\n\x05\x04\0\x02r\x05\x12\x04\xD6\x01\x0B\x10\n\r\n\x05\x04\0\x02r\x01\x12\x04\xD6\x01\x11\x1E\n\r\n\x05\x04\0\x02r\x03\x12\x04\xD6\x01!$\n\x0C\n\x04\x04\0\x02s\x12\x04\xD7\x01\x02%\n\r\n\x05\x04\0\x02s\x04\x12\x04\xD7\x01\x02\n\n\r\n\x05\x04\0\x02s\x05\x12\x04\xD7\x01\x0B\x10\n\r\n\x05\x04\0\x02s\x01\x12\x04\xD7\x01\x11\x1E\n\r\n\x05\x04\0\x02s\x03\x12\x04\xD7\x01!$\n\x0C\n\x04\x04\0\x02t\x12\x04\xD8\x01\x02&\n\r\n\x05\x04\0\x02t\x04\x12\x04\xD8\x01\x02\n\n\r\n\x05\x04\0\x02t\x05\x12\x04\xD8\x01\x0B\x10\n\r\n\x05\x04\0\x02t\x01\x12\x04\xD8\x01\x11\x1F\n\r\n\x05\x04\0\x02t\x03\x12\x04\xD8\x01\"%\n\x0C\n\x04\x04\0\x02u\x12\x04\xD9\x01\x02&\n\r\n\x05\x04\0\x02u\x04\x12\x04\xD9\x01\x02\n\n\r\n\x05\x04\0\x02u\x05\x12\x04\xD9\x01\x0B\x10\n\r\n\x05\x04\0\x02u\x01\x12\x04\xD9\x01\x11\x1F\n\r\n\x05\x04\0\x02u\x03\x12\x04\xD9\x01\"%\n0\n\x03\x04\0\t\x12\x04\xDC\x01\x02\x18\x1A# Reserved for unknown fields test.\n\n\x0C\n\x04\x04\0\t\0\x12\x04\xDC\x01\x0B\x17\n\r\n\x05\x04\0\t\0\x01\x12\x04\xDC\x01\x0B\x0F\n\r\n\x05\x04\0\t\0\x02\x12\x04\xDC\x01\x13\x17\n(\n\x04\x04\0\x03\x15\x12\x06\xDF\x01\x02\xE3\x01\x03\x1A\x18 message_set test case.\n\n\r\n\x05\x04\0\x03\x15\x01\x12\x04\xDF\x01\n\x1B\n\r\n\x05\x04\0\x03\x15\x07\x12\x04\xE0\x01\x04*\n\x0E\n\x06\x04\0\x03\x15\x07\x01\x12\x04\xE0\x01\x04*\n\r\n\x05\x04\0\x03\x15\x05\x12\x04\xE2\x01\x04\x18\n\x0E\n\x06\x04\0\x03\x15\x05\0\x12\x04\xE2\x01\x0F\x17\n\x0F\n\x07\x04\0\x03\x15\x05\0\x01\x12\x04\xE2\x01\x0F\x10\n\x0F\n\x07\x04\0\x03\x15\x05\0\x02\x12\x04\xE2\x01\x14\x17\n\x0E\n\x04\x04\0\x03\x16\x12\x06\xE5\x01\x02\xEA\x01\x03\n\r\n\x05\x04\0\x03\x16\x01\x12\x04\xE5\x01\n%\n\x0F\n\x05\x04\0\x03\x16\x06\x12\x06\xE6\x01\x04\xE8\x01\x05\n\x0E\n\x06\x04\0\x03\x16\x06\0\x12\x04\xE7\x01\x06K\n\x0F\n\x07\x04\0\x03\x16\x06\0\x02\x12\x04\xE6\x01\x0B\x1C\n\x0F\n\x07\x04\0\x03\x16\x06\0\x04\x12\x04\xE7\x01\x06\x0E\n\x0F\n\x07\x04\0\x03\x16\x06\0\x06\x12\x04\xE7\x01\x0F*\n\x0F\n\x07\x04\0\x03\x16\x06\0\x01\x12\x04\xE7\x01+@\n\x0F\n\x07\x04\0\x03\x16\x06\0\x03\x12\x04\xE7\x01CJ\n\x0E\n\x06\x04\0\x03\x16\x02\0\x12\x04\xE9\x01\x04\x1D\n\x0F\n\x07\x04\0\x03\x16\x02\0\x04\x12\x04\xE9\x01\x04\x0C\n\x0F\n\x07\x04\0\x03\x16\x02\0\x05\x12\x04\xE9\x01\r\x13\n\x0F\n\x07\x04\0\x03\x16\x02\0\x01\x12\x04\xE9\x01\x14\x17\n\x0F\n\x07\x04\0\x03\x16\x02\0\x03\x12\x04\xE9\x01\x1A\x1C\n\x0E\n\x04\x04\0\x03\x17\x12\x06\xEC\x01\x02\xF1\x01\x03\n\r\n\x05\x04\0\x03\x17\x01\x12\x04\xEC\x01\n%\n\x0F\n\x05\x04\0\x03\x17\x06\x12\x06\xED\x01\x04\xEF\x01\x05\n\x0E\n\x06\x04\0\x03\x17\x06\0\x12\x04\xEE\x01\x06K\n\x0F\n\x07\x04\0\x03\x17\x06\0\x02\x12\x04\xED\x01\x0B\x1C\n\x0F\n\x07\x04\0\x03\x17\x06\0\x04\x12\x04\xEE\x01\x06\x0E\n\x0F\n\x07\x04\0\x03\x17\x06\0\x06\x12\x04\xEE\x01\x0F*\n\x0F\n\x07\x04\0\x03\x17\x06\0\x01\x12\x04\xEE\x01+@\n\x0F\n\x07\x04\0\x03\x17\x06\0\x03\x12\x04\xEE\x01CJ\n\x0E\n\x06\x04\0\x03\x17\x02\0\x12\x04\xF0\x01\x04\x19\n\x0F\n\x07\x04\0\x03\x17\x02\0\x04\x12\x04\xF0\x01\x04\x0C\n\x0F\n\x07\x04\0\x03\x17\x02\0\x05\x12\x04\xF0\x01\r\x12\n\x0F\n\x07\x04\0\x03\x17\x02\0\x01\x12\x04\xF0\x01\x13\x14\n\x0F\n\x07\x04\0\x03\x17\x02\0\x03\x12\x04\xF0\x01\x17\x18\n\x0C\n\x02\x04\x01\x12\x06\xF4\x01\0\xF6\x01\x01\n\x0B\n\x03\x04\x01\x01\x12\x04\xF4\x01\x08\x1C\n\x0C\n\x04\x04\x01\x02\0\x12\x04\xF5\x01\x02\x17\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\xF5\x01\x02\n\n\r\n\x05\x04\x01\x02\0\x05\x12\x04\xF5\x01\x0B\x10\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\xF5\x01\x11\x12\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xF5\x01\x15\x16\n\x0C\n\x02\x05\0\x12\x06\xF8\x01\0\xFC\x01\x01\n\x0B\n\x03\x05\0\x01\x12\x04\xF8\x01\x05\x16\n\x0C\n\x04\x05\0\x02\0\x12\x04\xF9\x01\x02\x12\n\r\n\x05\x05\0\x02\0\x01\x12\x04\xF9\x01\x02\r\n\r\n\x05\x05\0\x02\0\x02\x12\x04\xF9\x01\x10\x11\n\x0C\n\x04\x05\0\x02\x01\x12\x04\xFA\x01\x02\x12\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\xFA\x01\x02\r\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xFA\x01\x10\x11\n\x0C\n\x04\x05\0\x02\x02\x12\x04\xFB\x01\x02\x12\n\r\n\x05\x05\0\x02\x02\x01\x12\x04\xFB\x01\x02\r\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\xFB\x01\x10\x11\n\x0B\n\x01\x07\x12\x06\xFE\x01\0\x80\x02\x01\n\n\n\x02\x07\0\x12\x04\xFF\x01\x02'\n\x0B\n\x03\x07\0\x02\x12\x04\xFE\x01\x07\x19\n\x0B\n\x03\x07\0\x04\x12\x04\xFF\x01\x02\n\n\x0B\n\x03\x07\0\x05\x12\x04\xFF\x01\x0B\x10\n\x0B\n\x03\x07\0\x01\x12\x04\xFF\x01\x11 \n\x0B\n\x03\x07\0\x03\x12\x04\xFF\x01#&\n\x0C\n\x02\x04\x02\x12\x06\x82\x02\0\x8B\x02\x01\n\x0B\n\x03\x04\x02\x01\x12\x04\x82\x02\x08\x1D\n\x0C\n\x04\x04\x02\x02\0\x12\x04\x83\x02\x02'\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x83\x02\x02\n\n\r\n\x05\x04\x02\x02\0\x05\x12\x04\x83\x02\x0B\x10\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\x83\x02\x11\x1F\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x83\x02\"&\n\x0C\n\x04\x04\x02\x02\x01\x12\x04\x84\x02\x02)\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x84\x02\x02\n\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\x84\x02\x0B\x11\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\x84\x02\x12!\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x84\x02$(\n\x0C\n\x04\x04\x02\x02\x02\x12\x04\x85\x02\x026\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x85\x02\x02\n\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\x85\x02\x0B\x1F\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\x85\x02 .\n\r\n\x05\x04\x02\x02\x02\x03\x12\x04\x85\x0215\n\x0E\n\x04\x04\x02\x02\x03\x12\x06\x86\x02\x02\x88\x02\x03\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\x86\x02\x02\n\n\r\n\x05\x04\x02\x02\x03\x05\x12\x04\x86\x02\x0B\x10\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\x86\x02\x11\x1E\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\x86\x02!%\n\x0E\n\x04\x04\x02\x03\0\x12\x06\x86\x02\x02\x88\x02\x03\n\r\n\x05\x04\x02\x03\0\x01\x12\x04\x86\x02\x11\x1E\n\r\n\x05\x04\x02\x02\x03\x06\x12\x04\x86\x02\x11\x1E\n\x0E\n\x06\x04\x02\x03\0\x02\0\x12\x04\x87\x02\x04\x19\n\x0F\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04\x87\x02\x04\x0C\n\x0F\n\x07\x04\x02\x03\0\x02\0\x05\x12\x04\x87\x02\r\x12\n\x0F\n\x07\x04\x02\x03\0\x02\0\x01\x12\x04\x87\x02\x13\x14\n\x0F\n\x07\x04\x02\x03\0\x02\0\x03\x12\x04\x87\x02\x17\x18\n\x0C\n\x04\x04\x02\x02\x04\x12\x04\x89\x02\x02%\n\r\n\x05\x04\x02\x02\x04\x04\x12\x04\x89\x02\x02\n\n\r\n\x05\x04\x02\x02\x04\x05\x12\x04\x89\x02\x0B\x0F\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\x89\x02\x10\x1D\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\x89\x02 $\n\x0C\n\x04\x04\x02\x02\x05\x12\x04\x8A\x02\x02'\n\r\n\x05\x04\x02\x02\x05\x04\x12\x04\x8A\x02\x02\n\n\r\n\x05\x04\x02\x02\x05\x05\x12\x04\x8A\x02\x0B\x10\n\r\n\x05\x04\x02\x02\x05\x01\x12\x04\x8A\x02\x11\x1F\n\r\n\x05\x04\x02\x02\x05\x03\x12\x04\x8A\x02\"&" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
