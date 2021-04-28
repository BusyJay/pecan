use crate::util::*;
use crate::Generator;
use bytes::Bytes;
use pecan::prelude::*;
use pecan::reflection::*;
use pecan_types::google::protobuf;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct File {
    module_prefix: String,
    target: String,
    proto: FileDescriptorProto,
    proto3: bool,
}

impl File {
    fn new(p: FileDescriptorProto, mut module_prefix: String) -> File {
        let proto_path = p.name();
        // TODO: load crate name from file.
        let module = rust_module(proto_path);
        if module_prefix.is_empty() {
            module_prefix = format!("crate::{}", module.join("::"));
        }
        File {
            target: target_path(proto_path),
            module_prefix,
            proto3: p.syntax() == "proto3",
            proto: p,
        }
    }

    pub fn proto(&self) -> &FileDescriptorProto {
        &self.proto
    }

    pub fn target_path(&self) -> &str {
        &self.target
    }

    pub fn full_package(&self) -> &str {
        &self.module_prefix
    }

    pub fn proto3(&self) -> bool {
        self.proto3
    }
}

#[derive(Debug, Clone)]
pub enum Proto {
    Enum(EnumDescriptorProto),
    Message(DescriptorProto),
}

#[derive(Debug, Clone)]
pub struct TypeReference {
    package: String,
    name: String,
    proto: Proto,
    group: i32,
}

impl TypeReference {
    pub fn new(package: impl Into<String>, name: String, proto: Proto) -> TypeReference {
        TypeReference {
            package: package.into(),
            name,
            proto,
            group: 0,
        }
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn message(&self) -> Option<&DescriptorProto> {
        match &self.proto {
            Proto::Message(m) => Some(m),
            _ => None,
        }
    }

    pub fn group(&self) -> i32 {
        self.group
    }
}

#[derive(Debug, Clone, Default)]
pub struct Database {
    // package name -> type name.
    types: HashMap<String, TypeReference>,
    files: HashMap<String, File>,
}

impl Database {
    fn load_descriptor(&mut self, descriptor: &Bytes, pkg: impl Into<String>) {
        let mut file = FileDescriptorProto::new();
        file.merge_from_buf(&mut descriptor.clone()).unwrap();
        self.load_impl(file, pkg.into());
    }

    pub fn load_reflection_descriptor(&mut self) {
        self.load_descriptor(&pecan::reflection::DESCRIPTOR, "pecan::reflection");
    }

    pub fn load_complier_descriptor(&mut self) {
        self.load_descriptor(&crate::options_pb::DESCRIPTOR, "pecan_compiler::options_pb");
        self.load_descriptor(&crate::plugin_pb::DESCRIPTOR, "pecan_compiler::plugin_pb");
    }

    pub fn load_well_known_descriptor(&mut self) {
        let well_known_descriptors = &[
            (&protobuf::any_pb::DESCRIPTOR, "any_pb"),
            (&protobuf::api_pb::DESCRIPTOR, "api_pb"),
            (&protobuf::duration_pb::DESCRIPTOR, "duration_pb"),
            (&protobuf::empty_pb::DESCRIPTOR, "empty_pb"),
            (&protobuf::field_mask_pb::DESCRIPTOR, "field_mask_pb"),
            (
                &protobuf::source_context_pb::DESCRIPTOR,
                "source_context_pb",
            ),
            (&protobuf::struct_pb::DESCRIPTOR, "struct_pb"),
            (&protobuf::timestamp_pb::DESCRIPTOR, "timestamp_pb"),
            (&protobuf::type_pb::DESCRIPTOR, "type_pb"),
            (&protobuf::wrappers_pb::DESCRIPTOR, "wrappers_pb"),
        ];
        for (desc, pkg) in well_known_descriptors {
            self.load_descriptor(desc, format!("pecan_types::google::protobuf::{}", pkg));
        }
    }

    pub fn load(&mut self, file: FileDescriptorProto) {
        self.load_impl(file, String::new());
    }

    fn load_impl(&mut self, file: FileDescriptorProto, module_prefix: String) {
        if self.files.contains_key(file.name()) {
            return;
        }
        let package_prefix = package_prefix(&file);
        let f = File::new(file, module_prefix);
        let proto_path = f.proto.name();
        for e in &f.proto.enum_type {
            self.register_enum(&package_prefix, &f, e);
        }
        for e in &f.proto.message_type {
            self.register_message(&package_prefix, &f, e);
        }
        assert!(self.files.insert(proto_path.to_string(), f).is_none());
    }

    fn register_enum(&mut self, prefix: &str, file: &File, e: &EnumDescriptorProto) {
        self.register_enum_impl(prefix, "", file, e)
    }

    fn register_enum_impl(
        &mut self,
        prefix: &str,
        ty_prefix: &str,
        file: &File,
        e: &EnumDescriptorProto,
    ) {
        let enum_name = e.name();
        let ty_name = type_name(enum_name, ty_prefix);
        let full_name = format!("{}{}", prefix, enum_name);
        assert!(self
            .types
            .insert(
                full_name,
                TypeReference::new(file.full_package(), ty_name, Proto::Enum(e.clone()))
            )
            .is_none());
    }

    fn register_message(&mut self, prefix: &str, file: &File, e: &DescriptorProto) {
        self.register_message_impl(prefix, "", file, e)
    }

    fn register_message_impl(
        &mut self,
        prefix: &str,
        ty_prefix: &str,
        file: &File,
        e: &DescriptorProto,
    ) {
        let msg_name = e.name();
        let ty_name = type_name(msg_name, ty_prefix);
        let full_name = format!("{}{}", prefix, msg_name);
        assert!(self
            .types
            .insert(
                full_name.clone(),
                TypeReference::new(
                    file.full_package(),
                    ty_name.clone(),
                    Proto::Message(e.clone())
                )
            )
            .is_none(),);

        let sub_prefix = full_name + ".";
        for m in &e.nested_type {
            self.register_message_impl(&sub_prefix, &ty_name, file, m);
        }
        for e in &e.enum_type {
            self.register_enum_impl(&sub_prefix, &ty_name, file, e);
        }
        if !e.nested_type.is_empty() {
            for f in &e.field {
                if f.r#type() == FieldDescriptorProto_Type::TYPE_GROUP {
                    self.types.get_mut(f.type_name()).unwrap().group = f.number();
                }
            }
        }
    }

    pub fn generator_for(&self, path: &str) -> Option<Generator> {
        let file = self.files.get(path)?;
        Some(Generator::new(self, file))
    }

    pub fn r#type(&self, name: &str) -> Option<&TypeReference> {
        self.types.get(name)
    }
}
