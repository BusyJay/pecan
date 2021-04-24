use crate::util::*;
use crate::Generator;
use pecan_types::google::protobuf::descriptor_pb::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct File {
    full_package: String,
    target: String,
    proto: FileDescriptorProto,
    proto3: bool,
}

impl File {
    fn new(p: FileDescriptorProto) -> File {
        let proto_path = p.name.as_ref().unwrap();
        // TODO: load crate name from file.
        let module = rust_module(proto_path);
        let full_package = format!("crate::{}", module.join("::"));
        File {
            target: target_path(proto_path),
            full_package,
            proto3: p.syntax.as_ref().map_or(false, |p| p == "proto3"),
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
        &self.full_package
    }

    pub fn proto3(&self) -> bool {
        self.proto3
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeReference {
    package: String,
    name: String,
}

impl TypeReference {
    pub fn new(package: String, name: String) -> TypeReference {
        TypeReference { package, name }
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Default)]
pub struct Database {
    // package name -> type name.
    types: HashMap<String, TypeReference>,
    files: HashMap<String, File>,
}

impl Database {
    pub fn load(&mut self, file: FileDescriptorProto) {
        let package_prefix = package_prefix(&file);
        let f = File::new(file);
        let proto_path = f.proto.name.as_ref().unwrap();
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
        let enum_name = e.name.as_ref().unwrap();
        let ty_name = type_name(enum_name, ty_prefix);
        let full_name = format!("{}{}", prefix, enum_name);
        assert_eq!(
            self.types.insert(
                full_name,
                TypeReference::new(file.full_package.clone(), ty_name)
            ),
            None
        );
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
        let msg_name = e.name.as_ref().unwrap();
        let ty_name = type_name(msg_name, ty_prefix);
        let full_name = format!("{}{}", prefix, msg_name);
        assert_eq!(
            self.types.insert(
                full_name.clone(),
                TypeReference::new(file.full_package.clone(), ty_name.clone())
            ),
            None
        );

        let sub_prefix = full_name + ".";
        for m in &e.nested_type {
            self.register_message_impl(&sub_prefix, &ty_name, file, m);
        }
        for e in &e.enum_type {
            self.register_enum_impl(&sub_prefix, &ty_name, file, e);
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
