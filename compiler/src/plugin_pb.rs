#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[allow(unused_imports)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
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
        self.major.get_or_insert_with(Default::default)
    }
    pub fn set_major(&mut self, val: i32) {
        self.major = Some(val);
    }
    pub fn minor(&self) -> i32 {
        self.minor.unwrap_or_default()
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        self.minor.get_or_insert_with(Default::default)
    }
    pub fn set_minor(&mut self, val: i32) {
        self.minor = Some(val);
    }
    pub fn patch(&self) -> i32 {
        self.patch.unwrap_or_default()
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        self.patch.get_or_insert_with(Default::default)
    }
    pub fn set_patch(&mut self, val: i32) {
        self.patch = Some(val);
    }
    pub fn suffix(&self) -> &String {
        match &self.suffix {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn suffix_mut(&mut self) -> &mut String {
        self.suffix.get_or_insert_with(Default::default)
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
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.major {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.minor {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = self.patch {
            l += 1 + Varint::size(v);
        }
        if let Some(v) = &self.suffix {
            l += 1 + LengthPrefixed::size(v);
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
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: Vec<String>,
    pub parameter: Option<String>,
    pub proto_file: Vec<pecan::reflection::FileDescriptorProto>,
    pub compiler_version: Option<Version>,
    _unknown: Vec<u8>,
}
impl CodeGeneratorRequest {
    pub const fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest {
            file_to_generate: Vec::new(),
            parameter: None,
            proto_file: Vec::new(),
            compiler_version: None,
            _unknown: Vec::new(),
        }
    }
    pub fn parameter(&self) -> &String {
        match &self.parameter {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn parameter_mut(&mut self) -> &mut String {
        self.parameter.get_or_insert_with(Default::default)
    }
    pub fn set_parameter(&mut self, val: String) {
        self.parameter = Some(val);
    }
    pub fn compiler_version(&self) -> &Version {
        match &self.compiler_version {
            Some(v) => v,
            _ => Version::default_instance(),
        }
    }
    pub fn compiler_version_mut(&mut self) -> &mut Version {
        self.compiler_version.get_or_insert_with(Default::default)
    }
    pub fn set_compiler_version(&mut self, val: Version) {
        self.compiler_version = Some(val);
    }
}
impl pecan::Message for CodeGeneratorRequest {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => RefArray::<LengthPrefixed>::merge_from(&mut self.file_to_generate, s)?,
                18 => self.parameter = Some(LengthPrefixed::read_from(s)?),
                26 => LengthPrefixed::merge_from(self.compiler_version_mut(), s)?,
                122 => RefArray::<LengthPrefixed>::merge_from(&mut self.proto_file, s)?,
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
    fn size(&self) -> u64 {
        let mut l = 0;
        if !self.file_to_generate.is_empty() {
            l += self.file_to_generate.len() as u64
                + RefArray::<LengthPrefixed>::size(&self.file_to_generate);
        }
        if let Some(v) = &self.parameter {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.compiler_version {
            l += 1 + LengthPrefixed::size(v);
        }
        if !self.proto_file.is_empty() {
            l += self.proto_file.len() as u64 + RefArray::<LengthPrefixed>::size(&self.proto_file);
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
impl std::fmt::Debug for CodeGeneratorResponse_Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "FEATURE_NONE"),
            1 => write!(f, "FEATURE_PROTO3_OPTIONAL"),
            v => write!(f, "CodeGeneratorResponse_Feature({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorResponse_File {
    pub name: Option<String>,
    pub insertion_point: Option<String>,
    pub content: Option<String>,
    pub generated_code_info: Option<pecan::reflection::GeneratedCodeInfo>,
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn insertion_point(&self) -> &String {
        match &self.insertion_point {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn insertion_point_mut(&mut self) -> &mut String {
        self.insertion_point.get_or_insert_with(Default::default)
    }
    pub fn set_insertion_point(&mut self, val: String) {
        self.insertion_point = Some(val);
    }
    pub fn content(&self) -> &String {
        match &self.content {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn content_mut(&mut self) -> &mut String {
        self.content.get_or_insert_with(Default::default)
    }
    pub fn set_content(&mut self, val: String) {
        self.content = Some(val);
    }
    pub fn generated_code_info(&self) -> &pecan::reflection::GeneratedCodeInfo {
        match &self.generated_code_info {
            Some(v) => v,
            _ => pecan::reflection::GeneratedCodeInfo::default_instance(),
        }
    }
    pub fn generated_code_info_mut(&mut self) -> &mut pecan::reflection::GeneratedCodeInfo {
        self.generated_code_info
            .get_or_insert_with(Default::default)
    }
    pub fn set_generated_code_info(&mut self, val: pecan::reflection::GeneratedCodeInfo) {
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
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.name {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.insertion_point {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.content {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = &self.generated_code_info {
            l += 2 + LengthPrefixed::size(v);
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
#[derive(Clone, Debug, PartialEq)]
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
            _ => String::default_instance(),
        }
    }
    pub fn error_mut(&mut self) -> &mut String {
        self.error.get_or_insert_with(Default::default)
    }
    pub fn set_error(&mut self, val: String) {
        self.error = Some(val);
    }
    pub fn supported_features(&self) -> u64 {
        self.supported_features.unwrap_or_default()
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        self.supported_features.get_or_insert_with(Default::default)
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
                122 => RefArray::<LengthPrefixed>::merge_from(&mut self.file, s)?,
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
    fn size(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.error {
            l += 1 + LengthPrefixed::size(v);
        }
        if let Some(v) = self.supported_features {
            l += 1 + Varint::size(v);
        }
        if !self.file.is_empty() {
            l += self.file.len() as u64 + RefArray::<LengthPrefixed>::size(&self.file);
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
static DESCRIPTOR_RAW : & [u8] = b"\n%google/protobuf/compiler/plugin.proto\x12\x18google.protobuf.compiler\x1A google/protobuf/descriptor.proto\"c\n\x07Version\x12\x14\n\x05major\x18\x01 \x01(\x05R\x05major\x12\x14\n\x05minor\x18\x02 \x01(\x05R\x05minor\x12\x14\n\x05patch\x18\x03 \x01(\x05R\x05patch\x12\x16\n\x06suffix\x18\x04 \x01(\tR\x06suffix\"\xF1\x01\n\x14CodeGeneratorRequest\x12(\n\x10file_to_generate\x18\x01 \x03(\tR\x0EfileToGenerate\x12\x1C\n\tparameter\x18\x02 \x01(\tR\tparameter\x12C\n\nproto_file\x18\x0F \x03(\x0B2$.google.protobuf.FileDescriptorProtoR\tprotoFile\x12L\n\x10compiler_version\x18\x03 \x01(\x0B2!.google.protobuf.compiler.VersionR\x0FcompilerVersion\"\x94\x03\n\x15CodeGeneratorResponse\x12\x14\n\x05error\x18\x01 \x01(\tR\x05error\x12-\n\x12supported_features\x18\x02 \x01(\x04R\x11supportedFeatures\x12H\n\x04file\x18\x0F \x03(\x0B24.google.protobuf.compiler.CodeGeneratorResponse.FileR\x04file\x1A\xB1\x01\n\x04File\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12'\n\x0Finsertion_point\x18\x02 \x01(\tR\x0EinsertionPoint\x12\x18\n\x07content\x18\x0F \x01(\tR\x07content\x12R\n\x13generated_code_info\x18\x10 \x01(\x0B2\".google.protobuf.GeneratedCodeInfoR\x11generatedCodeInfo\"8\n\x07Feature\x12\x10\n\x0CFEATURE_NONE\x10\0\x12\x1B\n\x17FEATURE_PROTO3_OPTIONAL\x10\x01BW\n\x1Ccom.google.protobuf.compilerB\x0CPluginProtosZ)google.golang.org/protobuf/types/pluginpbJ\xF9C\n\x07\x12\x05.\0\xB6\x01\x01\n\xCA\x11\n\x01\x0C\x12\x03.\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2\xFB\x04 Author: kenton@google.com (Kenton Varda)\n\n WARNING:  The plugin interface is currently EXPERIMENTAL and is subject to\n   change.\n\n protoc (aka the Protocol Compiler) can be extended via plugins.  A plugin is\n just a program that reads a CodeGeneratorRequest from stdin and writes a\n CodeGeneratorResponse to stdout.\n\n Plugins written using C++ can use google/protobuf/compiler/plugin.h instead\n of dealing with the raw protocol defined here.\n\n A plugin executable needs only to be placed somewhere in the path.  The\n plugin should be named \"protoc-gen-$NAME\", and will then be used when the\n flag \"--${NAME}_out\" is passed to protoc.\n\n\x08\n\x01\x02\x12\x030\0!\n\x08\n\x01\x08\x12\x031\05\n\t\n\x02\x08\x01\x12\x031\05\n\x08\n\x01\x08\x12\x032\0-\n\t\n\x02\x08\x08\x12\x032\0-\n\x08\n\x01\x08\x12\x034\0@\n\t\n\x02\x08\x0B\x12\x034\0@\n\t\n\x02\x03\0\x12\x036\0*\n6\n\x02\x04\0\x12\x049\0@\x01\x1A* The version number of protocol compiler.\n\n\n\n\x03\x04\0\x01\x12\x039\x08\x0F\n\x0B\n\x04\x04\0\x02\0\x12\x03:\x02\x1B\n\x0C\n\x05\x04\0\x02\0\x04\x12\x03:\x02\n\n\x0C\n\x05\x04\0\x02\0\x05\x12\x03:\x0B\x10\n\x0C\n\x05\x04\0\x02\0\x01\x12\x03:\x11\x16\n\x0C\n\x05\x04\0\x02\0\x03\x12\x03:\x19\x1A\n\x0B\n\x04\x04\0\x02\x01\x12\x03;\x02\x1B\n\x0C\n\x05\x04\0\x02\x01\x04\x12\x03;\x02\n\n\x0C\n\x05\x04\0\x02\x01\x05\x12\x03;\x0B\x10\n\x0C\n\x05\x04\0\x02\x01\x01\x12\x03;\x11\x16\n\x0C\n\x05\x04\0\x02\x01\x03\x12\x03;\x19\x1A\n\x0B\n\x04\x04\0\x02\x02\x12\x03<\x02\x1B\n\x0C\n\x05\x04\0\x02\x02\x04\x12\x03<\x02\n\n\x0C\n\x05\x04\0\x02\x02\x05\x12\x03<\x0B\x10\n\x0C\n\x05\x04\0\x02\x02\x01\x12\x03<\x11\x16\n\x0C\n\x05\x04\0\x02\x02\x03\x12\x03<\x19\x1A\n\x80\x01\n\x04\x04\0\x02\x03\x12\x03?\x02\x1D\x1As A suffix for alpha, beta or rc release, e.g., \"alpha-1\", \"rc2\". It should\n be empty for mainline stable releases.\n\n\x0C\n\x05\x04\0\x02\x03\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\0\x02\x03\x05\x12\x03?\x0B\x11\n\x0C\n\x05\x04\0\x02\x03\x01\x12\x03?\x12\x18\n\x0C\n\x05\x04\0\x02\x03\x03\x12\x03?\x1B\x1C\nO\n\x02\x04\x01\x12\x04C\0_\x01\x1AC An encoded CodeGeneratorRequest is written to the plugin's stdin.\n\n\n\n\x03\x04\x01\x01\x12\x03C\x08\x1C\n\xD1\x01\n\x04\x04\x01\x02\0\x12\x03G\x02'\x1A\xC3\x01 The .proto files that were explicitly listed on the command-line.  The\n code generator should generate code only for these files.  Each file's\n descriptor will be included in proto_file, below.\n\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x03G\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03G\x0B\x11\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03G\x12\"\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03G%&\nB\n\x04\x04\x01\x02\x01\x12\x03J\x02 \x1A5 The generator parameter passed on the command-line.\n\n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x03J\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03J\x0B\x11\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03J\x12\x1B\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03J\x1E\x1F\n\x87\x06\n\x04\x04\x01\x02\x02\x12\x03Z\x02/\x1A\xF9\x05 FileDescriptorProtos for all files in files_to_generate and everything\n they import.  The files will appear in topological order, so each file\n appears before any file that imports it.\n\n protoc guarantees that all proto_files will be written after\n the fields above, even though this is not technically guaranteed by the\n protobuf wire format.  This theoretically could allow a plugin to stream\n in the FileDescriptorProtos and handle them one by one rather than read\n the entire set into memory at once.  However, as of this writing, this\n is not similarly optimized on protoc's end -- it will store all fields in\n memory at once before sending them to the plugin.\n\n Type names of fields and extensions in the FileDescriptorProto are always\n fully qualified.\n\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x03Z\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x06\x12\x03Z\x0B\x1E\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03Z\x1F)\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03Z,.\n7\n\x04\x04\x01\x02\x03\x12\x03]\x02(\x1A* The version number of protocol compiler.\n\n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x03]\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x06\x12\x03]\x0B\x12\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03]\x13#\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03]&'\nL\n\x02\x04\x02\x12\x05b\0\xB6\x01\x01\x1A? The plugin writes an encoded CodeGeneratorResponse to stdout.\n\n\n\n\x03\x04\x02\x01\x12\x03b\x08\x1D\n\xED\x03\n\x04\x04\x02\x02\0\x12\x03k\x02\x1C\x1A\xDF\x03 Error message.  If non-empty, code generation failed.  The plugin process\n should exit with status code zero even if it reports an error in this way.\n\n This should be used to indicate errors in .proto files which prevent the\n code generator from generating correct code.  Errors which indicate a\n problem in protoc itself -- such as the input CodeGeneratorRequest being\n unparseable -- should be reported by writing a message to stderr and\n exiting with a non-zero status code.\n\n\x0C\n\x05\x04\x02\x02\0\x04\x12\x03k\x02\n\n\x0C\n\x05\x04\x02\x02\0\x05\x12\x03k\x0B\x11\n\x0C\n\x05\x04\x02\x02\0\x01\x12\x03k\x12\x17\n\x0C\n\x05\x04\x02\x02\0\x03\x12\x03k\x1A\x1B\n\x89\x01\n\x04\x04\x02\x02\x01\x12\x03o\x02)\x1A| A bitmask of supported features that the code generator supports.\n This is a bitwise \"or\" of values from the Feature enum.\n\n\x0C\n\x05\x04\x02\x02\x01\x04\x12\x03o\x02\n\n\x0C\n\x05\x04\x02\x02\x01\x05\x12\x03o\x0B\x11\n\x0C\n\x05\x04\x02\x02\x01\x01\x12\x03o\x12$\n\x0C\n\x05\x04\x02\x02\x01\x03\x12\x03o'(\n+\n\x04\x04\x02\x04\0\x12\x04r\x02u\x03\x1A\x1D Sync with code_generator.h.\n\n\x0C\n\x05\x04\x02\x04\0\x01\x12\x03r\x07\x0E\n\r\n\x06\x04\x02\x04\0\x02\0\x12\x03s\x04\x15\n\x0E\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03s\x04\x10\n\x0E\n\x07\x04\x02\x04\0\x02\0\x02\x12\x03s\x13\x14\n\r\n\x06\x04\x02\x04\0\x02\x01\x12\x03t\x04 \n\x0E\n\x07\x04\x02\x04\0\x02\x01\x01\x12\x03t\x04\x1B\n\x0E\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03t\x1E\x1F\n4\n\x04\x04\x02\x03\0\x12\x05x\x02\xB4\x01\x03\x1A% Represents a single generated file.\n\n\x0C\n\x05\x04\x02\x03\0\x01\x12\x03x\n\x0E\n\xAE\x05\n\x06\x04\x02\x03\0\x02\0\x12\x04\x84\x01\x04\x1D\x1A\x9D\x05 The file name, relative to the output directory.  The name must not\n contain \".\" or \"..\" components and must be relative, not be absolute (so,\n the file cannot lie outside the output directory).  \"/\" must be used as\n the path separator, not \"\\\".\n\n If the name is omitted, the content will be appended to the previous\n file.  This allows the generator to break large files into small chunks,\n and allows the generated text to be streamed back to protoc so that large\n files need not reside completely in memory at one time.  Note that as of\n this writing protoc does not optimize for this -- it will read the entire\n CodeGeneratorResponse before writing files to disk.\n\n\x0F\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04\x84\x01\x04\x0C\n\x0F\n\x07\x04\x02\x03\0\x02\0\x05\x12\x04\x84\x01\r\x13\n\x0F\n\x07\x04\x02\x03\0\x02\0\x01\x12\x04\x84\x01\x14\x18\n\x0F\n\x07\x04\x02\x03\0\x02\0\x03\x12\x04\x84\x01\x1B\x1C\n\xAE\x10\n\x06\x04\x02\x03\0\x02\x01\x12\x04\xAB\x01\x04(\x1A\x9D\x10 If non-empty, indicates that the named file should already exist, and the\n content here is to be inserted into that file at a defined insertion\n point.  This feature allows a code generator to extend the output\n produced by another code generator.  The original generator may provide\n insertion points by placing special annotations in the file that look\n like:\n   @@protoc_insertion_point(NAME)\n The annotation can have arbitrary text before and after it on the line,\n which allows it to be placed in a comment.  NAME should be replaced with\n an identifier naming the point -- this is what other generators will use\n as the insertion_point.  Code inserted at this point will be placed\n immediately above the line containing the insertion point (thus multiple\n insertions to the same point will come out in the order they were added).\n The double-@ is intended to make it unlikely that the generated code\n could contain things that look like insertion points by accident.\n\n For example, the C++ code generator places the following line in the\n .pb.h files that it generates:\n   // @@protoc_insertion_point(namespace_scope)\n This line appears within the scope of the file's package namespace, but\n outside of any particular class.  Another plugin can then specify the\n insertion_point \"namespace_scope\" to generate additional classes or\n other declarations that should be placed in this scope.\n\n Note that if the line containing the insertion point begins with\n whitespace, the same whitespace will be added to every line of the\n inserted text.  This is useful for languages like Python, where\n indentation matters.  In these languages, the insertion point comment\n should be indented the same amount as any inserted code will need to be\n in order to work correctly in that context.\n\n The code generator that generates the initial file and the one which\n inserts into it must both run as part of a single invocation of protoc.\n Code generators are executed in the order in which they appear on the\n command line.\n\n If |insertion_point| is present, |name| must also be present.\n\n\x0F\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x04\xAB\x01\x04\x0C\n\x0F\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x04\xAB\x01\r\x13\n\x0F\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x04\xAB\x01\x14#\n\x0F\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x04\xAB\x01&'\n$\n\x06\x04\x02\x03\0\x02\x02\x12\x04\xAE\x01\x04!\x1A\x14 The file contents.\n\n\x0F\n\x07\x04\x02\x03\0\x02\x02\x04\x12\x04\xAE\x01\x04\x0C\n\x0F\n\x07\x04\x02\x03\0\x02\x02\x05\x12\x04\xAE\x01\r\x13\n\x0F\n\x07\x04\x02\x03\0\x02\x02\x01\x12\x04\xAE\x01\x14\x1B\n\x0F\n\x07\x04\x02\x03\0\x02\x02\x03\x12\x04\xAE\x01\x1E \n\xE1\x01\n\x06\x04\x02\x03\0\x02\x03\x12\x04\xB3\x01\x048\x1A\xD0\x01 Information describing the file content being inserted. If an insertion\n point is used, this information will be appropriately offset and inserted\n into the code generation metadata for the generated files.\n\n\x0F\n\x07\x04\x02\x03\0\x02\x03\x04\x12\x04\xB3\x01\x04\x0C\n\x0F\n\x07\x04\x02\x03\0\x02\x03\x06\x12\x04\xB3\x01\r\x1E\n\x0F\n\x07\x04\x02\x03\0\x02\x03\x01\x12\x04\xB3\x01\x1F2\n\x0F\n\x07\x04\x02\x03\0\x02\x03\x03\x12\x04\xB3\x0157\n\x0C\n\x04\x04\x02\x02\x02\x12\x04\xB5\x01\x02\x1A\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\xB5\x01\x02\n\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\xB5\x01\x0B\x0F\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\xB5\x01\x10\x14\n\r\n\x05\x04\x02\x02\x02\x03\x12\x04\xB5\x01\x17\x19" ;
pub static DESCRIPTOR: pecan::Bytes = pecan::Bytes::from_static(DESCRIPTOR_RAW);
