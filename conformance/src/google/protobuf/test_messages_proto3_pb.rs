#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ForeignEnum(i32);
impl pecan::Enumerate for ForeignEnum {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> ForeignEnum {
        ForeignEnum(v)
    }
}
impl ForeignEnum {
    pub const FOREIGN_FOO: ForeignEnum = ForeignEnum(0);
    pub const FOREIGN_BAR: ForeignEnum = ForeignEnum(1);
    pub const FOREIGN_BAZ: ForeignEnum = ForeignEnum(2);
    pub const fn new() -> ForeignEnum {
        ForeignEnum(0)
    }
}
impl std::fmt::Debug for ForeignEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FOREIGN_FOO"),
            1 => write!(f, "FOREIGN_BAR"),
            2 => write!(f, "FOREIGN_BAZ"),
            v => write!(f, "ForeignEnum({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TestAllTypesProto3_NestedEnum(i32);
impl pecan::Enumerate for TestAllTypesProto3_NestedEnum {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> TestAllTypesProto3_NestedEnum {
        TestAllTypesProto3_NestedEnum(v)
    }
}
impl TestAllTypesProto3_NestedEnum {
    pub const FOO: TestAllTypesProto3_NestedEnum = TestAllTypesProto3_NestedEnum(0);
    pub const BAR: TestAllTypesProto3_NestedEnum = TestAllTypesProto3_NestedEnum(1);
    pub const BAZ: TestAllTypesProto3_NestedEnum = TestAllTypesProto3_NestedEnum(2);
    pub const NEG: TestAllTypesProto3_NestedEnum = TestAllTypesProto3_NestedEnum(-1);
    pub const fn new() -> TestAllTypesProto3_NestedEnum {
        TestAllTypesProto3_NestedEnum(0)
    }
}
impl std::fmt::Debug for TestAllTypesProto3_NestedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FOO"),
            1 => write!(f, "BAR"),
            2 => write!(f, "BAZ"),
            -1 => write!(f, "NEG"),
            v => write!(f, "TestAllTypesProto3_NestedEnum({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TestAllTypesProto3_AliasedEnum(i32);
impl pecan::Enumerate for TestAllTypesProto3_AliasedEnum {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> TestAllTypesProto3_AliasedEnum {
        TestAllTypesProto3_AliasedEnum(v)
    }
}
impl TestAllTypesProto3_AliasedEnum {
    pub const ALIAS_FOO: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(0);
    pub const ALIAS_BAR: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(1);
    pub const ALIAS_BAZ: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(2);
    pub const QUX: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(2);
    pub const qux: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(2);
    pub const bAz: TestAllTypesProto3_AliasedEnum = TestAllTypesProto3_AliasedEnum(2);
    pub const fn new() -> TestAllTypesProto3_AliasedEnum {
        TestAllTypesProto3_AliasedEnum(0)
    }
}
impl std::fmt::Debug for TestAllTypesProto3_AliasedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "ALIAS_FOO"),
            1 => write!(f, "ALIAS_BAR"),
            2 => write!(f, "ALIAS_BAZ"),
            v => write!(f, "TestAllTypesProto3_AliasedEnum({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto3_NestedMessage {
    pub a: i32,
    pub corecursive: Option<Box<TestAllTypesProto3>>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto3_NestedMessage {
    pub const fn new() -> TestAllTypesProto3_NestedMessage {
        TestAllTypesProto3_NestedMessage {
            a: 0,
            corecursive: None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn corecursive(&self) -> &TestAllTypesProto3 {
        match &self.corecursive {
            Some(v) => v,
            _ => TestAllTypesProto3::default_instance(),
        }
    }
    pub fn corecursive_mut(&mut self) -> &mut TestAllTypesProto3 {
        self.corecursive.get_or_insert_with(Default::default)
    }
    pub fn set_corecursive(&mut self, val: Box<TestAllTypesProto3>) {
        self.corecursive = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto3_NestedMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.a = Varint::read_from(s)?,
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
        if self.a != 0 {
            s.write_tag(8)?;
            Varint::write_to(self.a, s)?;
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
        if self.a != 0 {
            l += 1 + Varint::size(self.a);
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
    fn clear(&mut self) {
        self.a = 0;
        self.corecursive = None;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for TestAllTypesProto3_NestedMessage {
    fn default_instance() -> &'static TestAllTypesProto3_NestedMessage {
        static DEFAULT: TestAllTypesProto3_NestedMessage = TestAllTypesProto3_NestedMessage::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto3_NestedMessage {
    #[inline]
    fn default() -> TestAllTypesProto3_NestedMessage {
        TestAllTypesProto3_NestedMessage::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum TestAllTypesProto3_Oneof_Field {
    OneofUint32(u32),
    OneofNestedMessage(TestAllTypesProto3_NestedMessage),
    OneofString(String),
    OneofBytes(pecan::Bytes),
    OneofBool(bool),
    OneofUint64(u64),
    OneofFloat(f32),
    OneofDouble(f64),
    OneofEnum(TestAllTypesProto3_NestedEnum),
    OneofNullValue(pecan_types::google::protobuf::struct_pb::NullValue),
    None,
}
impl Default for TestAllTypesProto3_Oneof_Field {
    #[inline]
    fn default() -> TestAllTypesProto3_Oneof_Field {
        TestAllTypesProto3_Oneof_Field::None
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestAllTypesProto3 {
    pub optional_int32: i32,
    pub optional_int64: i64,
    pub optional_uint32: u32,
    pub optional_uint64: u64,
    pub optional_sint32: i32,
    pub optional_sint64: i64,
    pub optional_fixed32: u32,
    pub optional_fixed64: u64,
    pub optional_sfixed32: i32,
    pub optional_sfixed64: i64,
    pub optional_float: f32,
    pub optional_double: f64,
    pub optional_bool: bool,
    pub optional_string: String,
    pub optional_bytes: pecan::Bytes,
    pub optional_nested_message: Option<TestAllTypesProto3_NestedMessage>,
    pub optional_foreign_message: Option<ForeignMessage>,
    pub optional_nested_enum: TestAllTypesProto3_NestedEnum,
    pub optional_foreign_enum: ForeignEnum,
    pub optional_aliased_enum: TestAllTypesProto3_AliasedEnum,
    pub optional_string_piece: String,
    pub optional_cord: String,
    pub recursive_message: Option<Box<TestAllTypesProto3>>,
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
    pub repeated_nested_message: Vec<TestAllTypesProto3_NestedMessage>,
    pub repeated_foreign_message: Vec<ForeignMessage>,
    pub repeated_nested_enum: Vec<TestAllTypesProto3_NestedEnum>,
    pub repeated_foreign_enum: Vec<ForeignEnum>,
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
    pub packed_nested_enum: Vec<TestAllTypesProto3_NestedEnum>,
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
    pub unpacked_nested_enum: Vec<TestAllTypesProto3_NestedEnum>,
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
    pub map_string_nested_message: Option<pecan::HashMap<String, TestAllTypesProto3_NestedMessage>>,
    pub map_string_foreign_message: Option<pecan::HashMap<String, ForeignMessage>>,
    pub map_string_nested_enum: Option<pecan::HashMap<String, TestAllTypesProto3_NestedEnum>>,
    pub map_string_foreign_enum: Option<pecan::HashMap<String, ForeignEnum>>,
    pub oneof_field: TestAllTypesProto3_Oneof_Field,
    pub optional_bool_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::BoolValue>,
    pub optional_int32_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::Int32Value>,
    pub optional_int64_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::Int64Value>,
    pub optional_uint32_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::UInt32Value>,
    pub optional_uint64_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::UInt64Value>,
    pub optional_float_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::FloatValue>,
    pub optional_double_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::DoubleValue>,
    pub optional_string_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::StringValue>,
    pub optional_bytes_wrapper: Option<pecan_types::google::protobuf::wrappers_pb::BytesValue>,
    pub repeated_bool_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::BoolValue>,
    pub repeated_int32_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::Int32Value>,
    pub repeated_int64_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::Int64Value>,
    pub repeated_uint32_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::UInt32Value>,
    pub repeated_uint64_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::UInt64Value>,
    pub repeated_float_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::FloatValue>,
    pub repeated_double_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::DoubleValue>,
    pub repeated_string_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::StringValue>,
    pub repeated_bytes_wrapper: Vec<pecan_types::google::protobuf::wrappers_pb::BytesValue>,
    pub optional_duration: Option<pecan_types::google::protobuf::duration_pb::Duration>,
    pub optional_timestamp: Option<pecan_types::google::protobuf::timestamp_pb::Timestamp>,
    pub optional_field_mask: Option<pecan_types::google::protobuf::field_mask_pb::FieldMask>,
    pub optional_struct: Option<pecan_types::google::protobuf::struct_pb::Struct>,
    pub optional_any: Option<pecan_types::google::protobuf::any_pb::Any>,
    pub optional_value: Option<pecan_types::google::protobuf::struct_pb::Value>,
    pub optional_null_value: pecan_types::google::protobuf::struct_pb::NullValue,
    pub repeated_duration: Vec<pecan_types::google::protobuf::duration_pb::Duration>,
    pub repeated_timestamp: Vec<pecan_types::google::protobuf::timestamp_pb::Timestamp>,
    pub repeated_fieldmask: Vec<pecan_types::google::protobuf::field_mask_pb::FieldMask>,
    pub repeated_struct: Vec<pecan_types::google::protobuf::struct_pb::Struct>,
    pub repeated_any: Vec<pecan_types::google::protobuf::any_pb::Any>,
    pub repeated_value: Vec<pecan_types::google::protobuf::struct_pb::Value>,
    pub repeated_list_value: Vec<pecan_types::google::protobuf::struct_pb::ListValue>,
    pub fieldname1: i32,
    pub field_name2: i32,
    pub _field_name3: i32,
    pub field__name4_: i32,
    pub field0name5: i32,
    pub field_0_name6: i32,
    pub field_name7: i32,
    pub field_name8: i32,
    pub field_name9: i32,
    pub field_name10: i32,
    pub field_name11: i32,
    pub field_name12: i32,
    pub __field_name13: i32,
    pub __field_name14: i32,
    pub field__name15: i32,
    pub field__name16: i32,
    pub field_name17__: i32,
    pub field_name18__: i32,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl TestAllTypesProto3 {
    pub const fn new() -> TestAllTypesProto3 {
        TestAllTypesProto3 {
            optional_int32: 0,
            optional_int64: 0,
            optional_uint32: 0,
            optional_uint64: 0,
            optional_sint32: 0,
            optional_sint64: 0,
            optional_fixed32: 0,
            optional_fixed64: 0,
            optional_sfixed32: 0,
            optional_sfixed64: 0,
            optional_float: 0f32,
            optional_double: 0f64,
            optional_bool: false,
            optional_string: String::new(),
            optional_bytes: pecan::Bytes::new(),
            optional_nested_message: None,
            optional_foreign_message: None,
            optional_nested_enum: TestAllTypesProto3_NestedEnum::new(),
            optional_foreign_enum: ForeignEnum::new(),
            optional_aliased_enum: TestAllTypesProto3_AliasedEnum::new(),
            optional_string_piece: String::new(),
            optional_cord: String::new(),
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
            oneof_field: TestAllTypesProto3_Oneof_Field::None,
            optional_bool_wrapper: None,
            optional_int32_wrapper: None,
            optional_int64_wrapper: None,
            optional_uint32_wrapper: None,
            optional_uint64_wrapper: None,
            optional_float_wrapper: None,
            optional_double_wrapper: None,
            optional_string_wrapper: None,
            optional_bytes_wrapper: None,
            repeated_bool_wrapper: Vec::new(),
            repeated_int32_wrapper: Vec::new(),
            repeated_int64_wrapper: Vec::new(),
            repeated_uint32_wrapper: Vec::new(),
            repeated_uint64_wrapper: Vec::new(),
            repeated_float_wrapper: Vec::new(),
            repeated_double_wrapper: Vec::new(),
            repeated_string_wrapper: Vec::new(),
            repeated_bytes_wrapper: Vec::new(),
            optional_duration: None,
            optional_timestamp: None,
            optional_field_mask: None,
            optional_struct: None,
            optional_any: None,
            optional_value: None,
            optional_null_value: pecan_types::google::protobuf::struct_pb::NullValue::new(),
            repeated_duration: Vec::new(),
            repeated_timestamp: Vec::new(),
            repeated_fieldmask: Vec::new(),
            repeated_struct: Vec::new(),
            repeated_any: Vec::new(),
            repeated_value: Vec::new(),
            repeated_list_value: Vec::new(),
            fieldname1: 0,
            field_name2: 0,
            _field_name3: 0,
            field__name4_: 0,
            field0name5: 0,
            field_0_name6: 0,
            field_name7: 0,
            field_name8: 0,
            field_name9: 0,
            field_name10: 0,
            field_name11: 0,
            field_name12: 0,
            __field_name13: 0,
            __field_name14: 0,
            field__name15: 0,
            field__name16: 0,
            field_name17__: 0,
            field_name18__: 0,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn optional_nested_message(&self) -> &TestAllTypesProto3_NestedMessage {
        match &self.optional_nested_message {
            Some(v) => v,
            _ => TestAllTypesProto3_NestedMessage::default_instance(),
        }
    }
    pub fn optional_nested_message_mut(&mut self) -> &mut TestAllTypesProto3_NestedMessage {
        self.optional_nested_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_nested_message(&mut self, val: TestAllTypesProto3_NestedMessage) {
        self.optional_nested_message = Some(val);
    }
    pub fn optional_foreign_message(&self) -> &ForeignMessage {
        match &self.optional_foreign_message {
            Some(v) => v,
            _ => ForeignMessage::default_instance(),
        }
    }
    pub fn optional_foreign_message_mut(&mut self) -> &mut ForeignMessage {
        self.optional_foreign_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_foreign_message(&mut self, val: ForeignMessage) {
        self.optional_foreign_message = Some(val);
    }
    pub fn recursive_message(&self) -> &TestAllTypesProto3 {
        match &self.recursive_message {
            Some(v) => v,
            _ => TestAllTypesProto3::default_instance(),
        }
    }
    pub fn recursive_message_mut(&mut self) -> &mut TestAllTypesProto3 {
        self.recursive_message.get_or_insert_with(Default::default)
    }
    pub fn set_recursive_message(&mut self, val: Box<TestAllTypesProto3>) {
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
    ) -> &pecan::HashMap<String, TestAllTypesProto3_NestedMessage> {
        match &self.map_string_nested_message {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , TestAllTypesProto3_NestedMessage > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_nested_message_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, TestAllTypesProto3_NestedMessage> {
        self.map_string_nested_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_nested_message(
        &mut self,
        val: pecan::HashMap<String, TestAllTypesProto3_NestedMessage>,
    ) {
        self.map_string_nested_message = Some(val);
    }
    pub fn map_string_foreign_message(&self) -> &pecan::HashMap<String, ForeignMessage> {
        match &self.map_string_foreign_message {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , ForeignMessage > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_foreign_message_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, ForeignMessage> {
        self.map_string_foreign_message
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_foreign_message(&mut self, val: pecan::HashMap<String, ForeignMessage>) {
        self.map_string_foreign_message = Some(val);
    }
    pub fn map_string_nested_enum(&self) -> &pecan::HashMap<String, TestAllTypesProto3_NestedEnum> {
        match &self.map_string_nested_enum {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , TestAllTypesProto3_NestedEnum > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_nested_enum_mut(
        &mut self,
    ) -> &mut pecan::HashMap<String, TestAllTypesProto3_NestedEnum> {
        self.map_string_nested_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_nested_enum(
        &mut self,
        val: pecan::HashMap<String, TestAllTypesProto3_NestedEnum>,
    ) {
        self.map_string_nested_enum = Some(val);
    }
    pub fn map_string_foreign_enum(&self) -> &pecan::HashMap<String, ForeignEnum> {
        match &self.map_string_foreign_enum {
            Some(v) => v,
            _ => {
                pecan::lazy_static! { static ref DEFAULT : pecan :: HashMap < String , ForeignEnum > = pecan :: HashMap :: default () ; }
                &*DEFAULT
            }
        }
    }
    pub fn map_string_foreign_enum_mut(&mut self) -> &mut pecan::HashMap<String, ForeignEnum> {
        self.map_string_foreign_enum
            .get_or_insert_with(Default::default)
    }
    pub fn set_map_string_foreign_enum(&mut self, val: pecan::HashMap<String, ForeignEnum>) {
        self.map_string_foreign_enum = Some(val);
    }
    pub fn oneof_uint32(&self) -> u32 {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofUint32(v) => *v,
            _ => 0,
        }
    }
    pub fn oneof_uint32_mut(&mut self) -> &mut u32 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofUint32(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofUint32(0);
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofUint32(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_uint32(&mut self, val: u32) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofUint32(val);
    }
    pub fn oneof_nested_message(&self) -> &TestAllTypesProto3_NestedMessage {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofNestedMessage(v) => v,
            _ => TestAllTypesProto3_NestedMessage::default_instance(),
        }
    }
    pub fn oneof_nested_message_mut(&mut self) -> &mut TestAllTypesProto3_NestedMessage {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofNestedMessage(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofNestedMessage(
                TestAllTypesProto3_NestedMessage::new(),
            );
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofNestedMessage(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_nested_message(&mut self, val: TestAllTypesProto3_NestedMessage) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofNestedMessage(val);
    }
    pub fn oneof_string(&self) -> &String {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofString(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn oneof_string_mut(&mut self) -> &mut String {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofString(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofString(String::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofString(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_string(&mut self, val: String) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofString(val);
    }
    pub fn oneof_bytes(&self) -> &pecan::Bytes {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofBytes(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn oneof_bytes_mut(&mut self) -> &mut pecan::Bytes {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofBytes(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofBytes(pecan::Bytes::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofBytes(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_bytes(&mut self, val: pecan::Bytes) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofBytes(val);
    }
    pub fn oneof_bool(&self) -> bool {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofBool(v) => *v,
            _ => false,
        }
    }
    pub fn oneof_bool_mut(&mut self) -> &mut bool {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofBool(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofBool(false);
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofBool(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_bool(&mut self, val: bool) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofBool(val);
    }
    pub fn oneof_uint64(&self) -> u64 {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofUint64(v) => *v,
            _ => 0,
        }
    }
    pub fn oneof_uint64_mut(&mut self) -> &mut u64 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofUint64(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofUint64(0);
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofUint64(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_uint64(&mut self, val: u64) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofUint64(val);
    }
    pub fn oneof_float(&self) -> f32 {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofFloat(v) => *v,
            _ => 0f32,
        }
    }
    pub fn oneof_float_mut(&mut self) -> &mut f32 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofFloat(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofFloat(0f32);
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofFloat(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_float(&mut self, val: f32) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofFloat(val);
    }
    pub fn oneof_double(&self) -> f64 {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofDouble(v) => *v,
            _ => 0f64,
        }
    }
    pub fn oneof_double_mut(&mut self) -> &mut f64 {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofDouble(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofDouble(0f64);
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofDouble(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_double(&mut self, val: f64) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofDouble(val);
    }
    pub fn oneof_enum(&self) -> TestAllTypesProto3_NestedEnum {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofEnum(v) => *v,
            _ => TestAllTypesProto3_NestedEnum::new(),
        }
    }
    pub fn oneof_enum_mut(&mut self) -> &mut TestAllTypesProto3_NestedEnum {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofEnum(_)
        ) {
            self.oneof_field =
                TestAllTypesProto3_Oneof_Field::OneofEnum(TestAllTypesProto3_NestedEnum::new());
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofEnum(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_enum(&mut self, val: TestAllTypesProto3_NestedEnum) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofEnum(val);
    }
    pub fn oneof_null_value(&self) -> pecan_types::google::protobuf::struct_pb::NullValue {
        match &self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofNullValue(v) => *v,
            _ => pecan_types::google::protobuf::struct_pb::NullValue::new(),
        }
    }
    pub fn oneof_null_value_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::struct_pb::NullValue {
        if !matches!(
            self.oneof_field,
            TestAllTypesProto3_Oneof_Field::OneofNullValue(_)
        ) {
            self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofNullValue(
                pecan_types::google::protobuf::struct_pb::NullValue::new(),
            );
        }
        match &mut self.oneof_field {
            TestAllTypesProto3_Oneof_Field::OneofNullValue(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_oneof_null_value(
        &mut self,
        val: pecan_types::google::protobuf::struct_pb::NullValue,
    ) {
        self.oneof_field = TestAllTypesProto3_Oneof_Field::OneofNullValue(val);
    }
    pub fn optional_bool_wrapper(&self) -> &pecan_types::google::protobuf::wrappers_pb::BoolValue {
        match &self.optional_bool_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::BoolValue::default_instance(),
        }
    }
    pub fn optional_bool_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::BoolValue {
        self.optional_bool_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_bool_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::BoolValue,
    ) {
        self.optional_bool_wrapper = Some(val);
    }
    pub fn optional_int32_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::Int32Value {
        match &self.optional_int32_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::Int32Value::default_instance(),
        }
    }
    pub fn optional_int32_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::Int32Value {
        self.optional_int32_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_int32_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::Int32Value,
    ) {
        self.optional_int32_wrapper = Some(val);
    }
    pub fn optional_int64_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::Int64Value {
        match &self.optional_int64_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::Int64Value::default_instance(),
        }
    }
    pub fn optional_int64_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::Int64Value {
        self.optional_int64_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_int64_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::Int64Value,
    ) {
        self.optional_int64_wrapper = Some(val);
    }
    pub fn optional_uint32_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::UInt32Value {
        match &self.optional_uint32_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::UInt32Value::default_instance(),
        }
    }
    pub fn optional_uint32_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::UInt32Value {
        self.optional_uint32_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_uint32_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::UInt32Value,
    ) {
        self.optional_uint32_wrapper = Some(val);
    }
    pub fn optional_uint64_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::UInt64Value {
        match &self.optional_uint64_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::UInt64Value::default_instance(),
        }
    }
    pub fn optional_uint64_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::UInt64Value {
        self.optional_uint64_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_uint64_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::UInt64Value,
    ) {
        self.optional_uint64_wrapper = Some(val);
    }
    pub fn optional_float_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::FloatValue {
        match &self.optional_float_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::FloatValue::default_instance(),
        }
    }
    pub fn optional_float_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::FloatValue {
        self.optional_float_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_float_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::FloatValue,
    ) {
        self.optional_float_wrapper = Some(val);
    }
    pub fn optional_double_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::DoubleValue {
        match &self.optional_double_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::DoubleValue::default_instance(),
        }
    }
    pub fn optional_double_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::DoubleValue {
        self.optional_double_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_double_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::DoubleValue,
    ) {
        self.optional_double_wrapper = Some(val);
    }
    pub fn optional_string_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::StringValue {
        match &self.optional_string_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::StringValue::default_instance(),
        }
    }
    pub fn optional_string_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::StringValue {
        self.optional_string_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_string_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::StringValue,
    ) {
        self.optional_string_wrapper = Some(val);
    }
    pub fn optional_bytes_wrapper(
        &self,
    ) -> &pecan_types::google::protobuf::wrappers_pb::BytesValue {
        match &self.optional_bytes_wrapper {
            Some(v) => v,
            _ => pecan_types::google::protobuf::wrappers_pb::BytesValue::default_instance(),
        }
    }
    pub fn optional_bytes_wrapper_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::wrappers_pb::BytesValue {
        self.optional_bytes_wrapper
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_bytes_wrapper(
        &mut self,
        val: pecan_types::google::protobuf::wrappers_pb::BytesValue,
    ) {
        self.optional_bytes_wrapper = Some(val);
    }
    pub fn optional_duration(&self) -> &pecan_types::google::protobuf::duration_pb::Duration {
        match &self.optional_duration {
            Some(v) => v,
            _ => pecan_types::google::protobuf::duration_pb::Duration::default_instance(),
        }
    }
    pub fn optional_duration_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::duration_pb::Duration {
        self.optional_duration.get_or_insert_with(Default::default)
    }
    pub fn set_optional_duration(
        &mut self,
        val: pecan_types::google::protobuf::duration_pb::Duration,
    ) {
        self.optional_duration = Some(val);
    }
    pub fn optional_timestamp(&self) -> &pecan_types::google::protobuf::timestamp_pb::Timestamp {
        match &self.optional_timestamp {
            Some(v) => v,
            _ => pecan_types::google::protobuf::timestamp_pb::Timestamp::default_instance(),
        }
    }
    pub fn optional_timestamp_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::timestamp_pb::Timestamp {
        self.optional_timestamp.get_or_insert_with(Default::default)
    }
    pub fn set_optional_timestamp(
        &mut self,
        val: pecan_types::google::protobuf::timestamp_pb::Timestamp,
    ) {
        self.optional_timestamp = Some(val);
    }
    pub fn optional_field_mask(&self) -> &pecan_types::google::protobuf::field_mask_pb::FieldMask {
        match &self.optional_field_mask {
            Some(v) => v,
            _ => pecan_types::google::protobuf::field_mask_pb::FieldMask::default_instance(),
        }
    }
    pub fn optional_field_mask_mut(
        &mut self,
    ) -> &mut pecan_types::google::protobuf::field_mask_pb::FieldMask {
        self.optional_field_mask
            .get_or_insert_with(Default::default)
    }
    pub fn set_optional_field_mask(
        &mut self,
        val: pecan_types::google::protobuf::field_mask_pb::FieldMask,
    ) {
        self.optional_field_mask = Some(val);
    }
    pub fn optional_struct(&self) -> &pecan_types::google::protobuf::struct_pb::Struct {
        match &self.optional_struct {
            Some(v) => v,
            _ => pecan_types::google::protobuf::struct_pb::Struct::default_instance(),
        }
    }
    pub fn optional_struct_mut(&mut self) -> &mut pecan_types::google::protobuf::struct_pb::Struct {
        self.optional_struct.get_or_insert_with(Default::default)
    }
    pub fn set_optional_struct(&mut self, val: pecan_types::google::protobuf::struct_pb::Struct) {
        self.optional_struct = Some(val);
    }
    pub fn optional_any(&self) -> &pecan_types::google::protobuf::any_pb::Any {
        match &self.optional_any {
            Some(v) => v,
            _ => pecan_types::google::protobuf::any_pb::Any::default_instance(),
        }
    }
    pub fn optional_any_mut(&mut self) -> &mut pecan_types::google::protobuf::any_pb::Any {
        self.optional_any.get_or_insert_with(Default::default)
    }
    pub fn set_optional_any(&mut self, val: pecan_types::google::protobuf::any_pb::Any) {
        self.optional_any = Some(val);
    }
    pub fn optional_value(&self) -> &pecan_types::google::protobuf::struct_pb::Value {
        match &self.optional_value {
            Some(v) => v,
            _ => pecan_types::google::protobuf::struct_pb::Value::default_instance(),
        }
    }
    pub fn optional_value_mut(&mut self) -> &mut pecan_types::google::protobuf::struct_pb::Value {
        self.optional_value.get_or_insert_with(Default::default)
    }
    pub fn set_optional_value(&mut self, val: pecan_types::google::protobuf::struct_pb::Value) {
        self.optional_value = Some(val);
    }
}
impl pecan::Message for TestAllTypesProto3 {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.optional_int32 = Varint::read_from(s)?,
                16 => self.optional_int64 = Varint::read_from(s)?,
                24 => self.optional_uint32 = Varint::read_from(s)?,
                32 => self.optional_uint64 = Varint::read_from(s)?,
                40 => self.optional_sint32 = ZigZag::read_from(s)?,
                48 => self.optional_sint64 = ZigZag::read_from(s)?,
                61 => self.optional_fixed32 = Fixed32::read_from(s)?,
                65 => self.optional_fixed64 = Fixed64::read_from(s)?,
                77 => self.optional_sfixed32 = Fixed32::read_from(s)?,
                81 => self.optional_sfixed64 = Fixed64::read_from(s)?,
                93 => self.optional_float = Fixed32::read_from(s)?,
                97 => self.optional_double = Fixed64::read_from(s)?,
                104 => self.optional_bool = Varint::read_from(s)?,
                114 => LengthPrefixed::merge_from(&mut self.optional_string, s)?,
                122 => LengthPrefixed::merge_from(&mut self.optional_bytes, s)?,
                146 => LengthPrefixed::merge_from(self.optional_nested_message_mut(), s)?,
                154 => LengthPrefixed::merge_from(self.optional_foreign_message_mut(), s)?,
                168 => self.optional_nested_enum = Varint::read_from(s)?,
                176 => self.optional_foreign_enum = Varint::read_from(s)?,
                184 => self.optional_aliased_enum = Varint::read_from(s)?,
                194 => LengthPrefixed::merge_from(&mut self.optional_string_piece, s)?,
                202 => LengthPrefixed::merge_from(&mut self.optional_cord, s)?,
                218 => LengthPrefixed::merge_from(self.recursive_message_mut(), s)?,
                250 => PackedArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                248 => CopyArray::<Varint>::merge_from(&mut self.repeated_int32, s)?,
                258 => PackedArray::<Varint>::merge_from(&mut self.repeated_int64, s)?,
                256 => CopyArray::<Varint>::merge_from(&mut self.repeated_int64, s)?,
                266 => PackedArray::<Varint>::merge_from(&mut self.repeated_uint32, s)?,
                264 => CopyArray::<Varint>::merge_from(&mut self.repeated_uint32, s)?,
                274 => PackedArray::<Varint>::merge_from(&mut self.repeated_uint64, s)?,
                272 => CopyArray::<Varint>::merge_from(&mut self.repeated_uint64, s)?,
                282 => PackedArray::<ZigZag>::merge_from(&mut self.repeated_sint32, s)?,
                280 => CopyArray::<ZigZag>::merge_from(&mut self.repeated_sint32, s)?,
                290 => PackedArray::<ZigZag>::merge_from(&mut self.repeated_sint64, s)?,
                288 => CopyArray::<ZigZag>::merge_from(&mut self.repeated_sint64, s)?,
                298 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_fixed32, s)?,
                301 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_fixed32, s)?,
                306 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_fixed64, s)?,
                305 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_fixed64, s)?,
                314 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_sfixed32, s)?,
                317 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_sfixed32, s)?,
                322 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_sfixed64, s)?,
                321 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_sfixed64, s)?,
                330 => PackedArray::<Fixed32>::merge_from(&mut self.repeated_float, s)?,
                333 => CopyArray::<Fixed32>::merge_from(&mut self.repeated_float, s)?,
                338 => PackedArray::<Fixed64>::merge_from(&mut self.repeated_double, s)?,
                337 => CopyArray::<Fixed64>::merge_from(&mut self.repeated_double, s)?,
                346 => PackedArray::<Varint>::merge_from(&mut self.repeated_bool, s)?,
                344 => CopyArray::<Varint>::merge_from(&mut self.repeated_bool, s)?,
                354 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_string, s)?,
                362 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_bytes, s)?,
                386 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_nested_message, s)?
                }
                394 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_foreign_message, s)?
                }
                410 => PackedArray::<Varint>::merge_from(&mut self.repeated_nested_enum, s)?,
                408 => CopyArray::<Varint>::merge_from(&mut self.repeated_nested_enum, s)?,
                418 => PackedArray::<Varint>::merge_from(&mut self.repeated_foreign_enum, s)?,
                416 => CopyArray::<Varint>::merge_from(&mut self.repeated_foreign_enum, s)?,
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
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
                                18 => LengthPrefixed::merge_from(&mut val, s)?,
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
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
                                18 => LengthPrefixed::merge_from(&mut val, s)?,
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
                        let mut val = TestAllTypesProto3_NestedMessage::new();
                        loop {
                            match s.read_tag()? {
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
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
                        let mut val = ForeignMessage::new();
                        loop {
                            match s.read_tag()? {
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
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
                        let mut val = TestAllTypesProto3_NestedEnum::new();
                        loop {
                            match s.read_tag()? {
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
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
                        let mut val = ForeignEnum::new();
                        loop {
                            match s.read_tag()? {
                                10 => LengthPrefixed::merge_from(&mut key, s)?,
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
                        TestAllTypesProto3_Oneof_Field::OneofUint32(Varint::read_from(s)?)
                }
                898 => LengthPrefixed::merge_from(self.oneof_nested_message_mut(), s)?,
                906 => LengthPrefixed::merge_from(self.oneof_string_mut(), s)?,
                914 => LengthPrefixed::merge_from(self.oneof_bytes_mut(), s)?,
                920 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofBool(Varint::read_from(s)?)
                }
                928 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofUint64(Varint::read_from(s)?)
                }
                941 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofFloat(Fixed32::read_from(s)?)
                }
                945 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofDouble(Fixed64::read_from(s)?)
                }
                952 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofEnum(Varint::read_from(s)?)
                }
                960 => {
                    self.oneof_field =
                        TestAllTypesProto3_Oneof_Field::OneofNullValue(Varint::read_from(s)?)
                }
                1610 => LengthPrefixed::merge_from(self.optional_bool_wrapper_mut(), s)?,
                1618 => LengthPrefixed::merge_from(self.optional_int32_wrapper_mut(), s)?,
                1626 => LengthPrefixed::merge_from(self.optional_int64_wrapper_mut(), s)?,
                1634 => LengthPrefixed::merge_from(self.optional_uint32_wrapper_mut(), s)?,
                1642 => LengthPrefixed::merge_from(self.optional_uint64_wrapper_mut(), s)?,
                1650 => LengthPrefixed::merge_from(self.optional_float_wrapper_mut(), s)?,
                1658 => LengthPrefixed::merge_from(self.optional_double_wrapper_mut(), s)?,
                1666 => LengthPrefixed::merge_from(self.optional_string_wrapper_mut(), s)?,
                1674 => LengthPrefixed::merge_from(self.optional_bytes_wrapper_mut(), s)?,
                1690 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_bool_wrapper, s)?,
                1698 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_int32_wrapper, s)?
                }
                1706 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_int64_wrapper, s)?
                }
                1714 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_uint32_wrapper, s)?
                }
                1722 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_uint64_wrapper, s)?
                }
                1730 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_float_wrapper, s)?
                }
                1738 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_double_wrapper, s)?
                }
                1746 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_string_wrapper, s)?
                }
                1754 => {
                    RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_bytes_wrapper, s)?
                }
                2410 => LengthPrefixed::merge_from(self.optional_duration_mut(), s)?,
                2418 => LengthPrefixed::merge_from(self.optional_timestamp_mut(), s)?,
                2426 => LengthPrefixed::merge_from(self.optional_field_mask_mut(), s)?,
                2434 => LengthPrefixed::merge_from(self.optional_struct_mut(), s)?,
                2442 => LengthPrefixed::merge_from(self.optional_any_mut(), s)?,
                2450 => LengthPrefixed::merge_from(self.optional_value_mut(), s)?,
                2456 => self.optional_null_value = Varint::read_from(s)?,
                2490 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_duration, s)?,
                2498 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_timestamp, s)?,
                2506 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_fieldmask, s)?,
                2522 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_any, s)?,
                2530 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_value, s)?,
                2538 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_list_value, s)?,
                2594 => RefArray::<LengthPrefixed>::merge_from(&mut self.repeated_struct, s)?,
                3208 => self.fieldname1 = Varint::read_from(s)?,
                3216 => self.field_name2 = Varint::read_from(s)?,
                3224 => self._field_name3 = Varint::read_from(s)?,
                3232 => self.field__name4_ = Varint::read_from(s)?,
                3240 => self.field0name5 = Varint::read_from(s)?,
                3248 => self.field_0_name6 = Varint::read_from(s)?,
                3256 => self.field_name7 = Varint::read_from(s)?,
                3264 => self.field_name8 = Varint::read_from(s)?,
                3272 => self.field_name9 = Varint::read_from(s)?,
                3280 => self.field_name10 = Varint::read_from(s)?,
                3288 => self.field_name11 = Varint::read_from(s)?,
                3296 => self.field_name12 = Varint::read_from(s)?,
                3304 => self.__field_name13 = Varint::read_from(s)?,
                3312 => self.__field_name14 = Varint::read_from(s)?,
                3320 => self.field__name15 = Varint::read_from(s)?,
                3328 => self.field__name16 = Varint::read_from(s)?,
                3336 => self.field_name17__ = Varint::read_from(s)?,
                3344 => self.field_name18__ = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if self.optional_int32 != 0 {
            s.write_tag(8)?;
            Varint::write_to(self.optional_int32, s)?;
        }
        if self.optional_int64 != 0 {
            s.write_tag(16)?;
            Varint::write_to(self.optional_int64, s)?;
        }
        if self.optional_uint32 != 0 {
            s.write_tag(24)?;
            Varint::write_to(self.optional_uint32, s)?;
        }
        if self.optional_uint64 != 0 {
            s.write_tag(32)?;
            Varint::write_to(self.optional_uint64, s)?;
        }
        if self.optional_sint32 != 0 {
            s.write_tag(40)?;
            ZigZag::write_to(self.optional_sint32, s)?;
        }
        if self.optional_sint64 != 0 {
            s.write_tag(48)?;
            ZigZag::write_to(self.optional_sint64, s)?;
        }
        if self.optional_fixed32 != 0 {
            s.write_tag(61)?;
            Fixed32::write_to(self.optional_fixed32, s)?;
        }
        if self.optional_fixed64 != 0 {
            s.write_tag(65)?;
            Fixed64::write_to(self.optional_fixed64, s)?;
        }
        if self.optional_sfixed32 != 0 {
            s.write_tag(77)?;
            Fixed32::write_to(self.optional_sfixed32, s)?;
        }
        if self.optional_sfixed64 != 0 {
            s.write_tag(81)?;
            Fixed64::write_to(self.optional_sfixed64, s)?;
        }
        if self.optional_float != 0f32 {
            s.write_tag(93)?;
            Fixed32::write_to(self.optional_float, s)?;
        }
        if self.optional_double != 0f64 {
            s.write_tag(97)?;
            Fixed64::write_to(self.optional_double, s)?;
        }
        if self.optional_bool {
            s.write_tag(104)?;
            Varint::write_to(self.optional_bool, s)?;
        }
        if !self.optional_string.is_empty() {
            s.write_tag(114)?;
            LengthPrefixed::write_to(&self.optional_string, s)?;
        }
        if !self.optional_bytes.is_empty() {
            s.write_tag(122)?;
            LengthPrefixed::write_to(&self.optional_bytes, s)?;
        }
        if let Some(v) = &self.optional_nested_message {
            s.write_tag(146)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_foreign_message {
            s.write_tag(154)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if self.optional_nested_enum != TestAllTypesProto3_NestedEnum::new() {
            s.write_tag(168)?;
            Varint::write_to(self.optional_nested_enum, s)?;
        }
        if self.optional_foreign_enum != ForeignEnum::new() {
            s.write_tag(176)?;
            Varint::write_to(self.optional_foreign_enum, s)?;
        }
        if self.optional_aliased_enum != TestAllTypesProto3_AliasedEnum::new() {
            s.write_tag(184)?;
            Varint::write_to(self.optional_aliased_enum, s)?;
        }
        if !self.optional_string_piece.is_empty() {
            s.write_tag(194)?;
            LengthPrefixed::write_to(&self.optional_string_piece, s)?;
        }
        if !self.optional_cord.is_empty() {
            s.write_tag(202)?;
            LengthPrefixed::write_to(&self.optional_cord, s)?;
        }
        if let Some(v) = &self.recursive_message {
            s.write_tag(218)?;
            LengthPrefixed::write_to(v.as_ref(), s)?;
        }
        if !self.repeated_int32.is_empty() {
            s.write_tag(250)?;
            PackedArray::<Varint>::write_to(&self.repeated_int32, s)?
        }
        if !self.repeated_int64.is_empty() {
            s.write_tag(258)?;
            PackedArray::<Varint>::write_to(&self.repeated_int64, s)?
        }
        if !self.repeated_uint32.is_empty() {
            s.write_tag(266)?;
            PackedArray::<Varint>::write_to(&self.repeated_uint32, s)?
        }
        if !self.repeated_uint64.is_empty() {
            s.write_tag(274)?;
            PackedArray::<Varint>::write_to(&self.repeated_uint64, s)?
        }
        if !self.repeated_sint32.is_empty() {
            s.write_tag(282)?;
            PackedArray::<ZigZag>::write_to(&self.repeated_sint32, s)?
        }
        if !self.repeated_sint64.is_empty() {
            s.write_tag(290)?;
            PackedArray::<ZigZag>::write_to(&self.repeated_sint64, s)?
        }
        if !self.repeated_fixed32.is_empty() {
            s.write_tag(298)?;
            PackedArray::<Fixed32>::write_to(&self.repeated_fixed32, s)?
        }
        if !self.repeated_fixed64.is_empty() {
            s.write_tag(306)?;
            PackedArray::<Fixed64>::write_to(&self.repeated_fixed64, s)?
        }
        if !self.repeated_sfixed32.is_empty() {
            s.write_tag(314)?;
            PackedArray::<Fixed32>::write_to(&self.repeated_sfixed32, s)?
        }
        if !self.repeated_sfixed64.is_empty() {
            s.write_tag(322)?;
            PackedArray::<Fixed64>::write_to(&self.repeated_sfixed64, s)?
        }
        if !self.repeated_float.is_empty() {
            s.write_tag(330)?;
            PackedArray::<Fixed32>::write_to(&self.repeated_float, s)?
        }
        if !self.repeated_double.is_empty() {
            s.write_tag(338)?;
            PackedArray::<Fixed64>::write_to(&self.repeated_double, s)?
        }
        if !self.repeated_bool.is_empty() {
            s.write_tag(346)?;
            PackedArray::<Varint>::write_to(&self.repeated_bool, s)?
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
            s.write_tag(410)?;
            PackedArray::<Varint>::write_to(&self.repeated_nested_enum, s)?
        }
        if !self.repeated_foreign_enum.is_empty() {
            s.write_tag(418)?;
            PackedArray::<Varint>::write_to(&self.repeated_foreign_enum, s)?
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
            TestAllTypesProto3_Oneof_Field::None => (),
            TestAllTypesProto3_Oneof_Field::OneofUint32(v) => {
                s.write_tag(888)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofNestedMessage(v) => {
                s.write_tag(898)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofString(v) => {
                s.write_tag(906)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofBytes(v) => {
                s.write_tag(914)?;
                LengthPrefixed::write_to(v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofBool(v) => {
                s.write_tag(920)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofUint64(v) => {
                s.write_tag(928)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofFloat(v) => {
                s.write_tag(941)?;
                Fixed32::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofDouble(v) => {
                s.write_tag(945)?;
                Fixed64::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofEnum(v) => {
                s.write_tag(952)?;
                Varint::write_to(*v, s)?;
            }
            TestAllTypesProto3_Oneof_Field::OneofNullValue(v) => {
                s.write_tag(960)?;
                Varint::write_to(*v, s)?;
            }
        }
        if let Some(v) = &self.optional_bool_wrapper {
            s.write_tag(1610)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_int32_wrapper {
            s.write_tag(1618)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_int64_wrapper {
            s.write_tag(1626)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_uint32_wrapper {
            s.write_tag(1634)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_uint64_wrapper {
            s.write_tag(1642)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_float_wrapper {
            s.write_tag(1650)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_double_wrapper {
            s.write_tag(1658)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_string_wrapper {
            s.write_tag(1666)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_bytes_wrapper {
            s.write_tag(1674)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.repeated_bool_wrapper.is_empty() {
            for i in &self.repeated_bool_wrapper {
                s.write_tag(1690)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_int32_wrapper.is_empty() {
            for i in &self.repeated_int32_wrapper {
                s.write_tag(1698)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_int64_wrapper.is_empty() {
            for i in &self.repeated_int64_wrapper {
                s.write_tag(1706)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_uint32_wrapper.is_empty() {
            for i in &self.repeated_uint32_wrapper {
                s.write_tag(1714)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_uint64_wrapper.is_empty() {
            for i in &self.repeated_uint64_wrapper {
                s.write_tag(1722)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_float_wrapper.is_empty() {
            for i in &self.repeated_float_wrapper {
                s.write_tag(1730)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_double_wrapper.is_empty() {
            for i in &self.repeated_double_wrapper {
                s.write_tag(1738)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_string_wrapper.is_empty() {
            for i in &self.repeated_string_wrapper {
                s.write_tag(1746)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_bytes_wrapper.is_empty() {
            for i in &self.repeated_bytes_wrapper {
                s.write_tag(1754)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.optional_duration {
            s.write_tag(2410)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_timestamp {
            s.write_tag(2418)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_field_mask {
            s.write_tag(2426)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_struct {
            s.write_tag(2434)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_any {
            s.write_tag(2442)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.optional_value {
            s.write_tag(2450)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if self.optional_null_value != pecan_types::google::protobuf::struct_pb::NullValue::new() {
            s.write_tag(2456)?;
            Varint::write_to(self.optional_null_value, s)?;
        }
        if !self.repeated_duration.is_empty() {
            for i in &self.repeated_duration {
                s.write_tag(2490)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_timestamp.is_empty() {
            for i in &self.repeated_timestamp {
                s.write_tag(2498)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_fieldmask.is_empty() {
            for i in &self.repeated_fieldmask {
                s.write_tag(2506)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_any.is_empty() {
            for i in &self.repeated_any {
                s.write_tag(2522)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_value.is_empty() {
            for i in &self.repeated_value {
                s.write_tag(2530)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_list_value.is_empty() {
            for i in &self.repeated_list_value {
                s.write_tag(2538)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.repeated_struct.is_empty() {
            for i in &self.repeated_struct {
                s.write_tag(2594)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if self.fieldname1 != 0 {
            s.write_tag(3208)?;
            Varint::write_to(self.fieldname1, s)?;
        }
        if self.field_name2 != 0 {
            s.write_tag(3216)?;
            Varint::write_to(self.field_name2, s)?;
        }
        if self._field_name3 != 0 {
            s.write_tag(3224)?;
            Varint::write_to(self._field_name3, s)?;
        }
        if self.field__name4_ != 0 {
            s.write_tag(3232)?;
            Varint::write_to(self.field__name4_, s)?;
        }
        if self.field0name5 != 0 {
            s.write_tag(3240)?;
            Varint::write_to(self.field0name5, s)?;
        }
        if self.field_0_name6 != 0 {
            s.write_tag(3248)?;
            Varint::write_to(self.field_0_name6, s)?;
        }
        if self.field_name7 != 0 {
            s.write_tag(3256)?;
            Varint::write_to(self.field_name7, s)?;
        }
        if self.field_name8 != 0 {
            s.write_tag(3264)?;
            Varint::write_to(self.field_name8, s)?;
        }
        if self.field_name9 != 0 {
            s.write_tag(3272)?;
            Varint::write_to(self.field_name9, s)?;
        }
        if self.field_name10 != 0 {
            s.write_tag(3280)?;
            Varint::write_to(self.field_name10, s)?;
        }
        if self.field_name11 != 0 {
            s.write_tag(3288)?;
            Varint::write_to(self.field_name11, s)?;
        }
        if self.field_name12 != 0 {
            s.write_tag(3296)?;
            Varint::write_to(self.field_name12, s)?;
        }
        if self.__field_name13 != 0 {
            s.write_tag(3304)?;
            Varint::write_to(self.__field_name13, s)?;
        }
        if self.__field_name14 != 0 {
            s.write_tag(3312)?;
            Varint::write_to(self.__field_name14, s)?;
        }
        if self.field__name15 != 0 {
            s.write_tag(3320)?;
            Varint::write_to(self.field__name15, s)?;
        }
        if self.field__name16 != 0 {
            s.write_tag(3328)?;
            Varint::write_to(self.field__name16, s)?;
        }
        if self.field_name17__ != 0 {
            s.write_tag(3336)?;
            Varint::write_to(self.field_name17__, s)?;
        }
        if self.field_name18__ != 0 {
            s.write_tag(3344)?;
            Varint::write_to(self.field_name18__, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.optional_int32 != 0 {
            l += 1 + Varint::size(self.optional_int32);
        }
        if self.optional_int64 != 0 {
            l += 1 + Varint::size(self.optional_int64);
        }
        if self.optional_uint32 != 0 {
            l += 1 + Varint::size(self.optional_uint32);
        }
        if self.optional_uint64 != 0 {
            l += 1 + Varint::size(self.optional_uint64);
        }
        if self.optional_sint32 != 0 {
            l += 1 + ZigZag::size(self.optional_sint32);
        }
        if self.optional_sint64 != 0 {
            l += 1 + ZigZag::size(self.optional_sint64);
        }
        if self.optional_fixed32 != 0 {
            l += 1 + Fixed32::size(self.optional_fixed32);
        }
        if self.optional_fixed64 != 0 {
            l += 1 + Fixed64::size(self.optional_fixed64);
        }
        if self.optional_sfixed32 != 0 {
            l += 1 + Fixed32::size(self.optional_sfixed32);
        }
        if self.optional_sfixed64 != 0 {
            l += 1 + Fixed64::size(self.optional_sfixed64);
        }
        if self.optional_float != 0f32 {
            l += 1 + Fixed32::size(self.optional_float);
        }
        if self.optional_double != 0f64 {
            l += 1 + Fixed64::size(self.optional_double);
        }
        if self.optional_bool {
            l += 1 + Varint::size(self.optional_bool);
        }
        if !self.optional_string.is_empty() {
            l += 1 + LengthPrefixed::size(&self.optional_string);
        }
        if !self.optional_bytes.is_empty() {
            l += 1 + LengthPrefixed::size(&self.optional_bytes);
        }
        if let Some(v) = &self.optional_nested_message {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_foreign_message {
            l += 2 + LengthPrefixed::size(v);
        }
        if self.optional_nested_enum != TestAllTypesProto3_NestedEnum::new() {
            l += 2 + Varint::size(self.optional_nested_enum);
        }
        if self.optional_foreign_enum != ForeignEnum::new() {
            l += 2 + Varint::size(self.optional_foreign_enum);
        }
        if self.optional_aliased_enum != TestAllTypesProto3_AliasedEnum::new() {
            l += 2 + Varint::size(self.optional_aliased_enum);
        }
        if !self.optional_string_piece.is_empty() {
            l += 2 + LengthPrefixed::size(&self.optional_string_piece);
        }
        if !self.optional_cord.is_empty() {
            l += 2 + LengthPrefixed::size(&self.optional_cord);
        }
        if let Some(v) = &self.recursive_message {
            l += 2 + LengthPrefixed::size(v.as_ref());
        }
        if !self.repeated_int32.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_int32);
        }
        if !self.repeated_int64.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_int64);
        }
        if !self.repeated_uint32.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_uint32);
        }
        if !self.repeated_uint64.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_uint64);
        }
        if !self.repeated_sint32.is_empty() {
            l += 2 + PackedArray::<ZigZag>::size(&self.repeated_sint32);
        }
        if !self.repeated_sint64.is_empty() {
            l += 2 + PackedArray::<ZigZag>::size(&self.repeated_sint64);
        }
        if !self.repeated_fixed32.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.repeated_fixed32);
        }
        if !self.repeated_fixed64.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.repeated_fixed64);
        }
        if !self.repeated_sfixed32.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.repeated_sfixed32);
        }
        if !self.repeated_sfixed64.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.repeated_sfixed64);
        }
        if !self.repeated_float.is_empty() {
            l += 2 + PackedArray::<Fixed32>::size(&self.repeated_float);
        }
        if !self.repeated_double.is_empty() {
            l += 2 + PackedArray::<Fixed64>::size(&self.repeated_double);
        }
        if !self.repeated_bool.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_bool);
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
            l += 2 + PackedArray::<Varint>::size(&self.repeated_nested_enum);
        }
        if !self.repeated_foreign_enum.is_empty() {
            l += 2 + PackedArray::<Varint>::size(&self.repeated_foreign_enum);
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
            TestAllTypesProto3_Oneof_Field::None => (),
            TestAllTypesProto3_Oneof_Field::OneofUint32(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofNestedMessage(v) => {
                l += 2 + LengthPrefixed::size(v)
            }
            TestAllTypesProto3_Oneof_Field::OneofString(v) => l += 2 + LengthPrefixed::size(v),
            TestAllTypesProto3_Oneof_Field::OneofBytes(v) => l += 2 + LengthPrefixed::size(v),
            TestAllTypesProto3_Oneof_Field::OneofBool(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofUint64(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofFloat(v) => l += 2 + Fixed32::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofDouble(v) => l += 2 + Fixed64::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofEnum(v) => l += 2 + Varint::size(*v),
            TestAllTypesProto3_Oneof_Field::OneofNullValue(v) => l += 2 + Varint::size(*v),
        }
        if let Some(v) = &self.optional_bool_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_int32_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_int64_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_uint32_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_uint64_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_float_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_double_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_string_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_bytes_wrapper {
            l += 2 + LengthPrefixed::size(v);
        }
        if !self.repeated_bool_wrapper.is_empty() {
            l += 2 * self.repeated_bool_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_bool_wrapper);
        }
        if !self.repeated_int32_wrapper.is_empty() {
            l += 2 * self.repeated_int32_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_int32_wrapper);
        }
        if !self.repeated_int64_wrapper.is_empty() {
            l += 2 * self.repeated_int64_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_int64_wrapper);
        }
        if !self.repeated_uint32_wrapper.is_empty() {
            l += 2 * self.repeated_uint32_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_uint32_wrapper);
        }
        if !self.repeated_uint64_wrapper.is_empty() {
            l += 2 * self.repeated_uint64_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_uint64_wrapper);
        }
        if !self.repeated_float_wrapper.is_empty() {
            l += 2 * self.repeated_float_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_float_wrapper);
        }
        if !self.repeated_double_wrapper.is_empty() {
            l += 2 * self.repeated_double_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_double_wrapper);
        }
        if !self.repeated_string_wrapper.is_empty() {
            l += 2 * self.repeated_string_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_string_wrapper);
        }
        if !self.repeated_bytes_wrapper.is_empty() {
            l += 2 * self.repeated_bytes_wrapper.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_bytes_wrapper);
        }
        if let Some(v) = &self.optional_duration {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_timestamp {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_field_mask {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_struct {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_any {
            l += 2 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.optional_value {
            l += 2 + LengthPrefixed::size(v);
        }
        if self.optional_null_value != pecan_types::google::protobuf::struct_pb::NullValue::new() {
            l += 2 + Varint::size(self.optional_null_value);
        }
        if !self.repeated_duration.is_empty() {
            l += 2 * self.repeated_duration.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_duration);
        }
        if !self.repeated_timestamp.is_empty() {
            l += 2 * self.repeated_timestamp.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_timestamp);
        }
        if !self.repeated_fieldmask.is_empty() {
            l += 2 * self.repeated_fieldmask.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_fieldmask);
        }
        if !self.repeated_any.is_empty() {
            l += 2 * self.repeated_any.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_any);
        }
        if !self.repeated_value.is_empty() {
            l += 2 * self.repeated_value.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_value);
        }
        if !self.repeated_list_value.is_empty() {
            l += 2 * self.repeated_list_value.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_list_value);
        }
        if !self.repeated_struct.is_empty() {
            l += 2 * self.repeated_struct.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.repeated_struct);
        }
        if self.fieldname1 != 0 {
            l += 2 + Varint::size(self.fieldname1);
        }
        if self.field_name2 != 0 {
            l += 2 + Varint::size(self.field_name2);
        }
        if self._field_name3 != 0 {
            l += 2 + Varint::size(self._field_name3);
        }
        if self.field__name4_ != 0 {
            l += 2 + Varint::size(self.field__name4_);
        }
        if self.field0name5 != 0 {
            l += 2 + Varint::size(self.field0name5);
        }
        if self.field_0_name6 != 0 {
            l += 2 + Varint::size(self.field_0_name6);
        }
        if self.field_name7 != 0 {
            l += 2 + Varint::size(self.field_name7);
        }
        if self.field_name8 != 0 {
            l += 2 + Varint::size(self.field_name8);
        }
        if self.field_name9 != 0 {
            l += 2 + Varint::size(self.field_name9);
        }
        if self.field_name10 != 0 {
            l += 2 + Varint::size(self.field_name10);
        }
        if self.field_name11 != 0 {
            l += 2 + Varint::size(self.field_name11);
        }
        if self.field_name12 != 0 {
            l += 2 + Varint::size(self.field_name12);
        }
        if self.__field_name13 != 0 {
            l += 2 + Varint::size(self.__field_name13);
        }
        if self.__field_name14 != 0 {
            l += 2 + Varint::size(self.__field_name14);
        }
        if self.field__name15 != 0 {
            l += 2 + Varint::size(self.field__name15);
        }
        if self.field__name16 != 0 {
            l += 2 + Varint::size(self.field__name16);
        }
        if self.field_name17__ != 0 {
            l += 2 + Varint::size(self.field_name17__);
        }
        if self.field_name18__ != 0 {
            l += 2 + Varint::size(self.field_name18__);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self.optional_int32 = 0;
        self.optional_int64 = 0;
        self.optional_uint32 = 0;
        self.optional_uint64 = 0;
        self.optional_sint32 = 0;
        self.optional_sint64 = 0;
        self.optional_fixed32 = 0;
        self.optional_fixed64 = 0;
        self.optional_sfixed32 = 0;
        self.optional_sfixed64 = 0;
        self.optional_float = 0f32;
        self.optional_double = 0f64;
        self.optional_bool = false;
        self.optional_string.clear();
        self.optional_bytes.clear();
        self.optional_nested_message = None;
        self.optional_foreign_message = None;
        self.optional_nested_enum = TestAllTypesProto3_NestedEnum::new();
        self.optional_foreign_enum = ForeignEnum::new();
        self.optional_aliased_enum = TestAllTypesProto3_AliasedEnum::new();
        self.optional_string_piece.clear();
        self.optional_cord.clear();
        self.recursive_message = None;
        self.repeated_int32.clear();
        self.repeated_int64.clear();
        self.repeated_uint32.clear();
        self.repeated_uint64.clear();
        self.repeated_sint32.clear();
        self.repeated_sint64.clear();
        self.repeated_fixed32.clear();
        self.repeated_fixed64.clear();
        self.repeated_sfixed32.clear();
        self.repeated_sfixed64.clear();
        self.repeated_float.clear();
        self.repeated_double.clear();
        self.repeated_bool.clear();
        self.repeated_string.clear();
        self.repeated_bytes.clear();
        self.repeated_nested_message.clear();
        self.repeated_foreign_message.clear();
        self.repeated_nested_enum.clear();
        self.repeated_foreign_enum.clear();
        self.repeated_string_piece.clear();
        self.repeated_cord.clear();
        self.packed_int32.clear();
        self.packed_int64.clear();
        self.packed_uint32.clear();
        self.packed_uint64.clear();
        self.packed_sint32.clear();
        self.packed_sint64.clear();
        self.packed_fixed32.clear();
        self.packed_fixed64.clear();
        self.packed_sfixed32.clear();
        self.packed_sfixed64.clear();
        self.packed_float.clear();
        self.packed_double.clear();
        self.packed_bool.clear();
        self.packed_nested_enum.clear();
        self.unpacked_int32.clear();
        self.unpacked_int64.clear();
        self.unpacked_uint32.clear();
        self.unpacked_uint64.clear();
        self.unpacked_sint32.clear();
        self.unpacked_sint64.clear();
        self.unpacked_fixed32.clear();
        self.unpacked_fixed64.clear();
        self.unpacked_sfixed32.clear();
        self.unpacked_sfixed64.clear();
        self.unpacked_float.clear();
        self.unpacked_double.clear();
        self.unpacked_bool.clear();
        self.unpacked_nested_enum.clear();
        self.map_int32_int32 = None;
        self.map_int64_int64 = None;
        self.map_uint32_uint32 = None;
        self.map_uint64_uint64 = None;
        self.map_sint32_sint32 = None;
        self.map_sint64_sint64 = None;
        self.map_fixed32_fixed32 = None;
        self.map_fixed64_fixed64 = None;
        self.map_sfixed32_sfixed32 = None;
        self.map_sfixed64_sfixed64 = None;
        self.map_int32_float = None;
        self.map_int32_double = None;
        self.map_bool_bool = None;
        self.map_string_string = None;
        self.map_string_bytes = None;
        self.map_string_nested_message = None;
        self.map_string_foreign_message = None;
        self.map_string_nested_enum = None;
        self.map_string_foreign_enum = None;
        self.oneof_field = TestAllTypesProto3_Oneof_Field::None;
        self.optional_bool_wrapper = None;
        self.optional_int32_wrapper = None;
        self.optional_int64_wrapper = None;
        self.optional_uint32_wrapper = None;
        self.optional_uint64_wrapper = None;
        self.optional_float_wrapper = None;
        self.optional_double_wrapper = None;
        self.optional_string_wrapper = None;
        self.optional_bytes_wrapper = None;
        self.repeated_bool_wrapper.clear();
        self.repeated_int32_wrapper.clear();
        self.repeated_int64_wrapper.clear();
        self.repeated_uint32_wrapper.clear();
        self.repeated_uint64_wrapper.clear();
        self.repeated_float_wrapper.clear();
        self.repeated_double_wrapper.clear();
        self.repeated_string_wrapper.clear();
        self.repeated_bytes_wrapper.clear();
        self.optional_duration = None;
        self.optional_timestamp = None;
        self.optional_field_mask = None;
        self.optional_struct = None;
        self.optional_any = None;
        self.optional_value = None;
        self.optional_null_value = pecan_types::google::protobuf::struct_pb::NullValue::new();
        self.repeated_duration.clear();
        self.repeated_timestamp.clear();
        self.repeated_fieldmask.clear();
        self.repeated_struct.clear();
        self.repeated_any.clear();
        self.repeated_value.clear();
        self.repeated_list_value.clear();
        self.fieldname1 = 0;
        self.field_name2 = 0;
        self._field_name3 = 0;
        self.field__name4_ = 0;
        self.field0name5 = 0;
        self.field_0_name6 = 0;
        self.field_name7 = 0;
        self.field_name8 = 0;
        self.field_name9 = 0;
        self.field_name10 = 0;
        self.field_name11 = 0;
        self.field_name12 = 0;
        self.__field_name13 = 0;
        self.__field_name14 = 0;
        self.field__name15 = 0;
        self.field__name16 = 0;
        self.field_name17__ = 0;
        self.field_name18__ = 0;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for TestAllTypesProto3 {
    fn default_instance() -> &'static TestAllTypesProto3 {
        static DEFAULT: TestAllTypesProto3 = TestAllTypesProto3::new();
        &DEFAULT
    }
}
impl Default for TestAllTypesProto3 {
    #[inline]
    fn default() -> TestAllTypesProto3 {
        TestAllTypesProto3::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForeignMessage {
    pub c: i32,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl ForeignMessage {
    pub const fn new() -> ForeignMessage {
        ForeignMessage {
            c: 0,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for ForeignMessage {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.c = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if self.c != 0 {
            s.write_tag(8)?;
            Varint::write_to(self.c, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.c != 0 {
            l += 1 + Varint::size(self.c);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        self._cached_size.set(l);
        l
    }
    fn clear(&mut self) {
        self.c = 0;
        self._unknown.clear();
    }
    #[inline]
    fn cached_size(&self) -> u32 {
        self._cached_size.get()
    }
}
impl pecan::DefaultInstance for ForeignMessage {
    fn default_instance() -> &'static ForeignMessage {
        static DEFAULT: ForeignMessage = ForeignMessage::new();
        &DEFAULT
    }
}
impl Default for ForeignMessage {
    #[inline]
    fn default() -> ForeignMessage {
        ForeignMessage::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n*google/protobuf/test_messages_proto3.proto\x12\x1Dprotobuf_test_messages.proto3\x1A\x13pecan/options.proto\x1A\x19google/protobuf/any.proto\x1A\x1Egoogle/protobuf/duration.proto\x1A google/protobuf/field_mask.proto\x1A\x1Cgoogle/protobuf/struct.proto\x1A\x1Fgoogle/protobuf/timestamp.proto\x1A\x1Egoogle/protobuf/wrappers.proto\"\xA9[\n\x12TestAllTypesProto3\x12%\n\x0Eoptional_int32\x18\x01 \x01(\x05R\roptionalInt32\x12%\n\x0Eoptional_int64\x18\x02 \x01(\x03R\roptionalInt64\x12'\n\x0Foptional_uint32\x18\x03 \x01(\rR\x0EoptionalUint32\x12'\n\x0Foptional_uint64\x18\x04 \x01(\x04R\x0EoptionalUint64\x12'\n\x0Foptional_sint32\x18\x05 \x01(\x11R\x0EoptionalSint32\x12'\n\x0Foptional_sint64\x18\x06 \x01(\x12R\x0EoptionalSint64\x12)\n\x10optional_fixed32\x18\x07 \x01(\x07R\x0FoptionalFixed32\x12)\n\x10optional_fixed64\x18\x08 \x01(\x06R\x0FoptionalFixed64\x12+\n\x11optional_sfixed32\x18\t \x01(\x0FR\x10optionalSfixed32\x12+\n\x11optional_sfixed64\x18\n \x01(\x10R\x10optionalSfixed64\x12%\n\x0Eoptional_float\x18\x0B \x01(\x02R\roptionalFloat\x12'\n\x0Foptional_double\x18\x0C \x01(\x01R\x0EoptionalDouble\x12#\n\roptional_bool\x18\r \x01(\x08R\x0CoptionalBool\x12'\n\x0Foptional_string\x18\x0E \x01(\tR\x0EoptionalString\x12%\n\x0Eoptional_bytes\x18\x0F \x01(\x0CR\roptionalBytes\x12w\n\x17optional_nested_message\x18\x12 \x01(\x0B2?.protobuf_test_messages.proto3.TestAllTypesProto3.NestedMessageR\x15optionalNestedMessage\x12g\n\x18optional_foreign_message\x18\x13 \x01(\x0B2-.protobuf_test_messages.proto3.ForeignMessageR\x16optionalForeignMessage\x12n\n\x14optional_nested_enum\x18\x15 \x01(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumR\x12optionalNestedEnum\x12^\n\x15optional_foreign_enum\x18\x16 \x01(\x0E2*.protobuf_test_messages.proto3.ForeignEnumR\x13optionalForeignEnum\x12q\n\x15optional_aliased_enum\x18\x17 \x01(\x0E2=.protobuf_test_messages.proto3.TestAllTypesProto3.AliasedEnumR\x13optionalAliasedEnum\x126\n\x15optional_string_piece\x18\x18 \x01(\tB\x02\x08\x02R\x13optionalStringPiece\x12'\n\roptional_cord\x18\x19 \x01(\tB\x02\x08\x01R\x0CoptionalCord\x12f\n\x11recursive_message\x18\x1B \x01(\x0B21.protobuf_test_messages.proto3.TestAllTypesProto3B\x06\xEA\x91\x01\x02\x08\x01R\x10recursiveMessage\x12%\n\x0Erepeated_int32\x18\x1F \x03(\x05R\rrepeatedInt32\x12%\n\x0Erepeated_int64\x18  \x03(\x03R\rrepeatedInt64\x12'\n\x0Frepeated_uint32\x18! \x03(\rR\x0ErepeatedUint32\x12'\n\x0Frepeated_uint64\x18\" \x03(\x04R\x0ErepeatedUint64\x12'\n\x0Frepeated_sint32\x18# \x03(\x11R\x0ErepeatedSint32\x12'\n\x0Frepeated_sint64\x18$ \x03(\x12R\x0ErepeatedSint64\x12)\n\x10repeated_fixed32\x18% \x03(\x07R\x0FrepeatedFixed32\x12)\n\x10repeated_fixed64\x18& \x03(\x06R\x0FrepeatedFixed64\x12+\n\x11repeated_sfixed32\x18' \x03(\x0FR\x10repeatedSfixed32\x12+\n\x11repeated_sfixed64\x18( \x03(\x10R\x10repeatedSfixed64\x12%\n\x0Erepeated_float\x18) \x03(\x02R\rrepeatedFloat\x12'\n\x0Frepeated_double\x18* \x03(\x01R\x0ErepeatedDouble\x12#\n\rrepeated_bool\x18+ \x03(\x08R\x0CrepeatedBool\x12'\n\x0Frepeated_string\x18, \x03(\tR\x0ErepeatedString\x12%\n\x0Erepeated_bytes\x18- \x03(\x0CR\rrepeatedBytes\x12w\n\x17repeated_nested_message\x180 \x03(\x0B2?.protobuf_test_messages.proto3.TestAllTypesProto3.NestedMessageR\x15repeatedNestedMessage\x12g\n\x18repeated_foreign_message\x181 \x03(\x0B2-.protobuf_test_messages.proto3.ForeignMessageR\x16repeatedForeignMessage\x12n\n\x14repeated_nested_enum\x183 \x03(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumR\x12repeatedNestedEnum\x12^\n\x15repeated_foreign_enum\x184 \x03(\x0E2*.protobuf_test_messages.proto3.ForeignEnumR\x13repeatedForeignEnum\x126\n\x15repeated_string_piece\x186 \x03(\tB\x02\x08\x02R\x13repeatedStringPiece\x12'\n\rrepeated_cord\x187 \x03(\tB\x02\x08\x01R\x0CrepeatedCord\x12%\n\x0Cpacked_int32\x18K \x03(\x05B\x02\x10\x01R\x0BpackedInt32\x12%\n\x0Cpacked_int64\x18L \x03(\x03B\x02\x10\x01R\x0BpackedInt64\x12'\n\rpacked_uint32\x18M \x03(\rB\x02\x10\x01R\x0CpackedUint32\x12'\n\rpacked_uint64\x18N \x03(\x04B\x02\x10\x01R\x0CpackedUint64\x12'\n\rpacked_sint32\x18O \x03(\x11B\x02\x10\x01R\x0CpackedSint32\x12'\n\rpacked_sint64\x18P \x03(\x12B\x02\x10\x01R\x0CpackedSint64\x12)\n\x0Epacked_fixed32\x18Q \x03(\x07B\x02\x10\x01R\rpackedFixed32\x12)\n\x0Epacked_fixed64\x18R \x03(\x06B\x02\x10\x01R\rpackedFixed64\x12+\n\x0Fpacked_sfixed32\x18S \x03(\x0FB\x02\x10\x01R\x0EpackedSfixed32\x12+\n\x0Fpacked_sfixed64\x18T \x03(\x10B\x02\x10\x01R\x0EpackedSfixed64\x12%\n\x0Cpacked_float\x18U \x03(\x02B\x02\x10\x01R\x0BpackedFloat\x12'\n\rpacked_double\x18V \x03(\x01B\x02\x10\x01R\x0CpackedDouble\x12#\n\x0Bpacked_bool\x18W \x03(\x08B\x02\x10\x01R\npackedBool\x12n\n\x12packed_nested_enum\x18X \x03(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumB\x02\x10\x01R\x10packedNestedEnum\x12)\n\x0Eunpacked_int32\x18Y \x03(\x05B\x02\x10\0R\runpackedInt32\x12)\n\x0Eunpacked_int64\x18Z \x03(\x03B\x02\x10\0R\runpackedInt64\x12+\n\x0Funpacked_uint32\x18[ \x03(\rB\x02\x10\0R\x0EunpackedUint32\x12+\n\x0Funpacked_uint64\x18\\ \x03(\x04B\x02\x10\0R\x0EunpackedUint64\x12+\n\x0Funpacked_sint32\x18] \x03(\x11B\x02\x10\0R\x0EunpackedSint32\x12+\n\x0Funpacked_sint64\x18^ \x03(\x12B\x02\x10\0R\x0EunpackedSint64\x12-\n\x10unpacked_fixed32\x18_ \x03(\x07B\x02\x10\0R\x0FunpackedFixed32\x12-\n\x10unpacked_fixed64\x18` \x03(\x06B\x02\x10\0R\x0FunpackedFixed64\x12/\n\x11unpacked_sfixed32\x18a \x03(\x0FB\x02\x10\0R\x10unpackedSfixed32\x12/\n\x11unpacked_sfixed64\x18b \x03(\x10B\x02\x10\0R\x10unpackedSfixed64\x12)\n\x0Eunpacked_float\x18c \x03(\x02B\x02\x10\0R\runpackedFloat\x12+\n\x0Funpacked_double\x18d \x03(\x01B\x02\x10\0R\x0EunpackedDouble\x12'\n\runpacked_bool\x18e \x03(\x08B\x02\x10\0R\x0CunpackedBool\x12r\n\x14unpacked_nested_enum\x18f \x03(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumB\x02\x10\0R\x12unpackedNestedEnum\x12l\n\x0Fmap_int32_int32\x188 \x03(\x0B2D.protobuf_test_messages.proto3.TestAllTypesProto3.MapInt32Int32EntryR\rmapInt32Int32\x12l\n\x0Fmap_int64_int64\x189 \x03(\x0B2D.protobuf_test_messages.proto3.TestAllTypesProto3.MapInt64Int64EntryR\rmapInt64Int64\x12r\n\x11map_uint32_uint32\x18: \x03(\x0B2F.protobuf_test_messages.proto3.TestAllTypesProto3.MapUint32Uint32EntryR\x0FmapUint32Uint32\x12r\n\x11map_uint64_uint64\x18; \x03(\x0B2F.protobuf_test_messages.proto3.TestAllTypesProto3.MapUint64Uint64EntryR\x0FmapUint64Uint64\x12r\n\x11map_sint32_sint32\x18< \x03(\x0B2F.protobuf_test_messages.proto3.TestAllTypesProto3.MapSint32Sint32EntryR\x0FmapSint32Sint32\x12r\n\x11map_sint64_sint64\x18= \x03(\x0B2F.protobuf_test_messages.proto3.TestAllTypesProto3.MapSint64Sint64EntryR\x0FmapSint64Sint64\x12x\n\x13map_fixed32_fixed32\x18> \x03(\x0B2H.protobuf_test_messages.proto3.TestAllTypesProto3.MapFixed32Fixed32EntryR\x11mapFixed32Fixed32\x12x\n\x13map_fixed64_fixed64\x18? \x03(\x0B2H.protobuf_test_messages.proto3.TestAllTypesProto3.MapFixed64Fixed64EntryR\x11mapFixed64Fixed64\x12~\n\x15map_sfixed32_sfixed32\x18@ \x03(\x0B2J.protobuf_test_messages.proto3.TestAllTypesProto3.MapSfixed32Sfixed32EntryR\x13mapSfixed32Sfixed32\x12~\n\x15map_sfixed64_sfixed64\x18A \x03(\x0B2J.protobuf_test_messages.proto3.TestAllTypesProto3.MapSfixed64Sfixed64EntryR\x13mapSfixed64Sfixed64\x12l\n\x0Fmap_int32_float\x18B \x03(\x0B2D.protobuf_test_messages.proto3.TestAllTypesProto3.MapInt32FloatEntryR\rmapInt32Float\x12o\n\x10map_int32_double\x18C \x03(\x0B2E.protobuf_test_messages.proto3.TestAllTypesProto3.MapInt32DoubleEntryR\x0EmapInt32Double\x12f\n\rmap_bool_bool\x18D \x03(\x0B2B.protobuf_test_messages.proto3.TestAllTypesProto3.MapBoolBoolEntryR\x0BmapBoolBool\x12r\n\x11map_string_string\x18E \x03(\x0B2F.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringStringEntryR\x0FmapStringString\x12o\n\x10map_string_bytes\x18F \x03(\x0B2E.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringBytesEntryR\x0EmapStringBytes\x12\x88\x01\n\x19map_string_nested_message\x18G \x03(\x0B2M.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringNestedMessageEntryR\x16mapStringNestedMessage\x12\x8B\x01\n\x1Amap_string_foreign_message\x18H \x03(\x0B2N.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringForeignMessageEntryR\x17mapStringForeignMessage\x12\x7F\n\x16map_string_nested_enum\x18I \x03(\x0B2J.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringNestedEnumEntryR\x13mapStringNestedEnum\x12\x82\x01\n\x17map_string_foreign_enum\x18J \x03(\x0B2K.protobuf_test_messages.proto3.TestAllTypesProto3.MapStringForeignEnumEntryR\x14mapStringForeignEnum\x12#\n\x0Coneof_uint32\x18o \x01(\rH\0R\x0BoneofUint32\x12s\n\x14oneof_nested_message\x18p \x01(\x0B2?.protobuf_test_messages.proto3.TestAllTypesProto3.NestedMessageH\0R\x12oneofNestedMessage\x12#\n\x0Coneof_string\x18q \x01(\tH\0R\x0BoneofString\x12!\n\x0Boneof_bytes\x18r \x01(\x0CH\0R\noneofBytes\x12\x1F\n\noneof_bool\x18s \x01(\x08H\0R\toneofBool\x12#\n\x0Coneof_uint64\x18t \x01(\x04H\0R\x0BoneofUint64\x12!\n\x0Boneof_float\x18u \x01(\x02H\0R\noneofFloat\x12#\n\x0Coneof_double\x18v \x01(\x01H\0R\x0BoneofDouble\x12]\n\noneof_enum\x18w \x01(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumH\0R\toneofEnum\x12F\n\x10oneof_null_value\x18x \x01(\x0E2\x1A.google.protobuf.NullValueH\0R\x0EoneofNullValue\x12O\n\x15optional_bool_wrapper\x18\xC9\x01 \x01(\x0B2\x1A.google.protobuf.BoolValueR\x13optionalBoolWrapper\x12R\n\x16optional_int32_wrapper\x18\xCA\x01 \x01(\x0B2\x1B.google.protobuf.Int32ValueR\x14optionalInt32Wrapper\x12R\n\x16optional_int64_wrapper\x18\xCB\x01 \x01(\x0B2\x1B.google.protobuf.Int64ValueR\x14optionalInt64Wrapper\x12U\n\x17optional_uint32_wrapper\x18\xCC\x01 \x01(\x0B2\x1C.google.protobuf.UInt32ValueR\x15optionalUint32Wrapper\x12U\n\x17optional_uint64_wrapper\x18\xCD\x01 \x01(\x0B2\x1C.google.protobuf.UInt64ValueR\x15optionalUint64Wrapper\x12R\n\x16optional_float_wrapper\x18\xCE\x01 \x01(\x0B2\x1B.google.protobuf.FloatValueR\x14optionalFloatWrapper\x12U\n\x17optional_double_wrapper\x18\xCF\x01 \x01(\x0B2\x1C.google.protobuf.DoubleValueR\x15optionalDoubleWrapper\x12U\n\x17optional_string_wrapper\x18\xD0\x01 \x01(\x0B2\x1C.google.protobuf.StringValueR\x15optionalStringWrapper\x12R\n\x16optional_bytes_wrapper\x18\xD1\x01 \x01(\x0B2\x1B.google.protobuf.BytesValueR\x14optionalBytesWrapper\x12O\n\x15repeated_bool_wrapper\x18\xD3\x01 \x03(\x0B2\x1A.google.protobuf.BoolValueR\x13repeatedBoolWrapper\x12R\n\x16repeated_int32_wrapper\x18\xD4\x01 \x03(\x0B2\x1B.google.protobuf.Int32ValueR\x14repeatedInt32Wrapper\x12R\n\x16repeated_int64_wrapper\x18\xD5\x01 \x03(\x0B2\x1B.google.protobuf.Int64ValueR\x14repeatedInt64Wrapper\x12U\n\x17repeated_uint32_wrapper\x18\xD6\x01 \x03(\x0B2\x1C.google.protobuf.UInt32ValueR\x15repeatedUint32Wrapper\x12U\n\x17repeated_uint64_wrapper\x18\xD7\x01 \x03(\x0B2\x1C.google.protobuf.UInt64ValueR\x15repeatedUint64Wrapper\x12R\n\x16repeated_float_wrapper\x18\xD8\x01 \x03(\x0B2\x1B.google.protobuf.FloatValueR\x14repeatedFloatWrapper\x12U\n\x17repeated_double_wrapper\x18\xD9\x01 \x03(\x0B2\x1C.google.protobuf.DoubleValueR\x15repeatedDoubleWrapper\x12U\n\x17repeated_string_wrapper\x18\xDA\x01 \x03(\x0B2\x1C.google.protobuf.StringValueR\x15repeatedStringWrapper\x12R\n\x16repeated_bytes_wrapper\x18\xDB\x01 \x03(\x0B2\x1B.google.protobuf.BytesValueR\x14repeatedBytesWrapper\x12G\n\x11optional_duration\x18\xAD\x02 \x01(\x0B2\x19.google.protobuf.DurationR\x10optionalDuration\x12J\n\x12optional_timestamp\x18\xAE\x02 \x01(\x0B2\x1A.google.protobuf.TimestampR\x11optionalTimestamp\x12K\n\x13optional_field_mask\x18\xAF\x02 \x01(\x0B2\x1A.google.protobuf.FieldMaskR\x11optionalFieldMask\x12A\n\x0Foptional_struct\x18\xB0\x02 \x01(\x0B2\x17.google.protobuf.StructR\x0EoptionalStruct\x128\n\x0Coptional_any\x18\xB1\x02 \x01(\x0B2\x14.google.protobuf.AnyR\x0BoptionalAny\x12>\n\x0Eoptional_value\x18\xB2\x02 \x01(\x0B2\x16.google.protobuf.ValueR\roptionalValue\x12K\n\x13optional_null_value\x18\xB3\x02 \x01(\x0E2\x1A.google.protobuf.NullValueR\x11optionalNullValue\x12G\n\x11repeated_duration\x18\xB7\x02 \x03(\x0B2\x19.google.protobuf.DurationR\x10repeatedDuration\x12J\n\x12repeated_timestamp\x18\xB8\x02 \x03(\x0B2\x1A.google.protobuf.TimestampR\x11repeatedTimestamp\x12J\n\x12repeated_fieldmask\x18\xB9\x02 \x03(\x0B2\x1A.google.protobuf.FieldMaskR\x11repeatedFieldmask\x12A\n\x0Frepeated_struct\x18\xC4\x02 \x03(\x0B2\x17.google.protobuf.StructR\x0ErepeatedStruct\x128\n\x0Crepeated_any\x18\xBB\x02 \x03(\x0B2\x14.google.protobuf.AnyR\x0BrepeatedAny\x12>\n\x0Erepeated_value\x18\xBC\x02 \x03(\x0B2\x16.google.protobuf.ValueR\rrepeatedValue\x12K\n\x13repeated_list_value\x18\xBD\x02 \x03(\x0B2\x1A.google.protobuf.ListValueR\x11repeatedListValue\x12\x1F\n\nfieldname1\x18\x91\x03 \x01(\x05R\nfieldname1\x12 \n\x0Bfield_name2\x18\x92\x03 \x01(\x05R\nfieldName2\x12!\n\x0C_field_name3\x18\x93\x03 \x01(\x05R\nFieldName3\x12\"\n\rfield__name4_\x18\x94\x03 \x01(\x05R\nfieldName4\x12!\n\x0Bfield0name5\x18\x95\x03 \x01(\x05R\x0Bfield0name5\x12#\n\rfield_0_name6\x18\x96\x03 \x01(\x05R\x0Bfield0Name6\x12\x1F\n\nfieldName7\x18\x97\x03 \x01(\x05R\nfieldName7\x12\x1F\n\nFieldName8\x18\x98\x03 \x01(\x05R\nFieldName8\x12 \n\x0Bfield_Name9\x18\x99\x03 \x01(\x05R\nfieldName9\x12\"\n\x0CField_Name10\x18\x9A\x03 \x01(\x05R\x0BFieldName10\x12\"\n\x0CFIELD_NAME11\x18\x9B\x03 \x01(\x05R\x0BFIELDNAME11\x12\"\n\x0CFIELD_name12\x18\x9C\x03 \x01(\x05R\x0BFIELDName12\x12$\n\x0E__field_name13\x18\x9D\x03 \x01(\x05R\x0BFieldName13\x12$\n\x0E__Field_name14\x18\x9E\x03 \x01(\x05R\x0BFieldName14\x12#\n\rfield__name15\x18\x9F\x03 \x01(\x05R\x0BfieldName15\x12#\n\rfield__Name16\x18\xA0\x03 \x01(\x05R\x0BfieldName16\x12$\n\x0Efield_name17__\x18\xA1\x03 \x01(\x05R\x0BfieldName17\x12$\n\x0EField_name18__\x18\xA2\x03 \x01(\x05R\x0BFieldName18\x1Az\n\rNestedMessage\x12\x0C\n\x01a\x18\x01 \x01(\x05R\x01a\x12[\n\x0Bcorecursive\x18\x02 \x01(\x0B21.protobuf_test_messages.proto3.TestAllTypesProto3B\x06\xEA\x91\x01\x02\x08\x01R\x0Bcorecursive\x1A@\n\x12MapInt32Int32Entry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x05R\x05value:\x028\x01\x1A@\n\x12MapInt64Int64Entry\x12\x10\n\x03key\x18\x01 \x01(\x03R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x03R\x05value:\x028\x01\x1AB\n\x14MapUint32Uint32Entry\x12\x10\n\x03key\x18\x01 \x01(\rR\x03key\x12\x14\n\x05value\x18\x02 \x01(\rR\x05value:\x028\x01\x1AB\n\x14MapUint64Uint64Entry\x12\x10\n\x03key\x18\x01 \x01(\x04R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x04R\x05value:\x028\x01\x1AB\n\x14MapSint32Sint32Entry\x12\x10\n\x03key\x18\x01 \x01(\x11R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x11R\x05value:\x028\x01\x1AB\n\x14MapSint64Sint64Entry\x12\x10\n\x03key\x18\x01 \x01(\x12R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x12R\x05value:\x028\x01\x1AD\n\x16MapFixed32Fixed32Entry\x12\x10\n\x03key\x18\x01 \x01(\x07R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x07R\x05value:\x028\x01\x1AD\n\x16MapFixed64Fixed64Entry\x12\x10\n\x03key\x18\x01 \x01(\x06R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x06R\x05value:\x028\x01\x1AF\n\x18MapSfixed32Sfixed32Entry\x12\x10\n\x03key\x18\x01 \x01(\x0FR\x03key\x12\x14\n\x05value\x18\x02 \x01(\x0FR\x05value:\x028\x01\x1AF\n\x18MapSfixed64Sfixed64Entry\x12\x10\n\x03key\x18\x01 \x01(\x10R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x10R\x05value:\x028\x01\x1A@\n\x12MapInt32FloatEntry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x02R\x05value:\x028\x01\x1AA\n\x13MapInt32DoubleEntry\x12\x10\n\x03key\x18\x01 \x01(\x05R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x01R\x05value:\x028\x01\x1A>\n\x10MapBoolBoolEntry\x12\x10\n\x03key\x18\x01 \x01(\x08R\x03key\x12\x14\n\x05value\x18\x02 \x01(\x08R\x05value:\x028\x01\x1AB\n\x14MapStringStringEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x14\n\x05value\x18\x02 \x01(\tR\x05value:\x028\x01\x1AA\n\x13MapStringBytesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x14\n\x05value\x18\x02 \x01(\x0CR\x05value:\x028\x01\x1A\x8A\x01\n\x1BMapStringNestedMessageEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12U\n\x05value\x18\x02 \x01(\x0B2?.protobuf_test_messages.proto3.TestAllTypesProto3.NestedMessageR\x05value:\x028\x01\x1Ay\n\x1CMapStringForeignMessageEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12C\n\x05value\x18\x02 \x01(\x0B2-.protobuf_test_messages.proto3.ForeignMessageR\x05value:\x028\x01\x1A\x84\x01\n\x18MapStringNestedEnumEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12R\n\x05value\x18\x02 \x01(\x0E2<.protobuf_test_messages.proto3.TestAllTypesProto3.NestedEnumR\x05value:\x028\x01\x1As\n\x19MapStringForeignEnumEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12@\n\x05value\x18\x02 \x01(\x0E2*.protobuf_test_messages.proto3.ForeignEnumR\x05value:\x028\x01\"9\n\nNestedEnum\x12\x07\n\x03FOO\x10\0\x12\x07\n\x03BAR\x10\x01\x12\x07\n\x03BAZ\x10\x02\x12\x10\n\x03NEG\x10\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\"Y\n\x0BAliasedEnum\x12\r\n\tALIAS_FOO\x10\0\x12\r\n\tALIAS_BAR\x10\x01\x12\r\n\tALIAS_BAZ\x10\x02\x12\x07\n\x03QUX\x10\x02\x12\x07\n\x03qux\x10\x02\x12\x07\n\x03bAz\x10\x02\x1A\x02\x10\x01B\r\n\x0Boneof_fieldJ\x06\x08\xF5\x03\x10\xFF\x03\"\x1E\n\x0EForeignMessage\x12\x0C\n\x01c\x18\x01 \x01(\x05R\x01c*@\n\x0BForeignEnum\x12\x0F\n\x0BFOREIGN_FOO\x10\0\x12\x0F\n\x0BFOREIGN_BAR\x10\x01\x12\x0F\n\x0BFOREIGN_BAZ\x10\x02B8\n(com.google.protobuf_test_messages.proto3H\x01\xF8\x01\x01\xA2\x02\x06Proto3J\xB1q\n\x07\x12\x05%\0\x98\x02\x01\n\xC0\r\n\x01\x0C\x12\x03%\0\x122\xB5\r Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n Test schema for proto3 messages.  This test schema is used by:\n\n - benchmarks\n - fuzz tests\n - conformance tests\n\n\n\x08\n\x01\x02\x12\x03'\0&\n\t\n\x02\x03\0\x12\x03)\0\x1D\n\x08\n\x01\x08\x12\x03+\0A\n\t\n\x02\x08\x01\x12\x03+\0A\n\x08\n\x01\x08\x12\x03,\0$\n\t\n\x02\x08$\x12\x03,\0$\n\x08\n\x01\x08\x12\x03/\0\x1C\nD\n\x02\x08\t\x12\x03/\0\x1C\x1A9 This is the default, but we specify it here explicitly.\n\n\t\n\x02\x03\x01\x12\x031\0#\n\t\n\x02\x03\x02\x12\x032\0(\n\t\n\x02\x03\x03\x12\x033\0*\n\t\n\x02\x03\x04\x12\x034\0&\n\t\n\x02\x03\x05\x12\x035\0)\n\t\n\x02\x03\x06\x12\x036\0(\n\x08\n\x01\x08\x12\x038\0\x1F\n\t\n\x02\x08\x1F\x12\x038\0\x1F\n\xDB\x02\n\x02\x04\0\x12\x05A\0\x8E\x02\x01\x1A\xCD\x02 This proto includes every type of field in both singular and repeated\n forms.\n\n Also, crucially, all messages and enums in this file are eventually\n submessages of this message.  So for example, a fuzz test of TestAllTypes\n could trigger bugs that occur in any message type in this file.  We verify\n this stays true in a unit test.\n\n\n\n\x03\x04\0\x01\x12\x03A\x08\x1A\n\x0C\n\x04\x04\0\x03\0\x12\x04B\x02E\x03\n\x0C\n\x05\x04\0\x03\0\x01\x12\x03B\n\x17\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03C\x04\x10\n\x0E\n\x07\x04\0\x03\0\x02\0\x05\x12\x03C\x04\t\n\x0E\n\x07\x04\0\x03\0\x02\0\x01\x12\x03C\n\x0B\n\x0E\n\x07\x04\0\x03\0\x02\0\x03\x12\x03C\x0E\x0F\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03D\x04L\n\x0E\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03D\x04\x16\n\x0E\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03D\x17\"\n\x0E\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03D%&\n\x0E\n\x07\x04\0\x03\0\x02\x01\x08\x12\x03D'K\n\x11\n\n\x04\0\x03\0\x02\x01\x08\x9D\x12\x01\x12\x03D(J\n\x0C\n\x04\x04\0\x04\0\x12\x04G\x02L\x03\n\x0C\n\x05\x04\0\x04\0\x01\x12\x03G\x07\x11\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03H\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\0\x01\x12\x03H\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\0\x02\x12\x03H\n\x0B\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03I\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03I\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03I\n\x0B\n\r\n\x06\x04\0\x04\0\x02\x02\x12\x03J\x04\x0C\n\x0E\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03J\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03J\n\x0B\n(\n\x06\x04\0\x04\0\x02\x03\x12\x03K\x04\r\"\x19 Intentionally negative.\n\n\x0E\n\x07\x04\0\x04\0\x02\x03\x01\x12\x03K\x04\x07\n\x0E\n\x07\x04\0\x04\0\x02\x03\x02\x12\x03K\n\x0C\n\x0C\n\x04\x04\0\x04\x01\x12\x04N\x02W\x03\n\x0C\n\x05\x04\0\x04\x01\x01\x12\x03N\x07\x12\n\x0C\n\x05\x04\0\x04\x01\x03\x12\x03O\x04\x1E\n\r\n\x06\x04\0\x04\x01\x03\x02\x12\x03O\x04\x1E\n\r\n\x06\x04\0\x04\x01\x02\0\x12\x03Q\x04\x12\n\x0E\n\x07\x04\0\x04\x01\x02\0\x01\x12\x03Q\x04\r\n\x0E\n\x07\x04\0\x04\x01\x02\0\x02\x12\x03Q\x10\x11\n\r\n\x06\x04\0\x04\x01\x02\x01\x12\x03R\x04\x12\n\x0E\n\x07\x04\0\x04\x01\x02\x01\x01\x12\x03R\x04\r\n\x0E\n\x07\x04\0\x04\x01\x02\x01\x02\x12\x03R\x10\x11\n\r\n\x06\x04\0\x04\x01\x02\x02\x12\x03S\x04\x12\n\x0E\n\x07\x04\0\x04\x01\x02\x02\x01\x12\x03S\x04\r\n\x0E\n\x07\x04\0\x04\x01\x02\x02\x02\x12\x03S\x10\x11\n\r\n\x06\x04\0\x04\x01\x02\x03\x12\x03T\x04\x0C\n\x0E\n\x07\x04\0\x04\x01\x02\x03\x01\x12\x03T\x04\x07\n\x0E\n\x07\x04\0\x04\x01\x02\x03\x02\x12\x03T\n\x0B\n\r\n\x06\x04\0\x04\x01\x02\x04\x12\x03U\x04\x0C\n\x0E\n\x07\x04\0\x04\x01\x02\x04\x01\x12\x03U\x04\x07\n\x0E\n\x07\x04\0\x04\x01\x02\x04\x02\x12\x03U\n\x0B\n\r\n\x06\x04\0\x04\x01\x02\x05\x12\x03V\x04\x0C\n\x0E\n\x07\x04\0\x04\x01\x02\x05\x01\x12\x03V\x04\x07\n\x0E\n\x07\x04\0\x04\x01\x02\x05\x02\x12\x03V\n\x0B\n\x17\n\x04\x04\0\x02\0\x12\x03Z\x02\x1B\x1A\n Singular\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03Z\x02\x07\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03Z\x08\x16\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03Z\x19\x1A\n\x0B\n\x04\x04\0\x02\x01\x12\x03[\x02\x1B\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03[\x02\x07\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03[\x08\x16\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03[\x19\x1A\n\x0B\n\x04\x04\0\x02\x02\x12\x03\\\x02\x1D\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03\\\x02\x08\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03\\\t\x18\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03\\\x1B\x1C\n\x0B\n\x04\x04\0\x02\x03\x12\x03]\x02\x1D\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03]\x02\x08\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03]\t\x18\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03]\x1B\x1C\n\x0B\n\x04\x04\0\x02\x04\x12\x03^\x02\x1D\n\x0C\n\x05\x04\0\x02\x04\x05\x12\x03^\x02\x08\n\x0C\n\x05\x04\0\x02\x04\x01\x12\x03^\t\x18\n\x0C\n\x05\x04\0\x02\x04\x03\x12\x03^\x1B\x1C\n\x0B\n\x04\x04\0\x02\x05\x12\x03_\x02\x1D\n\x0C\n\x05\x04\0\x02\x05\x05\x12\x03_\x02\x08\n\x0C\n\x05\x04\0\x02\x05\x01\x12\x03_\t\x18\n\x0C\n\x05\x04\0\x02\x05\x03\x12\x03_\x1B\x1C\n\x0B\n\x04\x04\0\x02\x06\x12\x03`\x02\x1F\n\x0C\n\x05\x04\0\x02\x06\x05\x12\x03`\x02\t\n\x0C\n\x05\x04\0\x02\x06\x01\x12\x03`\n\x1A\n\x0C\n\x05\x04\0\x02\x06\x03\x12\x03`\x1D\x1E\n\x0B\n\x04\x04\0\x02\x07\x12\x03a\x02\x1F\n\x0C\n\x05\x04\0\x02\x07\x05\x12\x03a\x02\t\n\x0C\n\x05\x04\0\x02\x07\x01\x12\x03a\n\x1A\n\x0C\n\x05\x04\0\x02\x07\x03\x12\x03a\x1D\x1E\n\x0B\n\x04\x04\0\x02\x08\x12\x03b\x02!\n\x0C\n\x05\x04\0\x02\x08\x05\x12\x03b\x02\n\n\x0C\n\x05\x04\0\x02\x08\x01\x12\x03b\x0B\x1C\n\x0C\n\x05\x04\0\x02\x08\x03\x12\x03b\x1F \n\x0B\n\x04\x04\0\x02\t\x12\x03c\x02\"\n\x0C\n\x05\x04\0\x02\t\x05\x12\x03c\x02\n\n\x0C\n\x05\x04\0\x02\t\x01\x12\x03c\x0B\x1C\n\x0C\n\x05\x04\0\x02\t\x03\x12\x03c\x1F!\n\x0B\n\x04\x04\0\x02\n\x12\x03d\x02\x1C\n\x0C\n\x05\x04\0\x02\n\x05\x12\x03d\x02\x07\n\x0C\n\x05\x04\0\x02\n\x01\x12\x03d\x08\x16\n\x0C\n\x05\x04\0\x02\n\x03\x12\x03d\x19\x1B\n\x0B\n\x04\x04\0\x02\x0B\x12\x03e\x02\x1E\n\x0C\n\x05\x04\0\x02\x0B\x05\x12\x03e\x02\x08\n\x0C\n\x05\x04\0\x02\x0B\x01\x12\x03e\t\x18\n\x0C\n\x05\x04\0\x02\x0B\x03\x12\x03e\x1B\x1D\n\x0B\n\x04\x04\0\x02\x0C\x12\x03f\x02\x1A\n\x0C\n\x05\x04\0\x02\x0C\x05\x12\x03f\x02\x06\n\x0C\n\x05\x04\0\x02\x0C\x01\x12\x03f\x07\x14\n\x0C\n\x05\x04\0\x02\x0C\x03\x12\x03f\x17\x19\n\x0B\n\x04\x04\0\x02\r\x12\x03g\x02\x1E\n\x0C\n\x05\x04\0\x02\r\x05\x12\x03g\x02\x08\n\x0C\n\x05\x04\0\x02\r\x01\x12\x03g\t\x18\n\x0C\n\x05\x04\0\x02\r\x03\x12\x03g\x1B\x1D\n\x0B\n\x04\x04\0\x02\x0E\x12\x03h\x02\x1C\n\x0C\n\x05\x04\0\x02\x0E\x05\x12\x03h\x02\x07\n\x0C\n\x05\x04\0\x02\x0E\x01\x12\x03h\x08\x16\n\x0C\n\x05\x04\0\x02\x0E\x03\x12\x03h\x19\x1B\n\x0B\n\x04\x04\0\x02\x0F\x12\x03j\x02-\n\x0C\n\x05\x04\0\x02\x0F\x06\x12\x03j\x02\x0F\n\x0C\n\x05\x04\0\x02\x0F\x01\x12\x03j\x10'\n\x0C\n\x05\x04\0\x02\x0F\x03\x12\x03j*,\n\x0B\n\x04\x04\0\x02\x10\x12\x03k\x02/\n\x0C\n\x05\x04\0\x02\x10\x06\x12\x03k\x02\x10\n\x0C\n\x05\x04\0\x02\x10\x01\x12\x03k\x11)\n\x0C\n\x05\x04\0\x02\x10\x03\x12\x03k,.\n\x0B\n\x04\x04\0\x02\x11\x12\x03m\x02'\n\x0C\n\x05\x04\0\x02\x11\x06\x12\x03m\x02\x0C\n\x0C\n\x05\x04\0\x02\x11\x01\x12\x03m\r!\n\x0C\n\x05\x04\0\x02\x11\x03\x12\x03m$&\n\x0B\n\x04\x04\0\x02\x12\x12\x03n\x02)\n\x0C\n\x05\x04\0\x02\x12\x06\x12\x03n\x02\r\n\x0C\n\x05\x04\0\x02\x12\x01\x12\x03n\x0E#\n\x0C\n\x05\x04\0\x02\x12\x03\x12\x03n&(\n\x0B\n\x04\x04\0\x02\x13\x12\x03o\x02)\n\x0C\n\x05\x04\0\x02\x13\x06\x12\x03o\x02\r\n\x0C\n\x05\x04\0\x02\x13\x01\x12\x03o\x0E#\n\x0C\n\x05\x04\0\x02\x13\x03\x12\x03o&(\n\x0B\n\x04\x04\0\x02\x14\x12\x03q\x02;\n\x0C\n\x05\x04\0\x02\x14\x05\x12\x03q\x02\x08\n\x0C\n\x05\x04\0\x02\x14\x01\x12\x03q\t\x1E\n\x0C\n\x05\x04\0\x02\x14\x03\x12\x03q!#\n\x0C\n\x05\x04\0\x02\x14\x08\x12\x03q$:\n\r\n\x06\x04\0\x02\x14\x08\x01\x12\x03q%9\n\x0B\n\x04\x04\0\x02\x15\x12\x03r\x02+\n\x0C\n\x05\x04\0\x02\x15\x05\x12\x03r\x02\x08\n\x0C\n\x05\x04\0\x02\x15\x01\x12\x03r\t\x16\n\x0C\n\x05\x04\0\x02\x15\x03\x12\x03r\x19\x1B\n\x0C\n\x05\x04\0\x02\x15\x08\x12\x03r\x1C*\n\r\n\x06\x04\0\x02\x15\x08\x01\x12\x03r\x1D)\n\x0B\n\x04\x04\0\x02\x16\x12\x03t\x02Q\n\x0C\n\x05\x04\0\x02\x16\x06\x12\x03t\x02\x14\n\x0C\n\x05\x04\0\x02\x16\x01\x12\x03t\x15&\n\x0C\n\x05\x04\0\x02\x16\x03\x12\x03t)+\n\x0C\n\x05\x04\0\x02\x16\x08\x12\x03t,P\n\x0F\n\x08\x04\0\x02\x16\x08\x9D\x12\x01\x12\x03t-O\n\x17\n\x04\x04\0\x02\x17\x12\x03w\x02%\x1A\n Repeated\n\n\x0C\n\x05\x04\0\x02\x17\x04\x12\x03w\x02\n\n\x0C\n\x05\x04\0\x02\x17\x05\x12\x03w\x0B\x10\n\x0C\n\x05\x04\0\x02\x17\x01\x12\x03w\x11\x1F\n\x0C\n\x05\x04\0\x02\x17\x03\x12\x03w\"$\n\x0B\n\x04\x04\0\x02\x18\x12\x03x\x02%\n\x0C\n\x05\x04\0\x02\x18\x04\x12\x03x\x02\n\n\x0C\n\x05\x04\0\x02\x18\x05\x12\x03x\x0B\x10\n\x0C\n\x05\x04\0\x02\x18\x01\x12\x03x\x11\x1F\n\x0C\n\x05\x04\0\x02\x18\x03\x12\x03x\"$\n\x0B\n\x04\x04\0\x02\x19\x12\x03y\x02'\n\x0C\n\x05\x04\0\x02\x19\x04\x12\x03y\x02\n\n\x0C\n\x05\x04\0\x02\x19\x05\x12\x03y\x0B\x11\n\x0C\n\x05\x04\0\x02\x19\x01\x12\x03y\x12!\n\x0C\n\x05\x04\0\x02\x19\x03\x12\x03y$&\n\x0B\n\x04\x04\0\x02\x1A\x12\x03z\x02'\n\x0C\n\x05\x04\0\x02\x1A\x04\x12\x03z\x02\n\n\x0C\n\x05\x04\0\x02\x1A\x05\x12\x03z\x0B\x11\n\x0C\n\x05\x04\0\x02\x1A\x01\x12\x03z\x12!\n\x0C\n\x05\x04\0\x02\x1A\x03\x12\x03z$&\n\x0B\n\x04\x04\0\x02\x1B\x12\x03{\x02'\n\x0C\n\x05\x04\0\x02\x1B\x04\x12\x03{\x02\n\n\x0C\n\x05\x04\0\x02\x1B\x05\x12\x03{\x0B\x11\n\x0C\n\x05\x04\0\x02\x1B\x01\x12\x03{\x12!\n\x0C\n\x05\x04\0\x02\x1B\x03\x12\x03{$&\n\x0B\n\x04\x04\0\x02\x1C\x12\x03|\x02'\n\x0C\n\x05\x04\0\x02\x1C\x04\x12\x03|\x02\n\n\x0C\n\x05\x04\0\x02\x1C\x05\x12\x03|\x0B\x11\n\x0C\n\x05\x04\0\x02\x1C\x01\x12\x03|\x12!\n\x0C\n\x05\x04\0\x02\x1C\x03\x12\x03|$&\n\x0B\n\x04\x04\0\x02\x1D\x12\x03}\x02)\n\x0C\n\x05\x04\0\x02\x1D\x04\x12\x03}\x02\n\n\x0C\n\x05\x04\0\x02\x1D\x05\x12\x03}\x0B\x12\n\x0C\n\x05\x04\0\x02\x1D\x01\x12\x03}\x13#\n\x0C\n\x05\x04\0\x02\x1D\x03\x12\x03}&(\n\x0B\n\x04\x04\0\x02\x1E\x12\x03~\x02)\n\x0C\n\x05\x04\0\x02\x1E\x04\x12\x03~\x02\n\n\x0C\n\x05\x04\0\x02\x1E\x05\x12\x03~\x0B\x12\n\x0C\n\x05\x04\0\x02\x1E\x01\x12\x03~\x13#\n\x0C\n\x05\x04\0\x02\x1E\x03\x12\x03~&(\n\x0B\n\x04\x04\0\x02\x1F\x12\x03\x7F\x02+\n\x0C\n\x05\x04\0\x02\x1F\x04\x12\x03\x7F\x02\n\n\x0C\n\x05\x04\0\x02\x1F\x05\x12\x03\x7F\x0B\x13\n\x0C\n\x05\x04\0\x02\x1F\x01\x12\x03\x7F\x14%\n\x0C\n\x05\x04\0\x02\x1F\x03\x12\x03\x7F(*\n\x0C\n\x04\x04\0\x02 \x12\x04\x80\x01\x02+\n\r\n\x05\x04\0\x02 \x04\x12\x04\x80\x01\x02\n\n\r\n\x05\x04\0\x02 \x05\x12\x04\x80\x01\x0B\x13\n\r\n\x05\x04\0\x02 \x01\x12\x04\x80\x01\x14%\n\r\n\x05\x04\0\x02 \x03\x12\x04\x80\x01(*\n\x0C\n\x04\x04\0\x02!\x12\x04\x81\x01\x02%\n\r\n\x05\x04\0\x02!\x04\x12\x04\x81\x01\x02\n\n\r\n\x05\x04\0\x02!\x05\x12\x04\x81\x01\x0B\x10\n\r\n\x05\x04\0\x02!\x01\x12\x04\x81\x01\x11\x1F\n\r\n\x05\x04\0\x02!\x03\x12\x04\x81\x01\"$\n\x0C\n\x04\x04\0\x02\"\x12\x04\x82\x01\x02'\n\r\n\x05\x04\0\x02\"\x04\x12\x04\x82\x01\x02\n\n\r\n\x05\x04\0\x02\"\x05\x12\x04\x82\x01\x0B\x11\n\r\n\x05\x04\0\x02\"\x01\x12\x04\x82\x01\x12!\n\r\n\x05\x04\0\x02\"\x03\x12\x04\x82\x01$&\n\x0C\n\x04\x04\0\x02#\x12\x04\x83\x01\x02#\n\r\n\x05\x04\0\x02#\x04\x12\x04\x83\x01\x02\n\n\r\n\x05\x04\0\x02#\x05\x12\x04\x83\x01\x0B\x0F\n\r\n\x05\x04\0\x02#\x01\x12\x04\x83\x01\x10\x1D\n\r\n\x05\x04\0\x02#\x03\x12\x04\x83\x01 \"\n\x0C\n\x04\x04\0\x02$\x12\x04\x84\x01\x02'\n\r\n\x05\x04\0\x02$\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\x04\0\x02$\x05\x12\x04\x84\x01\x0B\x11\n\r\n\x05\x04\0\x02$\x01\x12\x04\x84\x01\x12!\n\r\n\x05\x04\0\x02$\x03\x12\x04\x84\x01$&\n\x0C\n\x04\x04\0\x02%\x12\x04\x85\x01\x02%\n\r\n\x05\x04\0\x02%\x04\x12\x04\x85\x01\x02\n\n\r\n\x05\x04\0\x02%\x05\x12\x04\x85\x01\x0B\x10\n\r\n\x05\x04\0\x02%\x01\x12\x04\x85\x01\x11\x1F\n\r\n\x05\x04\0\x02%\x03\x12\x04\x85\x01\"$\n\x0C\n\x04\x04\0\x02&\x12\x04\x87\x01\x026\n\r\n\x05\x04\0\x02&\x04\x12\x04\x87\x01\x02\n\n\r\n\x05\x04\0\x02&\x06\x12\x04\x87\x01\x0B\x18\n\r\n\x05\x04\0\x02&\x01\x12\x04\x87\x01\x190\n\r\n\x05\x04\0\x02&\x03\x12\x04\x87\x0135\n\x0C\n\x04\x04\0\x02'\x12\x04\x88\x01\x028\n\r\n\x05\x04\0\x02'\x04\x12\x04\x88\x01\x02\n\n\r\n\x05\x04\0\x02'\x06\x12\x04\x88\x01\x0B\x19\n\r\n\x05\x04\0\x02'\x01\x12\x04\x88\x01\x1A2\n\r\n\x05\x04\0\x02'\x03\x12\x04\x88\x0157\n\x0C\n\x04\x04\0\x02(\x12\x04\x8A\x01\x020\n\r\n\x05\x04\0\x02(\x04\x12\x04\x8A\x01\x02\n\n\r\n\x05\x04\0\x02(\x06\x12\x04\x8A\x01\x0B\x15\n\r\n\x05\x04\0\x02(\x01\x12\x04\x8A\x01\x16*\n\r\n\x05\x04\0\x02(\x03\x12\x04\x8A\x01-/\n\x0C\n\x04\x04\0\x02)\x12\x04\x8B\x01\x022\n\r\n\x05\x04\0\x02)\x04\x12\x04\x8B\x01\x02\n\n\r\n\x05\x04\0\x02)\x06\x12\x04\x8B\x01\x0B\x16\n\r\n\x05\x04\0\x02)\x01\x12\x04\x8B\x01\x17,\n\r\n\x05\x04\0\x02)\x03\x12\x04\x8B\x01/1\n\x0C\n\x04\x04\0\x02*\x12\x04\x8D\x01\x02D\n\r\n\x05\x04\0\x02*\x04\x12\x04\x8D\x01\x02\n\n\r\n\x05\x04\0\x02*\x05\x12\x04\x8D\x01\x0B\x11\n\r\n\x05\x04\0\x02*\x01\x12\x04\x8D\x01\x12'\n\r\n\x05\x04\0\x02*\x03\x12\x04\x8D\x01*,\n\r\n\x05\x04\0\x02*\x08\x12\x04\x8D\x01-C\n\x0E\n\x06\x04\0\x02*\x08\x01\x12\x04\x8D\x01.B\n\x0C\n\x04\x04\0\x02+\x12\x04\x8E\x01\x024\n\r\n\x05\x04\0\x02+\x04\x12\x04\x8E\x01\x02\n\n\r\n\x05\x04\0\x02+\x05\x12\x04\x8E\x01\x0B\x11\n\r\n\x05\x04\0\x02+\x01\x12\x04\x8E\x01\x12\x1F\n\r\n\x05\x04\0\x02+\x03\x12\x04\x8E\x01\"$\n\r\n\x05\x04\0\x02+\x08\x12\x04\x8E\x01%3\n\x0E\n\x06\x04\0\x02+\x08\x01\x12\x04\x8E\x01&2\n\x16\n\x04\x04\0\x02,\x12\x04\x91\x01\x023\x1A\x08 Packed\n\n\r\n\x05\x04\0\x02,\x04\x12\x04\x91\x01\x02\n\n\r\n\x05\x04\0\x02,\x05\x12\x04\x91\x01\x0B\x10\n\r\n\x05\x04\0\x02,\x01\x12\x04\x91\x01\x11\x1D\n\r\n\x05\x04\0\x02,\x03\x12\x04\x91\x01 \"\n\r\n\x05\x04\0\x02,\x08\x12\x04\x91\x01#2\n\x0E\n\x06\x04\0\x02,\x08\x02\x12\x04\x91\x01$1\n\x0C\n\x04\x04\0\x02-\x12\x04\x92\x01\x023\n\r\n\x05\x04\0\x02-\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\0\x02-\x05\x12\x04\x92\x01\x0B\x10\n\r\n\x05\x04\0\x02-\x01\x12\x04\x92\x01\x11\x1D\n\r\n\x05\x04\0\x02-\x03\x12\x04\x92\x01 \"\n\r\n\x05\x04\0\x02-\x08\x12\x04\x92\x01#2\n\x0E\n\x06\x04\0\x02-\x08\x02\x12\x04\x92\x01$1\n\x0C\n\x04\x04\0\x02.\x12\x04\x93\x01\x025\n\r\n\x05\x04\0\x02.\x04\x12\x04\x93\x01\x02\n\n\r\n\x05\x04\0\x02.\x05\x12\x04\x93\x01\x0B\x11\n\r\n\x05\x04\0\x02.\x01\x12\x04\x93\x01\x12\x1F\n\r\n\x05\x04\0\x02.\x03\x12\x04\x93\x01\"$\n\r\n\x05\x04\0\x02.\x08\x12\x04\x93\x01%4\n\x0E\n\x06\x04\0\x02.\x08\x02\x12\x04\x93\x01&3\n\x0C\n\x04\x04\0\x02/\x12\x04\x94\x01\x025\n\r\n\x05\x04\0\x02/\x04\x12\x04\x94\x01\x02\n\n\r\n\x05\x04\0\x02/\x05\x12\x04\x94\x01\x0B\x11\n\r\n\x05\x04\0\x02/\x01\x12\x04\x94\x01\x12\x1F\n\r\n\x05\x04\0\x02/\x03\x12\x04\x94\x01\"$\n\r\n\x05\x04\0\x02/\x08\x12\x04\x94\x01%4\n\x0E\n\x06\x04\0\x02/\x08\x02\x12\x04\x94\x01&3\n\x0C\n\x04\x04\0\x020\x12\x04\x95\x01\x025\n\r\n\x05\x04\0\x020\x04\x12\x04\x95\x01\x02\n\n\r\n\x05\x04\0\x020\x05\x12\x04\x95\x01\x0B\x11\n\r\n\x05\x04\0\x020\x01\x12\x04\x95\x01\x12\x1F\n\r\n\x05\x04\0\x020\x03\x12\x04\x95\x01\"$\n\r\n\x05\x04\0\x020\x08\x12\x04\x95\x01%4\n\x0E\n\x06\x04\0\x020\x08\x02\x12\x04\x95\x01&3\n\x0C\n\x04\x04\0\x021\x12\x04\x96\x01\x025\n\r\n\x05\x04\0\x021\x04\x12\x04\x96\x01\x02\n\n\r\n\x05\x04\0\x021\x05\x12\x04\x96\x01\x0B\x11\n\r\n\x05\x04\0\x021\x01\x12\x04\x96\x01\x12\x1F\n\r\n\x05\x04\0\x021\x03\x12\x04\x96\x01\"$\n\r\n\x05\x04\0\x021\x08\x12\x04\x96\x01%4\n\x0E\n\x06\x04\0\x021\x08\x02\x12\x04\x96\x01&3\n\x0C\n\x04\x04\0\x022\x12\x04\x97\x01\x027\n\r\n\x05\x04\0\x022\x04\x12\x04\x97\x01\x02\n\n\r\n\x05\x04\0\x022\x05\x12\x04\x97\x01\x0B\x12\n\r\n\x05\x04\0\x022\x01\x12\x04\x97\x01\x13!\n\r\n\x05\x04\0\x022\x03\x12\x04\x97\x01$&\n\r\n\x05\x04\0\x022\x08\x12\x04\x97\x01'6\n\x0E\n\x06\x04\0\x022\x08\x02\x12\x04\x97\x01(5\n\x0C\n\x04\x04\0\x023\x12\x04\x98\x01\x027\n\r\n\x05\x04\0\x023\x04\x12\x04\x98\x01\x02\n\n\r\n\x05\x04\0\x023\x05\x12\x04\x98\x01\x0B\x12\n\r\n\x05\x04\0\x023\x01\x12\x04\x98\x01\x13!\n\r\n\x05\x04\0\x023\x03\x12\x04\x98\x01$&\n\r\n\x05\x04\0\x023\x08\x12\x04\x98\x01'6\n\x0E\n\x06\x04\0\x023\x08\x02\x12\x04\x98\x01(5\n\x0C\n\x04\x04\0\x024\x12\x04\x99\x01\x029\n\r\n\x05\x04\0\x024\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\x04\0\x024\x05\x12\x04\x99\x01\x0B\x13\n\r\n\x05\x04\0\x024\x01\x12\x04\x99\x01\x14#\n\r\n\x05\x04\0\x024\x03\x12\x04\x99\x01&(\n\r\n\x05\x04\0\x024\x08\x12\x04\x99\x01)8\n\x0E\n\x06\x04\0\x024\x08\x02\x12\x04\x99\x01*7\n\x0C\n\x04\x04\0\x025\x12\x04\x9A\x01\x029\n\r\n\x05\x04\0\x025\x04\x12\x04\x9A\x01\x02\n\n\r\n\x05\x04\0\x025\x05\x12\x04\x9A\x01\x0B\x13\n\r\n\x05\x04\0\x025\x01\x12\x04\x9A\x01\x14#\n\r\n\x05\x04\0\x025\x03\x12\x04\x9A\x01&(\n\r\n\x05\x04\0\x025\x08\x12\x04\x9A\x01)8\n\x0E\n\x06\x04\0\x025\x08\x02\x12\x04\x9A\x01*7\n\x0C\n\x04\x04\0\x026\x12\x04\x9B\x01\x023\n\r\n\x05\x04\0\x026\x04\x12\x04\x9B\x01\x02\n\n\r\n\x05\x04\0\x026\x05\x12\x04\x9B\x01\x0B\x10\n\r\n\x05\x04\0\x026\x01\x12\x04\x9B\x01\x11\x1D\n\r\n\x05\x04\0\x026\x03\x12\x04\x9B\x01 \"\n\r\n\x05\x04\0\x026\x08\x12\x04\x9B\x01#2\n\x0E\n\x06\x04\0\x026\x08\x02\x12\x04\x9B\x01$1\n\x0C\n\x04\x04\0\x027\x12\x04\x9C\x01\x025\n\r\n\x05\x04\0\x027\x04\x12\x04\x9C\x01\x02\n\n\r\n\x05\x04\0\x027\x05\x12\x04\x9C\x01\x0B\x11\n\r\n\x05\x04\0\x027\x01\x12\x04\x9C\x01\x12\x1F\n\r\n\x05\x04\0\x027\x03\x12\x04\x9C\x01\"$\n\r\n\x05\x04\0\x027\x08\x12\x04\x9C\x01%4\n\x0E\n\x06\x04\0\x027\x08\x02\x12\x04\x9C\x01&3\n\x0C\n\x04\x04\0\x028\x12\x04\x9D\x01\x021\n\r\n\x05\x04\0\x028\x04\x12\x04\x9D\x01\x02\n\n\r\n\x05\x04\0\x028\x05\x12\x04\x9D\x01\x0B\x0F\n\r\n\x05\x04\0\x028\x01\x12\x04\x9D\x01\x10\x1B\n\r\n\x05\x04\0\x028\x03\x12\x04\x9D\x01\x1E \n\r\n\x05\x04\0\x028\x08\x12\x04\x9D\x01!0\n\x0E\n\x06\x04\0\x028\x08\x02\x12\x04\x9D\x01\"/\n\x0C\n\x04\x04\0\x029\x12\x04\x9E\x01\x02>\n\r\n\x05\x04\0\x029\x04\x12\x04\x9E\x01\x02\n\n\r\n\x05\x04\0\x029\x06\x12\x04\x9E\x01\x0B\x15\n\r\n\x05\x04\0\x029\x01\x12\x04\x9E\x01\x16(\n\r\n\x05\x04\0\x029\x03\x12\x04\x9E\x01+-\n\r\n\x05\x04\0\x029\x08\x12\x04\x9E\x01.=\n\x0E\n\x06\x04\0\x029\x08\x02\x12\x04\x9E\x01/<\n\x18\n\x04\x04\0\x02:\x12\x04\xA1\x01\x026\x1A\n Unpacked\n\n\r\n\x05\x04\0\x02:\x04\x12\x04\xA1\x01\x02\n\n\r\n\x05\x04\0\x02:\x05\x12\x04\xA1\x01\x0B\x10\n\r\n\x05\x04\0\x02:\x01\x12\x04\xA1\x01\x11\x1F\n\r\n\x05\x04\0\x02:\x03\x12\x04\xA1\x01\"$\n\r\n\x05\x04\0\x02:\x08\x12\x04\xA1\x01%5\n\x0E\n\x06\x04\0\x02:\x08\x02\x12\x04\xA1\x01&4\n\x0C\n\x04\x04\0\x02;\x12\x04\xA2\x01\x026\n\r\n\x05\x04\0\x02;\x04\x12\x04\xA2\x01\x02\n\n\r\n\x05\x04\0\x02;\x05\x12\x04\xA2\x01\x0B\x10\n\r\n\x05\x04\0\x02;\x01\x12\x04\xA2\x01\x11\x1F\n\r\n\x05\x04\0\x02;\x03\x12\x04\xA2\x01\"$\n\r\n\x05\x04\0\x02;\x08\x12\x04\xA2\x01%5\n\x0E\n\x06\x04\0\x02;\x08\x02\x12\x04\xA2\x01&4\n\x0C\n\x04\x04\0\x02<\x12\x04\xA3\x01\x028\n\r\n\x05\x04\0\x02<\x04\x12\x04\xA3\x01\x02\n\n\r\n\x05\x04\0\x02<\x05\x12\x04\xA3\x01\x0B\x11\n\r\n\x05\x04\0\x02<\x01\x12\x04\xA3\x01\x12!\n\r\n\x05\x04\0\x02<\x03\x12\x04\xA3\x01$&\n\r\n\x05\x04\0\x02<\x08\x12\x04\xA3\x01'7\n\x0E\n\x06\x04\0\x02<\x08\x02\x12\x04\xA3\x01(6\n\x0C\n\x04\x04\0\x02=\x12\x04\xA4\x01\x028\n\r\n\x05\x04\0\x02=\x04\x12\x04\xA4\x01\x02\n\n\r\n\x05\x04\0\x02=\x05\x12\x04\xA4\x01\x0B\x11\n\r\n\x05\x04\0\x02=\x01\x12\x04\xA4\x01\x12!\n\r\n\x05\x04\0\x02=\x03\x12\x04\xA4\x01$&\n\r\n\x05\x04\0\x02=\x08\x12\x04\xA4\x01'7\n\x0E\n\x06\x04\0\x02=\x08\x02\x12\x04\xA4\x01(6\n\x0C\n\x04\x04\0\x02>\x12\x04\xA5\x01\x028\n\r\n\x05\x04\0\x02>\x04\x12\x04\xA5\x01\x02\n\n\r\n\x05\x04\0\x02>\x05\x12\x04\xA5\x01\x0B\x11\n\r\n\x05\x04\0\x02>\x01\x12\x04\xA5\x01\x12!\n\r\n\x05\x04\0\x02>\x03\x12\x04\xA5\x01$&\n\r\n\x05\x04\0\x02>\x08\x12\x04\xA5\x01'7\n\x0E\n\x06\x04\0\x02>\x08\x02\x12\x04\xA5\x01(6\n\x0C\n\x04\x04\0\x02?\x12\x04\xA6\x01\x028\n\r\n\x05\x04\0\x02?\x04\x12\x04\xA6\x01\x02\n\n\r\n\x05\x04\0\x02?\x05\x12\x04\xA6\x01\x0B\x11\n\r\n\x05\x04\0\x02?\x01\x12\x04\xA6\x01\x12!\n\r\n\x05\x04\0\x02?\x03\x12\x04\xA6\x01$&\n\r\n\x05\x04\0\x02?\x08\x12\x04\xA6\x01'7\n\x0E\n\x06\x04\0\x02?\x08\x02\x12\x04\xA6\x01(6\n\x0C\n\x04\x04\0\x02@\x12\x04\xA7\x01\x02:\n\r\n\x05\x04\0\x02@\x04\x12\x04\xA7\x01\x02\n\n\r\n\x05\x04\0\x02@\x05\x12\x04\xA7\x01\x0B\x12\n\r\n\x05\x04\0\x02@\x01\x12\x04\xA7\x01\x13#\n\r\n\x05\x04\0\x02@\x03\x12\x04\xA7\x01&(\n\r\n\x05\x04\0\x02@\x08\x12\x04\xA7\x01)9\n\x0E\n\x06\x04\0\x02@\x08\x02\x12\x04\xA7\x01*8\n\x0C\n\x04\x04\0\x02A\x12\x04\xA8\x01\x02:\n\r\n\x05\x04\0\x02A\x04\x12\x04\xA8\x01\x02\n\n\r\n\x05\x04\0\x02A\x05\x12\x04\xA8\x01\x0B\x12\n\r\n\x05\x04\0\x02A\x01\x12\x04\xA8\x01\x13#\n\r\n\x05\x04\0\x02A\x03\x12\x04\xA8\x01&(\n\r\n\x05\x04\0\x02A\x08\x12\x04\xA8\x01)9\n\x0E\n\x06\x04\0\x02A\x08\x02\x12\x04\xA8\x01*8\n\x0C\n\x04\x04\0\x02B\x12\x04\xA9\x01\x02<\n\r\n\x05\x04\0\x02B\x04\x12\x04\xA9\x01\x02\n\n\r\n\x05\x04\0\x02B\x05\x12\x04\xA9\x01\x0B\x13\n\r\n\x05\x04\0\x02B\x01\x12\x04\xA9\x01\x14%\n\r\n\x05\x04\0\x02B\x03\x12\x04\xA9\x01(*\n\r\n\x05\x04\0\x02B\x08\x12\x04\xA9\x01+;\n\x0E\n\x06\x04\0\x02B\x08\x02\x12\x04\xA9\x01,:\n\x0C\n\x04\x04\0\x02C\x12\x04\xAA\x01\x02<\n\r\n\x05\x04\0\x02C\x04\x12\x04\xAA\x01\x02\n\n\r\n\x05\x04\0\x02C\x05\x12\x04\xAA\x01\x0B\x13\n\r\n\x05\x04\0\x02C\x01\x12\x04\xAA\x01\x14%\n\r\n\x05\x04\0\x02C\x03\x12\x04\xAA\x01(*\n\r\n\x05\x04\0\x02C\x08\x12\x04\xAA\x01+;\n\x0E\n\x06\x04\0\x02C\x08\x02\x12\x04\xAA\x01,:\n\x0C\n\x04\x04\0\x02D\x12\x04\xAB\x01\x026\n\r\n\x05\x04\0\x02D\x04\x12\x04\xAB\x01\x02\n\n\r\n\x05\x04\0\x02D\x05\x12\x04\xAB\x01\x0B\x10\n\r\n\x05\x04\0\x02D\x01\x12\x04\xAB\x01\x11\x1F\n\r\n\x05\x04\0\x02D\x03\x12\x04\xAB\x01\"$\n\r\n\x05\x04\0\x02D\x08\x12\x04\xAB\x01%5\n\x0E\n\x06\x04\0\x02D\x08\x02\x12\x04\xAB\x01&4\n\x0C\n\x04\x04\0\x02E\x12\x04\xAC\x01\x029\n\r\n\x05\x04\0\x02E\x04\x12\x04\xAC\x01\x02\n\n\r\n\x05\x04\0\x02E\x05\x12\x04\xAC\x01\x0B\x11\n\r\n\x05\x04\0\x02E\x01\x12\x04\xAC\x01\x12!\n\r\n\x05\x04\0\x02E\x03\x12\x04\xAC\x01$'\n\r\n\x05\x04\0\x02E\x08\x12\x04\xAC\x01(8\n\x0E\n\x06\x04\0\x02E\x08\x02\x12\x04\xAC\x01)7\n\x0C\n\x04\x04\0\x02F\x12\x04\xAD\x01\x025\n\r\n\x05\x04\0\x02F\x04\x12\x04\xAD\x01\x02\n\n\r\n\x05\x04\0\x02F\x05\x12\x04\xAD\x01\x0B\x0F\n\r\n\x05\x04\0\x02F\x01\x12\x04\xAD\x01\x10\x1D\n\r\n\x05\x04\0\x02F\x03\x12\x04\xAD\x01 #\n\r\n\x05\x04\0\x02F\x08\x12\x04\xAD\x01$4\n\x0E\n\x06\x04\0\x02F\x08\x02\x12\x04\xAD\x01%3\n\x0C\n\x04\x04\0\x02G\x12\x04\xAE\x01\x02B\n\r\n\x05\x04\0\x02G\x04\x12\x04\xAE\x01\x02\n\n\r\n\x05\x04\0\x02G\x06\x12\x04\xAE\x01\x0B\x15\n\r\n\x05\x04\0\x02G\x01\x12\x04\xAE\x01\x16*\n\r\n\x05\x04\0\x02G\x03\x12\x04\xAE\x01-0\n\r\n\x05\x04\0\x02G\x08\x12\x04\xAE\x011A\n\x0E\n\x06\x04\0\x02G\x08\x02\x12\x04\xAE\x012@\n\x13\n\x04\x04\0\x02H\x12\x04\xB1\x01\x02)\x1A\x05 Map\n\n\r\n\x05\x04\0\x02H\x06\x12\x04\xB1\x01\x02\x13\n\r\n\x05\x04\0\x02H\x01\x12\x04\xB1\x01\x14#\n\r\n\x05\x04\0\x02H\x03\x12\x04\xB1\x01&(\n\x0C\n\x04\x04\0\x02I\x12\x04\xB2\x01\x02)\n\r\n\x05\x04\0\x02I\x06\x12\x04\xB2\x01\x02\x13\n\r\n\x05\x04\0\x02I\x01\x12\x04\xB2\x01\x14#\n\r\n\x05\x04\0\x02I\x03\x12\x04\xB2\x01&(\n\x0C\n\x04\x04\0\x02J\x12\x04\xB3\x01\x02-\n\r\n\x05\x04\0\x02J\x06\x12\x04\xB3\x01\x02\x15\n\r\n\x05\x04\0\x02J\x01\x12\x04\xB3\x01\x16'\n\r\n\x05\x04\0\x02J\x03\x12\x04\xB3\x01*,\n\x0C\n\x04\x04\0\x02K\x12\x04\xB4\x01\x02-\n\r\n\x05\x04\0\x02K\x06\x12\x04\xB4\x01\x02\x15\n\r\n\x05\x04\0\x02K\x01\x12\x04\xB4\x01\x16'\n\r\n\x05\x04\0\x02K\x03\x12\x04\xB4\x01*,\n\x0C\n\x04\x04\0\x02L\x12\x04\xB5\x01\x02-\n\r\n\x05\x04\0\x02L\x06\x12\x04\xB5\x01\x02\x15\n\r\n\x05\x04\0\x02L\x01\x12\x04\xB5\x01\x16'\n\r\n\x05\x04\0\x02L\x03\x12\x04\xB5\x01*,\n\x0C\n\x04\x04\0\x02M\x12\x04\xB6\x01\x02-\n\r\n\x05\x04\0\x02M\x06\x12\x04\xB6\x01\x02\x15\n\r\n\x05\x04\0\x02M\x01\x12\x04\xB6\x01\x16'\n\r\n\x05\x04\0\x02M\x03\x12\x04\xB6\x01*,\n\x0C\n\x04\x04\0\x02N\x12\x04\xB7\x01\x021\n\r\n\x05\x04\0\x02N\x06\x12\x04\xB7\x01\x02\x17\n\r\n\x05\x04\0\x02N\x01\x12\x04\xB7\x01\x18+\n\r\n\x05\x04\0\x02N\x03\x12\x04\xB7\x01.0\n\x0C\n\x04\x04\0\x02O\x12\x04\xB8\x01\x021\n\r\n\x05\x04\0\x02O\x06\x12\x04\xB8\x01\x02\x17\n\r\n\x05\x04\0\x02O\x01\x12\x04\xB8\x01\x18+\n\r\n\x05\x04\0\x02O\x03\x12\x04\xB8\x01.0\n\x0C\n\x04\x04\0\x02P\x12\x04\xB9\x01\x025\n\r\n\x05\x04\0\x02P\x06\x12\x04\xB9\x01\x02\x19\n\r\n\x05\x04\0\x02P\x01\x12\x04\xB9\x01\x1A/\n\r\n\x05\x04\0\x02P\x03\x12\x04\xB9\x0124\n\x0C\n\x04\x04\0\x02Q\x12\x04\xBA\x01\x025\n\r\n\x05\x04\0\x02Q\x06\x12\x04\xBA\x01\x02\x19\n\r\n\x05\x04\0\x02Q\x01\x12\x04\xBA\x01\x1A/\n\r\n\x05\x04\0\x02Q\x03\x12\x04\xBA\x0124\n\x0C\n\x04\x04\0\x02R\x12\x04\xBB\x01\x02)\n\r\n\x05\x04\0\x02R\x06\x12\x04\xBB\x01\x02\x13\n\r\n\x05\x04\0\x02R\x01\x12\x04\xBB\x01\x14#\n\r\n\x05\x04\0\x02R\x03\x12\x04\xBB\x01&(\n\x0C\n\x04\x04\0\x02S\x12\x04\xBC\x01\x02+\n\r\n\x05\x04\0\x02S\x06\x12\x04\xBC\x01\x02\x14\n\r\n\x05\x04\0\x02S\x01\x12\x04\xBC\x01\x15%\n\r\n\x05\x04\0\x02S\x03\x12\x04\xBC\x01(*\n\x0C\n\x04\x04\0\x02T\x12\x04\xBD\x01\x02%\n\r\n\x05\x04\0\x02T\x06\x12\x04\xBD\x01\x02\x11\n\r\n\x05\x04\0\x02T\x01\x12\x04\xBD\x01\x12\x1F\n\r\n\x05\x04\0\x02T\x03\x12\x04\xBD\x01\"$\n\x0C\n\x04\x04\0\x02U\x12\x04\xBE\x01\x02-\n\r\n\x05\x04\0\x02U\x06\x12\x04\xBE\x01\x02\x15\n\r\n\x05\x04\0\x02U\x01\x12\x04\xBE\x01\x16'\n\r\n\x05\x04\0\x02U\x03\x12\x04\xBE\x01*,\n\x0C\n\x04\x04\0\x02V\x12\x04\xBF\x01\x02+\n\r\n\x05\x04\0\x02V\x06\x12\x04\xBF\x01\x02\x14\n\r\n\x05\x04\0\x02V\x01\x12\x04\xBF\x01\x15%\n\r\n\x05\x04\0\x02V\x03\x12\x04\xBF\x01(*\n\x0C\n\x04\x04\0\x02W\x12\x04\xC0\x01\x02<\n\r\n\x05\x04\0\x02W\x06\x12\x04\xC0\x01\x02\x1C\n\r\n\x05\x04\0\x02W\x01\x12\x04\xC0\x01\x1D6\n\r\n\x05\x04\0\x02W\x03\x12\x04\xC0\x019;\n\x0C\n\x04\x04\0\x02X\x12\x04\xC1\x01\x02>\n\r\n\x05\x04\0\x02X\x06\x12\x04\xC1\x01\x02\x1D\n\r\n\x05\x04\0\x02X\x01\x12\x04\xC1\x01\x1E8\n\r\n\x05\x04\0\x02X\x03\x12\x04\xC1\x01;=\n\x0C\n\x04\x04\0\x02Y\x12\x04\xC2\x01\x026\n\r\n\x05\x04\0\x02Y\x06\x12\x04\xC2\x01\x02\x19\n\r\n\x05\x04\0\x02Y\x01\x12\x04\xC2\x01\x1A0\n\r\n\x05\x04\0\x02Y\x03\x12\x04\xC2\x0135\n\x0C\n\x04\x04\0\x02Z\x12\x04\xC3\x01\x028\n\r\n\x05\x04\0\x02Z\x06\x12\x04\xC3\x01\x02\x1A\n\r\n\x05\x04\0\x02Z\x01\x12\x04\xC3\x01\x1B2\n\r\n\x05\x04\0\x02Z\x03\x12\x04\xC3\x0157\n\x0E\n\x04\x04\0\x08\0\x12\x06\xC5\x01\x02\xD0\x01\x03\n\r\n\x05\x04\0\x08\0\x01\x12\x04\xC5\x01\x08\x13\n\x0C\n\x04\x04\0\x02[\x12\x04\xC6\x01\x04\x1E\n\r\n\x05\x04\0\x02[\x05\x12\x04\xC6\x01\x04\n\n\r\n\x05\x04\0\x02[\x01\x12\x04\xC6\x01\x0B\x17\n\r\n\x05\x04\0\x02[\x03\x12\x04\xC6\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02\\\x12\x04\xC7\x01\x04-\n\r\n\x05\x04\0\x02\\\x06\x12\x04\xC7\x01\x04\x11\n\r\n\x05\x04\0\x02\\\x01\x12\x04\xC7\x01\x12&\n\r\n\x05\x04\0\x02\\\x03\x12\x04\xC7\x01),\n\x0C\n\x04\x04\0\x02]\x12\x04\xC8\x01\x04\x1E\n\r\n\x05\x04\0\x02]\x05\x12\x04\xC8\x01\x04\n\n\r\n\x05\x04\0\x02]\x01\x12\x04\xC8\x01\x0B\x17\n\r\n\x05\x04\0\x02]\x03\x12\x04\xC8\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02^\x12\x04\xC9\x01\x04\x1C\n\r\n\x05\x04\0\x02^\x05\x12\x04\xC9\x01\x04\t\n\r\n\x05\x04\0\x02^\x01\x12\x04\xC9\x01\n\x15\n\r\n\x05\x04\0\x02^\x03\x12\x04\xC9\x01\x18\x1B\n\x0C\n\x04\x04\0\x02_\x12\x04\xCA\x01\x04\x1A\n\r\n\x05\x04\0\x02_\x05\x12\x04\xCA\x01\x04\x08\n\r\n\x05\x04\0\x02_\x01\x12\x04\xCA\x01\t\x13\n\r\n\x05\x04\0\x02_\x03\x12\x04\xCA\x01\x16\x19\n\x0C\n\x04\x04\0\x02`\x12\x04\xCB\x01\x04\x1E\n\r\n\x05\x04\0\x02`\x05\x12\x04\xCB\x01\x04\n\n\r\n\x05\x04\0\x02`\x01\x12\x04\xCB\x01\x0B\x17\n\r\n\x05\x04\0\x02`\x03\x12\x04\xCB\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02a\x12\x04\xCC\x01\x04\x1C\n\r\n\x05\x04\0\x02a\x05\x12\x04\xCC\x01\x04\t\n\r\n\x05\x04\0\x02a\x01\x12\x04\xCC\x01\n\x15\n\r\n\x05\x04\0\x02a\x03\x12\x04\xCC\x01\x18\x1B\n\x0C\n\x04\x04\0\x02b\x12\x04\xCD\x01\x04\x1E\n\r\n\x05\x04\0\x02b\x05\x12\x04\xCD\x01\x04\n\n\r\n\x05\x04\0\x02b\x01\x12\x04\xCD\x01\x0B\x17\n\r\n\x05\x04\0\x02b\x03\x12\x04\xCD\x01\x1A\x1D\n\x0C\n\x04\x04\0\x02c\x12\x04\xCE\x01\x04 \n\r\n\x05\x04\0\x02c\x06\x12\x04\xCE\x01\x04\x0E\n\r\n\x05\x04\0\x02c\x01\x12\x04\xCE\x01\x0F\x19\n\r\n\x05\x04\0\x02c\x03\x12\x04\xCE\x01\x1C\x1F\n\x0C\n\x04\x04\0\x02d\x12\x04\xCF\x01\x045\n\r\n\x05\x04\0\x02d\x06\x12\x04\xCF\x01\x04\x1D\n\r\n\x05\x04\0\x02d\x01\x12\x04\xCF\x01\x1E.\n\r\n\x05\x04\0\x02d\x03\x12\x04\xCF\x0114\n \n\x04\x04\0\x02e\x12\x04\xD3\x01\x028\x1A\x12 Well-known types\n\n\r\n\x05\x04\0\x02e\x06\x12\x04\xD3\x01\x02\x1B\n\r\n\x05\x04\0\x02e\x01\x12\x04\xD3\x01\x1C1\n\r\n\x05\x04\0\x02e\x03\x12\x04\xD3\x0147\n\x0C\n\x04\x04\0\x02f\x12\x04\xD4\x01\x02:\n\r\n\x05\x04\0\x02f\x06\x12\x04\xD4\x01\x02\x1C\n\r\n\x05\x04\0\x02f\x01\x12\x04\xD4\x01\x1D3\n\r\n\x05\x04\0\x02f\x03\x12\x04\xD4\x0169\n\x0C\n\x04\x04\0\x02g\x12\x04\xD5\x01\x02:\n\r\n\x05\x04\0\x02g\x06\x12\x04\xD5\x01\x02\x1C\n\r\n\x05\x04\0\x02g\x01\x12\x04\xD5\x01\x1D3\n\r\n\x05\x04\0\x02g\x03\x12\x04\xD5\x0169\n\x0C\n\x04\x04\0\x02h\x12\x04\xD6\x01\x02<\n\r\n\x05\x04\0\x02h\x06\x12\x04\xD6\x01\x02\x1D\n\r\n\x05\x04\0\x02h\x01\x12\x04\xD6\x01\x1E5\n\r\n\x05\x04\0\x02h\x03\x12\x04\xD6\x018;\n\x0C\n\x04\x04\0\x02i\x12\x04\xD7\x01\x02<\n\r\n\x05\x04\0\x02i\x06\x12\x04\xD7\x01\x02\x1D\n\r\n\x05\x04\0\x02i\x01\x12\x04\xD7\x01\x1E5\n\r\n\x05\x04\0\x02i\x03\x12\x04\xD7\x018;\n\x0C\n\x04\x04\0\x02j\x12\x04\xD8\x01\x02:\n\r\n\x05\x04\0\x02j\x06\x12\x04\xD8\x01\x02\x1C\n\r\n\x05\x04\0\x02j\x01\x12\x04\xD8\x01\x1D3\n\r\n\x05\x04\0\x02j\x03\x12\x04\xD8\x0169\n\x0C\n\x04\x04\0\x02k\x12\x04\xD9\x01\x02<\n\r\n\x05\x04\0\x02k\x06\x12\x04\xD9\x01\x02\x1D\n\r\n\x05\x04\0\x02k\x01\x12\x04\xD9\x01\x1E5\n\r\n\x05\x04\0\x02k\x03\x12\x04\xD9\x018;\n\x0C\n\x04\x04\0\x02l\x12\x04\xDA\x01\x02<\n\r\n\x05\x04\0\x02l\x06\x12\x04\xDA\x01\x02\x1D\n\r\n\x05\x04\0\x02l\x01\x12\x04\xDA\x01\x1E5\n\r\n\x05\x04\0\x02l\x03\x12\x04\xDA\x018;\n\x0C\n\x04\x04\0\x02m\x12\x04\xDB\x01\x02:\n\r\n\x05\x04\0\x02m\x06\x12\x04\xDB\x01\x02\x1C\n\r\n\x05\x04\0\x02m\x01\x12\x04\xDB\x01\x1D3\n\r\n\x05\x04\0\x02m\x03\x12\x04\xDB\x0169\n\x0C\n\x04\x04\0\x02n\x12\x04\xDD\x01\x02A\n\r\n\x05\x04\0\x02n\x04\x12\x04\xDD\x01\x02\n\n\r\n\x05\x04\0\x02n\x06\x12\x04\xDD\x01\x0B$\n\r\n\x05\x04\0\x02n\x01\x12\x04\xDD\x01%:\n\r\n\x05\x04\0\x02n\x03\x12\x04\xDD\x01=@\n\x0C\n\x04\x04\0\x02o\x12\x04\xDE\x01\x02C\n\r\n\x05\x04\0\x02o\x04\x12\x04\xDE\x01\x02\n\n\r\n\x05\x04\0\x02o\x06\x12\x04\xDE\x01\x0B%\n\r\n\x05\x04\0\x02o\x01\x12\x04\xDE\x01&<\n\r\n\x05\x04\0\x02o\x03\x12\x04\xDE\x01?B\n\x0C\n\x04\x04\0\x02p\x12\x04\xDF\x01\x02C\n\r\n\x05\x04\0\x02p\x04\x12\x04\xDF\x01\x02\n\n\r\n\x05\x04\0\x02p\x06\x12\x04\xDF\x01\x0B%\n\r\n\x05\x04\0\x02p\x01\x12\x04\xDF\x01&<\n\r\n\x05\x04\0\x02p\x03\x12\x04\xDF\x01?B\n\x0C\n\x04\x04\0\x02q\x12\x04\xE0\x01\x02E\n\r\n\x05\x04\0\x02q\x04\x12\x04\xE0\x01\x02\n\n\r\n\x05\x04\0\x02q\x06\x12\x04\xE0\x01\x0B&\n\r\n\x05\x04\0\x02q\x01\x12\x04\xE0\x01'>\n\r\n\x05\x04\0\x02q\x03\x12\x04\xE0\x01AD\n\x0C\n\x04\x04\0\x02r\x12\x04\xE1\x01\x02E\n\r\n\x05\x04\0\x02r\x04\x12\x04\xE1\x01\x02\n\n\r\n\x05\x04\0\x02r\x06\x12\x04\xE1\x01\x0B&\n\r\n\x05\x04\0\x02r\x01\x12\x04\xE1\x01'>\n\r\n\x05\x04\0\x02r\x03\x12\x04\xE1\x01AD\n\x0C\n\x04\x04\0\x02s\x12\x04\xE2\x01\x02C\n\r\n\x05\x04\0\x02s\x04\x12\x04\xE2\x01\x02\n\n\r\n\x05\x04\0\x02s\x06\x12\x04\xE2\x01\x0B%\n\r\n\x05\x04\0\x02s\x01\x12\x04\xE2\x01&<\n\r\n\x05\x04\0\x02s\x03\x12\x04\xE2\x01?B\n\x0C\n\x04\x04\0\x02t\x12\x04\xE3\x01\x02E\n\r\n\x05\x04\0\x02t\x04\x12\x04\xE3\x01\x02\n\n\r\n\x05\x04\0\x02t\x06\x12\x04\xE3\x01\x0B&\n\r\n\x05\x04\0\x02t\x01\x12\x04\xE3\x01'>\n\r\n\x05\x04\0\x02t\x03\x12\x04\xE3\x01AD\n\x0C\n\x04\x04\0\x02u\x12\x04\xE4\x01\x02E\n\r\n\x05\x04\0\x02u\x04\x12\x04\xE4\x01\x02\n\n\r\n\x05\x04\0\x02u\x06\x12\x04\xE4\x01\x0B&\n\r\n\x05\x04\0\x02u\x01\x12\x04\xE4\x01'>\n\r\n\x05\x04\0\x02u\x03\x12\x04\xE4\x01AD\n\x0C\n\x04\x04\0\x02v\x12\x04\xE5\x01\x02C\n\r\n\x05\x04\0\x02v\x04\x12\x04\xE5\x01\x02\n\n\r\n\x05\x04\0\x02v\x06\x12\x04\xE5\x01\x0B%\n\r\n\x05\x04\0\x02v\x01\x12\x04\xE5\x01&<\n\r\n\x05\x04\0\x02v\x03\x12\x04\xE5\x01?B\n\x0C\n\x04\x04\0\x02w\x12\x04\xE7\x01\x023\n\r\n\x05\x04\0\x02w\x06\x12\x04\xE7\x01\x02\x1A\n\r\n\x05\x04\0\x02w\x01\x12\x04\xE7\x01\x1B,\n\r\n\x05\x04\0\x02w\x03\x12\x04\xE7\x01/2\n\x0C\n\x04\x04\0\x02x\x12\x04\xE8\x01\x025\n\r\n\x05\x04\0\x02x\x06\x12\x04\xE8\x01\x02\x1B\n\r\n\x05\x04\0\x02x\x01\x12\x04\xE8\x01\x1C.\n\r\n\x05\x04\0\x02x\x03\x12\x04\xE8\x0114\n\x0C\n\x04\x04\0\x02y\x12\x04\xE9\x01\x026\n\r\n\x05\x04\0\x02y\x06\x12\x04\xE9\x01\x02\x1B\n\r\n\x05\x04\0\x02y\x01\x12\x04\xE9\x01\x1C/\n\r\n\x05\x04\0\x02y\x03\x12\x04\xE9\x0125\n\x0C\n\x04\x04\0\x02z\x12\x04\xEA\x01\x02/\n\r\n\x05\x04\0\x02z\x06\x12\x04\xEA\x01\x02\x18\n\r\n\x05\x04\0\x02z\x01\x12\x04\xEA\x01\x19(\n\r\n\x05\x04\0\x02z\x03\x12\x04\xEA\x01+.\n\x0C\n\x04\x04\0\x02{\x12\x04\xEB\x01\x02)\n\r\n\x05\x04\0\x02{\x06\x12\x04\xEB\x01\x02\x15\n\r\n\x05\x04\0\x02{\x01\x12\x04\xEB\x01\x16\"\n\r\n\x05\x04\0\x02{\x03\x12\x04\xEB\x01%(\n\x0C\n\x04\x04\0\x02|\x12\x04\xEC\x01\x02-\n\r\n\x05\x04\0\x02|\x06\x12\x04\xEC\x01\x02\x17\n\r\n\x05\x04\0\x02|\x01\x12\x04\xEC\x01\x18&\n\r\n\x05\x04\0\x02|\x03\x12\x04\xEC\x01),\n\x0C\n\x04\x04\0\x02}\x12\x04\xED\x01\x026\n\r\n\x05\x04\0\x02}\x06\x12\x04\xED\x01\x02\x1B\n\r\n\x05\x04\0\x02}\x01\x12\x04\xED\x01\x1C/\n\r\n\x05\x04\0\x02}\x03\x12\x04\xED\x0125\n\x0C\n\x04\x04\0\x02~\x12\x04\xEF\x01\x02<\n\r\n\x05\x04\0\x02~\x04\x12\x04\xEF\x01\x02\n\n\r\n\x05\x04\0\x02~\x06\x12\x04\xEF\x01\x0B#\n\r\n\x05\x04\0\x02~\x01\x12\x04\xEF\x01$5\n\r\n\x05\x04\0\x02~\x03\x12\x04\xEF\x018;\n\x0C\n\x04\x04\0\x02\x7F\x12\x04\xF0\x01\x02>\n\r\n\x05\x04\0\x02\x7F\x04\x12\x04\xF0\x01\x02\n\n\r\n\x05\x04\0\x02\x7F\x06\x12\x04\xF0\x01\x0B$\n\r\n\x05\x04\0\x02\x7F\x01\x12\x04\xF0\x01%7\n\r\n\x05\x04\0\x02\x7F\x03\x12\x04\xF0\x01:=\n\r\n\x05\x04\0\x02\x80\x01\x12\x04\xF1\x01\x02>\n\x0E\n\x06\x04\0\x02\x80\x01\x04\x12\x04\xF1\x01\x02\n\n\x0E\n\x06\x04\0\x02\x80\x01\x06\x12\x04\xF1\x01\x0B$\n\x0E\n\x06\x04\0\x02\x80\x01\x01\x12\x04\xF1\x01%7\n\x0E\n\x06\x04\0\x02\x80\x01\x03\x12\x04\xF1\x01:=\n\r\n\x05\x04\0\x02\x81\x01\x12\x04\xF2\x01\x028\n\x0E\n\x06\x04\0\x02\x81\x01\x04\x12\x04\xF2\x01\x02\n\n\x0E\n\x06\x04\0\x02\x81\x01\x06\x12\x04\xF2\x01\x0B!\n\x0E\n\x06\x04\0\x02\x81\x01\x01\x12\x04\xF2\x01\"1\n\x0E\n\x06\x04\0\x02\x81\x01\x03\x12\x04\xF2\x0147\n\r\n\x05\x04\0\x02\x82\x01\x12\x04\xF3\x01\x022\n\x0E\n\x06\x04\0\x02\x82\x01\x04\x12\x04\xF3\x01\x02\n\n\x0E\n\x06\x04\0\x02\x82\x01\x06\x12\x04\xF3\x01\x0B\x1E\n\x0E\n\x06\x04\0\x02\x82\x01\x01\x12\x04\xF3\x01\x1F+\n\x0E\n\x06\x04\0\x02\x82\x01\x03\x12\x04\xF3\x01.1\n\r\n\x05\x04\0\x02\x83\x01\x12\x04\xF4\x01\x026\n\x0E\n\x06\x04\0\x02\x83\x01\x04\x12\x04\xF4\x01\x02\n\n\x0E\n\x06\x04\0\x02\x83\x01\x06\x12\x04\xF4\x01\x0B \n\x0E\n\x06\x04\0\x02\x83\x01\x01\x12\x04\xF4\x01!/\n\x0E\n\x06\x04\0\x02\x83\x01\x03\x12\x04\xF4\x0125\n\r\n\x05\x04\0\x02\x84\x01\x12\x04\xF5\x01\x02?\n\x0E\n\x06\x04\0\x02\x84\x01\x04\x12\x04\xF5\x01\x02\n\n\x0E\n\x06\x04\0\x02\x84\x01\x06\x12\x04\xF5\x01\x0B$\n\x0E\n\x06\x04\0\x02\x84\x01\x01\x12\x04\xF5\x01%8\n\x0E\n\x06\x04\0\x02\x84\x01\x03\x12\x04\xF5\x01;>\ns\n\x05\x04\0\x02\x85\x01\x12\x04\xF9\x01\x02\x19\x1Ad Test field-name-to-JSON-name convention.\n (protobuf says names can be any valid C/C++ identifier.)\n\n\x0E\n\x06\x04\0\x02\x85\x01\x05\x12\x04\xF9\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x85\x01\x01\x12\x04\xF9\x01\x08\x12\n\x0E\n\x06\x04\0\x02\x85\x01\x03\x12\x04\xF9\x01\x15\x18\n\r\n\x05\x04\0\x02\x86\x01\x12\x04\xFA\x01\x02\x1A\n\x0E\n\x06\x04\0\x02\x86\x01\x05\x12\x04\xFA\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x86\x01\x01\x12\x04\xFA\x01\x08\x13\n\x0E\n\x06\x04\0\x02\x86\x01\x03\x12\x04\xFA\x01\x16\x19\n\r\n\x05\x04\0\x02\x87\x01\x12\x04\xFB\x01\x02\x1B\n\x0E\n\x06\x04\0\x02\x87\x01\x05\x12\x04\xFB\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x87\x01\x01\x12\x04\xFB\x01\x08\x14\n\x0E\n\x06\x04\0\x02\x87\x01\x03\x12\x04\xFB\x01\x17\x1A\n\r\n\x05\x04\0\x02\x88\x01\x12\x04\xFC\x01\x02\x1C\n\x0E\n\x06\x04\0\x02\x88\x01\x05\x12\x04\xFC\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x88\x01\x01\x12\x04\xFC\x01\x08\x15\n\x0E\n\x06\x04\0\x02\x88\x01\x03\x12\x04\xFC\x01\x18\x1B\n\r\n\x05\x04\0\x02\x89\x01\x12\x04\xFD\x01\x02\x1A\n\x0E\n\x06\x04\0\x02\x89\x01\x05\x12\x04\xFD\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x89\x01\x01\x12\x04\xFD\x01\x08\x13\n\x0E\n\x06\x04\0\x02\x89\x01\x03\x12\x04\xFD\x01\x16\x19\n\r\n\x05\x04\0\x02\x8A\x01\x12\x04\xFE\x01\x02\x1C\n\x0E\n\x06\x04\0\x02\x8A\x01\x05\x12\x04\xFE\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x8A\x01\x01\x12\x04\xFE\x01\x08\x15\n\x0E\n\x06\x04\0\x02\x8A\x01\x03\x12\x04\xFE\x01\x18\x1B\n\r\n\x05\x04\0\x02\x8B\x01\x12\x04\xFF\x01\x02\x19\n\x0E\n\x06\x04\0\x02\x8B\x01\x05\x12\x04\xFF\x01\x02\x07\n\x0E\n\x06\x04\0\x02\x8B\x01\x01\x12\x04\xFF\x01\x08\x12\n\x0E\n\x06\x04\0\x02\x8B\x01\x03\x12\x04\xFF\x01\x15\x18\n\r\n\x05\x04\0\x02\x8C\x01\x12\x04\x80\x02\x02\x19\n\x0E\n\x06\x04\0\x02\x8C\x01\x05\x12\x04\x80\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x8C\x01\x01\x12\x04\x80\x02\x08\x12\n\x0E\n\x06\x04\0\x02\x8C\x01\x03\x12\x04\x80\x02\x15\x18\n\r\n\x05\x04\0\x02\x8D\x01\x12\x04\x81\x02\x02\x1A\n\x0E\n\x06\x04\0\x02\x8D\x01\x05\x12\x04\x81\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x8D\x01\x01\x12\x04\x81\x02\x08\x13\n\x0E\n\x06\x04\0\x02\x8D\x01\x03\x12\x04\x81\x02\x16\x19\n\r\n\x05\x04\0\x02\x8E\x01\x12\x04\x82\x02\x02\x1B\n\x0E\n\x06\x04\0\x02\x8E\x01\x05\x12\x04\x82\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x8E\x01\x01\x12\x04\x82\x02\x08\x14\n\x0E\n\x06\x04\0\x02\x8E\x01\x03\x12\x04\x82\x02\x17\x1A\n\r\n\x05\x04\0\x02\x8F\x01\x12\x04\x83\x02\x02\x1B\n\x0E\n\x06\x04\0\x02\x8F\x01\x05\x12\x04\x83\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x8F\x01\x01\x12\x04\x83\x02\x08\x14\n\x0E\n\x06\x04\0\x02\x8F\x01\x03\x12\x04\x83\x02\x17\x1A\n\r\n\x05\x04\0\x02\x90\x01\x12\x04\x84\x02\x02\x1B\n\x0E\n\x06\x04\0\x02\x90\x01\x05\x12\x04\x84\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x90\x01\x01\x12\x04\x84\x02\x08\x14\n\x0E\n\x06\x04\0\x02\x90\x01\x03\x12\x04\x84\x02\x17\x1A\n\r\n\x05\x04\0\x02\x91\x01\x12\x04\x85\x02\x02\x1D\n\x0E\n\x06\x04\0\x02\x91\x01\x05\x12\x04\x85\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x91\x01\x01\x12\x04\x85\x02\x08\x16\n\x0E\n\x06\x04\0\x02\x91\x01\x03\x12\x04\x85\x02\x19\x1C\n\r\n\x05\x04\0\x02\x92\x01\x12\x04\x86\x02\x02\x1D\n\x0E\n\x06\x04\0\x02\x92\x01\x05\x12\x04\x86\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x92\x01\x01\x12\x04\x86\x02\x08\x16\n\x0E\n\x06\x04\0\x02\x92\x01\x03\x12\x04\x86\x02\x19\x1C\n\r\n\x05\x04\0\x02\x93\x01\x12\x04\x87\x02\x02\x1C\n\x0E\n\x06\x04\0\x02\x93\x01\x05\x12\x04\x87\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x93\x01\x01\x12\x04\x87\x02\x08\x15\n\x0E\n\x06\x04\0\x02\x93\x01\x03\x12\x04\x87\x02\x18\x1B\n\r\n\x05\x04\0\x02\x94\x01\x12\x04\x88\x02\x02\x1C\n\x0E\n\x06\x04\0\x02\x94\x01\x05\x12\x04\x88\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x94\x01\x01\x12\x04\x88\x02\x08\x15\n\x0E\n\x06\x04\0\x02\x94\x01\x03\x12\x04\x88\x02\x18\x1B\n\r\n\x05\x04\0\x02\x95\x01\x12\x04\x89\x02\x02\x1D\n\x0E\n\x06\x04\0\x02\x95\x01\x05\x12\x04\x89\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x95\x01\x01\x12\x04\x89\x02\x08\x16\n\x0E\n\x06\x04\0\x02\x95\x01\x03\x12\x04\x89\x02\x19\x1C\n\r\n\x05\x04\0\x02\x96\x01\x12\x04\x8A\x02\x02\x1D\n\x0E\n\x06\x04\0\x02\x96\x01\x05\x12\x04\x8A\x02\x02\x07\n\x0E\n\x06\x04\0\x02\x96\x01\x01\x12\x04\x8A\x02\x08\x16\n\x0E\n\x06\x04\0\x02\x96\x01\x03\x12\x04\x8A\x02\x19\x1C\n2\n\x03\x04\0\t\x12\x04\x8D\x02\x02\x16\x1A% Reserved for testing unknown fields\n\n\x0C\n\x04\x04\0\t\0\x12\x04\x8D\x02\x0B\x15\n\r\n\x05\x04\0\t\0\x01\x12\x04\x8D\x02\x0B\x0E\n\r\n\x05\x04\0\t\0\x02\x12\x04\x8D\x02\x12\x15\n\x0C\n\x02\x04\x01\x12\x06\x90\x02\0\x92\x02\x01\n\x0B\n\x03\x04\x01\x01\x12\x04\x90\x02\x08\x16\n\x0C\n\x04\x04\x01\x02\0\x12\x04\x91\x02\x02\x0E\n\r\n\x05\x04\x01\x02\0\x05\x12\x04\x91\x02\x02\x07\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\x91\x02\x08\t\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\x91\x02\x0C\r\n\x0C\n\x02\x05\0\x12\x06\x94\x02\0\x98\x02\x01\n\x0B\n\x03\x05\0\x01\x12\x04\x94\x02\x05\x10\n\x0C\n\x04\x05\0\x02\0\x12\x04\x95\x02\x02\x12\n\r\n\x05\x05\0\x02\0\x01\x12\x04\x95\x02\x02\r\n\r\n\x05\x05\0\x02\0\x02\x12\x04\x95\x02\x10\x11\n\x0C\n\x04\x05\0\x02\x01\x12\x04\x96\x02\x02\x12\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\x96\x02\x02\r\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\x96\x02\x10\x11\n\x0C\n\x04\x05\0\x02\x02\x12\x04\x97\x02\x02\x12\n\r\n\x05\x05\0\x02\x02\x01\x12\x04\x97\x02\x02\r\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\x97\x02\x10\x11b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
