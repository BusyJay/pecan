#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WireFormat(i32);
impl pecan::Enumerate for WireFormat {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> WireFormat {
        WireFormat(v)
    }
}
impl WireFormat {
    pub const UNSPECIFIED: WireFormat = WireFormat(0);
    pub const PROTOBUF: WireFormat = WireFormat(1);
    pub const JSON: WireFormat = WireFormat(2);
    pub const JSPB: WireFormat = WireFormat(3);
    pub const TEXT_FORMAT: WireFormat = WireFormat(4);
    pub const fn new() -> WireFormat {
        WireFormat(0)
    }
}
impl std::fmt::Debug for WireFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "UNSPECIFIED"),
            1 => write!(f, "PROTOBUF"),
            2 => write!(f, "JSON"),
            3 => write!(f, "JSPB"),
            4 => write!(f, "TEXT_FORMAT"),
            v => write!(f, "WireFormat({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TestCategory(i32);
impl pecan::Enumerate for TestCategory {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> TestCategory {
        TestCategory(v)
    }
}
impl TestCategory {
    pub const UNSPECIFIED_TEST: TestCategory = TestCategory(0);
    pub const BINARY_TEST: TestCategory = TestCategory(1);
    pub const JSON_TEST: TestCategory = TestCategory(2);
    pub const JSON_IGNORE_UNKNOWN_PARSING_TEST: TestCategory = TestCategory(3);
    pub const JSPB_TEST: TestCategory = TestCategory(4);
    pub const TEXT_FORMAT_TEST: TestCategory = TestCategory(5);
    pub const fn new() -> TestCategory {
        TestCategory(0)
    }
}
impl std::fmt::Debug for TestCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "UNSPECIFIED_TEST"),
            1 => write!(f, "BINARY_TEST"),
            2 => write!(f, "JSON_TEST"),
            3 => write!(f, "JSON_IGNORE_UNKNOWN_PARSING_TEST"),
            4 => write!(f, "JSPB_TEST"),
            5 => write!(f, "TEXT_FORMAT_TEST"),
            v => write!(f, "TestCategory({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FailureSet {
    pub failure: Vec<String>,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl FailureSet {
    pub const fn new() -> FailureSet {
        FailureSet {
            failure: Vec::new(),
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for FailureSet {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => RefArray::<LengthPrefixed>::merge_from(&mut self.failure, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if !self.failure.is_empty() {
            for i in &self.failure {
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
        if !self.failure.is_empty() {
            l += self.failure.len() as u64 + RefArray::<LengthPrefixed>::size(&self.failure);
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
impl pecan::DefaultInstance for FailureSet {
    fn default_instance() -> &'static FailureSet {
        static DEFAULT: FailureSet = FailureSet::new();
        &DEFAULT
    }
}
impl Default for FailureSet {
    #[inline]
    fn default() -> FailureSet {
        FailureSet::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ConformanceRequest_Payload {
    ProtobufPayload(pecan::Bytes),
    JsonPayload(String),
    JspbPayload(String),
    TextPayload(String),
    None,
}
impl Default for ConformanceRequest_Payload {
    #[inline]
    fn default() -> ConformanceRequest_Payload {
        ConformanceRequest_Payload::None
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConformanceRequest {
    pub payload: ConformanceRequest_Payload,
    pub requested_output_format: WireFormat,
    pub message_type: String,
    pub test_category: TestCategory,
    pub jspb_encoding_options: Option<JspbEncodingConfig>,
    pub print_unknown_fields: bool,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl ConformanceRequest {
    pub const fn new() -> ConformanceRequest {
        ConformanceRequest {
            payload: ConformanceRequest_Payload::None,
            requested_output_format: WireFormat::new(),
            message_type: String::new(),
            test_category: TestCategory::new(),
            jspb_encoding_options: None,
            print_unknown_fields: false,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn protobuf_payload(&self) -> &pecan::Bytes {
        match &self.payload {
            ConformanceRequest_Payload::ProtobufPayload(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn protobuf_payload_mut(&mut self) -> &mut pecan::Bytes {
        if !matches!(self.payload, ConformanceRequest_Payload::ProtobufPayload(_)) {
            self.payload = ConformanceRequest_Payload::ProtobufPayload(pecan::Bytes::new());
        }
        match &mut self.payload {
            ConformanceRequest_Payload::ProtobufPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_protobuf_payload(&mut self, val: pecan::Bytes) {
        self.payload = ConformanceRequest_Payload::ProtobufPayload(val);
    }
    pub fn json_payload(&self) -> &String {
        match &self.payload {
            ConformanceRequest_Payload::JsonPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn json_payload_mut(&mut self) -> &mut String {
        if !matches!(self.payload, ConformanceRequest_Payload::JsonPayload(_)) {
            self.payload = ConformanceRequest_Payload::JsonPayload(String::new());
        }
        match &mut self.payload {
            ConformanceRequest_Payload::JsonPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_json_payload(&mut self, val: String) {
        self.payload = ConformanceRequest_Payload::JsonPayload(val);
    }
    pub fn jspb_payload(&self) -> &String {
        match &self.payload {
            ConformanceRequest_Payload::JspbPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn jspb_payload_mut(&mut self) -> &mut String {
        if !matches!(self.payload, ConformanceRequest_Payload::JspbPayload(_)) {
            self.payload = ConformanceRequest_Payload::JspbPayload(String::new());
        }
        match &mut self.payload {
            ConformanceRequest_Payload::JspbPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_jspb_payload(&mut self, val: String) {
        self.payload = ConformanceRequest_Payload::JspbPayload(val);
    }
    pub fn text_payload(&self) -> &String {
        match &self.payload {
            ConformanceRequest_Payload::TextPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn text_payload_mut(&mut self) -> &mut String {
        if !matches!(self.payload, ConformanceRequest_Payload::TextPayload(_)) {
            self.payload = ConformanceRequest_Payload::TextPayload(String::new());
        }
        match &mut self.payload {
            ConformanceRequest_Payload::TextPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_text_payload(&mut self, val: String) {
        self.payload = ConformanceRequest_Payload::TextPayload(val);
    }
    pub fn jspb_encoding_options(&self) -> &JspbEncodingConfig {
        match &self.jspb_encoding_options {
            Some(v) => v,
            _ => JspbEncodingConfig::default_instance(),
        }
    }
    pub fn jspb_encoding_options_mut(&mut self) -> &mut JspbEncodingConfig {
        self.jspb_encoding_options
            .get_or_insert_with(Default::default)
    }
    pub fn set_jspb_encoding_options(&mut self, val: JspbEncodingConfig) {
        self.jspb_encoding_options = Some(val);
    }
}
impl pecan::Message for ConformanceRequest {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => {
                    self.payload =
                        ConformanceRequest_Payload::ProtobufPayload(LengthPrefixed::read_from(s)?)
                }
                18 => {
                    self.payload =
                        ConformanceRequest_Payload::JsonPayload(LengthPrefixed::read_from(s)?)
                }
                24 => self.requested_output_format = Varint::read_from(s)?,
                34 => self.message_type = LengthPrefixed::read_from(s)?,
                40 => self.test_category = Varint::read_from(s)?,
                50 => LengthPrefixed::merge_from(self.jspb_encoding_options_mut(), s)?,
                58 => {
                    self.payload =
                        ConformanceRequest_Payload::JspbPayload(LengthPrefixed::read_from(s)?)
                }
                66 => {
                    self.payload =
                        ConformanceRequest_Payload::TextPayload(LengthPrefixed::read_from(s)?)
                }
                72 => self.print_unknown_fields = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        match &self.payload {
            ConformanceRequest_Payload::None => (),
            ConformanceRequest_Payload::ProtobufPayload(v) => {
                s.write_tag(10)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceRequest_Payload::JsonPayload(v) => {
                s.write_tag(18)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceRequest_Payload::JspbPayload(v) => {
                s.write_tag(58)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceRequest_Payload::TextPayload(v) => {
                s.write_tag(66)?;
                LengthPrefixed::write_to(v, s)?;
            }
        }
        if self.requested_output_format != WireFormat::new() {
            s.write_tag(24)?;
            Varint::write_to(self.requested_output_format, s)?;
        }
        if !self.message_type.is_empty() {
            s.write_tag(34)?;
            LengthPrefixed::write_to(&self.message_type, s)?;
        }
        if self.test_category != TestCategory::new() {
            s.write_tag(40)?;
            Varint::write_to(self.test_category, s)?;
        }
        if let Some(v) = &self.jspb_encoding_options {
            s.write_tag(50)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if self.print_unknown_fields {
            s.write_tag(72)?;
            Varint::write_to(self.print_unknown_fields, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        match &self.payload {
            ConformanceRequest_Payload::None => (),
            ConformanceRequest_Payload::ProtobufPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceRequest_Payload::JsonPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceRequest_Payload::JspbPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceRequest_Payload::TextPayload(v) => l += 1 + LengthPrefixed::size(v),
        }
        if self.requested_output_format != WireFormat::new() {
            l += 1 + Varint::size(self.requested_output_format);
        }
        if !self.message_type.is_empty() {
            l += 1 + LengthPrefixed::size(&self.message_type);
        }
        if self.test_category != TestCategory::new() {
            l += 1 + Varint::size(self.test_category);
        }
        if let Some(v) = &self.jspb_encoding_options {
            l += 1 + LengthPrefixed::size(v);
        }
        if self.print_unknown_fields {
            l += 1 + Varint::size(self.print_unknown_fields);
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
impl pecan::DefaultInstance for ConformanceRequest {
    fn default_instance() -> &'static ConformanceRequest {
        static DEFAULT: ConformanceRequest = ConformanceRequest::new();
        &DEFAULT
    }
}
impl Default for ConformanceRequest {
    #[inline]
    fn default() -> ConformanceRequest {
        ConformanceRequest::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ConformanceResponse_Result {
    ParseError(String),
    SerializeError(String),
    RuntimeError(String),
    ProtobufPayload(pecan::Bytes),
    JsonPayload(String),
    Skipped(String),
    JspbPayload(String),
    TextPayload(String),
    None,
}
impl Default for ConformanceResponse_Result {
    #[inline]
    fn default() -> ConformanceResponse_Result {
        ConformanceResponse_Result::None
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConformanceResponse {
    pub result: ConformanceResponse_Result,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl ConformanceResponse {
    pub const fn new() -> ConformanceResponse {
        ConformanceResponse {
            result: ConformanceResponse_Result::None,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
    pub fn parse_error(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::ParseError(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn parse_error_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::ParseError(_)) {
            self.result = ConformanceResponse_Result::ParseError(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::ParseError(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_parse_error(&mut self, val: String) {
        self.result = ConformanceResponse_Result::ParseError(val);
    }
    pub fn serialize_error(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::SerializeError(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn serialize_error_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::SerializeError(_)) {
            self.result = ConformanceResponse_Result::SerializeError(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::SerializeError(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_serialize_error(&mut self, val: String) {
        self.result = ConformanceResponse_Result::SerializeError(val);
    }
    pub fn runtime_error(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::RuntimeError(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn runtime_error_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::RuntimeError(_)) {
            self.result = ConformanceResponse_Result::RuntimeError(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::RuntimeError(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_runtime_error(&mut self, val: String) {
        self.result = ConformanceResponse_Result::RuntimeError(val);
    }
    pub fn protobuf_payload(&self) -> &pecan::Bytes {
        match &self.result {
            ConformanceResponse_Result::ProtobufPayload(v) => v,
            _ => pecan::Bytes::default_instance(),
        }
    }
    pub fn protobuf_payload_mut(&mut self) -> &mut pecan::Bytes {
        if !matches!(self.result, ConformanceResponse_Result::ProtobufPayload(_)) {
            self.result = ConformanceResponse_Result::ProtobufPayload(pecan::Bytes::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::ProtobufPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_protobuf_payload(&mut self, val: pecan::Bytes) {
        self.result = ConformanceResponse_Result::ProtobufPayload(val);
    }
    pub fn json_payload(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::JsonPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn json_payload_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::JsonPayload(_)) {
            self.result = ConformanceResponse_Result::JsonPayload(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::JsonPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_json_payload(&mut self, val: String) {
        self.result = ConformanceResponse_Result::JsonPayload(val);
    }
    pub fn skipped(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::Skipped(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn skipped_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::Skipped(_)) {
            self.result = ConformanceResponse_Result::Skipped(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::Skipped(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_skipped(&mut self, val: String) {
        self.result = ConformanceResponse_Result::Skipped(val);
    }
    pub fn jspb_payload(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::JspbPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn jspb_payload_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::JspbPayload(_)) {
            self.result = ConformanceResponse_Result::JspbPayload(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::JspbPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_jspb_payload(&mut self, val: String) {
        self.result = ConformanceResponse_Result::JspbPayload(val);
    }
    pub fn text_payload(&self) -> &String {
        match &self.result {
            ConformanceResponse_Result::TextPayload(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn text_payload_mut(&mut self) -> &mut String {
        if !matches!(self.result, ConformanceResponse_Result::TextPayload(_)) {
            self.result = ConformanceResponse_Result::TextPayload(String::new());
        }
        match &mut self.result {
            ConformanceResponse_Result::TextPayload(v) => v,
            _ => unreachable!(),
        }
    }
    pub fn set_text_payload(&mut self, val: String) {
        self.result = ConformanceResponse_Result::TextPayload(val);
    }
}
impl pecan::Message for ConformanceResponse {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => {
                    self.result =
                        ConformanceResponse_Result::ParseError(LengthPrefixed::read_from(s)?)
                }
                18 => {
                    self.result =
                        ConformanceResponse_Result::RuntimeError(LengthPrefixed::read_from(s)?)
                }
                26 => {
                    self.result =
                        ConformanceResponse_Result::ProtobufPayload(LengthPrefixed::read_from(s)?)
                }
                34 => {
                    self.result =
                        ConformanceResponse_Result::JsonPayload(LengthPrefixed::read_from(s)?)
                }
                42 => {
                    self.result = ConformanceResponse_Result::Skipped(LengthPrefixed::read_from(s)?)
                }
                50 => {
                    self.result =
                        ConformanceResponse_Result::SerializeError(LengthPrefixed::read_from(s)?)
                }
                58 => {
                    self.result =
                        ConformanceResponse_Result::JspbPayload(LengthPrefixed::read_from(s)?)
                }
                66 => {
                    self.result =
                        ConformanceResponse_Result::TextPayload(LengthPrefixed::read_from(s)?)
                }
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        match &self.result {
            ConformanceResponse_Result::None => (),
            ConformanceResponse_Result::ParseError(v) => {
                s.write_tag(10)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::SerializeError(v) => {
                s.write_tag(50)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::RuntimeError(v) => {
                s.write_tag(18)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::ProtobufPayload(v) => {
                s.write_tag(26)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::JsonPayload(v) => {
                s.write_tag(34)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::Skipped(v) => {
                s.write_tag(42)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::JspbPayload(v) => {
                s.write_tag(58)?;
                LengthPrefixed::write_to(v, s)?;
            }
            ConformanceResponse_Result::TextPayload(v) => {
                s.write_tag(66)?;
                LengthPrefixed::write_to(v, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        match &self.result {
            ConformanceResponse_Result::None => (),
            ConformanceResponse_Result::ParseError(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::SerializeError(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::RuntimeError(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::ProtobufPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::JsonPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::Skipped(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::JspbPayload(v) => l += 1 + LengthPrefixed::size(v),
            ConformanceResponse_Result::TextPayload(v) => l += 1 + LengthPrefixed::size(v),
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
impl pecan::DefaultInstance for ConformanceResponse {
    fn default_instance() -> &'static ConformanceResponse {
        static DEFAULT: ConformanceResponse = ConformanceResponse::new();
        &DEFAULT
    }
}
impl Default for ConformanceResponse {
    #[inline]
    fn default() -> ConformanceResponse {
        ConformanceResponse::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct JspbEncodingConfig {
    pub use_jspb_array_any_format: bool,
    _unknown: Vec<u8>,
    _cached_size: pecan::CachedSize,
}
impl JspbEncodingConfig {
    pub const fn new() -> JspbEncodingConfig {
        JspbEncodingConfig {
            use_jspb_array_any_format: false,
            _unknown: Vec::new(),
            _cached_size: pecan::CachedSize::new(),
        }
    }
}
impl pecan::Message for JspbEncodingConfig {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.use_jspb_array_any_format = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to_uncheck<B: pecan::BufMut>(
        &self,
        s: &mut CodedOutputStream<B>,
    ) -> pecan::Result<()> {
        if self.use_jspb_array_any_format {
            s.write_tag(8)?;
            Varint::write_to(self.use_jspb_array_any_format, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn size(&self) -> u64 {
        let mut l = 0;
        if self.use_jspb_array_any_format {
            l += 1 + Varint::size(self.use_jspb_array_any_format);
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
impl pecan::DefaultInstance for JspbEncodingConfig {
    fn default_instance() -> &'static JspbEncodingConfig {
        static DEFAULT: JspbEncodingConfig = JspbEncodingConfig::new();
        &DEFAULT
    }
}
impl Default for JspbEncodingConfig {
    #[inline]
    fn default() -> JspbEncodingConfig {
        JspbEncodingConfig::new()
    }
}
static DESCRIPTOR_RAW : & [u8] = b"\n\x1Dconformance/conformance.proto\x12\x0Bconformance\"&\n\nFailureSet\x12\x18\n\x07failure\x18\x01 \x03(\tR\x07failure\"\xF6\x03\n\x12ConformanceRequest\x12+\n\x10protobuf_payload\x18\x01 \x01(\x0CH\0R\x0FprotobufPayload\x12#\n\x0Cjson_payload\x18\x02 \x01(\tH\0R\x0BjsonPayload\x12#\n\x0Cjspb_payload\x18\x07 \x01(\tH\0R\x0BjspbPayload\x12#\n\x0Ctext_payload\x18\x08 \x01(\tH\0R\x0BtextPayload\x12O\n\x17requested_output_format\x18\x03 \x01(\x0E2\x17.conformance.WireFormatR\x15requestedOutputFormat\x12!\n\x0Cmessage_type\x18\x04 \x01(\tR\x0BmessageType\x12>\n\rtest_category\x18\x05 \x01(\x0E2\x19.conformance.TestCategoryR\x0CtestCategory\x12S\n\x15jspb_encoding_options\x18\x06 \x01(\x0B2\x1F.conformance.JspbEncodingConfigR\x13jspbEncodingOptions\x120\n\x14print_unknown_fields\x18\t \x01(\x08R\x12printUnknownFieldsB\t\n\x07payload\"\xCC\x02\n\x13ConformanceResponse\x12!\n\x0Bparse_error\x18\x01 \x01(\tH\0R\nparseError\x12)\n\x0Fserialize_error\x18\x06 \x01(\tH\0R\x0EserializeError\x12%\n\rruntime_error\x18\x02 \x01(\tH\0R\x0CruntimeError\x12+\n\x10protobuf_payload\x18\x03 \x01(\x0CH\0R\x0FprotobufPayload\x12#\n\x0Cjson_payload\x18\x04 \x01(\tH\0R\x0BjsonPayload\x12\x1A\n\x07skipped\x18\x05 \x01(\tH\0R\x07skipped\x12#\n\x0Cjspb_payload\x18\x07 \x01(\tH\0R\x0BjspbPayload\x12#\n\x0Ctext_payload\x18\x08 \x01(\tH\0R\x0BtextPayloadB\x08\n\x06result\"N\n\x12JspbEncodingConfig\x128\n\x19use_jspb_array_any_format\x18\x01 \x01(\x08R\x15useJspbArrayAnyFormat*P\n\nWireFormat\x12\x0F\n\x0BUNSPECIFIED\x10\0\x12\x0C\n\x08PROTOBUF\x10\x01\x12\x08\n\x04JSON\x10\x02\x12\x08\n\x04JSPB\x10\x03\x12\x0F\n\x0BTEXT_FORMAT\x10\x04*\x8F\x01\n\x0CTestCategory\x12\x14\n\x10UNSPECIFIED_TEST\x10\0\x12\x0F\n\x0BBINARY_TEST\x10\x01\x12\r\n\tJSON_TEST\x10\x02\x12$\n JSON_IGNORE_UNKNOWN_PARSING_TEST\x10\x03\x12\r\n\tJSPB_TEST\x10\x04\x12\x14\n\x10TEXT_FORMAT_TEST\x10\x05B!\n\x1Fcom.google.protobuf.conformanceJ\xB8<\n\x07\x12\x05\x1E\0\xAE\x01\x01\n\xCC\x0C\n\x01\x0C\x12\x03\x1E\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x1F\0\x14\n\x08\n\x01\x08\x12\x03 \08\n\t\n\x02\x08\x01\x12\x03 \08\n\xF3\x05\n\x02\x05\0\x12\x045\0;\x012\xE6\x05 This defines the conformance testing protocol.  This protocol exists between\n the conformance test suite itself and the code being tested.  For each test,\n the suite will send a ConformanceRequest message and expect a\n ConformanceResponse message.\n\n You can either run the tests in two different ways:\n\n   1. in-process (using the interface in conformance_test.h).\n\n   2. as a sub-process communicating over a pipe.  Information about how to\n      do this is in conformance_test_runner.cc.\n\n Pros/cons of the two approaches:\n\n   - running as a sub-process is much simpler for languages other than C/C++.\n\n   - running as a sub-process may be more tricky in unusual environments like\n     iOS apps, where fork/stdin/stdout are not available.\n\n\n\n\x03\x05\0\x01\x12\x035\x05\x0F\n\x0B\n\x04\x05\0\x02\0\x12\x036\x02\x12\n\x0C\n\x05\x05\0\x02\0\x01\x12\x036\x02\r\n\x0C\n\x05\x05\0\x02\0\x02\x12\x036\x10\x11\n\x0B\n\x04\x05\0\x02\x01\x12\x037\x02\x0F\n\x0C\n\x05\x05\0\x02\x01\x01\x12\x037\x02\n\n\x0C\n\x05\x05\0\x02\x01\x02\x12\x037\r\x0E\n\x0B\n\x04\x05\0\x02\x02\x12\x038\x02\x0B\n\x0C\n\x05\x05\0\x02\x02\x01\x12\x038\x02\x06\n\x0C\n\x05\x05\0\x02\x02\x02\x12\x038\t\n\nE\n\x04\x05\0\x02\x03\x12\x039\x02\x0B\"8 Google internal only. Opensource testees just skip it.\n\n\x0C\n\x05\x05\0\x02\x03\x01\x12\x039\x02\x06\n\x0C\n\x05\x05\0\x02\x03\x02\x12\x039\t\n\n\x0B\n\x04\x05\0\x02\x04\x12\x03:\x02\x12\n\x0C\n\x05\x05\0\x02\x04\x01\x12\x03:\x02\r\n\x0C\n\x05\x05\0\x02\x04\x02\x12\x03:\x10\x11\n\n\n\x02\x05\x01\x12\x04=\0L\x01\n\n\n\x03\x05\x01\x01\x12\x03=\x05\x11\n\x0B\n\x04\x05\x01\x02\0\x12\x03>\x02\x17\n\x0C\n\x05\x05\x01\x02\0\x01\x12\x03>\x02\x12\n\x0C\n\x05\x05\x01\x02\0\x02\x12\x03>\x15\x16\n'\n\x04\x05\x01\x02\x01\x12\x03?\x02\x12\"\x1A Test binary wire format.\n\n\x0C\n\x05\x05\x01\x02\x01\x01\x12\x03?\x02\r\n\x0C\n\x05\x05\x01\x02\x01\x02\x12\x03?\x10\x11\n%\n\x04\x05\x01\x02\x02\x12\x03@\x02\x10\"\x18 Test json wire format.\n\n\x0C\n\x05\x05\x01\x02\x02\x01\x12\x03@\x02\x0B\n\x0C\n\x05\x05\x01\x02\x02\x02\x12\x03@\x0E\x0F\n\x9A\x02\n\x04\x05\x01\x02\x03\x12\x03F\x02'\x1A\x8C\x02 Similar to JSON_TEST. However, during parsing json, testee should ignore\n unknown fields. This feature is optional. Each implementation can decide\n whether to support it.  See\n https://developers.google.com/protocol-buffers/docs/proto3#json_options\n for more detail.\n\n\x0C\n\x05\x05\x01\x02\x03\x01\x12\x03F\x02\"\n\x0C\n\x05\x05\x01\x02\x03\x02\x12\x03F%&\n\\\n\x04\x05\x01\x02\x04\x12\x03H\x02\x10\x1AO Test jspb wire format. Google internal only. Opensource testees just skip it.\n\n\x0C\n\x05\x05\x01\x02\x04\x01\x12\x03H\x02\x0B\n\x0C\n\x05\x05\x01\x02\x04\x02\x12\x03H\x0E\x0F\n\x94\x01\n\x04\x05\x01\x02\x05\x12\x03K\x02\x17\x1A\x86\x01 Test text format. For cpp, java and python, testees can already deal with\n this type. Testees of other languages can simply skip it.\n\n\x0C\n\x05\x05\x01\x02\x05\x01\x12\x03K\x02\x12\n\x0C\n\x05\x05\x01\x02\x05\x02\x12\x03K\x15\x16\n\xEB\x01\n\x02\x04\0\x12\x04Q\0S\x01\x1A\xDE\x01 The conformance runner will request a list of failures as the first request.\n This will be known by message_type == \"conformance.FailureSet\", a conformance\n test should return a serialized FailureSet in protobuf_payload.\n\n\n\n\x03\x04\0\x01\x12\x03Q\x08\x12\n\x0B\n\x04\x04\0\x02\0\x12\x03R\x02\x1E\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03R\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03R\x0B\x11\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03R\x12\x19\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03R\x1C\x1D\n\x93\x02\n\x02\x04\x01\x12\x04Z\0}\x01\x1A\x86\x02 Represents a single test case's input.  The testee should:\n\n   1. parse this proto (which should always succeed)\n   2. parse the protobuf or JSON payload in \"payload\" (which may fail)\n   3. if the parse succeeded, serialize the message in the requested format.\n\n\n\n\x03\x04\x01\x01\x12\x03Z\x08\x1A\n\x89\x03\n\x04\x04\x01\x08\0\x12\x04b\x02h\x03\x1A\xFA\x02 The payload (whether protobuf of JSON) is always for a\n protobuf_test_messages.proto3.TestAllTypes proto (as defined in\n src/google/protobuf/proto3_test_messages.proto).\n\n TODO(haberman): if/when we expand the conformance tests to support proto2,\n we will want to include a field that lets the payload/response be a\n protobuf_test_messages.proto2.TestAllTypes message instead.\n\n\x0C\n\x05\x04\x01\x08\0\x01\x12\x03b\x08\x0F\n\x0B\n\x04\x04\x01\x02\0\x12\x03c\x04\x1F\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03c\x04\t\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03c\n\x1A\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03c\x1D\x1E\n\x0B\n\x04\x04\x01\x02\x01\x12\x03d\x04\x1C\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03d\x04\n\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03d\x0B\x17\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03d\x1A\x1B\nF\n\x04\x04\x01\x02\x02\x12\x03f\x04\x1C\x1A9 Google internal only.  Opensource testees just skip it.\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03f\x04\n\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03f\x0B\x17\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03f\x1A\x1B\n\x0B\n\x04\x04\x01\x02\x03\x12\x03g\x04\x1C\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03g\x04\n\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03g\x0B\x17\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03g\x1A\x1B\nG\n\x04\x04\x01\x02\x04\x12\x03k\x02)\x1A: Which format should the testee serialize its message to?\n\n\x0C\n\x05\x04\x01\x02\x04\x06\x12\x03k\x02\x0C\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03k\r$\n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03k'(\n\xBA\x01\n\x04\x04\x01\x02\x05\x12\x03p\x02\x1A\x1A\xAC\x01 The full name for the test message to use; for the moment, either:\n protobuf_test_messages.proto3.TestAllTypesProto3 or\n protobuf_test_messages.proto2.TestAllTypesProto2.\n\n\x0C\n\x05\x04\x01\x02\x05\x05\x12\x03p\x02\x08\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03p\t\x15\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03p\x18\x19\n\xB8\x01\n\x04\x04\x01\x02\x06\x12\x03u\x02!\x1A\xAA\x01 Each test is given a specific test category. Some category may need\n specific support in testee programs. Refer to the definition of TestCategory\n for more information.\n\n\x0C\n\x05\x04\x01\x02\x06\x06\x12\x03u\x02\x0E\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03u\x0F\x1C\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03u\x1F \n6\n\x04\x04\x01\x02\x07\x12\x03x\x02/\x1A) Specify details for how to encode jspb.\n\n\x0C\n\x05\x04\x01\x02\x07\x06\x12\x03x\x02\x14\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03x\x15*\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03x-.\n\x93\x01\n\x04\x04\x01\x02\x08\x12\x03|\x02 \x1A\x85\x01 This can be used in json and text format. If true, testee should print\n unknown fields instead of ignore. This feature is optional.\n\n\x0C\n\x05\x04\x01\x02\x08\x05\x12\x03|\x02\x06\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03|\x07\x1B\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03|\x1E\x1F\n7\n\x02\x04\x02\x12\x06\x80\x01\0\xA8\x01\x01\x1A) Represents a single test case's output.\n\n\x0B\n\x03\x04\x02\x01\x12\x04\x80\x01\x08\x1B\n\x0E\n\x04\x04\x02\x08\0\x12\x06\x81\x01\x02\xA7\x01\x03\n\r\n\x05\x04\x02\x08\0\x01\x12\x04\x81\x01\x08\x0E\n\xA0\x02\n\x04\x04\x02\x02\0\x12\x04\x87\x01\x04\x1B\x1A\x91\x02 This string should be set to indicate parsing failed.  The string can\n provide more information about the parse error if it is available.\n\n Setting this string does not necessarily mean the testee failed the\n test.  Some of the test cases are intentionally invalid input.\n\n\r\n\x05\x04\x02\x02\0\x05\x12\x04\x87\x01\x04\n\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\x87\x01\x0B\x16\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x87\x01\x19\x1A\n\xA4\x01\n\x04\x04\x02\x02\x01\x12\x04\x8C\x01\x04\x1F\x1A\x95\x01 If the input was successfully parsed but errors occurred when\n serializing it to the requested output format, set the error message in\n this field.\n\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\x8C\x01\x04\n\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\x8C\x01\x0B\x1A\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x8C\x01\x1D\x1E\n\xB0\x01\n\x04\x04\x02\x02\x02\x12\x04\x91\x01\x04\x1D\x1A\xA1\x01 This should be set if some other error occurred.  This will always\n indicate that the test failed.  The string can provide more information\n about the failure.\n\n\r\n\x05\x04\x02\x02\x02\x05\x12\x04\x91\x01\x04\n\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\x91\x01\x0B\x18\n\r\n\x05\x04\x02\x02\x02\x03\x12\x04\x91\x01\x1B\x1C\n\x90\x01\n\x04\x04\x02\x02\x03\x12\x04\x95\x01\x04\x1F\x1A\x81\x01 If the input was successfully parsed and the requested output was\n protobuf, serialize it to protobuf and set it in this field.\n\n\r\n\x05\x04\x02\x02\x03\x05\x12\x04\x95\x01\x04\t\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\x95\x01\n\x1A\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\x95\x01\x1D\x1E\n\x84\x01\n\x04\x04\x02\x02\x04\x12\x04\x99\x01\x04\x1C\x1Av If the input was successfully parsed and the requested output was JSON,\n serialize to JSON and set it in this field.\n\n\r\n\x05\x04\x02\x02\x04\x05\x12\x04\x99\x01\x04\n\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\x99\x01\x0B\x17\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\x99\x01\x1A\x1B\n\x81\x01\n\x04\x04\x02\x02\x05\x12\x04\x9D\x01\x04\x17\x1As For when the testee skipped the test, likely because a certain feature\n wasn't supported, like JSON input/output.\n\n\r\n\x05\x04\x02\x02\x05\x05\x12\x04\x9D\x01\x04\n\n\r\n\x05\x04\x02\x02\x05\x01\x12\x04\x9D\x01\x0B\x12\n\r\n\x05\x04\x02\x02\x05\x03\x12\x04\x9D\x01\x15\x16\n\xD0\x01\n\x04\x04\x02\x02\x06\x12\x04\xA2\x01\x04\x1C\x1A\xC1\x01 If the input was successfully parsed and the requested output was JSPB,\n serialize to JSPB and set it in this field. JSPB is google internal only\n format. Opensource testees can just skip it.\n\n\r\n\x05\x04\x02\x02\x06\x05\x12\x04\xA2\x01\x04\n\n\r\n\x05\x04\x02\x02\x06\x01\x12\x04\xA2\x01\x0B\x17\n\r\n\x05\x04\x02\x02\x06\x03\x12\x04\xA2\x01\x1A\x1B\n\x93\x01\n\x04\x04\x02\x02\x07\x12\x04\xA6\x01\x04\x1C\x1A\x84\x01 If the input was successfully parsed and the requested output was\n TEXT_FORMAT, serialize to TEXT_FORMAT and set it in this field.\n\n\r\n\x05\x04\x02\x02\x07\x05\x12\x04\xA6\x01\x04\n\n\r\n\x05\x04\x02\x02\x07\x01\x12\x04\xA6\x01\x0B\x17\n\r\n\x05\x04\x02\x02\x07\x03\x12\x04\xA6\x01\x1A\x1B\n1\n\x02\x04\x03\x12\x06\xAB\x01\0\xAE\x01\x01\x1A# Encoding options for jspb format.\n\n\x0B\n\x03\x04\x03\x01\x12\x04\xAB\x01\x08\x1A\nV\n\x04\x04\x03\x02\0\x12\x04\xAD\x01\x02%\x1AH Encode the value field of Any as jspb array if true, otherwise binary.\n\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\xAD\x01\x02\x06\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\xAD\x01\x07 \n\r\n\x05\x04\x03\x02\0\x03\x12\x04\xAD\x01#$b\x06proto3" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
