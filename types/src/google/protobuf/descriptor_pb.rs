#![allow(non_camel_case_types)]
use pecan::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct FileDescriptorSet {
    pub file: Vec<FileDescriptorProto>,
    _unknown: Vec<u8>,
}
impl FileDescriptorSet {
    pub const fn new() -> FileDescriptorSet {
        FileDescriptorSet {
            file: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for FileDescriptorSet {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.file, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.file.is_empty() {
            for i in &self.file {
                s.write_tag(10)?;
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
        if !self.file.is_empty() {
            l += self.file.len() as u64 + LengthPrefixedArray::len(&self.file);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FileDescriptorSet {
    fn default_instance() -> &'static FileDescriptorSet {
        static DEFAULT: FileDescriptorSet = FileDescriptorSet::new();
        &DEFAULT
    }
}
impl Default for FileDescriptorSet {
    #[inline]
    fn default() -> FileDescriptorSet {
        FileDescriptorSet::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FileDescriptorProto {
    pub name: Option<String>,
    pub package: Option<String>,
    pub dependency: Vec<String>,
    pub public_dependency: Vec<i32>,
    pub weak_dependency: Vec<i32>,
    pub message_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub service: Vec<ServiceDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub options: Option<FileOptions>,
    pub source_code_info: Option<SourceCodeInfo>,
    pub syntax: Option<String>,
    _unknown: Vec<u8>,
}
impl FileDescriptorProto {
    pub const fn new() -> FileDescriptorProto {
        FileDescriptorProto {
            name: None,
            package: None,
            dependency: Vec::new(),
            message_type: Vec::new(),
            enum_type: Vec::new(),
            service: Vec::new(),
            extension: Vec::new(),
            options: None,
            source_code_info: None,
            public_dependency: Vec::new(),
            weak_dependency: Vec::new(),
            syntax: None,
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
    pub fn package(&self) -> &String {
        match &self.package {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn package_mut(&mut self) -> &mut String {
        if self.package.is_none() {
            self.package = Some(String::new());
        }
        self.package.as_mut().unwrap()
    }
    pub fn set_package(&mut self, val: String) {
        self.package = Some(val);
    }
    pub fn options(&self) -> &FileOptions {
        match &self.options {
            Some(v) => v,
            None => FileOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut FileOptions {
        if self.options.is_none() {
            self.options = Some(FileOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: FileOptions) {
        self.options = Some(val);
    }
    pub fn source_code_info(&self) -> &SourceCodeInfo {
        match &self.source_code_info {
            Some(v) => v,
            None => SourceCodeInfo::default_instance(),
        }
    }
    pub fn source_code_info_mut(&mut self) -> &mut SourceCodeInfo {
        if self.source_code_info.is_none() {
            self.source_code_info = Some(SourceCodeInfo::new());
        }
        self.source_code_info.as_mut().unwrap()
    }
    pub fn set_source_code_info(&mut self, val: SourceCodeInfo) {
        self.source_code_info = Some(val);
    }
    pub fn syntax(&self) -> &String {
        match &self.syntax {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn syntax_mut(&mut self) -> &mut String {
        if self.syntax.is_none() {
            self.syntax = Some(String::new());
        }
        self.syntax.as_mut().unwrap()
    }
    pub fn set_syntax(&mut self, val: String) {
        self.syntax = Some(val);
    }
}
impl pecan::Message for FileDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => self.package = Some(LengthPrefixed::read_from(s)?),
                26 => LengthPrefixedArray::merge_from(&mut self.dependency, s)?,
                34 => LengthPrefixedArray::merge_from(&mut self.message_type, s)?,
                42 => LengthPrefixedArray::merge_from(&mut self.enum_type, s)?,
                50 => LengthPrefixedArray::merge_from(&mut self.service, s)?,
                58 => LengthPrefixedArray::merge_from(&mut self.extension, s)?,
                66 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                74 => LengthPrefixed::merge_from(self.source_code_info_mut(), s)?,
                82 => VarintArray::merge_from(&mut self.public_dependency, s)?,
                90 => VarintArray::merge_from(&mut self.weak_dependency, s)?,
                98 => self.syntax = Some(LengthPrefixed::read_from(s)?),
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
        if let Some(v) = &self.package {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.dependency.is_empty() {
            for i in &self.dependency {
                s.write_tag(26)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.message_type.is_empty() {
            for i in &self.message_type {
                s.write_tag(34)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.enum_type.is_empty() {
            for i in &self.enum_type {
                s.write_tag(42)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.service.is_empty() {
            for i in &self.service {
                s.write_tag(50)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extension.is_empty() {
            for i in &self.extension {
                s.write_tag(58)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.options {
            s.write_tag(66)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.source_code_info {
            s.write_tag(74)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.public_dependency.is_empty() {
            s.write_tag(82)?;
            VarintArray::write_to(&self.public_dependency, s)?;
        }
        if !self.weak_dependency.is_empty() {
            s.write_tag(90)?;
            VarintArray::write_to(&self.weak_dependency, s)?;
        }
        if let Some(v) = &self.syntax {
            s.write_tag(98)?;
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
        if let Some(v) = &self.package {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.dependency.is_empty() {
            l += self.dependency.len() as u64 + LengthPrefixedArray::len(&self.dependency);
        }
        if !self.message_type.is_empty() {
            l += self.message_type.len() as u64 + LengthPrefixedArray::len(&self.message_type);
        }
        if !self.enum_type.is_empty() {
            l += self.enum_type.len() as u64 + LengthPrefixedArray::len(&self.enum_type);
        }
        if !self.service.is_empty() {
            l += self.service.len() as u64 + LengthPrefixedArray::len(&self.service);
        }
        if !self.extension.is_empty() {
            l += self.extension.len() as u64 + LengthPrefixedArray::len(&self.extension);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.source_code_info {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.public_dependency.is_empty() {
            l += 1 + VarintArray::len(&self.public_dependency);
        }
        if !self.weak_dependency.is_empty() {
            l += 1 + VarintArray::len(&self.weak_dependency);
        }
        if let Some(v) = &self.syntax {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FileDescriptorProto {
    fn default_instance() -> &'static FileDescriptorProto {
        static DEFAULT: FileDescriptorProto = FileDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for FileDescriptorProto {
    #[inline]
    fn default() -> FileDescriptorProto {
        FileDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DescriptorProto_ExtensionRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub options: Option<ExtensionRangeOptions>,
    _unknown: Vec<u8>,
}
impl DescriptorProto_ExtensionRange {
    pub const fn new() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange {
            start: None,
            end: None,
            options: None,
            _unknown: Vec::new(),
        }
    }
    pub fn start(&self) -> i32 {
        self.start.unwrap_or_default()
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        if self.start.is_none() {
            self.start = Some(0);
        }
        self.start.as_mut().unwrap()
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        }
        self.end.as_mut().unwrap()
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
    pub fn options(&self) -> &ExtensionRangeOptions {
        match &self.options {
            Some(v) => v,
            None => ExtensionRangeOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut ExtensionRangeOptions {
        if self.options.is_none() {
            self.options = Some(ExtensionRangeOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: ExtensionRangeOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for DescriptorProto_ExtensionRange {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.start = Some(Varint::read_from(s)?),
                16 => self.end = Some(Varint::read_from(s)?),
                26 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.start {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.end {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.options {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.start {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.end {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for DescriptorProto_ExtensionRange {
    fn default_instance() -> &'static DescriptorProto_ExtensionRange {
        static DEFAULT: DescriptorProto_ExtensionRange = DescriptorProto_ExtensionRange::new();
        &DEFAULT
    }
}
impl Default for DescriptorProto_ExtensionRange {
    #[inline]
    fn default() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DescriptorProto_ReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    _unknown: Vec<u8>,
}
impl DescriptorProto_ReservedRange {
    pub const fn new() -> DescriptorProto_ReservedRange {
        DescriptorProto_ReservedRange {
            start: None,
            end: None,
            _unknown: Vec::new(),
        }
    }
    pub fn start(&self) -> i32 {
        self.start.unwrap_or_default()
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        if self.start.is_none() {
            self.start = Some(0);
        }
        self.start.as_mut().unwrap()
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        }
        self.end.as_mut().unwrap()
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl pecan::Message for DescriptorProto_ReservedRange {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.start = Some(Varint::read_from(s)?),
                16 => self.end = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.start {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.end {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.start {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.end {
            l += 1 + Varint::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for DescriptorProto_ReservedRange {
    fn default_instance() -> &'static DescriptorProto_ReservedRange {
        static DEFAULT: DescriptorProto_ReservedRange = DescriptorProto_ReservedRange::new();
        &DEFAULT
    }
}
impl Default for DescriptorProto_ReservedRange {
    #[inline]
    fn default() -> DescriptorProto_ReservedRange {
        DescriptorProto_ReservedRange::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DescriptorProto {
    pub name: Option<String>,
    pub field: Vec<FieldDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub nested_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub extension_range: Vec<DescriptorProto_ExtensionRange>,
    pub oneof_decl: Vec<OneofDescriptorProto>,
    pub options: Option<MessageOptions>,
    pub reserved_range: Vec<DescriptorProto_ReservedRange>,
    pub reserved_name: Vec<String>,
    _unknown: Vec<u8>,
}
impl DescriptorProto {
    pub const fn new() -> DescriptorProto {
        DescriptorProto {
            name: None,
            field: Vec::new(),
            nested_type: Vec::new(),
            enum_type: Vec::new(),
            extension_range: Vec::new(),
            extension: Vec::new(),
            options: None,
            oneof_decl: Vec::new(),
            reserved_range: Vec::new(),
            reserved_name: Vec::new(),
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
    pub fn options(&self) -> &MessageOptions {
        match &self.options {
            Some(v) => v,
            None => MessageOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut MessageOptions {
        if self.options.is_none() {
            self.options = Some(MessageOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: MessageOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for DescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => LengthPrefixedArray::merge_from(&mut self.field, s)?,
                26 => LengthPrefixedArray::merge_from(&mut self.nested_type, s)?,
                34 => LengthPrefixedArray::merge_from(&mut self.enum_type, s)?,
                42 => LengthPrefixedArray::merge_from(&mut self.extension_range, s)?,
                50 => LengthPrefixedArray::merge_from(&mut self.extension, s)?,
                58 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                66 => LengthPrefixedArray::merge_from(&mut self.oneof_decl, s)?,
                74 => LengthPrefixedArray::merge_from(&mut self.reserved_range, s)?,
                82 => LengthPrefixedArray::merge_from(&mut self.reserved_name, s)?,
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
        if !self.field.is_empty() {
            for i in &self.field {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.nested_type.is_empty() {
            for i in &self.nested_type {
                s.write_tag(26)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.enum_type.is_empty() {
            for i in &self.enum_type {
                s.write_tag(34)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extension_range.is_empty() {
            for i in &self.extension_range {
                s.write_tag(42)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extension.is_empty() {
            for i in &self.extension {
                s.write_tag(50)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.options {
            s.write_tag(58)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.oneof_decl.is_empty() {
            for i in &self.oneof_decl {
                s.write_tag(66)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.reserved_range.is_empty() {
            for i in &self.reserved_range {
                s.write_tag(74)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.reserved_name.is_empty() {
            for i in &self.reserved_name {
                s.write_tag(82)?;
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
        if let Some(v) = &self.name {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.field.is_empty() {
            l += self.field.len() as u64 + LengthPrefixedArray::len(&self.field);
        }
        if !self.nested_type.is_empty() {
            l += self.nested_type.len() as u64 + LengthPrefixedArray::len(&self.nested_type);
        }
        if !self.enum_type.is_empty() {
            l += self.enum_type.len() as u64 + LengthPrefixedArray::len(&self.enum_type);
        }
        if !self.extension_range.is_empty() {
            l +=
                self.extension_range.len() as u64 + LengthPrefixedArray::len(&self.extension_range);
        }
        if !self.extension.is_empty() {
            l += self.extension.len() as u64 + LengthPrefixedArray::len(&self.extension);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.oneof_decl.is_empty() {
            l += self.oneof_decl.len() as u64 + LengthPrefixedArray::len(&self.oneof_decl);
        }
        if !self.reserved_range.is_empty() {
            l += self.reserved_range.len() as u64 + LengthPrefixedArray::len(&self.reserved_range);
        }
        if !self.reserved_name.is_empty() {
            l += self.reserved_name.len() as u64 + LengthPrefixedArray::len(&self.reserved_name);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for DescriptorProto {
    fn default_instance() -> &'static DescriptorProto {
        static DEFAULT: DescriptorProto = DescriptorProto::new();
        &DEFAULT
    }
}
impl Default for DescriptorProto {
    #[inline]
    fn default() -> DescriptorProto {
        DescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl ExtensionRangeOptions {
    pub const fn new() -> ExtensionRangeOptions {
        ExtensionRangeOptions {
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for ExtensionRangeOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for ExtensionRangeOptions {
    fn default_instance() -> &'static ExtensionRangeOptions {
        static DEFAULT: ExtensionRangeOptions = ExtensionRangeOptions::new();
        &DEFAULT
    }
}
impl Default for ExtensionRangeOptions {
    #[inline]
    fn default() -> ExtensionRangeOptions {
        ExtensionRangeOptions::new()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FieldDescriptorProto_Type(i32);
impl pecan::Enumerate for FieldDescriptorProto_Type {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> FieldDescriptorProto_Type {
        FieldDescriptorProto_Type(v)
    }
}
impl FieldDescriptorProto_Type {
    pub const TYPE_DOUBLE: FieldDescriptorProto_Type = FieldDescriptorProto_Type(1);
    pub const TYPE_FLOAT: FieldDescriptorProto_Type = FieldDescriptorProto_Type(2);
    pub const TYPE_INT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(3);
    pub const TYPE_UINT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(4);
    pub const TYPE_INT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(5);
    pub const TYPE_FIXED64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(6);
    pub const TYPE_FIXED32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(7);
    pub const TYPE_BOOL: FieldDescriptorProto_Type = FieldDescriptorProto_Type(8);
    pub const TYPE_STRING: FieldDescriptorProto_Type = FieldDescriptorProto_Type(9);
    pub const TYPE_GROUP: FieldDescriptorProto_Type = FieldDescriptorProto_Type(10);
    pub const TYPE_MESSAGE: FieldDescriptorProto_Type = FieldDescriptorProto_Type(11);
    pub const TYPE_BYTES: FieldDescriptorProto_Type = FieldDescriptorProto_Type(12);
    pub const TYPE_UINT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(13);
    pub const TYPE_ENUM: FieldDescriptorProto_Type = FieldDescriptorProto_Type(14);
    pub const TYPE_SFIXED32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(15);
    pub const TYPE_SFIXED64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(16);
    pub const TYPE_SINT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(17);
    pub const TYPE_SINT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(18);
    pub const fn new() -> FieldDescriptorProto_Type {
        FieldDescriptorProto_Type(0)
    }
}
impl std::fmt::Debug for FieldDescriptorProto_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "TYPE_DOUBLE"),
            2 => write!(f, "TYPE_FLOAT"),
            3 => write!(f, "TYPE_INT64"),
            4 => write!(f, "TYPE_UINT64"),
            5 => write!(f, "TYPE_INT32"),
            6 => write!(f, "TYPE_FIXED64"),
            7 => write!(f, "TYPE_FIXED32"),
            8 => write!(f, "TYPE_BOOL"),
            9 => write!(f, "TYPE_STRING"),
            10 => write!(f, "TYPE_GROUP"),
            11 => write!(f, "TYPE_MESSAGE"),
            12 => write!(f, "TYPE_BYTES"),
            13 => write!(f, "TYPE_UINT32"),
            14 => write!(f, "TYPE_ENUM"),
            15 => write!(f, "TYPE_SFIXED32"),
            16 => write!(f, "TYPE_SFIXED64"),
            17 => write!(f, "TYPE_SINT32"),
            18 => write!(f, "TYPE_SINT64"),
            v => write!(f, "FieldDescriptorProto_Type({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FieldDescriptorProto_Label(i32);
impl pecan::Enumerate for FieldDescriptorProto_Label {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> FieldDescriptorProto_Label {
        FieldDescriptorProto_Label(v)
    }
}
impl FieldDescriptorProto_Label {
    pub const LABEL_OPTIONAL: FieldDescriptorProto_Label = FieldDescriptorProto_Label(1);
    pub const LABEL_REQUIRED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(2);
    pub const LABEL_REPEATED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(3);
    pub const fn new() -> FieldDescriptorProto_Label {
        FieldDescriptorProto_Label(0)
    }
}
impl std::fmt::Debug for FieldDescriptorProto_Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "LABEL_OPTIONAL"),
            2 => write!(f, "LABEL_REQUIRED"),
            3 => write!(f, "LABEL_REPEATED"),
            v => write!(f, "FieldDescriptorProto_Label({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub label: Option<FieldDescriptorProto_Label>,
    pub r#type: Option<FieldDescriptorProto_Type>,
    pub type_name: Option<String>,
    pub extendee: Option<String>,
    pub default_value: Option<String>,
    pub oneof_index: Option<i32>,
    pub json_name: Option<String>,
    pub options: Option<FieldOptions>,
    pub proto3_optional: Option<bool>,
    _unknown: Vec<u8>,
}
impl FieldDescriptorProto {
    pub const fn new() -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: None,
            extendee: None,
            number: None,
            label: None,
            r#type: None,
            type_name: None,
            default_value: None,
            options: None,
            oneof_index: None,
            json_name: None,
            proto3_optional: None,
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
    pub fn extendee(&self) -> &String {
        match &self.extendee {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn extendee_mut(&mut self) -> &mut String {
        if self.extendee.is_none() {
            self.extendee = Some(String::new());
        }
        self.extendee.as_mut().unwrap()
    }
    pub fn set_extendee(&mut self, val: String) {
        self.extendee = Some(val);
    }
    pub fn number(&self) -> i32 {
        self.number.unwrap_or_default()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        if self.number.is_none() {
            self.number = Some(0);
        }
        self.number.as_mut().unwrap()
    }
    pub fn set_number(&mut self, val: i32) {
        self.number = Some(val);
    }
    pub fn label(&self) -> FieldDescriptorProto_Label {
        self.label.unwrap_or_default()
    }
    pub fn label_mut(&mut self) -> &mut FieldDescriptorProto_Label {
        if self.label.is_none() {
            self.label = Some(FieldDescriptorProto_Label::new());
        }
        self.label.as_mut().unwrap()
    }
    pub fn set_label(&mut self, val: FieldDescriptorProto_Label) {
        self.label = Some(val);
    }
    pub fn r#type(&self) -> FieldDescriptorProto_Type {
        self.r#type.unwrap_or_default()
    }
    pub fn type_mut(&mut self) -> &mut FieldDescriptorProto_Type {
        if self.r#type.is_none() {
            self.r#type = Some(FieldDescriptorProto_Type::new());
        }
        self.r#type.as_mut().unwrap()
    }
    pub fn set_type(&mut self, val: FieldDescriptorProto_Type) {
        self.r#type = Some(val);
    }
    pub fn type_name(&self) -> &String {
        match &self.type_name {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn type_name_mut(&mut self) -> &mut String {
        if self.type_name.is_none() {
            self.type_name = Some(String::new());
        }
        self.type_name.as_mut().unwrap()
    }
    pub fn set_type_name(&mut self, val: String) {
        self.type_name = Some(val);
    }
    pub fn default_value(&self) -> &String {
        match &self.default_value {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn default_value_mut(&mut self) -> &mut String {
        if self.default_value.is_none() {
            self.default_value = Some(String::new());
        }
        self.default_value.as_mut().unwrap()
    }
    pub fn set_default_value(&mut self, val: String) {
        self.default_value = Some(val);
    }
    pub fn options(&self) -> &FieldOptions {
        match &self.options {
            Some(v) => v,
            None => FieldOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut FieldOptions {
        if self.options.is_none() {
            self.options = Some(FieldOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: FieldOptions) {
        self.options = Some(val);
    }
    pub fn oneof_index(&self) -> i32 {
        self.oneof_index.unwrap_or_default()
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        if self.oneof_index.is_none() {
            self.oneof_index = Some(0);
        }
        self.oneof_index.as_mut().unwrap()
    }
    pub fn set_oneof_index(&mut self, val: i32) {
        self.oneof_index = Some(val);
    }
    pub fn json_name(&self) -> &String {
        match &self.json_name {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn json_name_mut(&mut self) -> &mut String {
        if self.json_name.is_none() {
            self.json_name = Some(String::new());
        }
        self.json_name.as_mut().unwrap()
    }
    pub fn set_json_name(&mut self, val: String) {
        self.json_name = Some(val);
    }
    pub fn proto3_optional(&self) -> bool {
        self.proto3_optional.unwrap_or_default()
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        if self.proto3_optional.is_none() {
            self.proto3_optional = Some(false);
        }
        self.proto3_optional.as_mut().unwrap()
    }
    pub fn set_proto3_optional(&mut self, val: bool) {
        self.proto3_optional = Some(val);
    }
}
impl pecan::Message for FieldDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => self.extendee = Some(LengthPrefixed::read_from(s)?),
                24 => self.number = Some(Varint::read_from(s)?),
                32 => self.label = Some(Varint::read_from(s)?),
                40 => self.r#type = Some(Varint::read_from(s)?),
                50 => self.type_name = Some(LengthPrefixed::read_from(s)?),
                58 => self.default_value = Some(LengthPrefixed::read_from(s)?),
                66 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                72 => self.oneof_index = Some(Varint::read_from(s)?),
                82 => self.json_name = Some(LengthPrefixed::read_from(s)?),
                136 => self.proto3_optional = Some(Varint::read_from(s)?),
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
        if let Some(v) = &self.extendee {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.number {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.label {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.r#type {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.type_name {
            s.write_tag(50)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.default_value {
            s.write_tag(58)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.options {
            s.write_tag(66)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.oneof_index {
            s.write_tag(72)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.json_name {
            s.write_tag(82)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.proto3_optional {
            s.write_tag(136)?;
            Varint::write_to(v, s)?;
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
        if let Some(v) = &self.extendee {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.number {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.label {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.r#type {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.type_name {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.default_value {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.oneof_index {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.json_name {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.proto3_optional {
            l += 2 + Varint::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FieldDescriptorProto {
    fn default_instance() -> &'static FieldDescriptorProto {
        static DEFAULT: FieldDescriptorProto = FieldDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for FieldDescriptorProto {
    #[inline]
    fn default() -> FieldDescriptorProto {
        FieldDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OneofDescriptorProto {
    pub name: Option<String>,
    pub options: Option<OneofOptions>,
    _unknown: Vec<u8>,
}
impl OneofDescriptorProto {
    pub const fn new() -> OneofDescriptorProto {
        OneofDescriptorProto {
            name: None,
            options: None,
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
    pub fn options(&self) -> &OneofOptions {
        match &self.options {
            Some(v) => v,
            None => OneofOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut OneofOptions {
        if self.options.is_none() {
            self.options = Some(OneofOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: OneofOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for OneofDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => LengthPrefixed::merge_from(self.options_mut(), s)?,
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
        if let Some(v) = &self.options {
            s.write_tag(18)?;
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
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for OneofDescriptorProto {
    fn default_instance() -> &'static OneofDescriptorProto {
        static DEFAULT: OneofDescriptorProto = OneofDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for OneofDescriptorProto {
    #[inline]
    fn default() -> OneofDescriptorProto {
        OneofDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumDescriptorProto_EnumReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    _unknown: Vec<u8>,
}
impl EnumDescriptorProto_EnumReservedRange {
    pub const fn new() -> EnumDescriptorProto_EnumReservedRange {
        EnumDescriptorProto_EnumReservedRange {
            start: None,
            end: None,
            _unknown: Vec::new(),
        }
    }
    pub fn start(&self) -> i32 {
        self.start.unwrap_or_default()
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        if self.start.is_none() {
            self.start = Some(0);
        }
        self.start.as_mut().unwrap()
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        }
        self.end.as_mut().unwrap()
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl pecan::Message for EnumDescriptorProto_EnumReservedRange {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.start = Some(Varint::read_from(s)?),
                16 => self.end = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.start {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.end {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.start {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.end {
            l += 1 + Varint::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumDescriptorProto_EnumReservedRange {
    fn default_instance() -> &'static EnumDescriptorProto_EnumReservedRange {
        static DEFAULT: EnumDescriptorProto_EnumReservedRange =
            EnumDescriptorProto_EnumReservedRange::new();
        &DEFAULT
    }
}
impl Default for EnumDescriptorProto_EnumReservedRange {
    #[inline]
    fn default() -> EnumDescriptorProto_EnumReservedRange {
        EnumDescriptorProto_EnumReservedRange::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumDescriptorProto {
    pub name: Option<String>,
    pub value: Vec<EnumValueDescriptorProto>,
    pub options: Option<EnumOptions>,
    pub reserved_range: Vec<EnumDescriptorProto_EnumReservedRange>,
    pub reserved_name: Vec<String>,
    _unknown: Vec<u8>,
}
impl EnumDescriptorProto {
    pub const fn new() -> EnumDescriptorProto {
        EnumDescriptorProto {
            name: None,
            value: Vec::new(),
            options: None,
            reserved_range: Vec::new(),
            reserved_name: Vec::new(),
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
    pub fn options(&self) -> &EnumOptions {
        match &self.options {
            Some(v) => v,
            None => EnumOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut EnumOptions {
        if self.options.is_none() {
            self.options = Some(EnumOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: EnumOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for EnumDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => LengthPrefixedArray::merge_from(&mut self.value, s)?,
                26 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                34 => LengthPrefixedArray::merge_from(&mut self.reserved_range, s)?,
                42 => LengthPrefixedArray::merge_from(&mut self.reserved_name, s)?,
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
        if !self.value.is_empty() {
            for i in &self.value {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.options {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.reserved_range.is_empty() {
            for i in &self.reserved_range {
                s.write_tag(34)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.reserved_name.is_empty() {
            for i in &self.reserved_name {
                s.write_tag(42)?;
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
        if let Some(v) = &self.name {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.value.is_empty() {
            l += self.value.len() as u64 + LengthPrefixedArray::len(&self.value);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.reserved_range.is_empty() {
            l += self.reserved_range.len() as u64 + LengthPrefixedArray::len(&self.reserved_range);
        }
        if !self.reserved_name.is_empty() {
            l += self.reserved_name.len() as u64 + LengthPrefixedArray::len(&self.reserved_name);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumDescriptorProto {
    fn default_instance() -> &'static EnumDescriptorProto {
        static DEFAULT: EnumDescriptorProto = EnumDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for EnumDescriptorProto {
    #[inline]
    fn default() -> EnumDescriptorProto {
        EnumDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumValueDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub options: Option<EnumValueOptions>,
    _unknown: Vec<u8>,
}
impl EnumValueDescriptorProto {
    pub const fn new() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto {
            name: None,
            number: None,
            options: None,
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
    pub fn number(&self) -> i32 {
        self.number.unwrap_or_default()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        if self.number.is_none() {
            self.number = Some(0);
        }
        self.number.as_mut().unwrap()
    }
    pub fn set_number(&mut self, val: i32) {
        self.number = Some(val);
    }
    pub fn options(&self) -> &EnumValueOptions {
        match &self.options {
            Some(v) => v,
            None => EnumValueOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut EnumValueOptions {
        if self.options.is_none() {
            self.options = Some(EnumValueOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: EnumValueOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for EnumValueDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                16 => self.number = Some(Varint::read_from(s)?),
                26 => LengthPrefixed::merge_from(self.options_mut(), s)?,
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
        if let Some(v) = self.number {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.options {
            s.write_tag(26)?;
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
        if let Some(v) = self.number {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumValueDescriptorProto {
    fn default_instance() -> &'static EnumValueDescriptorProto {
        static DEFAULT: EnumValueDescriptorProto = EnumValueDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for EnumValueDescriptorProto {
    #[inline]
    fn default() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ServiceDescriptorProto {
    pub name: Option<String>,
    pub method: Vec<MethodDescriptorProto>,
    pub options: Option<ServiceOptions>,
    _unknown: Vec<u8>,
}
impl ServiceDescriptorProto {
    pub const fn new() -> ServiceDescriptorProto {
        ServiceDescriptorProto {
            name: None,
            method: Vec::new(),
            options: None,
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
    pub fn options(&self) -> &ServiceOptions {
        match &self.options {
            Some(v) => v,
            None => ServiceOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut ServiceOptions {
        if self.options.is_none() {
            self.options = Some(ServiceOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: ServiceOptions) {
        self.options = Some(val);
    }
}
impl pecan::Message for ServiceDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => LengthPrefixedArray::merge_from(&mut self.method, s)?,
                26 => LengthPrefixed::merge_from(self.options_mut(), s)?,
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
        if !self.method.is_empty() {
            for i in &self.method {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.options {
            s.write_tag(26)?;
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
        if !self.method.is_empty() {
            l += self.method.len() as u64 + LengthPrefixedArray::len(&self.method);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for ServiceDescriptorProto {
    fn default_instance() -> &'static ServiceDescriptorProto {
        static DEFAULT: ServiceDescriptorProto = ServiceDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for ServiceDescriptorProto {
    #[inline]
    fn default() -> ServiceDescriptorProto {
        ServiceDescriptorProto::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MethodDescriptorProto {
    pub name: Option<String>,
    pub input_type: Option<String>,
    pub output_type: Option<String>,
    pub options: Option<MethodOptions>,
    pub client_streaming: Option<bool>,
    pub server_streaming: Option<bool>,
    _unknown: Vec<u8>,
}
impl MethodDescriptorProto {
    pub const fn new() -> MethodDescriptorProto {
        MethodDescriptorProto {
            name: None,
            input_type: None,
            output_type: None,
            options: None,
            client_streaming: None,
            server_streaming: None,
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
    pub fn input_type(&self) -> &String {
        match &self.input_type {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn input_type_mut(&mut self) -> &mut String {
        if self.input_type.is_none() {
            self.input_type = Some(String::new());
        }
        self.input_type.as_mut().unwrap()
    }
    pub fn set_input_type(&mut self, val: String) {
        self.input_type = Some(val);
    }
    pub fn output_type(&self) -> &String {
        match &self.output_type {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn output_type_mut(&mut self) -> &mut String {
        if self.output_type.is_none() {
            self.output_type = Some(String::new());
        }
        self.output_type.as_mut().unwrap()
    }
    pub fn set_output_type(&mut self, val: String) {
        self.output_type = Some(val);
    }
    pub fn options(&self) -> &MethodOptions {
        match &self.options {
            Some(v) => v,
            None => MethodOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut MethodOptions {
        if self.options.is_none() {
            self.options = Some(MethodOptions::new());
        }
        self.options.as_mut().unwrap()
    }
    pub fn set_options(&mut self, val: MethodOptions) {
        self.options = Some(val);
    }
    pub fn client_streaming(&self) -> bool {
        self.client_streaming.unwrap_or_default()
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        if self.client_streaming.is_none() {
            self.client_streaming = Some(false);
        }
        self.client_streaming.as_mut().unwrap()
    }
    pub fn set_client_streaming(&mut self, val: bool) {
        self.client_streaming = Some(val);
    }
    pub fn server_streaming(&self) -> bool {
        self.server_streaming.unwrap_or_default()
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        if self.server_streaming.is_none() {
            self.server_streaming = Some(false);
        }
        self.server_streaming.as_mut().unwrap()
    }
    pub fn set_server_streaming(&mut self, val: bool) {
        self.server_streaming = Some(val);
    }
}
impl pecan::Message for MethodDescriptorProto {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => self.input_type = Some(LengthPrefixed::read_from(s)?),
                26 => self.output_type = Some(LengthPrefixed::read_from(s)?),
                34 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                40 => self.client_streaming = Some(Varint::read_from(s)?),
                48 => self.server_streaming = Some(Varint::read_from(s)?),
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
        if let Some(v) = &self.input_type {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.output_type {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.options {
            s.write_tag(34)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.client_streaming {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.server_streaming {
            s.write_tag(48)?;
            Varint::write_to(v, s)?;
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
        if let Some(v) = &self.input_type {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.output_type {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.options {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.client_streaming {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.server_streaming {
            l += 1 + Varint::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for MethodDescriptorProto {
    fn default_instance() -> &'static MethodDescriptorProto {
        static DEFAULT: MethodDescriptorProto = MethodDescriptorProto::new();
        &DEFAULT
    }
}
impl Default for MethodDescriptorProto {
    #[inline]
    fn default() -> MethodDescriptorProto {
        MethodDescriptorProto::new()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FileOptions_OptimizeMode(i32);
impl pecan::Enumerate for FileOptions_OptimizeMode {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> FileOptions_OptimizeMode {
        FileOptions_OptimizeMode(v)
    }
}
impl FileOptions_OptimizeMode {
    pub const SPEED: FileOptions_OptimizeMode = FileOptions_OptimizeMode(1);
    pub const CODE_SIZE: FileOptions_OptimizeMode = FileOptions_OptimizeMode(2);
    pub const LITE_RUNTIME: FileOptions_OptimizeMode = FileOptions_OptimizeMode(3);
    pub const fn new() -> FileOptions_OptimizeMode {
        FileOptions_OptimizeMode(0)
    }
}
impl std::fmt::Debug for FileOptions_OptimizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "SPEED"),
            2 => write!(f, "CODE_SIZE"),
            3 => write!(f, "LITE_RUNTIME"),
            v => write!(f, "FileOptions_OptimizeMode({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FileOptions {
    pub java_package: Option<String>,
    pub java_outer_classname: Option<String>,
    pub java_multiple_files: Option<bool>,
    pub java_generate_equals_and_hash: Option<bool>,
    pub java_string_check_utf8: Option<bool>,
    pub optimize_for: Option<FileOptions_OptimizeMode>,
    pub go_package: Option<String>,
    pub cc_generic_services: Option<bool>,
    pub java_generic_services: Option<bool>,
    pub py_generic_services: Option<bool>,
    pub php_generic_services: Option<bool>,
    pub deprecated: Option<bool>,
    pub cc_enable_arenas: Option<bool>,
    pub objc_class_prefix: Option<String>,
    pub csharp_namespace: Option<String>,
    pub swift_prefix: Option<String>,
    pub php_class_prefix: Option<String>,
    pub php_namespace: Option<String>,
    pub php_metadata_namespace: Option<String>,
    pub ruby_package: Option<String>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl FileOptions {
    pub const fn new() -> FileOptions {
        FileOptions {
            java_package: None,
            java_outer_classname: None,
            optimize_for: None,
            java_multiple_files: None,
            go_package: None,
            cc_generic_services: None,
            java_generic_services: None,
            py_generic_services: None,
            java_generate_equals_and_hash: None,
            deprecated: None,
            java_string_check_utf8: None,
            cc_enable_arenas: None,
            objc_class_prefix: None,
            csharp_namespace: None,
            swift_prefix: None,
            php_class_prefix: None,
            php_namespace: None,
            php_generic_services: None,
            php_metadata_namespace: None,
            ruby_package: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn java_package(&self) -> &String {
        match &self.java_package {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn java_package_mut(&mut self) -> &mut String {
        if self.java_package.is_none() {
            self.java_package = Some(String::new());
        }
        self.java_package.as_mut().unwrap()
    }
    pub fn set_java_package(&mut self, val: String) {
        self.java_package = Some(val);
    }
    pub fn java_outer_classname(&self) -> &String {
        match &self.java_outer_classname {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut String {
        if self.java_outer_classname.is_none() {
            self.java_outer_classname = Some(String::new());
        }
        self.java_outer_classname.as_mut().unwrap()
    }
    pub fn set_java_outer_classname(&mut self, val: String) {
        self.java_outer_classname = Some(val);
    }
    pub fn optimize_for(&self) -> FileOptions_OptimizeMode {
        self.optimize_for.unwrap_or_default()
    }
    pub fn optimize_for_mut(&mut self) -> &mut FileOptions_OptimizeMode {
        if self.optimize_for.is_none() {
            self.optimize_for = Some(FileOptions_OptimizeMode::new());
        }
        self.optimize_for.as_mut().unwrap()
    }
    pub fn set_optimize_for(&mut self, val: FileOptions_OptimizeMode) {
        self.optimize_for = Some(val);
    }
    pub fn java_multiple_files(&self) -> bool {
        self.java_multiple_files.unwrap_or_default()
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        if self.java_multiple_files.is_none() {
            self.java_multiple_files = Some(false);
        }
        self.java_multiple_files.as_mut().unwrap()
    }
    pub fn set_java_multiple_files(&mut self, val: bool) {
        self.java_multiple_files = Some(val);
    }
    pub fn go_package(&self) -> &String {
        match &self.go_package {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn go_package_mut(&mut self) -> &mut String {
        if self.go_package.is_none() {
            self.go_package = Some(String::new());
        }
        self.go_package.as_mut().unwrap()
    }
    pub fn set_go_package(&mut self, val: String) {
        self.go_package = Some(val);
    }
    pub fn cc_generic_services(&self) -> bool {
        self.cc_generic_services.unwrap_or_default()
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        if self.cc_generic_services.is_none() {
            self.cc_generic_services = Some(false);
        }
        self.cc_generic_services.as_mut().unwrap()
    }
    pub fn set_cc_generic_services(&mut self, val: bool) {
        self.cc_generic_services = Some(val);
    }
    pub fn java_generic_services(&self) -> bool {
        self.java_generic_services.unwrap_or_default()
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        if self.java_generic_services.is_none() {
            self.java_generic_services = Some(false);
        }
        self.java_generic_services.as_mut().unwrap()
    }
    pub fn set_java_generic_services(&mut self, val: bool) {
        self.java_generic_services = Some(val);
    }
    pub fn py_generic_services(&self) -> bool {
        self.py_generic_services.unwrap_or_default()
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        if self.py_generic_services.is_none() {
            self.py_generic_services = Some(false);
        }
        self.py_generic_services.as_mut().unwrap()
    }
    pub fn set_py_generic_services(&mut self, val: bool) {
        self.py_generic_services = Some(val);
    }
    pub fn java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.unwrap_or_default()
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        if self.java_generate_equals_and_hash.is_none() {
            self.java_generate_equals_and_hash = Some(false);
        }
        self.java_generate_equals_and_hash.as_mut().unwrap()
    }
    pub fn set_java_generate_equals_and_hash(&mut self, val: bool) {
        self.java_generate_equals_and_hash = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.unwrap_or_default()
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        if self.java_string_check_utf8.is_none() {
            self.java_string_check_utf8 = Some(false);
        }
        self.java_string_check_utf8.as_mut().unwrap()
    }
    pub fn set_java_string_check_utf8(&mut self, val: bool) {
        self.java_string_check_utf8 = Some(val);
    }
    pub fn cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.unwrap_or_default()
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        if self.cc_enable_arenas.is_none() {
            self.cc_enable_arenas = Some(false);
        }
        self.cc_enable_arenas.as_mut().unwrap()
    }
    pub fn set_cc_enable_arenas(&mut self, val: bool) {
        self.cc_enable_arenas = Some(val);
    }
    pub fn objc_class_prefix(&self) -> &String {
        match &self.objc_class_prefix {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut String {
        if self.objc_class_prefix.is_none() {
            self.objc_class_prefix = Some(String::new());
        }
        self.objc_class_prefix.as_mut().unwrap()
    }
    pub fn set_objc_class_prefix(&mut self, val: String) {
        self.objc_class_prefix = Some(val);
    }
    pub fn csharp_namespace(&self) -> &String {
        match &self.csharp_namespace {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut String {
        if self.csharp_namespace.is_none() {
            self.csharp_namespace = Some(String::new());
        }
        self.csharp_namespace.as_mut().unwrap()
    }
    pub fn set_csharp_namespace(&mut self, val: String) {
        self.csharp_namespace = Some(val);
    }
    pub fn swift_prefix(&self) -> &String {
        match &self.swift_prefix {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn swift_prefix_mut(&mut self) -> &mut String {
        if self.swift_prefix.is_none() {
            self.swift_prefix = Some(String::new());
        }
        self.swift_prefix.as_mut().unwrap()
    }
    pub fn set_swift_prefix(&mut self, val: String) {
        self.swift_prefix = Some(val);
    }
    pub fn php_class_prefix(&self) -> &String {
        match &self.php_class_prefix {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut String {
        if self.php_class_prefix.is_none() {
            self.php_class_prefix = Some(String::new());
        }
        self.php_class_prefix.as_mut().unwrap()
    }
    pub fn set_php_class_prefix(&mut self, val: String) {
        self.php_class_prefix = Some(val);
    }
    pub fn php_namespace(&self) -> &String {
        match &self.php_namespace {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn php_namespace_mut(&mut self) -> &mut String {
        if self.php_namespace.is_none() {
            self.php_namespace = Some(String::new());
        }
        self.php_namespace.as_mut().unwrap()
    }
    pub fn set_php_namespace(&mut self, val: String) {
        self.php_namespace = Some(val);
    }
    pub fn php_generic_services(&self) -> bool {
        self.php_generic_services.unwrap_or_default()
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        if self.php_generic_services.is_none() {
            self.php_generic_services = Some(false);
        }
        self.php_generic_services.as_mut().unwrap()
    }
    pub fn set_php_generic_services(&mut self, val: bool) {
        self.php_generic_services = Some(val);
    }
    pub fn php_metadata_namespace(&self) -> &String {
        match &self.php_metadata_namespace {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut String {
        if self.php_metadata_namespace.is_none() {
            self.php_metadata_namespace = Some(String::new());
        }
        self.php_metadata_namespace.as_mut().unwrap()
    }
    pub fn set_php_metadata_namespace(&mut self, val: String) {
        self.php_metadata_namespace = Some(val);
    }
    pub fn ruby_package(&self) -> &String {
        match &self.ruby_package {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn ruby_package_mut(&mut self) -> &mut String {
        if self.ruby_package.is_none() {
            self.ruby_package = Some(String::new());
        }
        self.ruby_package.as_mut().unwrap()
    }
    pub fn set_ruby_package(&mut self, val: String) {
        self.ruby_package = Some(val);
    }
}
impl pecan::Message for FileOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.java_package = Some(LengthPrefixed::read_from(s)?),
                66 => self.java_outer_classname = Some(LengthPrefixed::read_from(s)?),
                72 => self.optimize_for = Some(Varint::read_from(s)?),
                80 => self.java_multiple_files = Some(Varint::read_from(s)?),
                90 => self.go_package = Some(LengthPrefixed::read_from(s)?),
                128 => self.cc_generic_services = Some(Varint::read_from(s)?),
                136 => self.java_generic_services = Some(Varint::read_from(s)?),
                144 => self.py_generic_services = Some(Varint::read_from(s)?),
                160 => self.java_generate_equals_and_hash = Some(Varint::read_from(s)?),
                184 => self.deprecated = Some(Varint::read_from(s)?),
                216 => self.java_string_check_utf8 = Some(Varint::read_from(s)?),
                248 => self.cc_enable_arenas = Some(Varint::read_from(s)?),
                290 => self.objc_class_prefix = Some(LengthPrefixed::read_from(s)?),
                298 => self.csharp_namespace = Some(LengthPrefixed::read_from(s)?),
                314 => self.swift_prefix = Some(LengthPrefixed::read_from(s)?),
                322 => self.php_class_prefix = Some(LengthPrefixed::read_from(s)?),
                330 => self.php_namespace = Some(LengthPrefixed::read_from(s)?),
                336 => self.php_generic_services = Some(Varint::read_from(s)?),
                354 => self.php_metadata_namespace = Some(LengthPrefixed::read_from(s)?),
                362 => self.ruby_package = Some(LengthPrefixed::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = &self.java_package {
            s.write_tag(10)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.java_outer_classname {
            s.write_tag(66)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.optimize_for {
            s.write_tag(72)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.java_multiple_files {
            s.write_tag(80)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.go_package {
            s.write_tag(90)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.cc_generic_services {
            s.write_tag(128)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.java_generic_services {
            s.write_tag(136)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.py_generic_services {
            s.write_tag(144)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.java_generate_equals_and_hash {
            s.write_tag(160)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.deprecated {
            s.write_tag(184)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.java_string_check_utf8 {
            s.write_tag(216)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.cc_enable_arenas {
            s.write_tag(248)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.objc_class_prefix {
            s.write_tag(290)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.csharp_namespace {
            s.write_tag(298)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.swift_prefix {
            s.write_tag(314)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.php_class_prefix {
            s.write_tag(322)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.php_namespace {
            s.write_tag(330)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.php_generic_services {
            s.write_tag(336)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = &self.php_metadata_namespace {
            s.write_tag(354)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.ruby_package {
            s.write_tag(362)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = &self.java_package {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.java_outer_classname {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.optimize_for {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.java_multiple_files {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = &self.go_package {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.cc_generic_services {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.java_generic_services {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.py_generic_services {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.java_generate_equals_and_hash {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.deprecated {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.java_string_check_utf8 {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.cc_enable_arenas {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = &self.objc_class_prefix {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.csharp_namespace {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.swift_prefix {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.php_class_prefix {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.php_namespace {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.php_generic_services {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = &self.php_metadata_namespace {
            l += 2 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.ruby_package {
            l += 2 + LengthPrefixed::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FileOptions {
    fn default_instance() -> &'static FileOptions {
        static DEFAULT: FileOptions = FileOptions::new();
        &DEFAULT
    }
}
impl Default for FileOptions {
    #[inline]
    fn default() -> FileOptions {
        FileOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MessageOptions {
    pub message_set_wire_format: Option<bool>,
    pub no_standard_descriptor_accessor: Option<bool>,
    pub deprecated: Option<bool>,
    pub map_entry: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl MessageOptions {
    pub const fn new() -> MessageOptions {
        MessageOptions {
            message_set_wire_format: None,
            no_standard_descriptor_accessor: None,
            deprecated: None,
            map_entry: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.unwrap_or_default()
    }
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        if self.message_set_wire_format.is_none() {
            self.message_set_wire_format = Some(false);
        }
        self.message_set_wire_format.as_mut().unwrap()
    }
    pub fn set_message_set_wire_format(&mut self, val: bool) {
        self.message_set_wire_format = Some(val);
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.unwrap_or_default()
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        if self.no_standard_descriptor_accessor.is_none() {
            self.no_standard_descriptor_accessor = Some(false);
        }
        self.no_standard_descriptor_accessor.as_mut().unwrap()
    }
    pub fn set_no_standard_descriptor_accessor(&mut self, val: bool) {
        self.no_standard_descriptor_accessor = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn map_entry(&self) -> bool {
        self.map_entry.unwrap_or_default()
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        if self.map_entry.is_none() {
            self.map_entry = Some(false);
        }
        self.map_entry.as_mut().unwrap()
    }
    pub fn set_map_entry(&mut self, val: bool) {
        self.map_entry = Some(val);
    }
}
impl pecan::Message for MessageOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.message_set_wire_format = Some(Varint::read_from(s)?),
                16 => self.no_standard_descriptor_accessor = Some(Varint::read_from(s)?),
                24 => self.deprecated = Some(Varint::read_from(s)?),
                56 => self.map_entry = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.message_set_wire_format {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.no_standard_descriptor_accessor {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.deprecated {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.map_entry {
            s.write_tag(56)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.message_set_wire_format {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.no_standard_descriptor_accessor {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.deprecated {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.map_entry {
            l += 1 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for MessageOptions {
    fn default_instance() -> &'static MessageOptions {
        static DEFAULT: MessageOptions = MessageOptions::new();
        &DEFAULT
    }
}
impl Default for MessageOptions {
    #[inline]
    fn default() -> MessageOptions {
        MessageOptions::new()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FieldOptions_CType(i32);
impl pecan::Enumerate for FieldOptions_CType {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> FieldOptions_CType {
        FieldOptions_CType(v)
    }
}
impl FieldOptions_CType {
    pub const STRING: FieldOptions_CType = FieldOptions_CType(0);
    pub const CORD: FieldOptions_CType = FieldOptions_CType(1);
    pub const STRING_PIECE: FieldOptions_CType = FieldOptions_CType(2);
    pub const fn new() -> FieldOptions_CType {
        FieldOptions_CType(0)
    }
}
impl std::fmt::Debug for FieldOptions_CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "STRING"),
            1 => write!(f, "CORD"),
            2 => write!(f, "STRING_PIECE"),
            v => write!(f, "FieldOptions_CType({})", v),
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FieldOptions_JsType(i32);
impl pecan::Enumerate for FieldOptions_JsType {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> FieldOptions_JsType {
        FieldOptions_JsType(v)
    }
}
impl FieldOptions_JsType {
    pub const JS_NORMAL: FieldOptions_JsType = FieldOptions_JsType(0);
    pub const JS_STRING: FieldOptions_JsType = FieldOptions_JsType(1);
    pub const JS_NUMBER: FieldOptions_JsType = FieldOptions_JsType(2);
    pub const fn new() -> FieldOptions_JsType {
        FieldOptions_JsType(0)
    }
}
impl std::fmt::Debug for FieldOptions_JsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "JS_NORMAL"),
            1 => write!(f, "JS_STRING"),
            2 => write!(f, "JS_NUMBER"),
            v => write!(f, "FieldOptions_JsType({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOptions {
    pub ctype: Option<FieldOptions_CType>,
    pub packed: Option<bool>,
    pub jstype: Option<FieldOptions_JsType>,
    pub lazy: Option<bool>,
    pub deprecated: Option<bool>,
    pub weak: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl FieldOptions {
    pub const fn new() -> FieldOptions {
        FieldOptions {
            ctype: None,
            packed: None,
            deprecated: None,
            lazy: None,
            jstype: None,
            weak: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn ctype(&self) -> FieldOptions_CType {
        self.ctype.unwrap_or_default()
    }
    pub fn ctype_mut(&mut self) -> &mut FieldOptions_CType {
        if self.ctype.is_none() {
            self.ctype = Some(FieldOptions_CType::new());
        }
        self.ctype.as_mut().unwrap()
    }
    pub fn set_ctype(&mut self, val: FieldOptions_CType) {
        self.ctype = Some(val);
    }
    pub fn packed(&self) -> bool {
        self.packed.unwrap_or_default()
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        if self.packed.is_none() {
            self.packed = Some(false);
        }
        self.packed.as_mut().unwrap()
    }
    pub fn set_packed(&mut self, val: bool) {
        self.packed = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn lazy(&self) -> bool {
        self.lazy.unwrap_or_default()
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        if self.lazy.is_none() {
            self.lazy = Some(false);
        }
        self.lazy.as_mut().unwrap()
    }
    pub fn set_lazy(&mut self, val: bool) {
        self.lazy = Some(val);
    }
    pub fn jstype(&self) -> FieldOptions_JsType {
        self.jstype.unwrap_or_default()
    }
    pub fn jstype_mut(&mut self) -> &mut FieldOptions_JsType {
        if self.jstype.is_none() {
            self.jstype = Some(FieldOptions_JsType::new());
        }
        self.jstype.as_mut().unwrap()
    }
    pub fn set_jstype(&mut self, val: FieldOptions_JsType) {
        self.jstype = Some(val);
    }
    pub fn weak(&self) -> bool {
        self.weak.unwrap_or_default()
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        if self.weak.is_none() {
            self.weak = Some(false);
        }
        self.weak.as_mut().unwrap()
    }
    pub fn set_weak(&mut self, val: bool) {
        self.weak = Some(val);
    }
}
impl pecan::Message for FieldOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.ctype = Some(Varint::read_from(s)?),
                16 => self.packed = Some(Varint::read_from(s)?),
                24 => self.deprecated = Some(Varint::read_from(s)?),
                40 => self.lazy = Some(Varint::read_from(s)?),
                48 => self.jstype = Some(Varint::read_from(s)?),
                80 => self.weak = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.ctype {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.packed {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.deprecated {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.lazy {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.jstype {
            s.write_tag(48)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.weak {
            s.write_tag(80)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.ctype {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.packed {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.deprecated {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.lazy {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.jstype {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.weak {
            l += 1 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for FieldOptions {
    fn default_instance() -> &'static FieldOptions {
        static DEFAULT: FieldOptions = FieldOptions::new();
        &DEFAULT
    }
}
impl Default for FieldOptions {
    #[inline]
    fn default() -> FieldOptions {
        FieldOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OneofOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl OneofOptions {
    pub const fn new() -> OneofOptions {
        OneofOptions {
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for OneofOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for OneofOptions {
    fn default_instance() -> &'static OneofOptions {
        static DEFAULT: OneofOptions = OneofOptions::new();
        &DEFAULT
    }
}
impl Default for OneofOptions {
    #[inline]
    fn default() -> OneofOptions {
        OneofOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumOptions {
    pub allow_alias: Option<bool>,
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl EnumOptions {
    pub const fn new() -> EnumOptions {
        EnumOptions {
            allow_alias: None,
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn allow_alias(&self) -> bool {
        self.allow_alias.unwrap_or_default()
    }
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        if self.allow_alias.is_none() {
            self.allow_alias = Some(false);
        }
        self.allow_alias.as_mut().unwrap()
    }
    pub fn set_allow_alias(&mut self, val: bool) {
        self.allow_alias = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl pecan::Message for EnumOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                16 => self.allow_alias = Some(Varint::read_from(s)?),
                24 => self.deprecated = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.allow_alias {
            s.write_tag(16)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.deprecated {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.allow_alias {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.deprecated {
            l += 1 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumOptions {
    fn default_instance() -> &'static EnumOptions {
        static DEFAULT: EnumOptions = EnumOptions::new();
        &DEFAULT
    }
}
impl Default for EnumOptions {
    #[inline]
    fn default() -> EnumOptions {
        EnumOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnumValueOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl EnumValueOptions {
    pub const fn new() -> EnumValueOptions {
        EnumValueOptions {
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl pecan::Message for EnumValueOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.deprecated = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.deprecated {
            s.write_tag(8)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.deprecated {
            l += 1 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for EnumValueOptions {
    fn default_instance() -> &'static EnumValueOptions {
        static DEFAULT: EnumValueOptions = EnumValueOptions::new();
        &DEFAULT
    }
}
impl Default for EnumValueOptions {
    #[inline]
    fn default() -> EnumValueOptions {
        EnumValueOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ServiceOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl ServiceOptions {
    pub const fn new() -> ServiceOptions {
        ServiceOptions {
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl pecan::Message for ServiceOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                264 => self.deprecated = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.deprecated {
            s.write_tag(264)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.deprecated {
            l += 2 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for ServiceOptions {
    fn default_instance() -> &'static ServiceOptions {
        static DEFAULT: ServiceOptions = ServiceOptions::new();
        &DEFAULT
    }
}
impl Default for ServiceOptions {
    #[inline]
    fn default() -> ServiceOptions {
        ServiceOptions::new()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MethodOptions_IdempotencyLevel(i32);
impl pecan::Enumerate for MethodOptions_IdempotencyLevel {
    #[inline]
    fn value(self) -> i32 {
        self.0
    }
    #[inline]
    fn from_value(v: i32) -> MethodOptions_IdempotencyLevel {
        MethodOptions_IdempotencyLevel(v)
    }
}
impl MethodOptions_IdempotencyLevel {
    pub const IDEMPOTENCY_UNKNOWN: MethodOptions_IdempotencyLevel =
        MethodOptions_IdempotencyLevel(0);
    pub const NO_SIDE_EFFECTS: MethodOptions_IdempotencyLevel = MethodOptions_IdempotencyLevel(1);
    pub const IDEMPOTENT: MethodOptions_IdempotencyLevel = MethodOptions_IdempotencyLevel(2);
    pub const fn new() -> MethodOptions_IdempotencyLevel {
        MethodOptions_IdempotencyLevel(0)
    }
}
impl std::fmt::Debug for MethodOptions_IdempotencyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "IDEMPOTENCY_UNKNOWN"),
            1 => write!(f, "NO_SIDE_EFFECTS"),
            2 => write!(f, "IDEMPOTENT"),
            v => write!(f, "MethodOptions_IdempotencyLevel({})", v),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MethodOptions {
    pub deprecated: Option<bool>,
    pub idempotency_level: Option<MethodOptions_IdempotencyLevel>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub extensions: pecan::ExtensionMap,
    _unknown: Vec<u8>,
}
impl MethodOptions {
    pub const fn new() -> MethodOptions {
        MethodOptions {
            deprecated: None,
            idempotency_level: None,
            uninterpreted_option: Vec::new(),
            extensions: pecan::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        }
        self.deprecated.as_mut().unwrap()
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn idempotency_level(&self) -> MethodOptions_IdempotencyLevel {
        self.idempotency_level.unwrap_or_default()
    }
    pub fn idempotency_level_mut(&mut self) -> &mut MethodOptions_IdempotencyLevel {
        if self.idempotency_level.is_none() {
            self.idempotency_level = Some(MethodOptions_IdempotencyLevel::new());
        }
        self.idempotency_level.as_mut().unwrap()
    }
    pub fn set_idempotency_level(&mut self, val: MethodOptions_IdempotencyLevel) {
        self.idempotency_level = Some(val);
    }
}
impl pecan::Message for MethodOptions {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                264 => self.deprecated = Some(Varint::read_from(s)?),
                272 => self.idempotency_level = Some(Varint::read_from(s)?),
                7994 => LengthPrefixedArray::merge_from(&mut self.uninterpreted_option, s)?,
                0 => return Ok(()),
                tag => {
                    if (8000..=4294967303).contains(&tag) {
                        s.read_extension(tag, &mut self.extensions)?;
                        continue;
                    }
                    s.read_unknown_field(tag, &mut self._unknown)?;
                }
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if let Some(v) = self.deprecated {
            s.write_tag(264)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.idempotency_level {
            s.write_tag(272)?;
            Varint::write_to(v, s)?;
        }
        if !self.uninterpreted_option.is_empty() {
            for i in &self.uninterpreted_option {
                s.write_tag(7994)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if !self.extensions.is_empty() {
            s.write_extensions(&self.extensions)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if let Some(v) = self.deprecated {
            l += 2 + Varint::len(v);
        }
        if let Some(v) = self.idempotency_level {
            l += 2 + Varint::len(v);
        }
        if !self.uninterpreted_option.is_empty() {
            l += 2 * self.uninterpreted_option.len() as u64
                + LengthPrefixedArray::len(&self.uninterpreted_option);
        }
        if !self.extensions.is_empty() {
            l += self.extensions.len();
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for MethodOptions {
    fn default_instance() -> &'static MethodOptions {
        static DEFAULT: MethodOptions = MethodOptions::new();
        &DEFAULT
    }
}
impl Default for MethodOptions {
    #[inline]
    fn default() -> MethodOptions {
        MethodOptions::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UninterpretedOption_NamePart {
    pub name_part: String,
    pub is_extension: bool,
    _unknown: Vec<u8>,
}
impl UninterpretedOption_NamePart {
    pub const fn new() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart {
            name_part: String::new(),
            is_extension: false,
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for UninterpretedOption_NamePart {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name_part = LengthPrefixed::read_from(s)?,
                16 => self.is_extension = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name_part.is_empty() {
            s.write_tag(10)?;
            LengthPrefixed::write_to(&self.name_part, s)?;
        }
        if self.is_extension {
            s.write_tag(16)?;
            Varint::write_to(self.is_extension, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.name_part.is_empty() {
            l += 1 + LengthPrefixed::len(&self.name_part);
        }
        if self.is_extension {
            l += 1 + Varint::len(self.is_extension);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for UninterpretedOption_NamePart {
    fn default_instance() -> &'static UninterpretedOption_NamePart {
        static DEFAULT: UninterpretedOption_NamePart = UninterpretedOption_NamePart::new();
        &DEFAULT
    }
}
impl Default for UninterpretedOption_NamePart {
    #[inline]
    fn default() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UninterpretedOption {
    pub name: Vec<UninterpretedOption_NamePart>,
    pub identifier_value: Option<String>,
    pub positive_int_value: Option<u64>,
    pub negative_int_value: Option<i64>,
    pub double_value: Option<f64>,
    pub string_value: Option<pecan::Bytes>,
    pub aggregate_value: Option<String>,
    _unknown: Vec<u8>,
}
impl UninterpretedOption {
    pub const fn new() -> UninterpretedOption {
        UninterpretedOption {
            name: Vec::new(),
            identifier_value: None,
            positive_int_value: None,
            negative_int_value: None,
            double_value: None,
            string_value: None,
            aggregate_value: None,
            _unknown: Vec::new(),
        }
    }
    pub fn identifier_value(&self) -> &String {
        match &self.identifier_value {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn identifier_value_mut(&mut self) -> &mut String {
        if self.identifier_value.is_none() {
            self.identifier_value = Some(String::new());
        }
        self.identifier_value.as_mut().unwrap()
    }
    pub fn set_identifier_value(&mut self, val: String) {
        self.identifier_value = Some(val);
    }
    pub fn positive_int_value(&self) -> u64 {
        self.positive_int_value.unwrap_or_default()
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        if self.positive_int_value.is_none() {
            self.positive_int_value = Some(0);
        }
        self.positive_int_value.as_mut().unwrap()
    }
    pub fn set_positive_int_value(&mut self, val: u64) {
        self.positive_int_value = Some(val);
    }
    pub fn negative_int_value(&self) -> i64 {
        self.negative_int_value.unwrap_or_default()
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        if self.negative_int_value.is_none() {
            self.negative_int_value = Some(0);
        }
        self.negative_int_value.as_mut().unwrap()
    }
    pub fn set_negative_int_value(&mut self, val: i64) {
        self.negative_int_value = Some(val);
    }
    pub fn double_value(&self) -> f64 {
        self.double_value.unwrap_or_default()
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        if self.double_value.is_none() {
            self.double_value = Some(0f64);
        }
        self.double_value.as_mut().unwrap()
    }
    pub fn set_double_value(&mut self, val: f64) {
        self.double_value = Some(val);
    }
    pub fn string_value(&self) -> &pecan::Bytes {
        match &self.string_value {
            Some(v) => v,
            None => pecan::Bytes::default_instance(),
        }
    }
    pub fn string_value_mut(&mut self) -> &mut pecan::Bytes {
        if self.string_value.is_none() {
            self.string_value = Some(pecan::Bytes::new());
        }
        self.string_value.as_mut().unwrap()
    }
    pub fn set_string_value(&mut self, val: pecan::Bytes) {
        self.string_value = Some(val);
    }
    pub fn aggregate_value(&self) -> &String {
        match &self.aggregate_value {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn aggregate_value_mut(&mut self) -> &mut String {
        if self.aggregate_value.is_none() {
            self.aggregate_value = Some(String::new());
        }
        self.aggregate_value.as_mut().unwrap()
    }
    pub fn set_aggregate_value(&mut self, val: String) {
        self.aggregate_value = Some(val);
    }
}
impl pecan::Message for UninterpretedOption {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                18 => LengthPrefixedArray::merge_from(&mut self.name, s)?,
                26 => self.identifier_value = Some(LengthPrefixed::read_from(s)?),
                32 => self.positive_int_value = Some(Varint::read_from(s)?),
                40 => self.negative_int_value = Some(Varint::read_from(s)?),
                49 => self.double_value = Some(Fixed64::read_from(s)?),
                58 => self.string_value = Some(LengthPrefixed::read_from(s)?),
                66 => self.aggregate_value = Some(LengthPrefixed::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.name.is_empty() {
            for i in &self.name {
                s.write_tag(18)?;
                LengthPrefixed::write_to(i, s)?;
            }
        }
        if let Some(v) = &self.identifier_value {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.positive_int_value {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.negative_int_value {
            s.write_tag(40)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.double_value {
            s.write_tag(49)?;
            Fixed64::write_to(v, s)?;
        }
        if let Some(v) = &self.string_value {
            s.write_tag(58)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.aggregate_value {
            s.write_tag(66)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.name.is_empty() {
            l += self.name.len() as u64 + LengthPrefixedArray::len(&self.name);
        }
        if let Some(v) = &self.identifier_value {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.positive_int_value {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.negative_int_value {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.double_value {
            l += 1 + Fixed64::len(v);
        }
        if let Some(v) = &self.string_value {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.aggregate_value {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for UninterpretedOption {
    fn default_instance() -> &'static UninterpretedOption {
        static DEFAULT: UninterpretedOption = UninterpretedOption::new();
        &DEFAULT
    }
}
impl Default for UninterpretedOption {
    #[inline]
    fn default() -> UninterpretedOption {
        UninterpretedOption::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SourceCodeInfo_Location {
    pub path: Vec<i32>,
    pub span: Vec<i32>,
    pub leading_comments: Option<String>,
    pub trailing_comments: Option<String>,
    pub leading_detached_comments: Vec<String>,
    _unknown: Vec<u8>,
}
impl SourceCodeInfo_Location {
    pub const fn new() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location {
            path: Vec::new(),
            span: Vec::new(),
            leading_comments: None,
            trailing_comments: None,
            leading_detached_comments: Vec::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn leading_comments(&self) -> &String {
        match &self.leading_comments {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn leading_comments_mut(&mut self) -> &mut String {
        if self.leading_comments.is_none() {
            self.leading_comments = Some(String::new());
        }
        self.leading_comments.as_mut().unwrap()
    }
    pub fn set_leading_comments(&mut self, val: String) {
        self.leading_comments = Some(val);
    }
    pub fn trailing_comments(&self) -> &String {
        match &self.trailing_comments {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn trailing_comments_mut(&mut self) -> &mut String {
        if self.trailing_comments.is_none() {
            self.trailing_comments = Some(String::new());
        }
        self.trailing_comments.as_mut().unwrap()
    }
    pub fn set_trailing_comments(&mut self, val: String) {
        self.trailing_comments = Some(val);
    }
}
impl pecan::Message for SourceCodeInfo_Location {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => VarintArray::merge_from(&mut self.path, s)?,
                18 => VarintArray::merge_from(&mut self.span, s)?,
                26 => self.leading_comments = Some(LengthPrefixed::read_from(s)?),
                34 => self.trailing_comments = Some(LengthPrefixed::read_from(s)?),
                50 => LengthPrefixedArray::merge_from(&mut self.leading_detached_comments, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.path.is_empty() {
            s.write_tag(10)?;
            VarintArray::write_to(&self.path, s)?;
        }
        if !self.span.is_empty() {
            s.write_tag(18)?;
            VarintArray::write_to(&self.span, s)?;
        }
        if let Some(v) = &self.leading_comments {
            s.write_tag(26)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = &self.trailing_comments {
            s.write_tag(34)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if !self.leading_detached_comments.is_empty() {
            for i in &self.leading_detached_comments {
                s.write_tag(50)?;
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
        if !self.path.is_empty() {
            l += 1 + VarintArray::len(&self.path);
        }
        if !self.span.is_empty() {
            l += 1 + VarintArray::len(&self.span);
        }
        if let Some(v) = &self.leading_comments {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = &self.trailing_comments {
            l += 1 + LengthPrefixed::len(v);
        }
        if !self.leading_detached_comments.is_empty() {
            l += self.leading_detached_comments.len() as u64
                + LengthPrefixedArray::len(&self.leading_detached_comments);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for SourceCodeInfo_Location {
    fn default_instance() -> &'static SourceCodeInfo_Location {
        static DEFAULT: SourceCodeInfo_Location = SourceCodeInfo_Location::new();
        &DEFAULT
    }
}
impl Default for SourceCodeInfo_Location {
    #[inline]
    fn default() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SourceCodeInfo {
    pub location: Vec<SourceCodeInfo_Location>,
    _unknown: Vec<u8>,
}
impl SourceCodeInfo {
    pub const fn new() -> SourceCodeInfo {
        SourceCodeInfo {
            location: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for SourceCodeInfo {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.location, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.location.is_empty() {
            for i in &self.location {
                s.write_tag(10)?;
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
        if !self.location.is_empty() {
            l += self.location.len() as u64 + LengthPrefixedArray::len(&self.location);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for SourceCodeInfo {
    fn default_instance() -> &'static SourceCodeInfo {
        static DEFAULT: SourceCodeInfo = SourceCodeInfo::new();
        &DEFAULT
    }
}
impl Default for SourceCodeInfo {
    #[inline]
    fn default() -> SourceCodeInfo {
        SourceCodeInfo::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedCodeInfo_Annotation {
    pub path: Vec<i32>,
    pub source_file: Option<String>,
    pub begin: Option<i32>,
    pub end: Option<i32>,
    _unknown: Vec<u8>,
}
impl GeneratedCodeInfo_Annotation {
    pub const fn new() -> GeneratedCodeInfo_Annotation {
        GeneratedCodeInfo_Annotation {
            path: Vec::new(),
            source_file: None,
            begin: None,
            end: None,
            _unknown: Vec::new(),
        }
    }
    pub fn source_file(&self) -> &String {
        match &self.source_file {
            Some(v) => v,
            None => String::default_instance(),
        }
    }
    pub fn source_file_mut(&mut self) -> &mut String {
        if self.source_file.is_none() {
            self.source_file = Some(String::new());
        }
        self.source_file.as_mut().unwrap()
    }
    pub fn set_source_file(&mut self, val: String) {
        self.source_file = Some(val);
    }
    pub fn begin(&self) -> i32 {
        self.begin.unwrap_or_default()
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        if self.begin.is_none() {
            self.begin = Some(0);
        }
        self.begin.as_mut().unwrap()
    }
    pub fn set_begin(&mut self, val: i32) {
        self.begin = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        }
        self.end.as_mut().unwrap()
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl pecan::Message for GeneratedCodeInfo_Annotation {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => VarintArray::merge_from(&mut self.path, s)?,
                18 => self.source_file = Some(LengthPrefixed::read_from(s)?),
                24 => self.begin = Some(Varint::read_from(s)?),
                32 => self.end = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.path.is_empty() {
            s.write_tag(10)?;
            VarintArray::write_to(&self.path, s)?;
        }
        if let Some(v) = &self.source_file {
            s.write_tag(18)?;
            LengthPrefixed::write_to(v, s)?;
        }
        if let Some(v) = self.begin {
            s.write_tag(24)?;
            Varint::write_to(v, s)?;
        }
        if let Some(v) = self.end {
            s.write_tag(32)?;
            Varint::write_to(v, s)?;
        }
        if !self._unknown.is_empty() {
            s.write_raw_bytes(&self._unknown)?;
        }
        Ok(())
    }
    fn len(&self) -> u64 {
        let mut l = 0;
        if !self.path.is_empty() {
            l += 1 + VarintArray::len(&self.path);
        }
        if let Some(v) = &self.source_file {
            l += 1 + LengthPrefixed::len(v);
        }
        if let Some(v) = self.begin {
            l += 1 + Varint::len(v);
        }
        if let Some(v) = self.end {
            l += 1 + Varint::len(v);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for GeneratedCodeInfo_Annotation {
    fn default_instance() -> &'static GeneratedCodeInfo_Annotation {
        static DEFAULT: GeneratedCodeInfo_Annotation = GeneratedCodeInfo_Annotation::new();
        &DEFAULT
    }
}
impl Default for GeneratedCodeInfo_Annotation {
    #[inline]
    fn default() -> GeneratedCodeInfo_Annotation {
        GeneratedCodeInfo_Annotation::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedCodeInfo {
    pub annotation: Vec<GeneratedCodeInfo_Annotation>,
    _unknown: Vec<u8>,
}
impl GeneratedCodeInfo {
    pub const fn new() -> GeneratedCodeInfo {
        GeneratedCodeInfo {
            annotation: Vec::new(),
            _unknown: Vec::new(),
        }
    }
}
impl pecan::Message for GeneratedCodeInfo {
    fn merge_from<B: pecan::Buf>(&mut self, s: &mut CodedInputStream<B>) -> pecan::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.annotation, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: pecan::BufMut>(&self, s: &mut CodedOutputStream<B>) -> pecan::Result<()> {
        if !self.annotation.is_empty() {
            for i in &self.annotation {
                s.write_tag(10)?;
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
        if !self.annotation.is_empty() {
            l += self.annotation.len() as u64 + LengthPrefixedArray::len(&self.annotation);
        }
        if !self._unknown.is_empty() {
            l += self._unknown.len() as u64;
        }
        l
    }
}
impl pecan::DefaultInstance for GeneratedCodeInfo {
    fn default_instance() -> &'static GeneratedCodeInfo {
        static DEFAULT: GeneratedCodeInfo = GeneratedCodeInfo::new();
        &DEFAULT
    }
}
impl Default for GeneratedCodeInfo {
    #[inline]
    fn default() -> GeneratedCodeInfo {
        GeneratedCodeInfo::new()
    }
}
