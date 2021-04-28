#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::prelude::*;
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
impl crate::Message for FileDescriptorSet {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.file, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for FileDescriptorSet {
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
            public_dependency: Vec::new(),
            weak_dependency: Vec::new(),
            message_type: Vec::new(),
            enum_type: Vec::new(),
            service: Vec::new(),
            extension: Vec::new(),
            options: None,
            source_code_info: None,
            syntax: None,
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
    pub fn package(&self) -> &String {
        match &self.package {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn package_mut(&mut self) -> &mut String {
        self.package.get_or_insert_with(Default::default)
    }
    pub fn set_package(&mut self, val: String) {
        self.package = Some(val);
    }
    pub fn options(&self) -> &FileOptions {
        match &self.options {
            Some(v) => v,
            _ => FileOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut FileOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: FileOptions) {
        self.options = Some(val);
    }
    pub fn source_code_info(&self) -> &SourceCodeInfo {
        match &self.source_code_info {
            Some(v) => v,
            _ => SourceCodeInfo::default_instance(),
        }
    }
    pub fn source_code_info_mut(&mut self) -> &mut SourceCodeInfo {
        self.source_code_info.get_or_insert_with(Default::default)
    }
    pub fn set_source_code_info(&mut self, val: SourceCodeInfo) {
        self.source_code_info = Some(val);
    }
    pub fn syntax(&self) -> &String {
        match &self.syntax {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn syntax_mut(&mut self) -> &mut String {
        self.syntax.get_or_insert_with(Default::default)
    }
    pub fn set_syntax(&mut self, val: String) {
        self.syntax = Some(val);
    }
}
impl crate::Message for FileDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for FileDescriptorProto {
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
        self.start.get_or_insert_with(Default::default)
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        self.end.get_or_insert_with(Default::default)
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
    pub fn options(&self) -> &ExtensionRangeOptions {
        match &self.options {
            Some(v) => v,
            _ => ExtensionRangeOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut ExtensionRangeOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: ExtensionRangeOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for DescriptorProto_ExtensionRange {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for DescriptorProto_ExtensionRange {
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
        self.start.get_or_insert_with(Default::default)
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        self.end.get_or_insert_with(Default::default)
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl crate::Message for DescriptorProto_ReservedRange {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.start = Some(Varint::read_from(s)?),
                16 => self.end = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for DescriptorProto_ReservedRange {
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
            extension: Vec::new(),
            nested_type: Vec::new(),
            enum_type: Vec::new(),
            extension_range: Vec::new(),
            oneof_decl: Vec::new(),
            options: None,
            reserved_range: Vec::new(),
            reserved_name: Vec::new(),
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
    pub fn options(&self) -> &MessageOptions {
        match &self.options {
            Some(v) => v,
            _ => MessageOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut MessageOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: MessageOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for DescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for DescriptorProto {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl ExtensionRangeOptions {
    pub const fn new() -> ExtensionRangeOptions {
        ExtensionRangeOptions {
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
}
impl crate::Message for ExtensionRangeOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for ExtensionRangeOptions {
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
impl crate::Enumerate for FieldDescriptorProto_Type {
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
impl crate::Enumerate for FieldDescriptorProto_Label {
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
            number: None,
            label: None,
            r#type: None,
            type_name: None,
            extendee: None,
            default_value: None,
            oneof_index: None,
            json_name: None,
            options: None,
            proto3_optional: None,
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
    pub fn number(&self) -> i32 {
        self.number.unwrap_or_default()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        self.number.get_or_insert_with(Default::default)
    }
    pub fn set_number(&mut self, val: i32) {
        self.number = Some(val);
    }
    pub fn label(&self) -> FieldDescriptorProto_Label {
        self.label.unwrap_or_default()
    }
    pub fn label_mut(&mut self) -> &mut FieldDescriptorProto_Label {
        self.label.get_or_insert_with(Default::default)
    }
    pub fn set_label(&mut self, val: FieldDescriptorProto_Label) {
        self.label = Some(val);
    }
    pub fn r#type(&self) -> FieldDescriptorProto_Type {
        self.r#type.unwrap_or_default()
    }
    pub fn type_mut(&mut self) -> &mut FieldDescriptorProto_Type {
        self.r#type.get_or_insert_with(Default::default)
    }
    pub fn set_type(&mut self, val: FieldDescriptorProto_Type) {
        self.r#type = Some(val);
    }
    pub fn type_name(&self) -> &String {
        match &self.type_name {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn type_name_mut(&mut self) -> &mut String {
        self.type_name.get_or_insert_with(Default::default)
    }
    pub fn set_type_name(&mut self, val: String) {
        self.type_name = Some(val);
    }
    pub fn extendee(&self) -> &String {
        match &self.extendee {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn extendee_mut(&mut self) -> &mut String {
        self.extendee.get_or_insert_with(Default::default)
    }
    pub fn set_extendee(&mut self, val: String) {
        self.extendee = Some(val);
    }
    pub fn default_value(&self) -> &String {
        match &self.default_value {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn default_value_mut(&mut self) -> &mut String {
        self.default_value.get_or_insert_with(Default::default)
    }
    pub fn set_default_value(&mut self, val: String) {
        self.default_value = Some(val);
    }
    pub fn oneof_index(&self) -> i32 {
        self.oneof_index.unwrap_or_default()
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        self.oneof_index.get_or_insert_with(Default::default)
    }
    pub fn set_oneof_index(&mut self, val: i32) {
        self.oneof_index = Some(val);
    }
    pub fn json_name(&self) -> &String {
        match &self.json_name {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn json_name_mut(&mut self) -> &mut String {
        self.json_name.get_or_insert_with(Default::default)
    }
    pub fn set_json_name(&mut self, val: String) {
        self.json_name = Some(val);
    }
    pub fn options(&self) -> &FieldOptions {
        match &self.options {
            Some(v) => v,
            _ => FieldOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut FieldOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: FieldOptions) {
        self.options = Some(val);
    }
    pub fn proto3_optional(&self) -> bool {
        self.proto3_optional.unwrap_or_default()
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        self.proto3_optional.get_or_insert_with(Default::default)
    }
    pub fn set_proto3_optional(&mut self, val: bool) {
        self.proto3_optional = Some(val);
    }
}
impl crate::Message for FieldDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for FieldDescriptorProto {
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn options(&self) -> &OneofOptions {
        match &self.options {
            Some(v) => v,
            _ => OneofOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut OneofOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: OneofOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for OneofDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name = Some(LengthPrefixed::read_from(s)?),
                18 => LengthPrefixed::merge_from(self.options_mut(), s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for OneofDescriptorProto {
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
        self.start.get_or_insert_with(Default::default)
    }
    pub fn set_start(&mut self, val: i32) {
        self.start = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        self.end.get_or_insert_with(Default::default)
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl crate::Message for EnumDescriptorProto_EnumReservedRange {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                8 => self.start = Some(Varint::read_from(s)?),
                16 => self.end = Some(Varint::read_from(s)?),
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for EnumDescriptorProto_EnumReservedRange {
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn options(&self) -> &EnumOptions {
        match &self.options {
            Some(v) => v,
            _ => EnumOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut EnumOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: EnumOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for EnumDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for EnumDescriptorProto {
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn number(&self) -> i32 {
        self.number.unwrap_or_default()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        self.number.get_or_insert_with(Default::default)
    }
    pub fn set_number(&mut self, val: i32) {
        self.number = Some(val);
    }
    pub fn options(&self) -> &EnumValueOptions {
        match &self.options {
            Some(v) => v,
            _ => EnumValueOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut EnumValueOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: EnumValueOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for EnumValueDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for EnumValueDescriptorProto {
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn options(&self) -> &ServiceOptions {
        match &self.options {
            Some(v) => v,
            _ => ServiceOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut ServiceOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: ServiceOptions) {
        self.options = Some(val);
    }
}
impl crate::Message for ServiceDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for ServiceDescriptorProto {
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
            _ => String::default_instance(),
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        self.name.get_or_insert_with(Default::default)
    }
    pub fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }
    pub fn input_type(&self) -> &String {
        match &self.input_type {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn input_type_mut(&mut self) -> &mut String {
        self.input_type.get_or_insert_with(Default::default)
    }
    pub fn set_input_type(&mut self, val: String) {
        self.input_type = Some(val);
    }
    pub fn output_type(&self) -> &String {
        match &self.output_type {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn output_type_mut(&mut self) -> &mut String {
        self.output_type.get_or_insert_with(Default::default)
    }
    pub fn set_output_type(&mut self, val: String) {
        self.output_type = Some(val);
    }
    pub fn options(&self) -> &MethodOptions {
        match &self.options {
            Some(v) => v,
            _ => MethodOptions::default_instance(),
        }
    }
    pub fn options_mut(&mut self) -> &mut MethodOptions {
        self.options.get_or_insert_with(Default::default)
    }
    pub fn set_options(&mut self, val: MethodOptions) {
        self.options = Some(val);
    }
    pub fn client_streaming(&self) -> bool {
        self.client_streaming.unwrap_or_default()
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        self.client_streaming.get_or_insert_with(Default::default)
    }
    pub fn set_client_streaming(&mut self, val: bool) {
        self.client_streaming = Some(val);
    }
    pub fn server_streaming(&self) -> bool {
        self.server_streaming.unwrap_or_default()
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        self.server_streaming.get_or_insert_with(Default::default)
    }
    pub fn set_server_streaming(&mut self, val: bool) {
        self.server_streaming = Some(val);
    }
}
impl crate::Message for MethodDescriptorProto {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for MethodDescriptorProto {
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
impl crate::Enumerate for FileOptions_OptimizeMode {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl FileOptions {
    pub const fn new() -> FileOptions {
        FileOptions {
            java_package: None,
            java_outer_classname: None,
            java_multiple_files: None,
            java_generate_equals_and_hash: None,
            java_string_check_utf8: None,
            optimize_for: None,
            go_package: None,
            cc_generic_services: None,
            java_generic_services: None,
            py_generic_services: None,
            php_generic_services: None,
            deprecated: None,
            cc_enable_arenas: None,
            objc_class_prefix: None,
            csharp_namespace: None,
            swift_prefix: None,
            php_class_prefix: None,
            php_namespace: None,
            php_metadata_namespace: None,
            ruby_package: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn java_package(&self) -> &String {
        match &self.java_package {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn java_package_mut(&mut self) -> &mut String {
        self.java_package.get_or_insert_with(Default::default)
    }
    pub fn set_java_package(&mut self, val: String) {
        self.java_package = Some(val);
    }
    pub fn java_outer_classname(&self) -> &String {
        match &self.java_outer_classname {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut String {
        self.java_outer_classname
            .get_or_insert_with(Default::default)
    }
    pub fn set_java_outer_classname(&mut self, val: String) {
        self.java_outer_classname = Some(val);
    }
    pub fn java_multiple_files(&self) -> bool {
        self.java_multiple_files.unwrap_or_default()
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        self.java_multiple_files
            .get_or_insert_with(Default::default)
    }
    pub fn set_java_multiple_files(&mut self, val: bool) {
        self.java_multiple_files = Some(val);
    }
    pub fn java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.unwrap_or_default()
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        self.java_generate_equals_and_hash
            .get_or_insert_with(Default::default)
    }
    pub fn set_java_generate_equals_and_hash(&mut self, val: bool) {
        self.java_generate_equals_and_hash = Some(val);
    }
    pub fn java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.unwrap_or_default()
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        self.java_string_check_utf8
            .get_or_insert_with(Default::default)
    }
    pub fn set_java_string_check_utf8(&mut self, val: bool) {
        self.java_string_check_utf8 = Some(val);
    }
    pub fn optimize_for(&self) -> FileOptions_OptimizeMode {
        self.optimize_for.unwrap_or_default()
    }
    pub fn optimize_for_mut(&mut self) -> &mut FileOptions_OptimizeMode {
        self.optimize_for.get_or_insert_with(Default::default)
    }
    pub fn set_optimize_for(&mut self, val: FileOptions_OptimizeMode) {
        self.optimize_for = Some(val);
    }
    pub fn go_package(&self) -> &String {
        match &self.go_package {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn go_package_mut(&mut self) -> &mut String {
        self.go_package.get_or_insert_with(Default::default)
    }
    pub fn set_go_package(&mut self, val: String) {
        self.go_package = Some(val);
    }
    pub fn cc_generic_services(&self) -> bool {
        self.cc_generic_services.unwrap_or_default()
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        self.cc_generic_services
            .get_or_insert_with(Default::default)
    }
    pub fn set_cc_generic_services(&mut self, val: bool) {
        self.cc_generic_services = Some(val);
    }
    pub fn java_generic_services(&self) -> bool {
        self.java_generic_services.unwrap_or_default()
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        self.java_generic_services
            .get_or_insert_with(Default::default)
    }
    pub fn set_java_generic_services(&mut self, val: bool) {
        self.java_generic_services = Some(val);
    }
    pub fn py_generic_services(&self) -> bool {
        self.py_generic_services.unwrap_or_default()
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        self.py_generic_services
            .get_or_insert_with(Default::default)
    }
    pub fn set_py_generic_services(&mut self, val: bool) {
        self.py_generic_services = Some(val);
    }
    pub fn php_generic_services(&self) -> bool {
        self.php_generic_services.unwrap_or_default()
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        self.php_generic_services
            .get_or_insert_with(Default::default)
    }
    pub fn set_php_generic_services(&mut self, val: bool) {
        self.php_generic_services = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.unwrap_or_default()
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        self.cc_enable_arenas.get_or_insert_with(Default::default)
    }
    pub fn set_cc_enable_arenas(&mut self, val: bool) {
        self.cc_enable_arenas = Some(val);
    }
    pub fn objc_class_prefix(&self) -> &String {
        match &self.objc_class_prefix {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut String {
        self.objc_class_prefix.get_or_insert_with(Default::default)
    }
    pub fn set_objc_class_prefix(&mut self, val: String) {
        self.objc_class_prefix = Some(val);
    }
    pub fn csharp_namespace(&self) -> &String {
        match &self.csharp_namespace {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut String {
        self.csharp_namespace.get_or_insert_with(Default::default)
    }
    pub fn set_csharp_namespace(&mut self, val: String) {
        self.csharp_namespace = Some(val);
    }
    pub fn swift_prefix(&self) -> &String {
        match &self.swift_prefix {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn swift_prefix_mut(&mut self) -> &mut String {
        self.swift_prefix.get_or_insert_with(Default::default)
    }
    pub fn set_swift_prefix(&mut self, val: String) {
        self.swift_prefix = Some(val);
    }
    pub fn php_class_prefix(&self) -> &String {
        match &self.php_class_prefix {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut String {
        self.php_class_prefix.get_or_insert_with(Default::default)
    }
    pub fn set_php_class_prefix(&mut self, val: String) {
        self.php_class_prefix = Some(val);
    }
    pub fn php_namespace(&self) -> &String {
        match &self.php_namespace {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn php_namespace_mut(&mut self) -> &mut String {
        self.php_namespace.get_or_insert_with(Default::default)
    }
    pub fn set_php_namespace(&mut self, val: String) {
        self.php_namespace = Some(val);
    }
    pub fn php_metadata_namespace(&self) -> &String {
        match &self.php_metadata_namespace {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut String {
        self.php_metadata_namespace
            .get_or_insert_with(Default::default)
    }
    pub fn set_php_metadata_namespace(&mut self, val: String) {
        self.php_metadata_namespace = Some(val);
    }
    pub fn ruby_package(&self) -> &String {
        match &self.ruby_package {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn ruby_package_mut(&mut self) -> &mut String {
        self.ruby_package.get_or_insert_with(Default::default)
    }
    pub fn set_ruby_package(&mut self, val: String) {
        self.ruby_package = Some(val);
    }
}
impl crate::Message for FileOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for FileOptions {
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
    pub extensions: crate::ExtensionMap,
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
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.unwrap_or_default()
    }
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        self.message_set_wire_format
            .get_or_insert_with(Default::default)
    }
    pub fn set_message_set_wire_format(&mut self, val: bool) {
        self.message_set_wire_format = Some(val);
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.unwrap_or_default()
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        self.no_standard_descriptor_accessor
            .get_or_insert_with(Default::default)
    }
    pub fn set_no_standard_descriptor_accessor(&mut self, val: bool) {
        self.no_standard_descriptor_accessor = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn map_entry(&self) -> bool {
        self.map_entry.unwrap_or_default()
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        self.map_entry.get_or_insert_with(Default::default)
    }
    pub fn set_map_entry(&mut self, val: bool) {
        self.map_entry = Some(val);
    }
}
impl crate::Message for MessageOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for MessageOptions {
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
impl crate::Enumerate for FieldOptions_CType {
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
impl crate::Enumerate for FieldOptions_JsType {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl FieldOptions {
    pub const fn new() -> FieldOptions {
        FieldOptions {
            ctype: None,
            packed: None,
            jstype: None,
            lazy: None,
            deprecated: None,
            weak: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn ctype(&self) -> FieldOptions_CType {
        self.ctype.unwrap_or_default()
    }
    pub fn ctype_mut(&mut self) -> &mut FieldOptions_CType {
        self.ctype.get_or_insert_with(Default::default)
    }
    pub fn set_ctype(&mut self, val: FieldOptions_CType) {
        self.ctype = Some(val);
    }
    pub fn packed(&self) -> bool {
        self.packed.unwrap_or_default()
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        self.packed.get_or_insert_with(Default::default)
    }
    pub fn set_packed(&mut self, val: bool) {
        self.packed = Some(val);
    }
    pub fn jstype(&self) -> FieldOptions_JsType {
        self.jstype.unwrap_or_default()
    }
    pub fn jstype_mut(&mut self) -> &mut FieldOptions_JsType {
        self.jstype.get_or_insert_with(Default::default)
    }
    pub fn set_jstype(&mut self, val: FieldOptions_JsType) {
        self.jstype = Some(val);
    }
    pub fn lazy(&self) -> bool {
        self.lazy.unwrap_or_default()
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        self.lazy.get_or_insert_with(Default::default)
    }
    pub fn set_lazy(&mut self, val: bool) {
        self.lazy = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn weak(&self) -> bool {
        self.weak.unwrap_or_default()
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        self.weak.get_or_insert_with(Default::default)
    }
    pub fn set_weak(&mut self, val: bool) {
        self.weak = Some(val);
    }
}
impl crate::Message for FieldOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for FieldOptions {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl OneofOptions {
    pub const fn new() -> OneofOptions {
        OneofOptions {
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
}
impl crate::Message for OneofOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for OneofOptions {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl EnumOptions {
    pub const fn new() -> EnumOptions {
        EnumOptions {
            allow_alias: None,
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn allow_alias(&self) -> bool {
        self.allow_alias.unwrap_or_default()
    }
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        self.allow_alias.get_or_insert_with(Default::default)
    }
    pub fn set_allow_alias(&mut self, val: bool) {
        self.allow_alias = Some(val);
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl crate::Message for EnumOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for EnumOptions {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl EnumValueOptions {
    pub const fn new() -> EnumValueOptions {
        EnumValueOptions {
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl crate::Message for EnumValueOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for EnumValueOptions {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl ServiceOptions {
    pub const fn new() -> ServiceOptions {
        ServiceOptions {
            deprecated: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
}
impl crate::Message for ServiceOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for ServiceOptions {
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
impl crate::Enumerate for MethodOptions_IdempotencyLevel {
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
    pub extensions: crate::ExtensionMap,
    _unknown: Vec<u8>,
}
impl MethodOptions {
    pub const fn new() -> MethodOptions {
        MethodOptions {
            deprecated: None,
            idempotency_level: None,
            uninterpreted_option: Vec::new(),
            extensions: crate::ExtensionMap::new(),
            _unknown: Vec::new(),
        }
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or_default()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        self.deprecated.get_or_insert_with(Default::default)
    }
    pub fn set_deprecated(&mut self, val: bool) {
        self.deprecated = Some(val);
    }
    pub fn idempotency_level(&self) -> MethodOptions_IdempotencyLevel {
        self.idempotency_level.unwrap_or_default()
    }
    pub fn idempotency_level_mut(&mut self) -> &mut MethodOptions_IdempotencyLevel {
        self.idempotency_level.get_or_insert_with(Default::default)
    }
    pub fn set_idempotency_level(&mut self, val: MethodOptions_IdempotencyLevel) {
        self.idempotency_level = Some(val);
    }
}
impl crate::Message for MethodOptions {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for MethodOptions {
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
impl crate::Message for UninterpretedOption_NamePart {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                10 => self.name_part = LengthPrefixed::read_from(s)?,
                16 => self.is_extension = Varint::read_from(s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for UninterpretedOption_NamePart {
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
    pub string_value: Option<crate::Bytes>,
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
            _ => String::default_instance(),
        }
    }
    pub fn identifier_value_mut(&mut self) -> &mut String {
        self.identifier_value.get_or_insert_with(Default::default)
    }
    pub fn set_identifier_value(&mut self, val: String) {
        self.identifier_value = Some(val);
    }
    pub fn positive_int_value(&self) -> u64 {
        self.positive_int_value.unwrap_or_default()
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        self.positive_int_value.get_or_insert_with(Default::default)
    }
    pub fn set_positive_int_value(&mut self, val: u64) {
        self.positive_int_value = Some(val);
    }
    pub fn negative_int_value(&self) -> i64 {
        self.negative_int_value.unwrap_or_default()
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        self.negative_int_value.get_or_insert_with(Default::default)
    }
    pub fn set_negative_int_value(&mut self, val: i64) {
        self.negative_int_value = Some(val);
    }
    pub fn double_value(&self) -> f64 {
        self.double_value.unwrap_or_default()
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        self.double_value.get_or_insert_with(Default::default)
    }
    pub fn set_double_value(&mut self, val: f64) {
        self.double_value = Some(val);
    }
    pub fn string_value(&self) -> &crate::Bytes {
        match &self.string_value {
            Some(v) => v,
            _ => crate::Bytes::default_instance(),
        }
    }
    pub fn string_value_mut(&mut self) -> &mut crate::Bytes {
        self.string_value.get_or_insert_with(Default::default)
    }
    pub fn set_string_value(&mut self, val: crate::Bytes) {
        self.string_value = Some(val);
    }
    pub fn aggregate_value(&self) -> &String {
        match &self.aggregate_value {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn aggregate_value_mut(&mut self) -> &mut String {
        self.aggregate_value.get_or_insert_with(Default::default)
    }
    pub fn set_aggregate_value(&mut self, val: String) {
        self.aggregate_value = Some(val);
    }
}
impl crate::Message for UninterpretedOption {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for UninterpretedOption {
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
            _ => String::default_instance(),
        }
    }
    pub fn leading_comments_mut(&mut self) -> &mut String {
        self.leading_comments.get_or_insert_with(Default::default)
    }
    pub fn set_leading_comments(&mut self, val: String) {
        self.leading_comments = Some(val);
    }
    pub fn trailing_comments(&self) -> &String {
        match &self.trailing_comments {
            Some(v) => v,
            _ => String::default_instance(),
        }
    }
    pub fn trailing_comments_mut(&mut self) -> &mut String {
        self.trailing_comments.get_or_insert_with(Default::default)
    }
    pub fn set_trailing_comments(&mut self, val: String) {
        self.trailing_comments = Some(val);
    }
}
impl crate::Message for SourceCodeInfo_Location {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for SourceCodeInfo_Location {
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
impl crate::Message for SourceCodeInfo {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.location, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for SourceCodeInfo {
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
            _ => String::default_instance(),
        }
    }
    pub fn source_file_mut(&mut self) -> &mut String {
        self.source_file.get_or_insert_with(Default::default)
    }
    pub fn set_source_file(&mut self, val: String) {
        self.source_file = Some(val);
    }
    pub fn begin(&self) -> i32 {
        self.begin.unwrap_or_default()
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        self.begin.get_or_insert_with(Default::default)
    }
    pub fn set_begin(&mut self, val: i32) {
        self.begin = Some(val);
    }
    pub fn end(&self) -> i32 {
        self.end.unwrap_or_default()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        self.end.get_or_insert_with(Default::default)
    }
    pub fn set_end(&mut self, val: i32) {
        self.end = Some(val);
    }
}
impl crate::Message for GeneratedCodeInfo_Annotation {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
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
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for GeneratedCodeInfo_Annotation {
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
impl crate::Message for GeneratedCodeInfo {
    fn merge_from<B: crate::Buf>(&mut self, s: &mut CodedInputStream<B>) -> crate::Result<()> {
        loop {
            match s.read_tag()? {
                10 => LengthPrefixedArray::merge_from(&mut self.annotation, s)?,
                0 => return Ok(()),
                tag => s.read_unknown_field(tag, &mut self._unknown)?,
            }
        }
    }
    fn write_to<B: crate::BufMut>(&self, s: &mut CodedOutputStream<B>) -> crate::Result<()> {
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
impl crate::DefaultInstance for GeneratedCodeInfo {
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
static DESCRIPTOR_RAW : & [u8] = b"\n google/protobuf/descriptor.proto\x12\x0Fgoogle.protobuf\"M\n\x11FileDescriptorSet\x128\n\x04file\x18\x01 \x03(\x0B2$.google.protobuf.FileDescriptorProtoR\x04file\"\xE4\x04\n\x13FileDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x18\n\x07package\x18\x02 \x01(\tR\x07package\x12\x1E\n\ndependency\x18\x03 \x03(\tR\ndependency\x12+\n\x11public_dependency\x18\n \x03(\x05R\x10publicDependency\x12'\n\x0Fweak_dependency\x18\x0B \x03(\x05R\x0EweakDependency\x12C\n\x0Cmessage_type\x18\x04 \x03(\x0B2 .google.protobuf.DescriptorProtoR\x0BmessageType\x12A\n\tenum_type\x18\x05 \x03(\x0B2$.google.protobuf.EnumDescriptorProtoR\x08enumType\x12A\n\x07service\x18\x06 \x03(\x0B2'.google.protobuf.ServiceDescriptorProtoR\x07service\x12C\n\textension\x18\x07 \x03(\x0B2%.google.protobuf.FieldDescriptorProtoR\textension\x126\n\x07options\x18\x08 \x01(\x0B2\x1C.google.protobuf.FileOptionsR\x07options\x12I\n\x10source_code_info\x18\t \x01(\x0B2\x1F.google.protobuf.SourceCodeInfoR\x0EsourceCodeInfo\x12\x16\n\x06syntax\x18\x0C \x01(\tR\x06syntax\"\xB9\x06\n\x0FDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12;\n\x05field\x18\x02 \x03(\x0B2%.google.protobuf.FieldDescriptorProtoR\x05field\x12C\n\textension\x18\x06 \x03(\x0B2%.google.protobuf.FieldDescriptorProtoR\textension\x12A\n\x0Bnested_type\x18\x03 \x03(\x0B2 .google.protobuf.DescriptorProtoR\nnestedType\x12A\n\tenum_type\x18\x04 \x03(\x0B2$.google.protobuf.EnumDescriptorProtoR\x08enumType\x12X\n\x0Fextension_range\x18\x05 \x03(\x0B2/.google.protobuf.DescriptorProto.ExtensionRangeR\x0EextensionRange\x12D\n\noneof_decl\x18\x08 \x03(\x0B2%.google.protobuf.OneofDescriptorProtoR\toneofDecl\x129\n\x07options\x18\x07 \x01(\x0B2\x1F.google.protobuf.MessageOptionsR\x07options\x12U\n\x0Ereserved_range\x18\t \x03(\x0B2..google.protobuf.DescriptorProto.ReservedRangeR\rreservedRange\x12#\n\rreserved_name\x18\n \x03(\tR\x0CreservedName\x1Az\n\x0EExtensionRange\x12\x14\n\x05start\x18\x01 \x01(\x05R\x05start\x12\x10\n\x03end\x18\x02 \x01(\x05R\x03end\x12@\n\x07options\x18\x03 \x01(\x0B2&.google.protobuf.ExtensionRangeOptionsR\x07options\x1A7\n\rReservedRange\x12\x14\n\x05start\x18\x01 \x01(\x05R\x05start\x12\x10\n\x03end\x18\x02 \x01(\x05R\x03end\"|\n\x15ExtensionRangeOptions\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02\"\xC1\x06\n\x14FieldDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x16\n\x06number\x18\x03 \x01(\x05R\x06number\x12A\n\x05label\x18\x04 \x01(\x0E2+.google.protobuf.FieldDescriptorProto.LabelR\x05label\x12>\n\x04type\x18\x05 \x01(\x0E2*.google.protobuf.FieldDescriptorProto.TypeR\x04type\x12\x1B\n\ttype_name\x18\x06 \x01(\tR\x08typeName\x12\x1A\n\x08extendee\x18\x02 \x01(\tR\x08extendee\x12#\n\rdefault_value\x18\x07 \x01(\tR\x0CdefaultValue\x12\x1F\n\x0Boneof_index\x18\t \x01(\x05R\noneofIndex\x12\x1B\n\tjson_name\x18\n \x01(\tR\x08jsonName\x127\n\x07options\x18\x08 \x01(\x0B2\x1D.google.protobuf.FieldOptionsR\x07options\x12'\n\x0Fproto3_optional\x18\x11 \x01(\x08R\x0Eproto3Optional\"\xB6\x02\n\x04Type\x12\x0F\n\x0BTYPE_DOUBLE\x10\x01\x12\x0E\n\nTYPE_FLOAT\x10\x02\x12\x0E\n\nTYPE_INT64\x10\x03\x12\x0F\n\x0BTYPE_UINT64\x10\x04\x12\x0E\n\nTYPE_INT32\x10\x05\x12\x10\n\x0CTYPE_FIXED64\x10\x06\x12\x10\n\x0CTYPE_FIXED32\x10\x07\x12\r\n\tTYPE_BOOL\x10\x08\x12\x0F\n\x0BTYPE_STRING\x10\t\x12\x0E\n\nTYPE_GROUP\x10\n\x12\x10\n\x0CTYPE_MESSAGE\x10\x0B\x12\x0E\n\nTYPE_BYTES\x10\x0C\x12\x0F\n\x0BTYPE_UINT32\x10\r\x12\r\n\tTYPE_ENUM\x10\x0E\x12\x11\n\rTYPE_SFIXED32\x10\x0F\x12\x11\n\rTYPE_SFIXED64\x10\x10\x12\x0F\n\x0BTYPE_SINT32\x10\x11\x12\x0F\n\x0BTYPE_SINT64\x10\x12\"C\n\x05Label\x12\x12\n\x0ELABEL_OPTIONAL\x10\x01\x12\x12\n\x0ELABEL_REQUIRED\x10\x02\x12\x12\n\x0ELABEL_REPEATED\x10\x03\"c\n\x14OneofDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x127\n\x07options\x18\x02 \x01(\x0B2\x1D.google.protobuf.OneofOptionsR\x07options\"\xE3\x02\n\x13EnumDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12?\n\x05value\x18\x02 \x03(\x0B2).google.protobuf.EnumValueDescriptorProtoR\x05value\x126\n\x07options\x18\x03 \x01(\x0B2\x1C.google.protobuf.EnumOptionsR\x07options\x12]\n\x0Ereserved_range\x18\x04 \x03(\x0B26.google.protobuf.EnumDescriptorProto.EnumReservedRangeR\rreservedRange\x12#\n\rreserved_name\x18\x05 \x03(\tR\x0CreservedName\x1A;\n\x11EnumReservedRange\x12\x14\n\x05start\x18\x01 \x01(\x05R\x05start\x12\x10\n\x03end\x18\x02 \x01(\x05R\x03end\"\x83\x01\n\x18EnumValueDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x16\n\x06number\x18\x02 \x01(\x05R\x06number\x12;\n\x07options\x18\x03 \x01(\x0B2!.google.protobuf.EnumValueOptionsR\x07options\"\xA7\x01\n\x16ServiceDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12>\n\x06method\x18\x02 \x03(\x0B2&.google.protobuf.MethodDescriptorProtoR\x06method\x129\n\x07options\x18\x03 \x01(\x0B2\x1F.google.protobuf.ServiceOptionsR\x07options\"\x89\x02\n\x15MethodDescriptorProto\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x1D\n\ninput_type\x18\x02 \x01(\tR\tinputType\x12\x1F\n\x0Boutput_type\x18\x03 \x01(\tR\noutputType\x128\n\x07options\x18\x04 \x01(\x0B2\x1E.google.protobuf.MethodOptionsR\x07options\x120\n\x10client_streaming\x18\x05 \x01(\x08:\x05falseR\x0FclientStreaming\x120\n\x10server_streaming\x18\x06 \x01(\x08:\x05falseR\x0FserverStreaming\"\x91\t\n\x0BFileOptions\x12!\n\x0Cjava_package\x18\x01 \x01(\tR\x0BjavaPackage\x120\n\x14java_outer_classname\x18\x08 \x01(\tR\x12javaOuterClassname\x125\n\x13java_multiple_files\x18\n \x01(\x08:\x05falseR\x11javaMultipleFiles\x12D\n\x1Djava_generate_equals_and_hash\x18\x14 \x01(\x08B\x02\x18\x01R\x19javaGenerateEqualsAndHash\x12:\n\x16java_string_check_utf8\x18\x1B \x01(\x08:\x05falseR\x13javaStringCheckUtf8\x12S\n\x0Coptimize_for\x18\t \x01(\x0E2).google.protobuf.FileOptions.OptimizeMode:\x05SPEEDR\x0BoptimizeFor\x12\x1D\n\ngo_package\x18\x0B \x01(\tR\tgoPackage\x125\n\x13cc_generic_services\x18\x10 \x01(\x08:\x05falseR\x11ccGenericServices\x129\n\x15java_generic_services\x18\x11 \x01(\x08:\x05falseR\x13javaGenericServices\x125\n\x13py_generic_services\x18\x12 \x01(\x08:\x05falseR\x11pyGenericServices\x127\n\x14php_generic_services\x18* \x01(\x08:\x05falseR\x12phpGenericServices\x12%\n\ndeprecated\x18\x17 \x01(\x08:\x05falseR\ndeprecated\x12.\n\x10cc_enable_arenas\x18\x1F \x01(\x08:\x04trueR\x0EccEnableArenas\x12*\n\x11objc_class_prefix\x18$ \x01(\tR\x0FobjcClassPrefix\x12)\n\x10csharp_namespace\x18% \x01(\tR\x0FcsharpNamespace\x12!\n\x0Cswift_prefix\x18' \x01(\tR\x0BswiftPrefix\x12(\n\x10php_class_prefix\x18( \x01(\tR\x0EphpClassPrefix\x12#\n\rphp_namespace\x18) \x01(\tR\x0CphpNamespace\x124\n\x16php_metadata_namespace\x18, \x01(\tR\x14phpMetadataNamespace\x12!\n\x0Cruby_package\x18- \x01(\tR\x0BrubyPackage\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption\":\n\x0COptimizeMode\x12\t\n\x05SPEED\x10\x01\x12\r\n\tCODE_SIZE\x10\x02\x12\x10\n\x0CLITE_RUNTIME\x10\x03*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02J\x04\x08&\x10'\"\xE3\x02\n\x0EMessageOptions\x12<\n\x17message_set_wire_format\x18\x01 \x01(\x08:\x05falseR\x14messageSetWireFormat\x12L\n\x1Fno_standard_descriptor_accessor\x18\x02 \x01(\x08:\x05falseR\x1CnoStandardDescriptorAccessor\x12%\n\ndeprecated\x18\x03 \x01(\x08:\x05falseR\ndeprecated\x12\x1B\n\tmap_entry\x18\x07 \x01(\x08R\x08mapEntry\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02J\x04\x08\x04\x10\x05J\x04\x08\x05\x10\x06J\x04\x08\x06\x10\x07J\x04\x08\x08\x10\tJ\x04\x08\t\x10\n\"\xE2\x03\n\x0CFieldOptions\x12A\n\x05ctype\x18\x01 \x01(\x0E2#.google.protobuf.FieldOptions.CType:\x06STRINGR\x05ctype\x12\x16\n\x06packed\x18\x02 \x01(\x08R\x06packed\x12G\n\x06jstype\x18\x06 \x01(\x0E2$.google.protobuf.FieldOptions.JSType:\tJS_NORMALR\x06jstype\x12\x19\n\x04lazy\x18\x05 \x01(\x08:\x05falseR\x04lazy\x12%\n\ndeprecated\x18\x03 \x01(\x08:\x05falseR\ndeprecated\x12\x19\n\x04weak\x18\n \x01(\x08:\x05falseR\x04weak\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption\"/\n\x05CType\x12\n\n\x06STRING\x10\0\x12\x08\n\x04CORD\x10\x01\x12\x10\n\x0CSTRING_PIECE\x10\x02\"5\n\x06JSType\x12\r\n\tJS_NORMAL\x10\0\x12\r\n\tJS_STRING\x10\x01\x12\r\n\tJS_NUMBER\x10\x02*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02J\x04\x08\x04\x10\x05\"s\n\x0COneofOptions\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02\"\xC0\x01\n\x0BEnumOptions\x12\x1F\n\x0Ballow_alias\x18\x02 \x01(\x08R\nallowAlias\x12%\n\ndeprecated\x18\x03 \x01(\x08:\x05falseR\ndeprecated\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02J\x04\x08\x05\x10\x06\"\x9E\x01\n\x10EnumValueOptions\x12%\n\ndeprecated\x18\x01 \x01(\x08:\x05falseR\ndeprecated\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02\"\x9C\x01\n\x0EServiceOptions\x12%\n\ndeprecated\x18! \x01(\x08:\x05falseR\ndeprecated\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02\"\xE0\x02\n\rMethodOptions\x12%\n\ndeprecated\x18! \x01(\x08:\x05falseR\ndeprecated\x12q\n\x11idempotency_level\x18\" \x01(\x0E2/.google.protobuf.MethodOptions.IdempotencyLevel:\x13IDEMPOTENCY_UNKNOWNR\x10idempotencyLevel\x12X\n\x14uninterpreted_option\x18\xE7\x07 \x03(\x0B2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOption\"P\n\x10IdempotencyLevel\x12\x17\n\x13IDEMPOTENCY_UNKNOWN\x10\0\x12\x13\n\x0FNO_SIDE_EFFECTS\x10\x01\x12\x0E\n\nIDEMPOTENT\x10\x02*\t\x08\xE8\x07\x10\x80\x80\x80\x80\x02\"\x9A\x03\n\x13UninterpretedOption\x12A\n\x04name\x18\x02 \x03(\x0B2-.google.protobuf.UninterpretedOption.NamePartR\x04name\x12)\n\x10identifier_value\x18\x03 \x01(\tR\x0FidentifierValue\x12,\n\x12positive_int_value\x18\x04 \x01(\x04R\x10positiveIntValue\x12,\n\x12negative_int_value\x18\x05 \x01(\x03R\x10negativeIntValue\x12!\n\x0Cdouble_value\x18\x06 \x01(\x01R\x0BdoubleValue\x12!\n\x0Cstring_value\x18\x07 \x01(\x0CR\x0BstringValue\x12'\n\x0Faggregate_value\x18\x08 \x01(\tR\x0EaggregateValue\x1AJ\n\x08NamePart\x12\x1B\n\tname_part\x18\x01 \x02(\tR\x08namePart\x12!\n\x0Cis_extension\x18\x02 \x02(\x08R\x0BisExtension\"\xA7\x02\n\x0ESourceCodeInfo\x12D\n\x08location\x18\x01 \x03(\x0B2(.google.protobuf.SourceCodeInfo.LocationR\x08location\x1A\xCE\x01\n\x08Location\x12\x16\n\x04path\x18\x01 \x03(\x05B\x02\x10\x01R\x04path\x12\x16\n\x04span\x18\x02 \x03(\x05B\x02\x10\x01R\x04span\x12)\n\x10leading_comments\x18\x03 \x01(\tR\x0FleadingComments\x12+\n\x11trailing_comments\x18\x04 \x01(\tR\x10trailingComments\x12:\n\x19leading_detached_comments\x18\x06 \x03(\tR\x17leadingDetachedComments\"\xD1\x01\n\x11GeneratedCodeInfo\x12M\n\nannotation\x18\x01 \x03(\x0B2-.google.protobuf.GeneratedCodeInfo.AnnotationR\nannotation\x1Am\n\nAnnotation\x12\x16\n\x04path\x18\x01 \x03(\x05B\x02\x10\x01R\x04path\x12\x1F\n\x0Bsource_file\x18\x02 \x01(\tR\nsourceFile\x12\x14\n\x05begin\x18\x03 \x01(\x05R\x05begin\x12\x10\n\x03end\x18\x04 \x01(\x05R\x03endB~\n\x13com.google.protobufB\x10DescriptorProtosH\x01Z-google.golang.org/protobuf/types/descriptorpb\xF8\x01\x01\xA2\x02\x03GPB\xAA\x02\x1AGoogle.Protobuf.ReflectionJ\x82\xCA\x02\n\x07\x12\x05'\0\x8E\x07\x01\n\xAA\x0F\n\x01\x0C\x12\x03'\0\x122\xC1\x0C Protocol Buffers - Google's data interchange format\n Copyright 2008 Google Inc.  All rights reserved.\n https://developers.google.com/protocol-buffers/\n\n Redistribution and use in source and binary forms, with or without\n modification, are permitted provided that the following conditions are\n met:\n\n     * Redistributions of source code must retain the above copyright\n notice, this list of conditions and the following disclaimer.\n     * Redistributions in binary form must reproduce the above\n copyright notice, this list of conditions and the following disclaimer\n in the documentation and/or other materials provided with the\n distribution.\n     * Neither the name of Google Inc. nor the names of its\n contributors may be used to endorse or promote products derived from\n this software without specific prior written permission.\n\n THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS\n \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT\n LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR\n A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT\n OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,\n SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT\n LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,\n DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY\n THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT\n (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n2\xDB\x02 Author: kenton@google.com (Kenton Varda)\n  Based on original Protocol Buffers design by\n  Sanjay Ghemawat, Jeff Dean, and others.\n\n The messages in this file describe the definitions found in .proto files.\n A valid .proto file can be translated directly to a FileDescriptorProto\n without any other information (e.g. without reading its imports).\n\n\x08\n\x01\x02\x12\x03)\0\x18\n\x08\n\x01\x08\x12\x03+\0D\n\t\n\x02\x08\x0B\x12\x03+\0D\n\x08\n\x01\x08\x12\x03,\0,\n\t\n\x02\x08\x01\x12\x03,\0,\n\x08\n\x01\x08\x12\x03-\01\n\t\n\x02\x08\x08\x12\x03-\01\n\x08\n\x01\x08\x12\x03.\07\n\t\n\x02\x08%\x12\x03.\07\n\x08\n\x01\x08\x12\x03/\0!\n\t\n\x02\x08$\x12\x03/\0!\n\x08\n\x01\x08\x12\x030\0\x1F\n\t\n\x02\x08\x1F\x12\x030\0\x1F\n\x08\n\x01\x08\x12\x034\0\x1C\n\x7F\n\x02\x08\t\x12\x034\0\x1C\x1At descriptor.proto must be optimized for speed because reflection-based\n algorithms don't work during bootstrapping.\n\nj\n\x02\x04\0\x12\x048\0:\x01\x1A^ The protocol compiler can output a FileDescriptorSet containing the .proto\n files it parses.\n\n\n\n\x03\x04\0\x01\x12\x038\x08\x19\n\x0B\n\x04\x04\0\x02\0\x12\x039\x02(\n\x0C\n\x05\x04\0\x02\0\x04\x12\x039\x02\n\n\x0C\n\x05\x04\0\x02\0\x06\x12\x039\x0B\x1E\n\x0C\n\x05\x04\0\x02\0\x01\x12\x039\x1F#\n\x0C\n\x05\x04\0\x02\0\x03\x12\x039&'\n/\n\x02\x04\x01\x12\x04=\0Z\x01\x1A# Describes a complete .proto file.\n\n\n\n\x03\x04\x01\x01\x12\x03=\x08\x1B\n9\n\x04\x04\x01\x02\0\x12\x03>\x02\x1B\", file name, relative to root of source tree\n\n\x0C\n\x05\x04\x01\x02\0\x04\x12\x03>\x02\n\n\x0C\n\x05\x04\x01\x02\0\x05\x12\x03>\x0B\x11\n\x0C\n\x05\x04\x01\x02\0\x01\x12\x03>\x12\x16\n\x0C\n\x05\x04\x01\x02\0\x03\x12\x03>\x19\x1A\n*\n\x04\x04\x01\x02\x01\x12\x03?\x02\x1E\"\x1D e.g. \"foo\", \"foo.bar\", etc.\n\n\x0C\n\x05\x04\x01\x02\x01\x04\x12\x03?\x02\n\n\x0C\n\x05\x04\x01\x02\x01\x05\x12\x03?\x0B\x11\n\x0C\n\x05\x04\x01\x02\x01\x01\x12\x03?\x12\x19\n\x0C\n\x05\x04\x01\x02\x01\x03\x12\x03?\x1C\x1D\n4\n\x04\x04\x01\x02\x02\x12\x03B\x02!\x1A' Names of files imported by this file.\n\n\x0C\n\x05\x04\x01\x02\x02\x04\x12\x03B\x02\n\n\x0C\n\x05\x04\x01\x02\x02\x05\x12\x03B\x0B\x11\n\x0C\n\x05\x04\x01\x02\x02\x01\x12\x03B\x12\x1C\n\x0C\n\x05\x04\x01\x02\x02\x03\x12\x03B\x1F \nQ\n\x04\x04\x01\x02\x03\x12\x03D\x02(\x1AD Indexes of the public imported files in the dependency list above.\n\n\x0C\n\x05\x04\x01\x02\x03\x04\x12\x03D\x02\n\n\x0C\n\x05\x04\x01\x02\x03\x05\x12\x03D\x0B\x10\n\x0C\n\x05\x04\x01\x02\x03\x01\x12\x03D\x11\"\n\x0C\n\x05\x04\x01\x02\x03\x03\x12\x03D%'\nz\n\x04\x04\x01\x02\x04\x12\x03G\x02&\x1Am Indexes of the weak imported files in the dependency list.\n For Google-internal migration only. Do not use.\n\n\x0C\n\x05\x04\x01\x02\x04\x04\x12\x03G\x02\n\n\x0C\n\x05\x04\x01\x02\x04\x05\x12\x03G\x0B\x10\n\x0C\n\x05\x04\x01\x02\x04\x01\x12\x03G\x11 \n\x0C\n\x05\x04\x01\x02\x04\x03\x12\x03G#%\n6\n\x04\x04\x01\x02\x05\x12\x03J\x02,\x1A) All top-level definitions in this file.\n\n\x0C\n\x05\x04\x01\x02\x05\x04\x12\x03J\x02\n\n\x0C\n\x05\x04\x01\x02\x05\x06\x12\x03J\x0B\x1A\n\x0C\n\x05\x04\x01\x02\x05\x01\x12\x03J\x1B'\n\x0C\n\x05\x04\x01\x02\x05\x03\x12\x03J*+\n\x0B\n\x04\x04\x01\x02\x06\x12\x03K\x02-\n\x0C\n\x05\x04\x01\x02\x06\x04\x12\x03K\x02\n\n\x0C\n\x05\x04\x01\x02\x06\x06\x12\x03K\x0B\x1E\n\x0C\n\x05\x04\x01\x02\x06\x01\x12\x03K\x1F(\n\x0C\n\x05\x04\x01\x02\x06\x03\x12\x03K+,\n\x0B\n\x04\x04\x01\x02\x07\x12\x03L\x02.\n\x0C\n\x05\x04\x01\x02\x07\x04\x12\x03L\x02\n\n\x0C\n\x05\x04\x01\x02\x07\x06\x12\x03L\x0B!\n\x0C\n\x05\x04\x01\x02\x07\x01\x12\x03L\")\n\x0C\n\x05\x04\x01\x02\x07\x03\x12\x03L,-\n\x0B\n\x04\x04\x01\x02\x08\x12\x03M\x02.\n\x0C\n\x05\x04\x01\x02\x08\x04\x12\x03M\x02\n\n\x0C\n\x05\x04\x01\x02\x08\x06\x12\x03M\x0B\x1F\n\x0C\n\x05\x04\x01\x02\x08\x01\x12\x03M )\n\x0C\n\x05\x04\x01\x02\x08\x03\x12\x03M,-\n\x0B\n\x04\x04\x01\x02\t\x12\x03O\x02#\n\x0C\n\x05\x04\x01\x02\t\x04\x12\x03O\x02\n\n\x0C\n\x05\x04\x01\x02\t\x06\x12\x03O\x0B\x16\n\x0C\n\x05\x04\x01\x02\t\x01\x12\x03O\x17\x1E\n\x0C\n\x05\x04\x01\x02\t\x03\x12\x03O!\"\n\xF4\x01\n\x04\x04\x01\x02\n\x12\x03U\x02/\x1A\xE6\x01 This field contains optional information about the original source code.\n You may safely remove this entire field without harming runtime\n functionality of the descriptors -- the information is needed only by\n development tools.\n\n\x0C\n\x05\x04\x01\x02\n\x04\x12\x03U\x02\n\n\x0C\n\x05\x04\x01\x02\n\x06\x12\x03U\x0B\x19\n\x0C\n\x05\x04\x01\x02\n\x01\x12\x03U\x1A*\n\x0C\n\x05\x04\x01\x02\n\x03\x12\x03U-.\n]\n\x04\x04\x01\x02\x0B\x12\x03Y\x02\x1E\x1AP The syntax of the proto file.\n The supported values are \"proto2\" and \"proto3\".\n\n\x0C\n\x05\x04\x01\x02\x0B\x04\x12\x03Y\x02\n\n\x0C\n\x05\x04\x01\x02\x0B\x05\x12\x03Y\x0B\x11\n\x0C\n\x05\x04\x01\x02\x0B\x01\x12\x03Y\x12\x18\n\x0C\n\x05\x04\x01\x02\x0B\x03\x12\x03Y\x1B\x1D\n'\n\x02\x04\x02\x12\x04]\0}\x01\x1A\x1B Describes a message type.\n\n\n\n\x03\x04\x02\x01\x12\x03]\x08\x17\n\x0B\n\x04\x04\x02\x02\0\x12\x03^\x02\x1B\n\x0C\n\x05\x04\x02\x02\0\x04\x12\x03^\x02\n\n\x0C\n\x05\x04\x02\x02\0\x05\x12\x03^\x0B\x11\n\x0C\n\x05\x04\x02\x02\0\x01\x12\x03^\x12\x16\n\x0C\n\x05\x04\x02\x02\0\x03\x12\x03^\x19\x1A\n\x0B\n\x04\x04\x02\x02\x01\x12\x03`\x02*\n\x0C\n\x05\x04\x02\x02\x01\x04\x12\x03`\x02\n\n\x0C\n\x05\x04\x02\x02\x01\x06\x12\x03`\x0B\x1F\n\x0C\n\x05\x04\x02\x02\x01\x01\x12\x03` %\n\x0C\n\x05\x04\x02\x02\x01\x03\x12\x03`()\n\x0B\n\x04\x04\x02\x02\x02\x12\x03a\x02.\n\x0C\n\x05\x04\x02\x02\x02\x04\x12\x03a\x02\n\n\x0C\n\x05\x04\x02\x02\x02\x06\x12\x03a\x0B\x1F\n\x0C\n\x05\x04\x02\x02\x02\x01\x12\x03a )\n\x0C\n\x05\x04\x02\x02\x02\x03\x12\x03a,-\n\x0B\n\x04\x04\x02\x02\x03\x12\x03c\x02+\n\x0C\n\x05\x04\x02\x02\x03\x04\x12\x03c\x02\n\n\x0C\n\x05\x04\x02\x02\x03\x06\x12\x03c\x0B\x1A\n\x0C\n\x05\x04\x02\x02\x03\x01\x12\x03c\x1B&\n\x0C\n\x05\x04\x02\x02\x03\x03\x12\x03c)*\n\x0B\n\x04\x04\x02\x02\x04\x12\x03d\x02-\n\x0C\n\x05\x04\x02\x02\x04\x04\x12\x03d\x02\n\n\x0C\n\x05\x04\x02\x02\x04\x06\x12\x03d\x0B\x1E\n\x0C\n\x05\x04\x02\x02\x04\x01\x12\x03d\x1F(\n\x0C\n\x05\x04\x02\x02\x04\x03\x12\x03d+,\n\x0C\n\x04\x04\x02\x03\0\x12\x04f\x02k\x03\n\x0C\n\x05\x04\x02\x03\0\x01\x12\x03f\n\x18\n\x1B\n\x06\x04\x02\x03\0\x02\0\x12\x03g\x04\x1D\"\x0C Inclusive.\n\n\x0E\n\x07\x04\x02\x03\0\x02\0\x04\x12\x03g\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03g\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03g\x13\x18\n\x0E\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03g\x1B\x1C\n\x1B\n\x06\x04\x02\x03\0\x02\x01\x12\x03h\x04\x1B\"\x0C Exclusive.\n\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x03h\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x03h\r\x12\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x03h\x13\x16\n\x0E\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03h\x19\x1A\n\r\n\x06\x04\x02\x03\0\x02\x02\x12\x03j\x04/\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x04\x12\x03j\x04\x0C\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x06\x12\x03j\r\"\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x01\x12\x03j#*\n\x0E\n\x07\x04\x02\x03\0\x02\x02\x03\x12\x03j-.\n\x0B\n\x04\x04\x02\x02\x05\x12\x03l\x02.\n\x0C\n\x05\x04\x02\x02\x05\x04\x12\x03l\x02\n\n\x0C\n\x05\x04\x02\x02\x05\x06\x12\x03l\x0B\x19\n\x0C\n\x05\x04\x02\x02\x05\x01\x12\x03l\x1A)\n\x0C\n\x05\x04\x02\x02\x05\x03\x12\x03l,-\n\x0B\n\x04\x04\x02\x02\x06\x12\x03n\x02/\n\x0C\n\x05\x04\x02\x02\x06\x04\x12\x03n\x02\n\n\x0C\n\x05\x04\x02\x02\x06\x06\x12\x03n\x0B\x1F\n\x0C\n\x05\x04\x02\x02\x06\x01\x12\x03n *\n\x0C\n\x05\x04\x02\x02\x06\x03\x12\x03n-.\n\x0B\n\x04\x04\x02\x02\x07\x12\x03p\x02&\n\x0C\n\x05\x04\x02\x02\x07\x04\x12\x03p\x02\n\n\x0C\n\x05\x04\x02\x02\x07\x06\x12\x03p\x0B\x19\n\x0C\n\x05\x04\x02\x02\x07\x01\x12\x03p\x1A!\n\x0C\n\x05\x04\x02\x02\x07\x03\x12\x03p$%\n\xAA\x01\n\x04\x04\x02\x03\x01\x12\x04u\x02x\x03\x1A\x9B\x01 Range of reserved tag numbers. Reserved tag numbers may not be used by\n fields or extension ranges in the same message. Reserved ranges may\n not overlap.\n\n\x0C\n\x05\x04\x02\x03\x01\x01\x12\x03u\n\x17\n\x1B\n\x06\x04\x02\x03\x01\x02\0\x12\x03v\x04\x1D\"\x0C Inclusive.\n\n\x0E\n\x07\x04\x02\x03\x01\x02\0\x04\x12\x03v\x04\x0C\n\x0E\n\x07\x04\x02\x03\x01\x02\0\x05\x12\x03v\r\x12\n\x0E\n\x07\x04\x02\x03\x01\x02\0\x01\x12\x03v\x13\x18\n\x0E\n\x07\x04\x02\x03\x01\x02\0\x03\x12\x03v\x1B\x1C\n\x1B\n\x06\x04\x02\x03\x01\x02\x01\x12\x03w\x04\x1B\"\x0C Exclusive.\n\n\x0E\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x03w\x04\x0C\n\x0E\n\x07\x04\x02\x03\x01\x02\x01\x05\x12\x03w\r\x12\n\x0E\n\x07\x04\x02\x03\x01\x02\x01\x01\x12\x03w\x13\x16\n\x0E\n\x07\x04\x02\x03\x01\x02\x01\x03\x12\x03w\x19\x1A\n\x0B\n\x04\x04\x02\x02\x08\x12\x03y\x02,\n\x0C\n\x05\x04\x02\x02\x08\x04\x12\x03y\x02\n\n\x0C\n\x05\x04\x02\x02\x08\x06\x12\x03y\x0B\x18\n\x0C\n\x05\x04\x02\x02\x08\x01\x12\x03y\x19'\n\x0C\n\x05\x04\x02\x02\x08\x03\x12\x03y*+\n\x82\x01\n\x04\x04\x02\x02\t\x12\x03|\x02%\x1Au Reserved field names, which may not be used by fields in the same message.\n A given name may only be reserved once.\n\n\x0C\n\x05\x04\x02\x02\t\x04\x12\x03|\x02\n\n\x0C\n\x05\x04\x02\x02\t\x05\x12\x03|\x0B\x11\n\x0C\n\x05\x04\x02\x02\t\x01\x12\x03|\x12\x1F\n\x0C\n\x05\x04\x02\x02\t\x03\x12\x03|\"$\n\x0B\n\x02\x04\x03\x12\x05\x7F\0\x86\x01\x01\n\n\n\x03\x04\x03\x01\x12\x03\x7F\x08\x1D\nO\n\x04\x04\x03\x02\0\x12\x04\x81\x01\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x81\x01\x02\n\n\r\n\x05\x04\x03\x02\0\x06\x12\x04\x81\x01\x0B\x1E\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\x81\x01\x1F3\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\x81\x0169\nZ\n\x03\x04\x03\x05\x12\x04\x85\x01\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x03\x05\0\x12\x04\x85\x01\r\x18\n\r\n\x05\x04\x03\x05\0\x01\x12\x04\x85\x01\r\x11\n\r\n\x05\x04\x03\x05\0\x02\x12\x04\x85\x01\x15\x18\n3\n\x02\x04\x04\x12\x06\x89\x01\0\xEE\x01\x01\x1A% Describes a field within a message.\n\n\x0B\n\x03\x04\x04\x01\x12\x04\x89\x01\x08\x1C\n\x0E\n\x04\x04\x04\x04\0\x12\x06\x8A\x01\x02\xA9\x01\x03\n\r\n\x05\x04\x04\x04\0\x01\x12\x04\x8A\x01\x07\x0B\nS\n\x06\x04\x04\x04\0\x02\0\x12\x04\x8D\x01\x04\x14\x1AC 0 is reserved for errors.\n Order is weird for historical reasons.\n\n\x0F\n\x07\x04\x04\x04\0\x02\0\x01\x12\x04\x8D\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\0\x02\x12\x04\x8D\x01\x12\x13\n\x0E\n\x06\x04\x04\x04\0\x02\x01\x12\x04\x8E\x01\x04\x13\n\x0F\n\x07\x04\x04\x04\0\x02\x01\x01\x12\x04\x8E\x01\x04\x0E\n\x0F\n\x07\x04\x04\x04\0\x02\x01\x02\x12\x04\x8E\x01\x11\x12\nw\n\x06\x04\x04\x04\0\x02\x02\x12\x04\x91\x01\x04\x13\x1Ag Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if\n negative values are likely.\n\n\x0F\n\x07\x04\x04\x04\0\x02\x02\x01\x12\x04\x91\x01\x04\x0E\n\x0F\n\x07\x04\x04\x04\0\x02\x02\x02\x12\x04\x91\x01\x11\x12\n\x0E\n\x06\x04\x04\x04\0\x02\x03\x12\x04\x92\x01\x04\x14\n\x0F\n\x07\x04\x04\x04\0\x02\x03\x01\x12\x04\x92\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\x03\x02\x12\x04\x92\x01\x12\x13\nw\n\x06\x04\x04\x04\0\x02\x04\x12\x04\x95\x01\x04\x13\x1Ag Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if\n negative values are likely.\n\n\x0F\n\x07\x04\x04\x04\0\x02\x04\x01\x12\x04\x95\x01\x04\x0E\n\x0F\n\x07\x04\x04\x04\0\x02\x04\x02\x12\x04\x95\x01\x11\x12\n\x0E\n\x06\x04\x04\x04\0\x02\x05\x12\x04\x96\x01\x04\x15\n\x0F\n\x07\x04\x04\x04\0\x02\x05\x01\x12\x04\x96\x01\x04\x10\n\x0F\n\x07\x04\x04\x04\0\x02\x05\x02\x12\x04\x96\x01\x13\x14\n\x0E\n\x06\x04\x04\x04\0\x02\x06\x12\x04\x97\x01\x04\x15\n\x0F\n\x07\x04\x04\x04\0\x02\x06\x01\x12\x04\x97\x01\x04\x10\n\x0F\n\x07\x04\x04\x04\0\x02\x06\x02\x12\x04\x97\x01\x13\x14\n\x0E\n\x06\x04\x04\x04\0\x02\x07\x12\x04\x98\x01\x04\x12\n\x0F\n\x07\x04\x04\x04\0\x02\x07\x01\x12\x04\x98\x01\x04\r\n\x0F\n\x07\x04\x04\x04\0\x02\x07\x02\x12\x04\x98\x01\x10\x11\n\x0E\n\x06\x04\x04\x04\0\x02\x08\x12\x04\x99\x01\x04\x14\n\x0F\n\x07\x04\x04\x04\0\x02\x08\x01\x12\x04\x99\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\x08\x02\x12\x04\x99\x01\x12\x13\n\xE2\x01\n\x06\x04\x04\x04\0\x02\t\x12\x04\x9E\x01\x04\x14\x1A\xD1\x01 Tag-delimited aggregate.\n Group type is deprecated and not supported in proto3. However, Proto3\n implementations should still be able to parse the group wire format and\n treat group fields as unknown fields.\n\n\x0F\n\x07\x04\x04\x04\0\x02\t\x01\x12\x04\x9E\x01\x04\x0E\n\x0F\n\x07\x04\x04\x04\0\x02\t\x02\x12\x04\x9E\x01\x11\x13\n-\n\x06\x04\x04\x04\0\x02\n\x12\x04\x9F\x01\x04\x16\"\x1D Length-delimited aggregate.\n\n\x0F\n\x07\x04\x04\x04\0\x02\n\x01\x12\x04\x9F\x01\x04\x10\n\x0F\n\x07\x04\x04\x04\0\x02\n\x02\x12\x04\x9F\x01\x13\x15\n#\n\x06\x04\x04\x04\0\x02\x0B\x12\x04\xA2\x01\x04\x14\x1A\x13 New in version 2.\n\n\x0F\n\x07\x04\x04\x04\0\x02\x0B\x01\x12\x04\xA2\x01\x04\x0E\n\x0F\n\x07\x04\x04\x04\0\x02\x0B\x02\x12\x04\xA2\x01\x11\x13\n\x0E\n\x06\x04\x04\x04\0\x02\x0C\x12\x04\xA3\x01\x04\x15\n\x0F\n\x07\x04\x04\x04\0\x02\x0C\x01\x12\x04\xA3\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\x0C\x02\x12\x04\xA3\x01\x12\x14\n\x0E\n\x06\x04\x04\x04\0\x02\r\x12\x04\xA4\x01\x04\x13\n\x0F\n\x07\x04\x04\x04\0\x02\r\x01\x12\x04\xA4\x01\x04\r\n\x0F\n\x07\x04\x04\x04\0\x02\r\x02\x12\x04\xA4\x01\x10\x12\n\x0E\n\x06\x04\x04\x04\0\x02\x0E\x12\x04\xA5\x01\x04\x17\n\x0F\n\x07\x04\x04\x04\0\x02\x0E\x01\x12\x04\xA5\x01\x04\x11\n\x0F\n\x07\x04\x04\x04\0\x02\x0E\x02\x12\x04\xA5\x01\x14\x16\n\x0E\n\x06\x04\x04\x04\0\x02\x0F\x12\x04\xA6\x01\x04\x17\n\x0F\n\x07\x04\x04\x04\0\x02\x0F\x01\x12\x04\xA6\x01\x04\x11\n\x0F\n\x07\x04\x04\x04\0\x02\x0F\x02\x12\x04\xA6\x01\x14\x16\n'\n\x06\x04\x04\x04\0\x02\x10\x12\x04\xA7\x01\x04\x15\"\x17 Uses ZigZag encoding.\n\n\x0F\n\x07\x04\x04\x04\0\x02\x10\x01\x12\x04\xA7\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\x10\x02\x12\x04\xA7\x01\x12\x14\n'\n\x06\x04\x04\x04\0\x02\x11\x12\x04\xA8\x01\x04\x15\"\x17 Uses ZigZag encoding.\n\n\x0F\n\x07\x04\x04\x04\0\x02\x11\x01\x12\x04\xA8\x01\x04\x0F\n\x0F\n\x07\x04\x04\x04\0\x02\x11\x02\x12\x04\xA8\x01\x12\x14\n\x0E\n\x04\x04\x04\x04\x01\x12\x06\xAB\x01\x02\xB0\x01\x03\n\r\n\x05\x04\x04\x04\x01\x01\x12\x04\xAB\x01\x07\x0C\n*\n\x06\x04\x04\x04\x01\x02\0\x12\x04\xAD\x01\x04\x17\x1A\x1A 0 is reserved for errors\n\n\x0F\n\x07\x04\x04\x04\x01\x02\0\x01\x12\x04\xAD\x01\x04\x12\n\x0F\n\x07\x04\x04\x04\x01\x02\0\x02\x12\x04\xAD\x01\x15\x16\n\x0E\n\x06\x04\x04\x04\x01\x02\x01\x12\x04\xAE\x01\x04\x17\n\x0F\n\x07\x04\x04\x04\x01\x02\x01\x01\x12\x04\xAE\x01\x04\x12\n\x0F\n\x07\x04\x04\x04\x01\x02\x01\x02\x12\x04\xAE\x01\x15\x16\n\x0E\n\x06\x04\x04\x04\x01\x02\x02\x12\x04\xAF\x01\x04\x17\n\x0F\n\x07\x04\x04\x04\x01\x02\x02\x01\x12\x04\xAF\x01\x04\x12\n\x0F\n\x07\x04\x04\x04\x01\x02\x02\x02\x12\x04\xAF\x01\x15\x16\n\x0C\n\x04\x04\x04\x02\0\x12\x04\xB2\x01\x02\x1B\n\r\n\x05\x04\x04\x02\0\x04\x12\x04\xB2\x01\x02\n\n\r\n\x05\x04\x04\x02\0\x05\x12\x04\xB2\x01\x0B\x11\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\xB2\x01\x12\x16\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xB2\x01\x19\x1A\n\x0C\n\x04\x04\x04\x02\x01\x12\x04\xB3\x01\x02\x1C\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\xB3\x01\x02\n\n\r\n\x05\x04\x04\x02\x01\x05\x12\x04\xB3\x01\x0B\x10\n\r\n\x05\x04\x04\x02\x01\x01\x12\x04\xB3\x01\x11\x17\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\xB3\x01\x1A\x1B\n\x0C\n\x04\x04\x04\x02\x02\x12\x04\xB4\x01\x02\x1B\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04\xB4\x01\x02\n\n\r\n\x05\x04\x04\x02\x02\x06\x12\x04\xB4\x01\x0B\x10\n\r\n\x05\x04\x04\x02\x02\x01\x12\x04\xB4\x01\x11\x16\n\r\n\x05\x04\x04\x02\x02\x03\x12\x04\xB4\x01\x19\x1A\n\x9C\x01\n\x04\x04\x04\x02\x03\x12\x04\xB8\x01\x02\x19\x1A\x8D\x01 If type_name is set, this need not be set.  If both this and type_name\n are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.\n\n\r\n\x05\x04\x04\x02\x03\x04\x12\x04\xB8\x01\x02\n\n\r\n\x05\x04\x04\x02\x03\x06\x12\x04\xB8\x01\x0B\x0F\n\r\n\x05\x04\x04\x02\x03\x01\x12\x04\xB8\x01\x10\x14\n\r\n\x05\x04\x04\x02\x03\x03\x12\x04\xB8\x01\x17\x18\n\xB7\x02\n\x04\x04\x04\x02\x04\x12\x04\xBF\x01\x02 \x1A\xA8\x02 For message and enum types, this is the name of the type.  If the name\n starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping\n rules are used to find the type (i.e. first the nested types within this\n message are searched, then within the parent, on up to the root\n namespace).\n\n\r\n\x05\x04\x04\x02\x04\x04\x12\x04\xBF\x01\x02\n\n\r\n\x05\x04\x04\x02\x04\x05\x12\x04\xBF\x01\x0B\x11\n\r\n\x05\x04\x04\x02\x04\x01\x12\x04\xBF\x01\x12\x1B\n\r\n\x05\x04\x04\x02\x04\x03\x12\x04\xBF\x01\x1E\x1F\n~\n\x04\x04\x04\x02\x05\x12\x04\xC3\x01\x02\x1F\x1Ap For extensions, this is the name of the type being extended.  It is\n resolved in the same manner as type_name.\n\n\r\n\x05\x04\x04\x02\x05\x04\x12\x04\xC3\x01\x02\n\n\r\n\x05\x04\x04\x02\x05\x05\x12\x04\xC3\x01\x0B\x11\n\r\n\x05\x04\x04\x02\x05\x01\x12\x04\xC3\x01\x12\x1A\n\r\n\x05\x04\x04\x02\x05\x03\x12\x04\xC3\x01\x1D\x1E\n\xB1\x02\n\x04\x04\x04\x02\x06\x12\x04\xCA\x01\x02$\x1A\xA2\x02 For numeric types, contains the original text representation of the value.\n For booleans, \"true\" or \"false\".\n For strings, contains the default text contents (not escaped in any way).\n For bytes, contains the C escaped value.  All bytes >= 128 are escaped.\n TODO(kenton):  Base-64 encode?\n\n\r\n\x05\x04\x04\x02\x06\x04\x12\x04\xCA\x01\x02\n\n\r\n\x05\x04\x04\x02\x06\x05\x12\x04\xCA\x01\x0B\x11\n\r\n\x05\x04\x04\x02\x06\x01\x12\x04\xCA\x01\x12\x1F\n\r\n\x05\x04\x04\x02\x06\x03\x12\x04\xCA\x01\"#\n\x84\x01\n\x04\x04\x04\x02\x07\x12\x04\xCE\x01\x02!\x1Av If set, gives the index of a oneof in the containing type's oneof_decl\n list.  This field is a member of that oneof.\n\n\r\n\x05\x04\x04\x02\x07\x04\x12\x04\xCE\x01\x02\n\n\r\n\x05\x04\x04\x02\x07\x05\x12\x04\xCE\x01\x0B\x10\n\r\n\x05\x04\x04\x02\x07\x01\x12\x04\xCE\x01\x11\x1C\n\r\n\x05\x04\x04\x02\x07\x03\x12\x04\xCE\x01\x1F \n\xFA\x01\n\x04\x04\x04\x02\x08\x12\x04\xD4\x01\x02!\x1A\xEB\x01 JSON name of this field. The value is set by protocol compiler. If the\n user has set a \"json_name\" option on this field, that option's value\n will be used. Otherwise, it's deduced from the field's name by converting\n it to camelCase.\n\n\r\n\x05\x04\x04\x02\x08\x04\x12\x04\xD4\x01\x02\n\n\r\n\x05\x04\x04\x02\x08\x05\x12\x04\xD4\x01\x0B\x11\n\r\n\x05\x04\x04\x02\x08\x01\x12\x04\xD4\x01\x12\x1B\n\r\n\x05\x04\x04\x02\x08\x03\x12\x04\xD4\x01\x1E \n\x0C\n\x04\x04\x04\x02\t\x12\x04\xD6\x01\x02$\n\r\n\x05\x04\x04\x02\t\x04\x12\x04\xD6\x01\x02\n\n\r\n\x05\x04\x04\x02\t\x06\x12\x04\xD6\x01\x0B\x17\n\r\n\x05\x04\x04\x02\t\x01\x12\x04\xD6\x01\x18\x1F\n\r\n\x05\x04\x04\x02\t\x03\x12\x04\xD6\x01\"#\n\xB3\t\n\x04\x04\x04\x02\n\x12\x04\xED\x01\x02%\x1A\xA4\t If true, this is a proto3 \"optional\". When a proto3 field is optional, it\n tracks presence regardless of field type.\n\n When proto3_optional is true, this field must be belong to a oneof to\n signal to old proto3 clients that presence is tracked for this field. This\n oneof is known as a \"synthetic\" oneof, and this field must be its sole\n member (each proto3 optional field gets its own synthetic oneof). Synthetic\n oneofs exist in the descriptor only, and do not generate any API. Synthetic\n oneofs must be ordered after all \"real\" oneofs.\n\n For message fields, proto3_optional doesn't create any semantic change,\n since non-repeated message fields always track presence. However it still\n indicates the semantic detail of whether the user wrote \"optional\" or not.\n This can be useful for round-tripping the .proto file. For consistency we\n give message fields a synthetic oneof also, even though it is not required\n to track presence. This is especially important because the parser can't\n tell if a field is a message or an enum, so it must always create a\n synthetic oneof.\n\n Proto2 optional fields do not set this flag, because they already indicate\n optional with `LABEL_OPTIONAL`.\n\n\r\n\x05\x04\x04\x02\n\x04\x12\x04\xED\x01\x02\n\n\r\n\x05\x04\x04\x02\n\x05\x12\x04\xED\x01\x0B\x0F\n\r\n\x05\x04\x04\x02\n\x01\x12\x04\xED\x01\x10\x1F\n\r\n\x05\x04\x04\x02\n\x03\x12\x04\xED\x01\"$\n\"\n\x02\x04\x05\x12\x06\xF1\x01\0\xF4\x01\x01\x1A\x14 Describes a oneof.\n\n\x0B\n\x03\x04\x05\x01\x12\x04\xF1\x01\x08\x1C\n\x0C\n\x04\x04\x05\x02\0\x12\x04\xF2\x01\x02\x1B\n\r\n\x05\x04\x05\x02\0\x04\x12\x04\xF2\x01\x02\n\n\r\n\x05\x04\x05\x02\0\x05\x12\x04\xF2\x01\x0B\x11\n\r\n\x05\x04\x05\x02\0\x01\x12\x04\xF2\x01\x12\x16\n\r\n\x05\x04\x05\x02\0\x03\x12\x04\xF2\x01\x19\x1A\n\x0C\n\x04\x04\x05\x02\x01\x12\x04\xF3\x01\x02$\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04\xF3\x01\x02\n\n\r\n\x05\x04\x05\x02\x01\x06\x12\x04\xF3\x01\x0B\x17\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xF3\x01\x18\x1F\n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\xF3\x01\"#\n'\n\x02\x04\x06\x12\x06\xF7\x01\0\x91\x02\x01\x1A\x19 Describes an enum type.\n\n\x0B\n\x03\x04\x06\x01\x12\x04\xF7\x01\x08\x1B\n\x0C\n\x04\x04\x06\x02\0\x12\x04\xF8\x01\x02\x1B\n\r\n\x05\x04\x06\x02\0\x04\x12\x04\xF8\x01\x02\n\n\r\n\x05\x04\x06\x02\0\x05\x12\x04\xF8\x01\x0B\x11\n\r\n\x05\x04\x06\x02\0\x01\x12\x04\xF8\x01\x12\x16\n\r\n\x05\x04\x06\x02\0\x03\x12\x04\xF8\x01\x19\x1A\n\x0C\n\x04\x04\x06\x02\x01\x12\x04\xFA\x01\x02.\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04\xFA\x01\x02\n\n\r\n\x05\x04\x06\x02\x01\x06\x12\x04\xFA\x01\x0B#\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\xFA\x01$)\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\xFA\x01,-\n\x0C\n\x04\x04\x06\x02\x02\x12\x04\xFC\x01\x02#\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04\xFC\x01\x02\n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\xFC\x01\x0B\x16\n\r\n\x05\x04\x06\x02\x02\x01\x12\x04\xFC\x01\x17\x1E\n\r\n\x05\x04\x06\x02\x02\x03\x12\x04\xFC\x01!\"\n\xAF\x02\n\x04\x04\x06\x03\0\x12\x06\x84\x02\x02\x87\x02\x03\x1A\x9E\x02 Range of reserved numeric values. Reserved values may not be used by\n entries in the same enum. Reserved ranges may not overlap.\n\n Note that this is distinct from DescriptorProto.ReservedRange in that it\n is inclusive such that it can appropriately represent the entire int32\n domain.\n\n\r\n\x05\x04\x06\x03\0\x01\x12\x04\x84\x02\n\x1B\n\x1C\n\x06\x04\x06\x03\0\x02\0\x12\x04\x85\x02\x04\x1D\"\x0C Inclusive.\n\n\x0F\n\x07\x04\x06\x03\0\x02\0\x04\x12\x04\x85\x02\x04\x0C\n\x0F\n\x07\x04\x06\x03\0\x02\0\x05\x12\x04\x85\x02\r\x12\n\x0F\n\x07\x04\x06\x03\0\x02\0\x01\x12\x04\x85\x02\x13\x18\n\x0F\n\x07\x04\x06\x03\0\x02\0\x03\x12\x04\x85\x02\x1B\x1C\n\x1C\n\x06\x04\x06\x03\0\x02\x01\x12\x04\x86\x02\x04\x1B\"\x0C Inclusive.\n\n\x0F\n\x07\x04\x06\x03\0\x02\x01\x04\x12\x04\x86\x02\x04\x0C\n\x0F\n\x07\x04\x06\x03\0\x02\x01\x05\x12\x04\x86\x02\r\x12\n\x0F\n\x07\x04\x06\x03\0\x02\x01\x01\x12\x04\x86\x02\x13\x16\n\x0F\n\x07\x04\x06\x03\0\x02\x01\x03\x12\x04\x86\x02\x19\x1A\n\xAA\x01\n\x04\x04\x06\x02\x03\x12\x04\x8C\x02\x020\x1A\x9B\x01 Range of reserved numeric values. Reserved numeric values may not be used\n by enum values in the same enum declaration. Reserved ranges may not\n overlap.\n\n\r\n\x05\x04\x06\x02\x03\x04\x12\x04\x8C\x02\x02\n\n\r\n\x05\x04\x06\x02\x03\x06\x12\x04\x8C\x02\x0B\x1C\n\r\n\x05\x04\x06\x02\x03\x01\x12\x04\x8C\x02\x1D+\n\r\n\x05\x04\x06\x02\x03\x03\x12\x04\x8C\x02./\nl\n\x04\x04\x06\x02\x04\x12\x04\x90\x02\x02$\x1A^ Reserved enum value names, which may not be reused. A given name may only\n be reserved once.\n\n\r\n\x05\x04\x06\x02\x04\x04\x12\x04\x90\x02\x02\n\n\r\n\x05\x04\x06\x02\x04\x05\x12\x04\x90\x02\x0B\x11\n\r\n\x05\x04\x06\x02\x04\x01\x12\x04\x90\x02\x12\x1F\n\r\n\x05\x04\x06\x02\x04\x03\x12\x04\x90\x02\"#\n1\n\x02\x04\x07\x12\x06\x94\x02\0\x99\x02\x01\x1A# Describes a value within an enum.\n\n\x0B\n\x03\x04\x07\x01\x12\x04\x94\x02\x08 \n\x0C\n\x04\x04\x07\x02\0\x12\x04\x95\x02\x02\x1B\n\r\n\x05\x04\x07\x02\0\x04\x12\x04\x95\x02\x02\n\n\r\n\x05\x04\x07\x02\0\x05\x12\x04\x95\x02\x0B\x11\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\x95\x02\x12\x16\n\r\n\x05\x04\x07\x02\0\x03\x12\x04\x95\x02\x19\x1A\n\x0C\n\x04\x04\x07\x02\x01\x12\x04\x96\x02\x02\x1C\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\x96\x02\x02\n\n\r\n\x05\x04\x07\x02\x01\x05\x12\x04\x96\x02\x0B\x10\n\r\n\x05\x04\x07\x02\x01\x01\x12\x04\x96\x02\x11\x17\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\x96\x02\x1A\x1B\n\x0C\n\x04\x04\x07\x02\x02\x12\x04\x98\x02\x02(\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04\x98\x02\x02\n\n\r\n\x05\x04\x07\x02\x02\x06\x12\x04\x98\x02\x0B\x1B\n\r\n\x05\x04\x07\x02\x02\x01\x12\x04\x98\x02\x1C#\n\r\n\x05\x04\x07\x02\x02\x03\x12\x04\x98\x02&'\n$\n\x02\x04\x08\x12\x06\x9C\x02\0\xA1\x02\x01\x1A\x16 Describes a service.\n\n\x0B\n\x03\x04\x08\x01\x12\x04\x9C\x02\x08\x1E\n\x0C\n\x04\x04\x08\x02\0\x12\x04\x9D\x02\x02\x1B\n\r\n\x05\x04\x08\x02\0\x04\x12\x04\x9D\x02\x02\n\n\r\n\x05\x04\x08\x02\0\x05\x12\x04\x9D\x02\x0B\x11\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x9D\x02\x12\x16\n\r\n\x05\x04\x08\x02\0\x03\x12\x04\x9D\x02\x19\x1A\n\x0C\n\x04\x04\x08\x02\x01\x12\x04\x9E\x02\x02,\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04\x9E\x02\x02\n\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\x9E\x02\x0B \n\r\n\x05\x04\x08\x02\x01\x01\x12\x04\x9E\x02!'\n\r\n\x05\x04\x08\x02\x01\x03\x12\x04\x9E\x02*+\n\x0C\n\x04\x04\x08\x02\x02\x12\x04\xA0\x02\x02&\n\r\n\x05\x04\x08\x02\x02\x04\x12\x04\xA0\x02\x02\n\n\r\n\x05\x04\x08\x02\x02\x06\x12\x04\xA0\x02\x0B\x19\n\r\n\x05\x04\x08\x02\x02\x01\x12\x04\xA0\x02\x1A!\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\xA0\x02$%\n0\n\x02\x04\t\x12\x06\xA4\x02\0\xB2\x02\x01\x1A\" Describes a method of a service.\n\n\x0B\n\x03\x04\t\x01\x12\x04\xA4\x02\x08\x1D\n\x0C\n\x04\x04\t\x02\0\x12\x04\xA5\x02\x02\x1B\n\r\n\x05\x04\t\x02\0\x04\x12\x04\xA5\x02\x02\n\n\r\n\x05\x04\t\x02\0\x05\x12\x04\xA5\x02\x0B\x11\n\r\n\x05\x04\t\x02\0\x01\x12\x04\xA5\x02\x12\x16\n\r\n\x05\x04\t\x02\0\x03\x12\x04\xA5\x02\x19\x1A\n\x97\x01\n\x04\x04\t\x02\x01\x12\x04\xA9\x02\x02!\x1A\x88\x01 Input and output type names.  These are resolved in the same way as\n FieldDescriptorProto.type_name, but must refer to a message type.\n\n\r\n\x05\x04\t\x02\x01\x04\x12\x04\xA9\x02\x02\n\n\r\n\x05\x04\t\x02\x01\x05\x12\x04\xA9\x02\x0B\x11\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\xA9\x02\x12\x1C\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\xA9\x02\x1F \n\x0C\n\x04\x04\t\x02\x02\x12\x04\xAA\x02\x02\"\n\r\n\x05\x04\t\x02\x02\x04\x12\x04\xAA\x02\x02\n\n\r\n\x05\x04\t\x02\x02\x05\x12\x04\xAA\x02\x0B\x11\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\xAA\x02\x12\x1D\n\r\n\x05\x04\t\x02\x02\x03\x12\x04\xAA\x02 !\n\x0C\n\x04\x04\t\x02\x03\x12\x04\xAC\x02\x02%\n\r\n\x05\x04\t\x02\x03\x04\x12\x04\xAC\x02\x02\n\n\r\n\x05\x04\t\x02\x03\x06\x12\x04\xAC\x02\x0B\x18\n\r\n\x05\x04\t\x02\x03\x01\x12\x04\xAC\x02\x19 \n\r\n\x05\x04\t\x02\x03\x03\x12\x04\xAC\x02#$\nE\n\x04\x04\t\x02\x04\x12\x04\xAF\x02\x027\x1A7 Identifies if client streams multiple client messages\n\n\r\n\x05\x04\t\x02\x04\x04\x12\x04\xAF\x02\x02\n\n\r\n\x05\x04\t\x02\x04\x05\x12\x04\xAF\x02\x0B\x0F\n\r\n\x05\x04\t\x02\x04\x01\x12\x04\xAF\x02\x10 \n\r\n\x05\x04\t\x02\x04\x03\x12\x04\xAF\x02#$\n\r\n\x05\x04\t\x02\x04\x08\x12\x04\xAF\x02%6\n\r\n\x05\x04\t\x02\x04\x07\x12\x04\xAF\x0205\nE\n\x04\x04\t\x02\x05\x12\x04\xB1\x02\x027\x1A7 Identifies if server streams multiple server messages\n\n\r\n\x05\x04\t\x02\x05\x04\x12\x04\xB1\x02\x02\n\n\r\n\x05\x04\t\x02\x05\x05\x12\x04\xB1\x02\x0B\x0F\n\r\n\x05\x04\t\x02\x05\x01\x12\x04\xB1\x02\x10 \n\r\n\x05\x04\t\x02\x05\x03\x12\x04\xB1\x02#$\n\r\n\x05\x04\t\x02\x05\x08\x12\x04\xB1\x02%6\n\r\n\x05\x04\t\x02\x05\x07\x12\x04\xB1\x0205\n\xAF\x0E\n\x02\x04\n\x12\x06\xD5\x02\0\xD0\x03\x012N ===================================================================\n Options\n2\xD0\r Each of the definitions above may have \"options\" attached.  These are\n just annotations which may cause code to be generated slightly differently\n or may contain hints for code that manipulates protocol messages.\n\n Clients may define custom options as extensions of the *Options messages.\n These extensions may not yet be known at parsing time, so the parser cannot\n store the values in them.  Instead it stores them in a field in the *Options\n message called uninterpreted_option. This field must have the same name\n across all *Options messages. We then use this field to populate the\n extensions when we build a descriptor, at which point all protos have been\n parsed and so all extensions are known.\n\n Extension numbers for custom options may be chosen as follows:\n * For options which will only be used within a single application or\n   organization, or for experimental options, use field numbers 50000\n   through 99999.  It is up to you to ensure that you do not use the\n   same number for multiple options.\n * For options which will be published and used publicly by multiple\n   independent entities, e-mail protobuf-global-extension-registry@google.com\n   to reserve extension numbers. Simply provide your project name (e.g.\n   Objective-C plugin) and your project website (if available) -- there's no\n   need to explain how you intend to use them. Usually you only need one\n   extension number. You can declare multiple options with only one extension\n   number by putting them in a sub-message. See the Custom Options section of\n   the docs for examples:\n   https://developers.google.com/protocol-buffers/docs/proto#options\n   If this turns out to be popular, a web service will be set up\n   to automatically assign option numbers.\n\n\x0B\n\x03\x04\n\x01\x12\x04\xD5\x02\x08\x13\n\xF4\x01\n\x04\x04\n\x02\0\x12\x04\xDB\x02\x02#\x1A\xE5\x01 Sets the Java package where classes generated from this .proto will be\n placed.  By default, the proto package is used, but this is often\n inappropriate because proto packages do not normally start with backwards\n domain names.\n\n\r\n\x05\x04\n\x02\0\x04\x12\x04\xDB\x02\x02\n\n\r\n\x05\x04\n\x02\0\x05\x12\x04\xDB\x02\x0B\x11\n\r\n\x05\x04\n\x02\0\x01\x12\x04\xDB\x02\x12\x1E\n\r\n\x05\x04\n\x02\0\x03\x12\x04\xDB\x02!\"\n\xF1\x02\n\x04\x04\n\x02\x01\x12\x04\xE3\x02\x02+\x1A\xE2\x02 Controls the name of the wrapper Java class generated for the .proto file.\n That class will always contain the .proto file's getDescriptor() method as\n well as any top-level extensions defined in the .proto file.\n If java_multiple_files is disabled, then all the other classes from the\n .proto file will be nested inside the single wrapper outer class.\n\n\r\n\x05\x04\n\x02\x01\x04\x12\x04\xE3\x02\x02\n\n\r\n\x05\x04\n\x02\x01\x05\x12\x04\xE3\x02\x0B\x11\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\xE3\x02\x12&\n\r\n\x05\x04\n\x02\x01\x03\x12\x04\xE3\x02)*\n\xA6\x03\n\x04\x04\n\x02\x02\x12\x04\xEB\x02\x02;\x1A\x97\x03 If enabled, then the Java code generator will generate a separate .java\n file for each top-level message, enum, and service defined in the .proto\n file.  Thus, these types will *not* be nested inside the wrapper class\n named by java_outer_classname.  However, the wrapper class will still be\n generated to contain the file's getDescriptor() method as well as any\n top-level extensions defined in the file.\n\n\r\n\x05\x04\n\x02\x02\x04\x12\x04\xEB\x02\x02\n\n\r\n\x05\x04\n\x02\x02\x05\x12\x04\xEB\x02\x0B\x0F\n\r\n\x05\x04\n\x02\x02\x01\x12\x04\xEB\x02\x10#\n\r\n\x05\x04\n\x02\x02\x03\x12\x04\xEB\x02&(\n\r\n\x05\x04\n\x02\x02\x08\x12\x04\xEB\x02):\n\r\n\x05\x04\n\x02\x02\x07\x12\x04\xEB\x0249\n)\n\x04\x04\n\x02\x03\x12\x04\xEE\x02\x02E\x1A\x1B This option does nothing.\n\n\r\n\x05\x04\n\x02\x03\x04\x12\x04\xEE\x02\x02\n\n\r\n\x05\x04\n\x02\x03\x05\x12\x04\xEE\x02\x0B\x0F\n\r\n\x05\x04\n\x02\x03\x01\x12\x04\xEE\x02\x10-\n\r\n\x05\x04\n\x02\x03\x03\x12\x04\xEE\x0202\n\r\n\x05\x04\n\x02\x03\x08\x12\x04\xEE\x023D\n\x0E\n\x06\x04\n\x02\x03\x08\x03\x12\x04\xEE\x024C\n\xE6\x02\n\x04\x04\n\x02\x04\x12\x04\xF6\x02\x02>\x1A\xD7\x02 If set true, then the Java2 code generator will generate code that\n throws an exception whenever an attempt is made to assign a non-UTF-8\n byte sequence to a string field.\n Message reflection will do the same.\n However, an extension field still accepts non-UTF-8 byte sequences.\n This option has no effect on when used with the lite runtime.\n\n\r\n\x05\x04\n\x02\x04\x04\x12\x04\xF6\x02\x02\n\n\r\n\x05\x04\n\x02\x04\x05\x12\x04\xF6\x02\x0B\x0F\n\r\n\x05\x04\n\x02\x04\x01\x12\x04\xF6\x02\x10&\n\r\n\x05\x04\n\x02\x04\x03\x12\x04\xF6\x02)+\n\r\n\x05\x04\n\x02\x04\x08\x12\x04\xF6\x02,=\n\r\n\x05\x04\n\x02\x04\x07\x12\x04\xF6\x027<\nL\n\x04\x04\n\x04\0\x12\x06\xFA\x02\x02\xFF\x02\x03\x1A< Generated classes can be optimized for speed or code size.\n\n\r\n\x05\x04\n\x04\0\x01\x12\x04\xFA\x02\x07\x13\nD\n\x06\x04\n\x04\0\x02\0\x12\x04\xFB\x02\x04\x0E\"4 Generate complete code for parsing, serialization,\n\n\x0F\n\x07\x04\n\x04\0\x02\0\x01\x12\x04\xFB\x02\x04\t\n\x0F\n\x07\x04\n\x04\0\x02\0\x02\x12\x04\xFB\x02\x0C\r\nG\n\x06\x04\n\x04\0\x02\x01\x12\x04\xFD\x02\x04\x12\x1A\x06 etc.\n\"/ Use ReflectionOps to implement these methods.\n\n\x0F\n\x07\x04\n\x04\0\x02\x01\x01\x12\x04\xFD\x02\x04\r\n\x0F\n\x07\x04\n\x04\0\x02\x01\x02\x12\x04\xFD\x02\x10\x11\nG\n\x06\x04\n\x04\0\x02\x02\x12\x04\xFE\x02\x04\x15\"7 Generate code using MessageLite and the lite runtime.\n\n\x0F\n\x07\x04\n\x04\0\x02\x02\x01\x12\x04\xFE\x02\x04\x10\n\x0F\n\x07\x04\n\x04\0\x02\x02\x02\x12\x04\xFE\x02\x13\x14\n\x0C\n\x04\x04\n\x02\x05\x12\x04\x80\x03\x02;\n\r\n\x05\x04\n\x02\x05\x04\x12\x04\x80\x03\x02\n\n\r\n\x05\x04\n\x02\x05\x06\x12\x04\x80\x03\x0B\x17\n\r\n\x05\x04\n\x02\x05\x01\x12\x04\x80\x03\x18$\n\r\n\x05\x04\n\x02\x05\x03\x12\x04\x80\x03'(\n\r\n\x05\x04\n\x02\x05\x08\x12\x04\x80\x03):\n\r\n\x05\x04\n\x02\x05\x07\x12\x04\x80\x0349\n\xE2\x02\n\x04\x04\n\x02\x06\x12\x04\x87\x03\x02\"\x1A\xD3\x02 Sets the Go package where structs generated from this .proto will be\n placed. If omitted, the Go package will be derived from the following:\n   - The basename of the package import path, if provided.\n   - Otherwise, the package statement in the .proto file, if present.\n   - Otherwise, the basename of the .proto file, without extension.\n\n\r\n\x05\x04\n\x02\x06\x04\x12\x04\x87\x03\x02\n\n\r\n\x05\x04\n\x02\x06\x05\x12\x04\x87\x03\x0B\x11\n\r\n\x05\x04\n\x02\x06\x01\x12\x04\x87\x03\x12\x1C\n\r\n\x05\x04\n\x02\x06\x03\x12\x04\x87\x03\x1F!\n\xD4\x04\n\x04\x04\n\x02\x07\x12\x04\x96\x03\x02;\x1A\xC5\x04 Should generic services be generated in each language?  \"Generic\" services\n are not specific to any particular RPC system.  They are generated by the\n main code generators in each language (without additional plugins).\n Generic services were the only kind of service generation supported by\n early versions of google.protobuf.\n\n Generic services are now considered deprecated in favor of using plugins\n that generate code specific to your particular RPC system.  Therefore,\n these default to false.  Old code which depends on generic services should\n explicitly set them to true.\n\n\r\n\x05\x04\n\x02\x07\x04\x12\x04\x96\x03\x02\n\n\r\n\x05\x04\n\x02\x07\x05\x12\x04\x96\x03\x0B\x0F\n\r\n\x05\x04\n\x02\x07\x01\x12\x04\x96\x03\x10#\n\r\n\x05\x04\n\x02\x07\x03\x12\x04\x96\x03&(\n\r\n\x05\x04\n\x02\x07\x08\x12\x04\x96\x03):\n\r\n\x05\x04\n\x02\x07\x07\x12\x04\x96\x0349\n\x0C\n\x04\x04\n\x02\x08\x12\x04\x97\x03\x02=\n\r\n\x05\x04\n\x02\x08\x04\x12\x04\x97\x03\x02\n\n\r\n\x05\x04\n\x02\x08\x05\x12\x04\x97\x03\x0B\x0F\n\r\n\x05\x04\n\x02\x08\x01\x12\x04\x97\x03\x10%\n\r\n\x05\x04\n\x02\x08\x03\x12\x04\x97\x03(*\n\r\n\x05\x04\n\x02\x08\x08\x12\x04\x97\x03+<\n\r\n\x05\x04\n\x02\x08\x07\x12\x04\x97\x036;\n\x0C\n\x04\x04\n\x02\t\x12\x04\x98\x03\x02;\n\r\n\x05\x04\n\x02\t\x04\x12\x04\x98\x03\x02\n\n\r\n\x05\x04\n\x02\t\x05\x12\x04\x98\x03\x0B\x0F\n\r\n\x05\x04\n\x02\t\x01\x12\x04\x98\x03\x10#\n\r\n\x05\x04\n\x02\t\x03\x12\x04\x98\x03&(\n\r\n\x05\x04\n\x02\t\x08\x12\x04\x98\x03):\n\r\n\x05\x04\n\x02\t\x07\x12\x04\x98\x0349\n\x0C\n\x04\x04\n\x02\n\x12\x04\x99\x03\x02<\n\r\n\x05\x04\n\x02\n\x04\x12\x04\x99\x03\x02\n\n\r\n\x05\x04\n\x02\n\x05\x12\x04\x99\x03\x0B\x0F\n\r\n\x05\x04\n\x02\n\x01\x12\x04\x99\x03\x10$\n\r\n\x05\x04\n\x02\n\x03\x12\x04\x99\x03')\n\r\n\x05\x04\n\x02\n\x08\x12\x04\x99\x03*;\n\r\n\x05\x04\n\x02\n\x07\x12\x04\x99\x035:\n\xF3\x01\n\x04\x04\n\x02\x0B\x12\x04\x9F\x03\x022\x1A\xE4\x01 Is this file deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for everything in the file, or it will be completely ignored; in the very\n least, this is a formalization for deprecating files.\n\n\r\n\x05\x04\n\x02\x0B\x04\x12\x04\x9F\x03\x02\n\n\r\n\x05\x04\n\x02\x0B\x05\x12\x04\x9F\x03\x0B\x0F\n\r\n\x05\x04\n\x02\x0B\x01\x12\x04\x9F\x03\x10\x1A\n\r\n\x05\x04\n\x02\x0B\x03\x12\x04\x9F\x03\x1D\x1F\n\r\n\x05\x04\n\x02\x0B\x08\x12\x04\x9F\x03 1\n\r\n\x05\x04\n\x02\x0B\x07\x12\x04\x9F\x03+0\n\x7F\n\x04\x04\n\x02\x0C\x12\x04\xA3\x03\x027\x1Aq Enables the use of arenas for the proto messages in this file. This applies\n only to generated classes for C++.\n\n\r\n\x05\x04\n\x02\x0C\x04\x12\x04\xA3\x03\x02\n\n\r\n\x05\x04\n\x02\x0C\x05\x12\x04\xA3\x03\x0B\x0F\n\r\n\x05\x04\n\x02\x0C\x01\x12\x04\xA3\x03\x10 \n\r\n\x05\x04\n\x02\x0C\x03\x12\x04\xA3\x03#%\n\r\n\x05\x04\n\x02\x0C\x08\x12\x04\xA3\x03&6\n\r\n\x05\x04\n\x02\x0C\x07\x12\x04\xA3\x0315\n\x92\x01\n\x04\x04\n\x02\r\x12\x04\xA8\x03\x02)\x1A\x83\x01 Sets the objective c class prefix which is prepended to all objective c\n generated classes from this .proto. There is no default.\n\n\r\n\x05\x04\n\x02\r\x04\x12\x04\xA8\x03\x02\n\n\r\n\x05\x04\n\x02\r\x05\x12\x04\xA8\x03\x0B\x11\n\r\n\x05\x04\n\x02\r\x01\x12\x04\xA8\x03\x12#\n\r\n\x05\x04\n\x02\r\x03\x12\x04\xA8\x03&(\nI\n\x04\x04\n\x02\x0E\x12\x04\xAB\x03\x02(\x1A; Namespace for generated classes; defaults to the package.\n\n\r\n\x05\x04\n\x02\x0E\x04\x12\x04\xAB\x03\x02\n\n\r\n\x05\x04\n\x02\x0E\x05\x12\x04\xAB\x03\x0B\x11\n\r\n\x05\x04\n\x02\x0E\x01\x12\x04\xAB\x03\x12\"\n\r\n\x05\x04\n\x02\x0E\x03\x12\x04\xAB\x03%'\n\x91\x02\n\x04\x04\n\x02\x0F\x12\x04\xB1\x03\x02$\x1A\x82\x02 By default Swift generators will take the proto package and CamelCase it\n replacing '.' with underscore and use that to prefix the types/symbols\n defined. When this options is provided, they will use this value instead\n to prefix the types/symbols defined.\n\n\r\n\x05\x04\n\x02\x0F\x04\x12\x04\xB1\x03\x02\n\n\r\n\x05\x04\n\x02\x0F\x05\x12\x04\xB1\x03\x0B\x11\n\r\n\x05\x04\n\x02\x0F\x01\x12\x04\xB1\x03\x12\x1E\n\r\n\x05\x04\n\x02\x0F\x03\x12\x04\xB1\x03!#\n~\n\x04\x04\n\x02\x10\x12\x04\xB5\x03\x02(\x1Ap Sets the php class prefix which is prepended to all php generated classes\n from this .proto. Default is empty.\n\n\r\n\x05\x04\n\x02\x10\x04\x12\x04\xB5\x03\x02\n\n\r\n\x05\x04\n\x02\x10\x05\x12\x04\xB5\x03\x0B\x11\n\r\n\x05\x04\n\x02\x10\x01\x12\x04\xB5\x03\x12\"\n\r\n\x05\x04\n\x02\x10\x03\x12\x04\xB5\x03%'\n\xBE\x01\n\x04\x04\n\x02\x11\x12\x04\xBA\x03\x02%\x1A\xAF\x01 Use this option to change the namespace of php generated classes. Default\n is empty. When this option is empty, the package name will be used for\n determining the namespace.\n\n\r\n\x05\x04\n\x02\x11\x04\x12\x04\xBA\x03\x02\n\n\r\n\x05\x04\n\x02\x11\x05\x12\x04\xBA\x03\x0B\x11\n\r\n\x05\x04\n\x02\x11\x01\x12\x04\xBA\x03\x12\x1F\n\r\n\x05\x04\n\x02\x11\x03\x12\x04\xBA\x03\"$\n\xCA\x01\n\x04\x04\n\x02\x12\x12\x04\xBF\x03\x02.\x1A\xBB\x01 Use this option to change the namespace of php generated metadata classes.\n Default is empty. When this option is empty, the proto file name will be\n used for determining the namespace.\n\n\r\n\x05\x04\n\x02\x12\x04\x12\x04\xBF\x03\x02\n\n\r\n\x05\x04\n\x02\x12\x05\x12\x04\xBF\x03\x0B\x11\n\r\n\x05\x04\n\x02\x12\x01\x12\x04\xBF\x03\x12(\n\r\n\x05\x04\n\x02\x12\x03\x12\x04\xBF\x03+-\n\xC2\x01\n\x04\x04\n\x02\x13\x12\x04\xC4\x03\x02$\x1A\xB3\x01 Use this option to change the package of ruby generated classes. Default\n is empty. When this option is not set, the package name will be used for\n determining the ruby package.\n\n\r\n\x05\x04\n\x02\x13\x04\x12\x04\xC4\x03\x02\n\n\r\n\x05\x04\n\x02\x13\x05\x12\x04\xC4\x03\x0B\x11\n\r\n\x05\x04\n\x02\x13\x01\x12\x04\xC4\x03\x12\x1E\n\r\n\x05\x04\n\x02\x13\x03\x12\x04\xC4\x03!#\n|\n\x04\x04\n\x02\x14\x12\x04\xC9\x03\x02:\x1An The parser stores options it doesn't recognize here.\n See the documentation for the \"Options\" section above.\n\n\r\n\x05\x04\n\x02\x14\x04\x12\x04\xC9\x03\x02\n\n\r\n\x05\x04\n\x02\x14\x06\x12\x04\xC9\x03\x0B\x1E\n\r\n\x05\x04\n\x02\x14\x01\x12\x04\xC9\x03\x1F3\n\r\n\x05\x04\n\x02\x14\x03\x12\x04\xC9\x0369\n\x87\x01\n\x03\x04\n\x05\x12\x04\xCD\x03\x02\x19\x1Az Clients can define custom options in extensions of this message.\n See the documentation for the \"Options\" section above.\n\n\x0C\n\x04\x04\n\x05\0\x12\x04\xCD\x03\r\x18\n\r\n\x05\x04\n\x05\0\x01\x12\x04\xCD\x03\r\x11\n\r\n\x05\x04\n\x05\0\x02\x12\x04\xCD\x03\x15\x18\n\x0B\n\x03\x04\n\t\x12\x04\xCF\x03\x02\x0E\n\x0C\n\x04\x04\n\t\0\x12\x04\xCF\x03\x0B\r\n\r\n\x05\x04\n\t\0\x01\x12\x04\xCF\x03\x0B\r\n\r\n\x05\x04\n\t\0\x02\x12\x04\xCF\x03\x0B\r\n\x0C\n\x02\x04\x0B\x12\x06\xD2\x03\0\x94\x04\x01\n\x0B\n\x03\x04\x0B\x01\x12\x04\xD2\x03\x08\x16\n\xD8\x05\n\x04\x04\x0B\x02\0\x12\x04\xE5\x03\x02>\x1A\xC9\x05 Set true to use the old proto1 MessageSet wire format for extensions.\n This is provided for backwards-compatibility with the MessageSet wire\n format.  You should not use this for any other reason:  It's less\n efficient, has fewer features, and is more complicated.\n\n The message must be defined exactly as follows:\n   message Foo {\n     option message_set_wire_format = true;\n     extensions 4 to max;\n   }\n Note that the message cannot have any defined fields; MessageSets only\n have extensions.\n\n All extensions of your type must be singular messages; e.g. they cannot\n be int32s, enums, or repeated messages.\n\n Because this is an option, the above two restrictions are not enforced by\n the protocol compiler.\n\n\r\n\x05\x04\x0B\x02\0\x04\x12\x04\xE5\x03\x02\n\n\r\n\x05\x04\x0B\x02\0\x05\x12\x04\xE5\x03\x0B\x0F\n\r\n\x05\x04\x0B\x02\0\x01\x12\x04\xE5\x03\x10'\n\r\n\x05\x04\x0B\x02\0\x03\x12\x04\xE5\x03*+\n\r\n\x05\x04\x0B\x02\0\x08\x12\x04\xE5\x03,=\n\r\n\x05\x04\x0B\x02\0\x07\x12\x04\xE5\x037<\n\xEB\x01\n\x04\x04\x0B\x02\x01\x12\x04\xEA\x03\x02F\x1A\xDC\x01 Disables the generation of the standard \"descriptor()\" accessor, which can\n conflict with a field of the same name.  This is meant to make migration\n from proto1 easier; new code should avoid fields named \"descriptor\".\n\n\r\n\x05\x04\x0B\x02\x01\x04\x12\x04\xEA\x03\x02\n\n\r\n\x05\x04\x0B\x02\x01\x05\x12\x04\xEA\x03\x0B\x0F\n\r\n\x05\x04\x0B\x02\x01\x01\x12\x04\xEA\x03\x10/\n\r\n\x05\x04\x0B\x02\x01\x03\x12\x04\xEA\x0323\n\r\n\x05\x04\x0B\x02\x01\x08\x12\x04\xEA\x034E\n\r\n\x05\x04\x0B\x02\x01\x07\x12\x04\xEA\x03?D\n\xEE\x01\n\x04\x04\x0B\x02\x02\x12\x04\xF0\x03\x021\x1A\xDF\x01 Is this message deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for the message, or it will be completely ignored; in the very least,\n this is a formalization for deprecating messages.\n\n\r\n\x05\x04\x0B\x02\x02\x04\x12\x04\xF0\x03\x02\n\n\r\n\x05\x04\x0B\x02\x02\x05\x12\x04\xF0\x03\x0B\x0F\n\r\n\x05\x04\x0B\x02\x02\x01\x12\x04\xF0\x03\x10\x1A\n\r\n\x05\x04\x0B\x02\x02\x03\x12\x04\xF0\x03\x1D\x1E\n\r\n\x05\x04\x0B\x02\x02\x08\x12\x04\xF0\x03\x1F0\n\r\n\x05\x04\x0B\x02\x02\x07\x12\x04\xF0\x03*/\n\x0B\n\x03\x04\x0B\t\x12\x04\xF2\x03\x02\x13\n\x0C\n\x04\x04\x0B\t\0\x12\x04\xF2\x03\x0B\x0C\n\r\n\x05\x04\x0B\t\0\x01\x12\x04\xF2\x03\x0B\x0C\n\r\n\x05\x04\x0B\t\0\x02\x12\x04\xF2\x03\x0B\x0C\n\x0C\n\x04\x04\x0B\t\x01\x12\x04\xF2\x03\x0E\x0F\n\r\n\x05\x04\x0B\t\x01\x01\x12\x04\xF2\x03\x0E\x0F\n\r\n\x05\x04\x0B\t\x01\x02\x12\x04\xF2\x03\x0E\x0F\n\x0C\n\x04\x04\x0B\t\x02\x12\x04\xF2\x03\x11\x12\n\r\n\x05\x04\x0B\t\x02\x01\x12\x04\xF2\x03\x11\x12\n\r\n\x05\x04\x0B\t\x02\x02\x12\x04\xF2\x03\x11\x12\n\xA0\x06\n\x04\x04\x0B\x02\x03\x12\x04\x89\x04\x02\x1E\x1A\x91\x06 Whether the message is an automatically generated map entry type for the\n maps field.\n\n For maps fields:\n     map<KeyType, ValueType> map_field = 1;\n The parsed descriptor looks like:\n     message MapFieldEntry {\n         option map_entry = true;\n         optional KeyType key = 1;\n         optional ValueType value = 2;\n     }\n     repeated MapFieldEntry map_field = 1;\n\n Implementations may choose not to generate the map_entry=true message, but\n use a native map in the target language to hold the keys and values.\n The reflection APIs in such implementations still need to work as\n if the field is a repeated message field.\n\n NOTE: Do not set the option in .proto files. Always use the maps syntax\n instead. The option should only be implicitly set by the proto compiler\n parser.\n\n\r\n\x05\x04\x0B\x02\x03\x04\x12\x04\x89\x04\x02\n\n\r\n\x05\x04\x0B\x02\x03\x05\x12\x04\x89\x04\x0B\x0F\n\r\n\x05\x04\x0B\x02\x03\x01\x12\x04\x89\x04\x10\x19\n\r\n\x05\x04\x0B\x02\x03\x03\x12\x04\x89\x04\x1C\x1D\n$\n\x03\x04\x0B\t\x12\x04\x8B\x04\x02\r\"\x17 javalite_serializable\n\n\x0C\n\x04\x04\x0B\t\x03\x12\x04\x8B\x04\x0B\x0C\n\r\n\x05\x04\x0B\t\x03\x01\x12\x04\x8B\x04\x0B\x0C\n\r\n\x05\x04\x0B\t\x03\x02\x12\x04\x8B\x04\x0B\x0C\n\x1F\n\x03\x04\x0B\t\x12\x04\x8C\x04\x02\r\"\x12 javanano_as_lite\n\n\x0C\n\x04\x04\x0B\t\x04\x12\x04\x8C\x04\x0B\x0C\n\r\n\x05\x04\x0B\t\x04\x01\x12\x04\x8C\x04\x0B\x0C\n\r\n\x05\x04\x0B\t\x04\x02\x12\x04\x8C\x04\x0B\x0C\nO\n\x04\x04\x0B\x02\x04\x12\x04\x90\x04\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x0B\x02\x04\x04\x12\x04\x90\x04\x02\n\n\r\n\x05\x04\x0B\x02\x04\x06\x12\x04\x90\x04\x0B\x1E\n\r\n\x05\x04\x0B\x02\x04\x01\x12\x04\x90\x04\x1F3\n\r\n\x05\x04\x0B\x02\x04\x03\x12\x04\x90\x0469\nZ\n\x03\x04\x0B\x05\x12\x04\x93\x04\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x0B\x05\0\x12\x04\x93\x04\r\x18\n\r\n\x05\x04\x0B\x05\0\x01\x12\x04\x93\x04\r\x11\n\r\n\x05\x04\x0B\x05\0\x02\x12\x04\x93\x04\x15\x18\n\x0C\n\x02\x04\x0C\x12\x06\x96\x04\0\xF1\x04\x01\n\x0B\n\x03\x04\x0C\x01\x12\x04\x96\x04\x08\x14\n\xA3\x02\n\x04\x04\x0C\x02\0\x12\x04\x9B\x04\x02.\x1A\x94\x02 The ctype option instructs the C++ code generator to use a different\n representation of the field than it normally would.  See the specific\n options below.  This option is not yet implemented in the open source\n release -- sorry, we'll try to include it in a future version!\n\n\r\n\x05\x04\x0C\x02\0\x04\x12\x04\x9B\x04\x02\n\n\r\n\x05\x04\x0C\x02\0\x06\x12\x04\x9B\x04\x0B\x10\n\r\n\x05\x04\x0C\x02\0\x01\x12\x04\x9B\x04\x11\x16\n\r\n\x05\x04\x0C\x02\0\x03\x12\x04\x9B\x04\x19\x1A\n\r\n\x05\x04\x0C\x02\0\x08\x12\x04\x9B\x04\x1B-\n\r\n\x05\x04\x0C\x02\0\x07\x12\x04\x9B\x04&,\n\x0E\n\x04\x04\x0C\x04\0\x12\x06\x9C\x04\x02\xA3\x04\x03\n\r\n\x05\x04\x0C\x04\0\x01\x12\x04\x9C\x04\x07\x0C\n\x1F\n\x06\x04\x0C\x04\0\x02\0\x12\x04\x9E\x04\x04\x0F\x1A\x0F Default mode.\n\n\x0F\n\x07\x04\x0C\x04\0\x02\0\x01\x12\x04\x9E\x04\x04\n\n\x0F\n\x07\x04\x0C\x04\0\x02\0\x02\x12\x04\x9E\x04\r\x0E\n\x0E\n\x06\x04\x0C\x04\0\x02\x01\x12\x04\xA0\x04\x04\r\n\x0F\n\x07\x04\x0C\x04\0\x02\x01\x01\x12\x04\xA0\x04\x04\x08\n\x0F\n\x07\x04\x0C\x04\0\x02\x01\x02\x12\x04\xA0\x04\x0B\x0C\n\x0E\n\x06\x04\x0C\x04\0\x02\x02\x12\x04\xA2\x04\x04\x15\n\x0F\n\x07\x04\x0C\x04\0\x02\x02\x01\x12\x04\xA2\x04\x04\x10\n\x0F\n\x07\x04\x0C\x04\0\x02\x02\x02\x12\x04\xA2\x04\x13\x14\n\xDA\x02\n\x04\x04\x0C\x02\x01\x12\x04\xA9\x04\x02\x1B\x1A\xCB\x02 The packed option can be enabled for repeated primitive fields to enable\n a more efficient representation on the wire. Rather than repeatedly\n writing the tag and type for each element, the entire array is encoded as\n a single length-delimited blob. In proto3, only explicit setting it to\n false will avoid using packed encoding.\n\n\r\n\x05\x04\x0C\x02\x01\x04\x12\x04\xA9\x04\x02\n\n\r\n\x05\x04\x0C\x02\x01\x05\x12\x04\xA9\x04\x0B\x0F\n\r\n\x05\x04\x0C\x02\x01\x01\x12\x04\xA9\x04\x10\x16\n\r\n\x05\x04\x0C\x02\x01\x03\x12\x04\xA9\x04\x19\x1A\n\x9A\x05\n\x04\x04\x0C\x02\x02\x12\x04\xB6\x04\x023\x1A\x8B\x05 The jstype option determines the JavaScript type used for values of the\n field.  The option is permitted only for 64 bit integral and fixed types\n (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING\n is represented as JavaScript string, which avoids loss of precision that\n can happen when a large value is converted to a floating point JavaScript.\n Specifying JS_NUMBER for the jstype causes the generated JavaScript code to\n use the JavaScript \"number\" type.  The behavior of the default option\n JS_NORMAL is implementation dependent.\n\n This option is an enum to permit additional types to be added, e.g.\n goog.math.Integer.\n\n\r\n\x05\x04\x0C\x02\x02\x04\x12\x04\xB6\x04\x02\n\n\r\n\x05\x04\x0C\x02\x02\x06\x12\x04\xB6\x04\x0B\x11\n\r\n\x05\x04\x0C\x02\x02\x01\x12\x04\xB6\x04\x12\x18\n\r\n\x05\x04\x0C\x02\x02\x03\x12\x04\xB6\x04\x1B\x1C\n\r\n\x05\x04\x0C\x02\x02\x08\x12\x04\xB6\x04\x1D2\n\r\n\x05\x04\x0C\x02\x02\x07\x12\x04\xB6\x04(1\n\x0E\n\x04\x04\x0C\x04\x01\x12\x06\xB7\x04\x02\xC0\x04\x03\n\r\n\x05\x04\x0C\x04\x01\x01\x12\x04\xB7\x04\x07\r\n'\n\x06\x04\x0C\x04\x01\x02\0\x12\x04\xB9\x04\x04\x12\x1A\x17 Use the default type.\n\n\x0F\n\x07\x04\x0C\x04\x01\x02\0\x01\x12\x04\xB9\x04\x04\r\n\x0F\n\x07\x04\x0C\x04\x01\x02\0\x02\x12\x04\xB9\x04\x10\x11\n)\n\x06\x04\x0C\x04\x01\x02\x01\x12\x04\xBC\x04\x04\x12\x1A\x19 Use JavaScript strings.\n\n\x0F\n\x07\x04\x0C\x04\x01\x02\x01\x01\x12\x04\xBC\x04\x04\r\n\x0F\n\x07\x04\x0C\x04\x01\x02\x01\x02\x12\x04\xBC\x04\x10\x11\n)\n\x06\x04\x0C\x04\x01\x02\x02\x12\x04\xBF\x04\x04\x12\x1A\x19 Use JavaScript numbers.\n\n\x0F\n\x07\x04\x0C\x04\x01\x02\x02\x01\x12\x04\xBF\x04\x04\r\n\x0F\n\x07\x04\x0C\x04\x01\x02\x02\x02\x12\x04\xBF\x04\x10\x11\n\xEF\x0C\n\x04\x04\x0C\x02\x03\x12\x04\xDE\x04\x02+\x1A\xE0\x0C Should this field be parsed lazily?  Lazy applies only to message-type\n fields.  It means that when the outer message is initially parsed, the\n inner message's contents will not be parsed but instead stored in encoded\n form.  The inner message will actually be parsed when it is first accessed.\n\n This is only a hint.  Implementations are free to choose whether to use\n eager or lazy parsing regardless of the value of this option.  However,\n setting this option true suggests that the protocol author believes that\n using lazy parsing on this field is worth the additional bookkeeping\n overhead typically needed to implement it.\n\n This option does not affect the public interface of any generated code;\n all method signatures remain the same.  Furthermore, thread-safety of the\n interface is not affected by this option; const methods remain safe to\n call from multiple threads concurrently, while non-const methods continue\n to require exclusive access.\n\n\n Note that implementations may choose not to check required fields within\n a lazy sub-message.  That is, calling IsInitialized() on the outer message\n may return true even if the inner message has missing required fields.\n This is necessary because otherwise the inner message would have to be\n parsed in order to perform the check, defeating the purpose of lazy\n parsing.  An implementation which chooses not to check required fields\n must be consistent about it.  That is, for any particular sub-message, the\n implementation must either *always* check its required fields, or *never*\n check its required fields, regardless of whether or not the message has\n been parsed.\n\n\r\n\x05\x04\x0C\x02\x03\x04\x12\x04\xDE\x04\x02\n\n\r\n\x05\x04\x0C\x02\x03\x05\x12\x04\xDE\x04\x0B\x0F\n\r\n\x05\x04\x0C\x02\x03\x01\x12\x04\xDE\x04\x10\x14\n\r\n\x05\x04\x0C\x02\x03\x03\x12\x04\xDE\x04\x17\x18\n\r\n\x05\x04\x0C\x02\x03\x08\x12\x04\xDE\x04\x19*\n\r\n\x05\x04\x0C\x02\x03\x07\x12\x04\xDE\x04$)\n\xE8\x01\n\x04\x04\x0C\x02\x04\x12\x04\xE4\x04\x021\x1A\xD9\x01 Is this field deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for accessors, or it will be completely ignored; in the very least, this\n is a formalization for deprecating fields.\n\n\r\n\x05\x04\x0C\x02\x04\x04\x12\x04\xE4\x04\x02\n\n\r\n\x05\x04\x0C\x02\x04\x05\x12\x04\xE4\x04\x0B\x0F\n\r\n\x05\x04\x0C\x02\x04\x01\x12\x04\xE4\x04\x10\x1A\n\r\n\x05\x04\x0C\x02\x04\x03\x12\x04\xE4\x04\x1D\x1E\n\r\n\x05\x04\x0C\x02\x04\x08\x12\x04\xE4\x04\x1F0\n\r\n\x05\x04\x0C\x02\x04\x07\x12\x04\xE4\x04*/\n?\n\x04\x04\x0C\x02\x05\x12\x04\xE7\x04\x02,\x1A1 For Google-internal migration only. Do not use.\n\n\r\n\x05\x04\x0C\x02\x05\x04\x12\x04\xE7\x04\x02\n\n\r\n\x05\x04\x0C\x02\x05\x05\x12\x04\xE7\x04\x0B\x0F\n\r\n\x05\x04\x0C\x02\x05\x01\x12\x04\xE7\x04\x10\x14\n\r\n\x05\x04\x0C\x02\x05\x03\x12\x04\xE7\x04\x17\x19\n\r\n\x05\x04\x0C\x02\x05\x08\x12\x04\xE7\x04\x1A+\n\r\n\x05\x04\x0C\x02\x05\x07\x12\x04\xE7\x04%*\nO\n\x04\x04\x0C\x02\x06\x12\x04\xEB\x04\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x0C\x02\x06\x04\x12\x04\xEB\x04\x02\n\n\r\n\x05\x04\x0C\x02\x06\x06\x12\x04\xEB\x04\x0B\x1E\n\r\n\x05\x04\x0C\x02\x06\x01\x12\x04\xEB\x04\x1F3\n\r\n\x05\x04\x0C\x02\x06\x03\x12\x04\xEB\x0469\nZ\n\x03\x04\x0C\x05\x12\x04\xEE\x04\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x0C\x05\0\x12\x04\xEE\x04\r\x18\n\r\n\x05\x04\x0C\x05\0\x01\x12\x04\xEE\x04\r\x11\n\r\n\x05\x04\x0C\x05\0\x02\x12\x04\xEE\x04\x15\x18\n\x1C\n\x03\x04\x0C\t\x12\x04\xF0\x04\x02\r\"\x0F removed jtype\n\n\x0C\n\x04\x04\x0C\t\0\x12\x04\xF0\x04\x0B\x0C\n\r\n\x05\x04\x0C\t\0\x01\x12\x04\xF0\x04\x0B\x0C\n\r\n\x05\x04\x0C\t\0\x02\x12\x04\xF0\x04\x0B\x0C\n\x0C\n\x02\x04\r\x12\x06\xF3\x04\0\xF9\x04\x01\n\x0B\n\x03\x04\r\x01\x12\x04\xF3\x04\x08\x14\nO\n\x04\x04\r\x02\0\x12\x04\xF5\x04\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\r\x02\0\x04\x12\x04\xF5\x04\x02\n\n\r\n\x05\x04\r\x02\0\x06\x12\x04\xF5\x04\x0B\x1E\n\r\n\x05\x04\r\x02\0\x01\x12\x04\xF5\x04\x1F3\n\r\n\x05\x04\r\x02\0\x03\x12\x04\xF5\x0469\nZ\n\x03\x04\r\x05\x12\x04\xF8\x04\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\r\x05\0\x12\x04\xF8\x04\r\x18\n\r\n\x05\x04\r\x05\0\x01\x12\x04\xF8\x04\r\x11\n\r\n\x05\x04\r\x05\0\x02\x12\x04\xF8\x04\x15\x18\n\x0C\n\x02\x04\x0E\x12\x06\xFB\x04\0\x8E\x05\x01\n\x0B\n\x03\x04\x0E\x01\x12\x04\xFB\x04\x08\x13\n`\n\x04\x04\x0E\x02\0\x12\x04\xFF\x04\x02 \x1AR Set this option to true to allow mapping different tag names to the same\n value.\n\n\r\n\x05\x04\x0E\x02\0\x04\x12\x04\xFF\x04\x02\n\n\r\n\x05\x04\x0E\x02\0\x05\x12\x04\xFF\x04\x0B\x0F\n\r\n\x05\x04\x0E\x02\0\x01\x12\x04\xFF\x04\x10\x1B\n\r\n\x05\x04\x0E\x02\0\x03\x12\x04\xFF\x04\x1E\x1F\n\xE5\x01\n\x04\x04\x0E\x02\x01\x12\x04\x85\x05\x021\x1A\xD6\x01 Is this enum deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for the enum, or it will be completely ignored; in the very least, this\n is a formalization for deprecating enums.\n\n\r\n\x05\x04\x0E\x02\x01\x04\x12\x04\x85\x05\x02\n\n\r\n\x05\x04\x0E\x02\x01\x05\x12\x04\x85\x05\x0B\x0F\n\r\n\x05\x04\x0E\x02\x01\x01\x12\x04\x85\x05\x10\x1A\n\r\n\x05\x04\x0E\x02\x01\x03\x12\x04\x85\x05\x1D\x1E\n\r\n\x05\x04\x0E\x02\x01\x08\x12\x04\x85\x05\x1F0\n\r\n\x05\x04\x0E\x02\x01\x07\x12\x04\x85\x05*/\n\x1F\n\x03\x04\x0E\t\x12\x04\x87\x05\x02\r\"\x12 javanano_as_lite\n\n\x0C\n\x04\x04\x0E\t\0\x12\x04\x87\x05\x0B\x0C\n\r\n\x05\x04\x0E\t\0\x01\x12\x04\x87\x05\x0B\x0C\n\r\n\x05\x04\x0E\t\0\x02\x12\x04\x87\x05\x0B\x0C\nO\n\x04\x04\x0E\x02\x02\x12\x04\x8A\x05\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x0E\x02\x02\x04\x12\x04\x8A\x05\x02\n\n\r\n\x05\x04\x0E\x02\x02\x06\x12\x04\x8A\x05\x0B\x1E\n\r\n\x05\x04\x0E\x02\x02\x01\x12\x04\x8A\x05\x1F3\n\r\n\x05\x04\x0E\x02\x02\x03\x12\x04\x8A\x0569\nZ\n\x03\x04\x0E\x05\x12\x04\x8D\x05\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x0E\x05\0\x12\x04\x8D\x05\r\x18\n\r\n\x05\x04\x0E\x05\0\x01\x12\x04\x8D\x05\r\x11\n\r\n\x05\x04\x0E\x05\0\x02\x12\x04\x8D\x05\x15\x18\n\x0C\n\x02\x04\x0F\x12\x06\x90\x05\0\x9C\x05\x01\n\x0B\n\x03\x04\x0F\x01\x12\x04\x90\x05\x08\x18\n\xF7\x01\n\x04\x04\x0F\x02\0\x12\x04\x95\x05\x021\x1A\xE8\x01 Is this enum value deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for the enum value, or it will be completely ignored; in the very least,\n this is a formalization for deprecating enum values.\n\n\r\n\x05\x04\x0F\x02\0\x04\x12\x04\x95\x05\x02\n\n\r\n\x05\x04\x0F\x02\0\x05\x12\x04\x95\x05\x0B\x0F\n\r\n\x05\x04\x0F\x02\0\x01\x12\x04\x95\x05\x10\x1A\n\r\n\x05\x04\x0F\x02\0\x03\x12\x04\x95\x05\x1D\x1E\n\r\n\x05\x04\x0F\x02\0\x08\x12\x04\x95\x05\x1F0\n\r\n\x05\x04\x0F\x02\0\x07\x12\x04\x95\x05*/\nO\n\x04\x04\x0F\x02\x01\x12\x04\x98\x05\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x0F\x02\x01\x04\x12\x04\x98\x05\x02\n\n\r\n\x05\x04\x0F\x02\x01\x06\x12\x04\x98\x05\x0B\x1E\n\r\n\x05\x04\x0F\x02\x01\x01\x12\x04\x98\x05\x1F3\n\r\n\x05\x04\x0F\x02\x01\x03\x12\x04\x98\x0569\nZ\n\x03\x04\x0F\x05\x12\x04\x9B\x05\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x0F\x05\0\x12\x04\x9B\x05\r\x18\n\r\n\x05\x04\x0F\x05\0\x01\x12\x04\x9B\x05\r\x11\n\r\n\x05\x04\x0F\x05\0\x02\x12\x04\x9B\x05\x15\x18\n\x0C\n\x02\x04\x10\x12\x06\x9E\x05\0\xB0\x05\x01\n\x0B\n\x03\x04\x10\x01\x12\x04\x9E\x05\x08\x16\n\xD9\x03\n\x04\x04\x10\x02\0\x12\x04\xA9\x05\x022\x1A\xDF\x01 Is this service deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for the service, or it will be completely ignored; in the very least,\n this is a formalization for deprecating services.\n2\xE8\x01 Note:  Field numbers 1 through 32 are reserved for Google's internal RPC\n   framework.  We apologize for hoarding these numbers to ourselves, but\n   we were already using them long before we decided to release Protocol\n   Buffers.\n\n\r\n\x05\x04\x10\x02\0\x04\x12\x04\xA9\x05\x02\n\n\r\n\x05\x04\x10\x02\0\x05\x12\x04\xA9\x05\x0B\x0F\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xA9\x05\x10\x1A\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\xA9\x05\x1D\x1F\n\r\n\x05\x04\x10\x02\0\x08\x12\x04\xA9\x05 1\n\r\n\x05\x04\x10\x02\0\x07\x12\x04\xA9\x05+0\nO\n\x04\x04\x10\x02\x01\x12\x04\xAC\x05\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x10\x02\x01\x04\x12\x04\xAC\x05\x02\n\n\r\n\x05\x04\x10\x02\x01\x06\x12\x04\xAC\x05\x0B\x1E\n\r\n\x05\x04\x10\x02\x01\x01\x12\x04\xAC\x05\x1F3\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\xAC\x0569\nZ\n\x03\x04\x10\x05\x12\x04\xAF\x05\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x10\x05\0\x12\x04\xAF\x05\r\x18\n\r\n\x05\x04\x10\x05\0\x01\x12\x04\xAF\x05\r\x11\n\r\n\x05\x04\x10\x05\0\x02\x12\x04\xAF\x05\x15\x18\n\x0C\n\x02\x04\x11\x12\x06\xB2\x05\0\xCF\x05\x01\n\x0B\n\x03\x04\x11\x01\x12\x04\xB2\x05\x08\x15\n\xD6\x03\n\x04\x04\x11\x02\0\x12\x04\xBD\x05\x022\x1A\xDC\x01 Is this method deprecated?\n Depending on the target platform, this can emit Deprecated annotations\n for the method, or it will be completely ignored; in the very least,\n this is a formalization for deprecating methods.\n2\xE8\x01 Note:  Field numbers 1 through 32 are reserved for Google's internal RPC\n   framework.  We apologize for hoarding these numbers to ourselves, but\n   we were already using them long before we decided to release Protocol\n   Buffers.\n\n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xBD\x05\x02\n\n\r\n\x05\x04\x11\x02\0\x05\x12\x04\xBD\x05\x0B\x0F\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xBD\x05\x10\x1A\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xBD\x05\x1D\x1F\n\r\n\x05\x04\x11\x02\0\x08\x12\x04\xBD\x05 1\n\r\n\x05\x04\x11\x02\0\x07\x12\x04\xBD\x05+0\n\xF0\x01\n\x04\x04\x11\x04\0\x12\x06\xC2\x05\x02\xC6\x05\x03\x1A\xDF\x01 Is this method side-effect-free (or safe in HTTP parlance), or idempotent,\n or neither? HTTP based RPC implementation may choose GET verb for safe\n methods, and PUT verb for idempotent methods instead of the default POST.\n\n\r\n\x05\x04\x11\x04\0\x01\x12\x04\xC2\x05\x07\x17\n\x0E\n\x06\x04\x11\x04\0\x02\0\x12\x04\xC3\x05\x04\x1C\n\x0F\n\x07\x04\x11\x04\0\x02\0\x01\x12\x04\xC3\x05\x04\x17\n\x0F\n\x07\x04\x11\x04\0\x02\0\x02\x12\x04\xC3\x05\x1A\x1B\n$\n\x06\x04\x11\x04\0\x02\x01\x12\x04\xC4\x05\x04\x18\"\x14 implies idempotent\n\n\x0F\n\x07\x04\x11\x04\0\x02\x01\x01\x12\x04\xC4\x05\x04\x13\n\x0F\n\x07\x04\x11\x04\0\x02\x01\x02\x12\x04\xC4\x05\x16\x17\n7\n\x06\x04\x11\x04\0\x02\x02\x12\x04\xC5\x05\x04\x13\"' idempotent, but may have side effects\n\n\x0F\n\x07\x04\x11\x04\0\x02\x02\x01\x12\x04\xC5\x05\x04\x0E\n\x0F\n\x07\x04\x11\x04\0\x02\x02\x02\x12\x04\xC5\x05\x11\x12\n\x0E\n\x04\x04\x11\x02\x01\x12\x06\xC7\x05\x02\xC8\x05&\n\r\n\x05\x04\x11\x02\x01\x04\x12\x04\xC7\x05\x02\n\n\r\n\x05\x04\x11\x02\x01\x06\x12\x04\xC7\x05\x0B\x1B\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xC7\x05\x1C-\n\r\n\x05\x04\x11\x02\x01\x03\x12\x04\xC7\x0502\n\r\n\x05\x04\x11\x02\x01\x08\x12\x04\xC8\x05\x06%\n\r\n\x05\x04\x11\x02\x01\x07\x12\x04\xC8\x05\x11$\nO\n\x04\x04\x11\x02\x02\x12\x04\xCB\x05\x02:\x1AA The parser stores options it doesn't recognize here. See above.\n\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\xCB\x05\x02\n\n\r\n\x05\x04\x11\x02\x02\x06\x12\x04\xCB\x05\x0B\x1E\n\r\n\x05\x04\x11\x02\x02\x01\x12\x04\xCB\x05\x1F3\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\xCB\x0569\nZ\n\x03\x04\x11\x05\x12\x04\xCE\x05\x02\x19\x1AM Clients can define custom options in extensions of this message. See above.\n\n\x0C\n\x04\x04\x11\x05\0\x12\x04\xCE\x05\r\x18\n\r\n\x05\x04\x11\x05\0\x01\x12\x04\xCE\x05\r\x11\n\r\n\x05\x04\x11\x05\0\x02\x12\x04\xCE\x05\x15\x18\n\x8B\x03\n\x02\x04\x12\x12\x06\xD8\x05\0\xEC\x05\x01\x1A\xFC\x02 A message representing a option the parser does not recognize. This only\n appears in options protos created by the compiler::Parser class.\n DescriptorPool resolves these when building Descriptor objects. Therefore,\n options protos in descriptor objects (e.g. returned by Descriptor::options(),\n or produced by Descriptor::CopyTo()) will never have UninterpretedOptions\n in them.\n\n\x0B\n\x03\x04\x12\x01\x12\x04\xD8\x05\x08\x1B\n\xCB\x02\n\x04\x04\x12\x03\0\x12\x06\xDE\x05\x02\xE1\x05\x03\x1A\xBA\x02 The name of the uninterpreted option.  Each string represents a segment in\n a dot-separated name.  is_extension is true iff a segment represents an\n extension (denoted with parentheses in options specs in .proto files).\n E.g.,{ [\"foo\", false], [\"bar.baz\", true], [\"qux\", false] } represents\n \"foo.(bar.baz).qux\".\n\n\r\n\x05\x04\x12\x03\0\x01\x12\x04\xDE\x05\n\x12\n\x0E\n\x06\x04\x12\x03\0\x02\0\x12\x04\xDF\x05\x04\"\n\x0F\n\x07\x04\x12\x03\0\x02\0\x04\x12\x04\xDF\x05\x04\x0C\n\x0F\n\x07\x04\x12\x03\0\x02\0\x05\x12\x04\xDF\x05\r\x13\n\x0F\n\x07\x04\x12\x03\0\x02\0\x01\x12\x04\xDF\x05\x14\x1D\n\x0F\n\x07\x04\x12\x03\0\x02\0\x03\x12\x04\xDF\x05 !\n\x0E\n\x06\x04\x12\x03\0\x02\x01\x12\x04\xE0\x05\x04#\n\x0F\n\x07\x04\x12\x03\0\x02\x01\x04\x12\x04\xE0\x05\x04\x0C\n\x0F\n\x07\x04\x12\x03\0\x02\x01\x05\x12\x04\xE0\x05\r\x11\n\x0F\n\x07\x04\x12\x03\0\x02\x01\x01\x12\x04\xE0\x05\x12\x1E\n\x0F\n\x07\x04\x12\x03\0\x02\x01\x03\x12\x04\xE0\x05!\"\n\x0C\n\x04\x04\x12\x02\0\x12\x04\xE2\x05\x02\x1D\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xE2\x05\x02\n\n\r\n\x05\x04\x12\x02\0\x06\x12\x04\xE2\x05\x0B\x13\n\r\n\x05\x04\x12\x02\0\x01\x12\x04\xE2\x05\x14\x18\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xE2\x05\x1B\x1C\n\x9C\x01\n\x04\x04\x12\x02\x01\x12\x04\xE6\x05\x02'\x1A\x8D\x01 The value of the uninterpreted option, in whatever type the tokenizer\n identified it as during parsing. Exactly one of these should be set.\n\n\r\n\x05\x04\x12\x02\x01\x04\x12\x04\xE6\x05\x02\n\n\r\n\x05\x04\x12\x02\x01\x05\x12\x04\xE6\x05\x0B\x11\n\r\n\x05\x04\x12\x02\x01\x01\x12\x04\xE6\x05\x12\"\n\r\n\x05\x04\x12\x02\x01\x03\x12\x04\xE6\x05%&\n\x0C\n\x04\x04\x12\x02\x02\x12\x04\xE7\x05\x02)\n\r\n\x05\x04\x12\x02\x02\x04\x12\x04\xE7\x05\x02\n\n\r\n\x05\x04\x12\x02\x02\x05\x12\x04\xE7\x05\x0B\x11\n\r\n\x05\x04\x12\x02\x02\x01\x12\x04\xE7\x05\x12$\n\r\n\x05\x04\x12\x02\x02\x03\x12\x04\xE7\x05'(\n\x0C\n\x04\x04\x12\x02\x03\x12\x04\xE8\x05\x02(\n\r\n\x05\x04\x12\x02\x03\x04\x12\x04\xE8\x05\x02\n\n\r\n\x05\x04\x12\x02\x03\x05\x12\x04\xE8\x05\x0B\x10\n\r\n\x05\x04\x12\x02\x03\x01\x12\x04\xE8\x05\x11#\n\r\n\x05\x04\x12\x02\x03\x03\x12\x04\xE8\x05&'\n\x0C\n\x04\x04\x12\x02\x04\x12\x04\xE9\x05\x02#\n\r\n\x05\x04\x12\x02\x04\x04\x12\x04\xE9\x05\x02\n\n\r\n\x05\x04\x12\x02\x04\x05\x12\x04\xE9\x05\x0B\x11\n\r\n\x05\x04\x12\x02\x04\x01\x12\x04\xE9\x05\x12\x1E\n\r\n\x05\x04\x12\x02\x04\x03\x12\x04\xE9\x05!\"\n\x0C\n\x04\x04\x12\x02\x05\x12\x04\xEA\x05\x02\"\n\r\n\x05\x04\x12\x02\x05\x04\x12\x04\xEA\x05\x02\n\n\r\n\x05\x04\x12\x02\x05\x05\x12\x04\xEA\x05\x0B\x10\n\r\n\x05\x04\x12\x02\x05\x01\x12\x04\xEA\x05\x11\x1D\n\r\n\x05\x04\x12\x02\x05\x03\x12\x04\xEA\x05 !\n\x0C\n\x04\x04\x12\x02\x06\x12\x04\xEB\x05\x02&\n\r\n\x05\x04\x12\x02\x06\x04\x12\x04\xEB\x05\x02\n\n\r\n\x05\x04\x12\x02\x06\x05\x12\x04\xEB\x05\x0B\x11\n\r\n\x05\x04\x12\x02\x06\x01\x12\x04\xEB\x05\x12!\n\r\n\x05\x04\x12\x02\x06\x03\x12\x04\xEB\x05$%\n\xDA\x01\n\x02\x04\x13\x12\x06\xF3\x05\0\xF4\x06\x01\x1Aj Encapsulates information about the original source file from which a\n FileDescriptorProto was generated.\n2` ===================================================================\n Optional source code info\n\n\x0B\n\x03\x04\x13\x01\x12\x04\xF3\x05\x08\x16\n\x82\x11\n\x04\x04\x13\x02\0\x12\x04\x9F\x06\x02!\x1A\xF3\x10 A Location identifies a piece of source code in a .proto file which\n corresponds to a particular definition.  This information is intended\n to be useful to IDEs, code indexers, documentation generators, and similar\n tools.\n\n For example, say we have a file like:\n   message Foo {\n     optional string foo = 1;\n   }\n Let's look at just the field definition:\n   optional string foo = 1;\n   ^       ^^     ^^  ^  ^^^\n   a       bc     de  f  ghi\n We have the following locations:\n   span   path               represents\n   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.\n   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).\n   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).\n   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).\n   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).\n\n Notes:\n - A location may refer to a repeated field itself (i.e. not to any\n   particular index within it).  This is used whenever a set of elements are\n   logically enclosed in a single code segment.  For example, an entire\n   extend block (possibly containing multiple extension definitions) will\n   have an outer location whose path refers to the \"extensions\" repeated\n   field without an index.\n - Multiple locations may have the same path.  This happens when a single\n   logical declaration is spread out across multiple places.  The most\n   obvious example is the \"extend\" block again -- there may be multiple\n   extend blocks in the same scope, each of which will have the same path.\n - A location's span is not always a subset of its parent's span.  For\n   example, the \"extendee\" of an extension declaration appears at the\n   beginning of the \"extend\" block and is shared by all extensions within\n   the block.\n - Just because a location's span is a subset of some other location's span\n   does not mean that it is a descendant.  For example, a \"group\" defines\n   both a type and a field in a single declaration.  Thus, the locations\n   corresponding to the type and field and their components will overlap.\n - Code which tries to interpret locations should probably be designed to\n   ignore those that it doesn't understand, as more types of locations could\n   be recorded in the future.\n\n\r\n\x05\x04\x13\x02\0\x04\x12\x04\x9F\x06\x02\n\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\x9F\x06\x0B\x13\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\x9F\x06\x14\x1C\n\r\n\x05\x04\x13\x02\0\x03\x12\x04\x9F\x06\x1F \n\x0E\n\x04\x04\x13\x03\0\x12\x06\xA0\x06\x02\xF3\x06\x03\n\r\n\x05\x04\x13\x03\0\x01\x12\x04\xA0\x06\n\x12\n\x83\x07\n\x06\x04\x13\x03\0\x02\0\x12\x04\xB8\x06\x04,\x1A\xF2\x06 Identifies which part of the FileDescriptorProto was defined at this\n location.\n\n Each element is a field number or an index.  They form a path from\n the root FileDescriptorProto to the place where the definition.  For\n example, this path:\n   [ 4, 3, 2, 7, 1 ]\n refers to:\n   file.message_type(3)  // 4, 3\n       .field(7)         // 2, 7\n       .name()           // 1\n This is because FileDescriptorProto.message_type has field number 4:\n   repeated DescriptorProto message_type = 4;\n and DescriptorProto.field has field number 2:\n   repeated FieldDescriptorProto field = 2;\n and FieldDescriptorProto.name has field number 1:\n   optional string name = 1;\n\n Thus, the above path gives the location of a field name.  If we removed\n the last element:\n   [ 4, 3, 2, 7 ]\n this path refers to the whole field declaration (from the beginning\n of the label to the terminating semicolon).\n\n\x0F\n\x07\x04\x13\x03\0\x02\0\x04\x12\x04\xB8\x06\x04\x0C\n\x0F\n\x07\x04\x13\x03\0\x02\0\x05\x12\x04\xB8\x06\r\x12\n\x0F\n\x07\x04\x13\x03\0\x02\0\x01\x12\x04\xB8\x06\x13\x17\n\x0F\n\x07\x04\x13\x03\0\x02\0\x03\x12\x04\xB8\x06\x1A\x1B\n\x0F\n\x07\x04\x13\x03\0\x02\0\x08\x12\x04\xB8\x06\x1C+\n\x10\n\x08\x04\x13\x03\0\x02\0\x08\x02\x12\x04\xB8\x06\x1D*\n\xD2\x02\n\x06\x04\x13\x03\0\x02\x01\x12\x04\xBF\x06\x04,\x1A\xC1\x02 Always has exactly three or four elements: start line, start column,\n end line (optional, otherwise assumed same as start line), end column.\n These are packed into a single field for efficiency.  Note that line\n and column numbers are zero-based -- typically you will want to add\n 1 to each before displaying to a user.\n\n\x0F\n\x07\x04\x13\x03\0\x02\x01\x04\x12\x04\xBF\x06\x04\x0C\n\x0F\n\x07\x04\x13\x03\0\x02\x01\x05\x12\x04\xBF\x06\r\x12\n\x0F\n\x07\x04\x13\x03\0\x02\x01\x01\x12\x04\xBF\x06\x13\x17\n\x0F\n\x07\x04\x13\x03\0\x02\x01\x03\x12\x04\xBF\x06\x1A\x1B\n\x0F\n\x07\x04\x13\x03\0\x02\x01\x08\x12\x04\xBF\x06\x1C+\n\x10\n\x08\x04\x13\x03\0\x02\x01\x08\x02\x12\x04\xBF\x06\x1D*\n\xA5\x0C\n\x06\x04\x13\x03\0\x02\x02\x12\x04\xF0\x06\x04)\x1A\x94\x0C If this SourceCodeInfo represents a complete declaration, these are any\n comments appearing before and after the declaration which appear to be\n attached to the declaration.\n\n A series of line comments appearing on consecutive lines, with no other\n tokens appearing on those lines, will be treated as a single comment.\n\n leading_detached_comments will keep paragraphs of comments that appear\n before (but not connected to) the current element. Each paragraph,\n separated by empty lines, will be one comment element in the repeated\n field.\n\n Only the comment content is provided; comment markers (e.g. //) are\n stripped out.  For block comments, leading whitespace and an asterisk\n will be stripped from the beginning of each line other than the first.\n Newlines are included in the output.\n\n Examples:\n\n   optional int32 foo = 1;  // Comment attached to foo.\n   // Comment attached to bar.\n   optional int32 bar = 2;\n\n   optional string baz = 3;\n   // Comment attached to baz.\n   // Another line attached to baz.\n\n   // Comment attached to qux.\n   //\n   // Another line attached to qux.\n   optional double qux = 4;\n\n   // Detached comment for corge. This is not leading or trailing comments\n   // to qux or corge because there are blank lines separating it from\n   // both.\n\n   // Detached comment for corge paragraph 2.\n\n   optional string corge = 5;\n   /* Block comment attached\n    * to corge.  Leading asterisks\n    * will be removed. */\n   /* Block comment attached to\n    * grault. */\n   optional int32 grault = 6;\n\n   // ignored detached comments.\n\n\x0F\n\x07\x04\x13\x03\0\x02\x02\x04\x12\x04\xF0\x06\x04\x0C\n\x0F\n\x07\x04\x13\x03\0\x02\x02\x05\x12\x04\xF0\x06\r\x13\n\x0F\n\x07\x04\x13\x03\0\x02\x02\x01\x12\x04\xF0\x06\x14$\n\x0F\n\x07\x04\x13\x03\0\x02\x02\x03\x12\x04\xF0\x06'(\n\x0E\n\x06\x04\x13\x03\0\x02\x03\x12\x04\xF1\x06\x04*\n\x0F\n\x07\x04\x13\x03\0\x02\x03\x04\x12\x04\xF1\x06\x04\x0C\n\x0F\n\x07\x04\x13\x03\0\x02\x03\x05\x12\x04\xF1\x06\r\x13\n\x0F\n\x07\x04\x13\x03\0\x02\x03\x01\x12\x04\xF1\x06\x14%\n\x0F\n\x07\x04\x13\x03\0\x02\x03\x03\x12\x04\xF1\x06()\n\x0E\n\x06\x04\x13\x03\0\x02\x04\x12\x04\xF2\x06\x042\n\x0F\n\x07\x04\x13\x03\0\x02\x04\x04\x12\x04\xF2\x06\x04\x0C\n\x0F\n\x07\x04\x13\x03\0\x02\x04\x05\x12\x04\xF2\x06\r\x13\n\x0F\n\x07\x04\x13\x03\0\x02\x04\x01\x12\x04\xF2\x06\x14-\n\x0F\n\x07\x04\x13\x03\0\x02\x04\x03\x12\x04\xF2\x0601\n\xEE\x01\n\x02\x04\x14\x12\x06\xF9\x06\0\x8E\x07\x01\x1A\xDF\x01 Describes the relationship between generated code and its original source\n file. A GeneratedCodeInfo message is associated with only one generated\n source file, but may contain references to different source .proto files.\n\n\x0B\n\x03\x04\x14\x01\x12\x04\xF9\x06\x08\x19\nx\n\x04\x04\x14\x02\0\x12\x04\xFC\x06\x02%\x1Aj An Annotation connects some span of text in generated code to an element\n of its generating .proto file.\n\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\xFC\x06\x02\n\n\r\n\x05\x04\x14\x02\0\x06\x12\x04\xFC\x06\x0B\x15\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\xFC\x06\x16 \n\r\n\x05\x04\x14\x02\0\x03\x12\x04\xFC\x06#$\n\x0E\n\x04\x04\x14\x03\0\x12\x06\xFD\x06\x02\x8D\x07\x03\n\r\n\x05\x04\x14\x03\0\x01\x12\x04\xFD\x06\n\x14\n\x8F\x01\n\x06\x04\x14\x03\0\x02\0\x12\x04\x80\x07\x04,\x1A\x7F Identifies the element in the original source .proto file. This field\n is formatted the same as SourceCodeInfo.Location.path.\n\n\x0F\n\x07\x04\x14\x03\0\x02\0\x04\x12\x04\x80\x07\x04\x0C\n\x0F\n\x07\x04\x14\x03\0\x02\0\x05\x12\x04\x80\x07\r\x12\n\x0F\n\x07\x04\x14\x03\0\x02\0\x01\x12\x04\x80\x07\x13\x17\n\x0F\n\x07\x04\x14\x03\0\x02\0\x03\x12\x04\x80\x07\x1A\x1B\n\x0F\n\x07\x04\x14\x03\0\x02\0\x08\x12\x04\x80\x07\x1C+\n\x10\n\x08\x04\x14\x03\0\x02\0\x08\x02\x12\x04\x80\x07\x1D*\nO\n\x06\x04\x14\x03\0\x02\x01\x12\x04\x83\x07\x04$\x1A? Identifies the filesystem path to the original source .proto.\n\n\x0F\n\x07\x04\x14\x03\0\x02\x01\x04\x12\x04\x83\x07\x04\x0C\n\x0F\n\x07\x04\x14\x03\0\x02\x01\x05\x12\x04\x83\x07\r\x13\n\x0F\n\x07\x04\x14\x03\0\x02\x01\x01\x12\x04\x83\x07\x14\x1F\n\x0F\n\x07\x04\x14\x03\0\x02\x01\x03\x12\x04\x83\x07\"#\nw\n\x06\x04\x14\x03\0\x02\x02\x12\x04\x87\x07\x04\x1D\x1Ag Identifies the starting offset in bytes in the generated code\n that relates to the identified object.\n\n\x0F\n\x07\x04\x14\x03\0\x02\x02\x04\x12\x04\x87\x07\x04\x0C\n\x0F\n\x07\x04\x14\x03\0\x02\x02\x05\x12\x04\x87\x07\r\x12\n\x0F\n\x07\x04\x14\x03\0\x02\x02\x01\x12\x04\x87\x07\x13\x18\n\x0F\n\x07\x04\x14\x03\0\x02\x02\x03\x12\x04\x87\x07\x1B\x1C\n\xDB\x01\n\x06\x04\x14\x03\0\x02\x03\x12\x04\x8C\x07\x04\x1B\x1A\xCA\x01 Identifies the ending offset in bytes in the generated code that\n relates to the identified offset. The end offset should be one past\n the last relevant byte (so the length of the text = end - begin).\n\n\x0F\n\x07\x04\x14\x03\0\x02\x03\x04\x12\x04\x8C\x07\x04\x0C\n\x0F\n\x07\x04\x14\x03\0\x02\x03\x05\x12\x04\x8C\x07\r\x12\n\x0F\n\x07\x04\x14\x03\0\x02\x03\x01\x12\x04\x8C\x07\x13\x16\n\x0F\n\x07\x04\x14\x03\0\x02\x03\x03\x12\x04\x8C\x07\x19\x1A" ;
pub static DESCRIPTOR: crate::Bytes = crate::Bytes::from_static(DESCRIPTOR_RAW);
