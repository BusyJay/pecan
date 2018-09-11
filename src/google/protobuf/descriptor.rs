use std::io::BufRead;
use std::cell::Cell;
use ::{codec, Error, Result};

#[derive(Default, Clone, PartialEq)]
pub struct UninterpretedOptionNamePart {
    pub name_part: String,
    pub is_extension: bool,

    cache_size: Cell<u32>,
}

impl UninterpretedOptionNamePart {
    pub fn merge_from(&mut self, data: &mut &[u8]) -> Result<()> {
        let (mut number, mut wired_id) = (0, 0);
        check_return_early!(data, number, wired_id);
        loop {
            if number == 1 {
                self.name_part = codec::decode_wired_str(data, 1, wired_id)?.to_string();
                check_return_early!(data, number, wired_id);
            }
            if number == 2 {
                self.is_extension = codec::decode_wired_bool(data, 2, wired_id)?;
                check_return_early!(data, number, wired_id);
            }
            if number != 1 && number != 2 {
                decode_unknown_tag(data, wired_id);
                check_return_early!(data, number, wired_id);
            }
        }
    }

    pub fn size(&self) -> u32 {
        let mut cnt = 0;
        if !self.name_part.is_empty() {
            cnt += 1 + codec::str_bytes_len(self.name_part);
        }
        if self.is_extension {
            cnt += 1 + 1;
        }
        self.cache_size.set(cnt);
        cnt
    }

    pub unsafe fn encode_with_cached_size(&self, buf: &mut Vec<u8>) {
        let size = self.cache_size.get();
        if size != 0 {
            codec::encode_varint_u32(buf, size);
            self.encode(buf);
        }
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        if !self.name_part.is_empty() {
            codec::wired_str(buf, 1, &self.name_part);
        }
        if self.is_extension {
            codec::wired_bool(buf, 2, true);
        }
    }

    #[inline]
    pub fn serialize_to(&self, buf: &mut Vec<u8>) {
        if self.size() != 0 {
            self.encode(buf);
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct UninterpretedOption {
    pub name: Vec<UninterpretedOptionNamePart>,
    pub identifier_value: String,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: Vec<u8>,
    pub aggregate_value: String,

    cache_size: Cell<u32>,
    name_cache_size: Cell<u32>,
}

impl UninterpretedOption {
    pub fn merge_from(&mut self, data: &mut &[u8]) -> Result<()> {
        let (mut number, mut wired_id) = (0, 0);
        check_return_early!(data, number, wired_id);
        loop {
            if number == 2 {
                let mut bytes = codec::decode_wired_bytes(data, 1, wired_id)?;
                while !bytes.is_empty() {
                    let mut msg_bytes = codec::decode_bytes(&mut bytes)?;
                    let mut msg = UninterpretedOptionNamePart::default();
                    msg.merge_from(&mut msg_bytes)?;
                    self.name.push(msg);
                }
                check_return_early!(data, number, wired_id);
            }
            if number == 3 {
                self.identifier_value = codec::decode_wired_str(data, 3, wired_id)?.to_string();
                check_return_early!(data, number, wired_id);
            }
            if number == 4 {
                self.positive_int_value = codec::decode_wired_varint_u64(data, 4, wired_id)?;
                check_return_early!(data, number, wired_id);
            }
            if number == 5 {
                self.negative_int_value = codec::decode_wired_varint_i64(data, 5, vired_id)?;
                check_return_early!(data, number, wired_id);
            }
            if number == 6 {
                self.double_value = codec::decode_wired_f64(data, 6, wired_id)?;
                check_return_early!(data, number, wired_id);
            }
            if number == 7 {
                self.string_value = codec::decode_wired_bytes(data, 7, wired_id)?.to_vec();
                check_return_early!(data, number, wired_id);
            }
            if number == 8 {
                self.aggregate_value = codec::decode_wired_str(data, 8, wired_id)?.to_string();
                check_return_early!(data, number, wired_id);
            }
            if number < 2 || number > 8 {
                decode_unknown_tag(data, wired_id);
                check_return_early!(data, number, wired_id);
            }
        }
    }

    pub fn size(&self) -> u32 {
        let mut cnt = 0;
        if !self.name.is_empty() {
            cnt += 1;
            let mut tmp = 0;
            for n in self.name {
                tmp += n.size();
            }
            self.name_cache_size.set(tmp);
            cnt += codec::varint_u32_bytes_len(tmp);
            cnt += tmp;
        }
        if !self.identifier_value.is_empty() {
            cnt += 1 + codec::str_bytes_len(&self.identifier_value);
        }
        if self.positive_int_value != 0 {
            cnt += 1 + codec::varint_u64_bytes_len(self.positive_int_value);
        }
        if self.negative_int_value != 0 {
            cnt += 1 + codec::varint_i64_bytes_len(self.negative_int_value);
        }
        if self.double_value != 0.0 {
            cnt += 1 + codec::f64_bytes_len(self.double_value);
        }
        if !self.string_value.is_empty() {
            cnt += 1 + codec::bytes_len(&self.string_value);
        }
        if !self.aggregate_value.is_empty() {
            cnt += 1 + codec::str_bytes_len(&self.aggregate_value);
        }
        
        self.cache_size.set(cnt);
        cnt
    }

    pub unsafe fn encode_with_cached_size(&self, buf: &mut Vec<u8>) {
        let size = self.cache_size.get();
        if size != 0 {
            codec::encode_varint_u32(buf, size);
            self.encode(buf);
        }
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        if !self.name.is_empty() {
            let tag = codec::encode_tag(2, 2);
            buf.push(tag);
            codec::encode_varint_u32(self.name_cache_size.get());
            for n in &self.name {
                unsafe {
                    n.encode_with_cached_size(buf);
                }
            }
        }
        if !self.identifier_value.is_empty() {
            codec::wired_str(buf, 3, &self.identifier_value);
        }
        if self.positive_int_value != 0 {
            codec::wired_varint_u64(buf, 4, self.positive_int_value);
        }
        if self.negative_int_value != 0 {
            codec::wired_varint_i64(buf, 5, self.negative_int_value);
        }
        if self.double_value != 0.0 {
            codec::wired_fixed_f64(buf, 6, self.double_value);
        }
        if !self.string_value.is_empty() {
            codec::wired_bytes(buf, 7, &self.string_value);
        }
        if !self.aggregate_value.is_empty() {
            codec::wired_str(buf, 8, &self.aggregate_value);
        }
    }

    pub fn serialize_to(&self, buf: &mut Vec<u8>) {
        if self.size() != 0 {
            self.encode(buf);
        }
    }
}

pub enum FieldOptionsCType {
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl FieldOptionsCType {
    pub fn merge_from(&mut self, data: &mut &[u8]) -> Result<()> {
        if !data.is_empty() {
            match data[0] {
                1 => *self = FieldOptionsCType::Cord,
                2 => *self = FieldOptionsCType::StringPiece,
                0 => *self = FieldOptionsCType::String,
                n => return Err(Error::invalid_data());
            }
        } else {
            *self = FieldOptionsCType::String;
        }
        Ok(())
    }

    pub fn size(&self) -> u32 {
        match *self {
            FieldOptionsCType::String => 0,
            _ => 1,
        }
    }

    pub unsafe fn encode_with_cached_size(&self, buf: &mut Vec<u8>) {
        let size = self.size();
        if size != 0 {
            codec::encode_varint_u32(buf, size);
            self.encode(buf);
        }
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        match *self {
            FieldOptionsCType::Cord => buf.push(1),
            FieldOptionsCType::StringPiece => buf.push(2),
            FieldOptionsCType::String => unreachable!(),
        }
    }

    pub fn serialize_to(&self, buf: &mut Vec<u8>) {
        if self.size() != 0 {
            self.encode(buf);
        }
    }
}

pub enum FieldOptionsJsType {
    JsNormal = 0,
    JsString = 1,
    JsNumber = 2,
}

impl FieldOptionsCType {
    pub fn merge_from(&mut self, data: &mut &[u8]) -> Result<()> {
        if !data.is_empty() {
            match data[0] {
                1 => *self = FieldOptionsJsType::JsString,
                2 => *self = FieldOptionsJsType::JsNumber,
                0 => *self = FieldOptionsJsType::JsNormal,
                n => return Err(Error::invalid_data());
            }
        } else {
            *self = FieldOptionsJsType::JsNormal;
        }
        Ok(())
    }

    pub fn size(&self) -> u32 {
        match *self {
            FieldOptionsJsType::JsNormal => 0,
            _ => 1,
        }
    }

    pub unsafe fn encode_with_cached_size(&self, buf: &mut Vec<u8>) {
        let size = self.size();
        if size != 0 {
            codec::encode_varint_u32(buf, size);
            self.encode(buf);
        }
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        match *self {
            FieldOptionsJsType::JsString => buf.push(1),
            FieldOptionsJsType::JsNumber => buf.push(2),
            FieldOptionsJsType::JsNormal => unreachable!(),
        }
    }

    pub fn serialize_to(&self, buf: &mut Vec<u8>) {
        if self.size() != 0 {
            self.encode(buf);
        }
    }
}

pub struct FieldOptions {
    pub ctype: FieldOptionsCType,
    pub packed: bool,
    pub jstype: JsType,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: UninterpretedOption,
}

pub enum FieldDescriptorProtoType {
    TypeDouble = 1,
    TypeFloat = 2,
    TypeInt64 = 3,
    TypeUInt64 = 4,
    TypeInt32 = 5,
    TypeFixed64 = 6,
    TypeFixed32 = 7,
    TypeBool = 8,
    TypeString = 9,
    TypeGroup = 10,
    TypeMessage = 11,
    TypeBytes = 12,
    TypeUInt32 = 13,
    TypeEnum = 14,
    TypeSFixed32 = 15,
    TypeSFixed64 = 16,
    TypeSInt32 = 17,
    TypeSInt64 = 18,
}

pub enum FieldDescriptorProtoLabel {
    LabelOptional = 1,
    LabelRequired = 2,
    LabelRepeated = 3,
}

pub struct FieldDescriptorProto {
    pub name: String,
    pub number: i32,
    pub label: FieldDescriptorProtoLabel,
    pub type: FieldDescriptorProtoType,
    pub type_name: String,
    pub extendee: String,
    pub default_value: String,
    pub oneof_index: i32,
    pub json_name: String,
    pub options: FieldOptions,
}

pub struct EnumValueOptions {
    pub deprecated: bool,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct EnumValueDescriptorProto {
    pub name: String,
    pub number: i32,
    pub options: EnumValueOptions,
}

pub struct EnumOptions {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct EnumReservedRange {
    pub start: i32,
    pub end: i32,
}

pub struct EnumDescriptorProto {
    pub name: String,
    pub value: EnumValueDescriptorProto,
    pub options: EnumOptions,
    pub reserved_range: EnumReservedRange,
    pub reserved_name: String,
}

pub struct ExtensionRangeOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct ExtensionRange {
    pub start: i32,
    pub end: i32,
    pub options: ExtensionRangeOptions,
}

pub struct OneofOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct OneofDescriptorProto {
    pub name: String,
    pub options: OneofOptions,
}

pub struct MessageOptions {
    pub message_set_wire_format: bool,
    pub no_standard_descriptor_accessor: bool,
    pub deprecated: bool,
    pub map_entry: bool,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct ReservedRange {
    pub start: i32,
    pub end: i32,
}

pub struct DescriptorProto {
    pub name: String,
    pub field: Vec<FieldDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub nested_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub extension_range: Vec<ExtensionRange>,
    pub oneof_decl: Vec<OneofDescriptorProto>,
    pub options: MessageOptions,
    pub reserved_range: Vec<ReservedRange>,
    pub reserved_name: Vec<String>,
}

pub enum MethodOptionsIdempotencyLevel {
    IdempotencyUnknown = 0,
    NoSideEffects = 1,
    Idempotent = 2,
}

pub struct MethodOptions {
    pub deprecated: bool,
    pub idempotency_level: IdempotencyLevel,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct MethodDescriptorProto {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub options: MethodOptions,
    pub client_streaming: bool,
    pub server_string: bool,
}

pub struct ServiceOptions {
    pub deprecated: bool,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct ServiceDescriptorProto {
    pub name: String,
    pub method: Vec<MethodDescriptorProto>,
    pub options: ServiceOptions,
}

pub enum FileOptionsOptimizeMode {
    Speed = 1,
    CodeSize = 2,
    LiteRuntime = 3,
}

pub struct FileOptions {
    pub java_package: String,
    pub java_outer_classname: String,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for: FileOptionsOptimizeMode,
    pub go_package: String,
    pub cc_generic_services: bool,
    pub java_generic_services: bool,
    pub py_generic_services: bool,
    pub php_generic_services: bool,
    pub deprecated: bool,
    pub cc_enable_arenas: bool,
    pub objc_class_prefix: String,
    pub csharp_namespace: String,
    pub swift_prefix: String,
    pub php_class_prefix: String,
    pub php_namespace: String,
    pub php_metadata_namespace: String,
    pub ruby_package: String,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}

pub struct Location {
    pub path: Vec<i32>,
    pub span: Vec<i32>,
    pub leading_comments: String,
    pub trailing_comments: String,
    pub leading_detached_comments: Vec<String>,
}

pub struct SourceCodeInfo {
    pub location: Location,
}

pub struct FileDescriptorProto {
    pub name: String,
    pub package: String,
    pub dependency: Vec<String>,
    pub public_dependency: Vec<i32>,
    pub weak_dependency: Vec<i32>,
    pub message_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub service: Vec<ServiceDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub options: FileOptions,
    pub source_code_info: SourceCodeInfo,
    pub syntax: String,
}

pub struct FileDescriptorSet {
    pub file: Vec<FileDescriptorProto>,
}

pub struct GeneratedCodeInfoAnnotation {
    pub path: Vec<i32>,
    pub source_file: String,
    pub begin: i32,
    pub end: i32,
}

pub struct GeneratedCodeInfo {
    pub annotation: Vec<GeneratedCodeInfoAnnotation>,
}
