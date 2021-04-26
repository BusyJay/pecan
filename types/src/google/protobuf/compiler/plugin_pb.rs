#![allow(non_camel_case_types)]
use pecan::prelude::*;
#[derive(Clone, Debug)]
pub struct Version {
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
    pub suffix: Option<String>,
    _unknown: Vec<u8>,
}
impl Version {
    pub const fn new() -> Version {
        Version {
            major: None,
            minor: None,
            patch: None,
            suffix: None,
            _unknown: Vec::new(),
        }
    }
    pub fn major(&self) -> i32 {
        self.major.unwrap_or_default()
    }
    pub fn major_mut(&mut self) -> &mut i32 {
        if self.major.is_none() {
            self.major = Some(0);
        }
        self.major.as_mut().unwrap()
    }
    pub fn set_major(&mut self, val: i32) {
        self.major = Some(val);
    }
    pub fn minor(&self) -> i32 {
        self.minor.unwrap_or_default()
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        if self.minor.is_none() {
            self.minor = Some(0);
        }
        self.minor.as_mut().unwrap()
    }
    pub fn set_minor(&mut self, val: i32) {
        self.minor = Some(val);
    }
    pub fn patch(&self) -> i32 {
        self.patch.unwrap_or_default()
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        if self.patch.is_none() {
            self.patch = Some(0);
        }
        self.patch.as_mut().unwrap()
    }
    pub fn set_patch(&mut self, val: i32) {
        self.patch = Some(val);
    }
    pub fn suffix(&self) -> &String {
        match &self.suffix {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn suffix_mut(&mut self) -> &mut String {
        if self.suffix.is_none() {
            self.suffix = Some(String::new());
        }
        self.suffix.as_mut().unwrap()
    }
    pub fn set_suffix(&mut self, val: String) {
        self.suffix = Some(val);
    }
}
impl pecan::Message for Version {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.major = Some(Varint::read_from(s)?),
                16 => self.minor = Some(Varint::read_from(s)?),
                24 => self.patch = Some(Varint::read_from(s)?),
                34 => self.suffix = Some(LengthPrefixed::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.major {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.minor {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.patch {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.suffix {
            s.write_tag(34)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.major {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.minor {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.patch {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.suffix {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for Version {
    fn default_instance() -> &'static Version {
        static DEFAULT: Version = Version::new();
        &DEFAULT
    }
}
impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::new()
    }
}
#[derive(Clone, Debug)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: Vec<String>,
    pub parameter: Option<String>,
    pub proto_file: Vec<crate::google::protobuf::descriptor_pb::FileDescriptorProto>,
    pub compiler_version: Option<Version>,
    _unknown: Vec<u8>,
}
impl CodeGeneratorRequest {
    pub const fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest {
            file_to_generate: Vec::new(),
            parameter: None,
            compiler_version: None,
            proto_file: Vec::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn parameter(&self) -> &String {
        match &self.parameter {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn parameter_mut(&mut self) -> &mut String {
        if self.parameter.is_none() {
            self.parameter = Some(String::new());
        }
        self.parameter.as_mut().unwrap()
    }
    pub fn set_parameter(&mut self, val: String) {
        self.parameter = Some(val);
    }
    pub fn compiler_version(&self) -> &Version {
        match &self.compiler_version {
            Some(v) => v,
            None => Version::default_instance(),
        }
    }
    pub fn compiler_version_mut(&mut self) -> &mut Version {
        if self.compiler_version.is_none() {
            self.compiler_version = Some(Version::new());
        }
        self.compiler_version.as_mut().unwrap()
    }
    pub fn set_compiler_version(&mut self, val: Version) {
        self.compiler_version = Some(val);
    }
}
impl pecan::Message for CodeGeneratorRequest {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.file_to_generate, s)?,
                18 => self.parameter = Some(LengthPrefixed::read_from(s)?),
                26 => LengthPrefixed::merge_from(self.compiler_version_mut(), s)?,
                122 => LengthPrefixedArray::merge_from(&mut self.proto_file, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.file_to_generate.is_empty() {
            for i in &self.file_to_generate {
                s.write_tag(10)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.parameter {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.compiler_version {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.proto_file.is_empty() {
            for i in &self.proto_file {
                s.write_tag(122)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.file_to_generate.is_empty() {
            l += self.file_to_generate.len() as u64
                + LengthPrefixedArray::len(&self.file_to_generate);
        }
        if let Some(v) = &self.parameter {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.compiler_version {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.proto_file.is_empty() {
            l += self.proto_file.len() as u64 + LengthPrefixedArray::len(&self.proto_file);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for CodeGeneratorRequest {
    fn default_instance() -> &'static CodeGeneratorRequest {
        static DEFAULT: CodeGeneratorRequest = CodeGeneratorRequest::new();
        &DEFAULT
    }
}
impl Default for CodeGeneratorRequest {
    #[inline]
    fn default() -> CodeGeneratorRequest {
        CodeGeneratorRequest::new()
    }
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeGeneratorResponse_Feature(i32);
impl pecan::Enumerate for CodeGeneratorResponse_Feature {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> CodeGeneratorResponse_Feature {
        CodeGeneratorResponse_Feature(v)
    }
}
impl CodeGeneratorResponse_Feature {
    pub const FEATURE_NONE: CodeGeneratorResponse_Feature = CodeGeneratorResponse_Feature(0);
    pub const FEATURE_PROTO3_OPTIONAL: CodeGeneratorResponse_Feature =
        CodeGeneratorResponse_Feature(1);
    pub const fn new() -> CodeGeneratorResponse_Feature {
        CodeGeneratorResponse_Feature(0)
    }
}
impl std::fmt::Display for CodeGeneratorResponse_Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FEATURE_NONE"),
            1 => write!(f, "FEATURE_PROTO3_OPTIONAL"),
            v => write!(f, "CodeGeneratorResponse_Feature({})", v),
        }
    }
}
#[derive(Clone, Debug)]
pub struct CodeGeneratorResponse_File {
    pub name: Option<String>,
    pub insertion_point: Option<String>,
    pub content: Option<String>,
    pub generated_code_info: Option<crate::google::protobuf::descriptor_pb::GeneratedCodeInfo>,
    _unknown: Vec<u8>,
}
impl CodeGeneratorResponse_File {
    pub const fn new() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File {
            name: None,
            insertion_point: None,
            content: None,
            generated_code_info: None,
            _unknown: Vec::new(),
        }
    }
    pub fn name(&self) -> &String {
        match &self.name {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        if self.name.is_none() {
            self.name = Some(String::new());
        }
        self.name.as_mut().unwrap()
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn insertion_point(&self) -> &String {
        match &self.insertion_point {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn insertion_point_mut(&mut self) -> &mut String {
        if self.insertion_point.is_none() {
            self.insertion_point = Some(String::new());
        }
        self.insertion_point.as_mut().unwrap()
    }
    pub fn set_insertion_point(&mut self, val: String) {
        self.insertion_point = Some(val);
    }
    pub fn content(&self) -> &String {
        match &self.content {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn content_mut(&mut self) -> &mut String {
        if self.content.is_none() {
            self.content = Some(String::new());
        }
        self.content.as_mut().unwrap()
    }
    pub fn set_content(&mut self, val: String) {
        self.content = Some(val);
    }
    pub fn generated_code_info(
        &self,
    ) -> &crate::google::protobuf::descriptor_pb::GeneratedCodeInfo {
        match &self.generated_code_info {
            Some(v) => v,
            None => crate::google::protobuf::descriptor_pb::GeneratedCodeInfo::default_instance(),
        }
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut crate::google::protobuf::descriptor_pb::GeneratedCodeInfo {
        if self.generated_code_info.is_none() {
            self.generated_code_info =
                Some(crate::google::protobuf::descriptor_pb::GeneratedCodeInfo::new());
        }
        self.generated_code_info.as_mut().unwrap()
    }
    pub fn set_generated_code_info(
        &mut self,
        val: crate::google::protobuf::descriptor_pb::GeneratedCodeInfo,
    ) {
        self.generated_code_info = Some(val);
    }
}
impl pecan::Message for CodeGeneratorResponse_File {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => self.insertion_point = Some(LengthPrefixed::read_from(s)?),
                122 => self.content = Some(LengthPrefixed::read_from(s)?),
                130 => LengthPrefixed::merge_from(self.generated_code_info_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = &self.name {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.insertion_point {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.content {
            s.write_tag(122)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.generated_code_info {
            s.write_tag(130)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.name {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.insertion_point {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.content {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.generated_code_info {
            l += 2 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for CodeGeneratorResponse_File {
    fn default_instance() -> &'static CodeGeneratorResponse_File {
        static DEFAULT: CodeGeneratorResponse_File = CodeGeneratorResponse_File::new();
        &DEFAULT
    }
}
impl Default for CodeGeneratorResponse_File {
    #[inline]
    fn default() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File::new()
    }
}
#[derive(Clone, Debug)]
pub struct CodeGeneratorResponse {
    pub error: Option<String>,
    pub supported_features: Option<u64>,
    pub file: Vec<CodeGeneratorResponse_File>,
    _unknown: Vec<u8>,
}
impl CodeGeneratorResponse {
    pub const fn new() -> CodeGeneratorResponse {
        CodeGeneratorResponse {
            error: None,
            supported_features: None,
            file: Vec::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn error(&self) -> &String {
        match &self.error {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn error_mut(&mut self) -> &mut String {
        if self.error.is_none() {
            self.error = Some(String::new());
        }
        self.error.as_mut().unwrap()
    }
    pub fn set_error(&mut self, val: String) {
        self.error = Some(val);
    }
    pub fn supported_features(&self) -> u64 {
        self.supported_features.unwrap_or_default()
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        if self.supported_features.is_none() {
            self.supported_features = Some(0);
        }
        self.supported_features.as_mut().unwrap()
    }
    pub fn set_supported_features(&mut self, val: u64) {
        self.supported_features = Some(val);
    }
}
impl pecan::Message for CodeGeneratorResponse {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.error = Some(LengthPrefixed::read_from(s)?),
                16 => self.supported_features = Some(Varint::read_from(s)?),
                122 => LengthPrefixedArray::merge_from(&mut self.file, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = &self.error {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.supported_features {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if !self.file.is_empty() {
            for i in &self.file {
                s.write_tag(122)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.error {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.supported_features {
            l += 1 + Varint::len(v);
        }
        if !self.file.is_empty() {
            l += self.file.len() as u64 + LengthPrefixedArray::len(&self.file);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for CodeGeneratorResponse {
    fn default_instance() -> &'static CodeGeneratorResponse {
        static DEFAULT: CodeGeneratorResponse = CodeGeneratorResponse::new();
        &DEFAULT
    }
}
impl Default for CodeGeneratorResponse {
    #[inline]
    fn default() -> CodeGeneratorResponse {
        CodeGeneratorResponse::new()
    }
}
