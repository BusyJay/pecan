use crate::pecan_descriptor::google::protobuf::descriptor_pb;
use std::collections::HashMap;
use std::mem;

use id_arena::{Arena, Id};

pub struct Database {
    pub files: Arena<FileDescriptor>,
    pub messages: Arena<MessageDescriptor>,
    pub enums: Arena<EnumDescriptor>,
}

impl Database {
    pub fn with_capacity(capacity: usize) -> Database {
        Database {
            files: Arena::with_capacity(capacity),
            messages: Arena::with_capacity(capacity),
            enums: Arena::with_capacity(capacity),
        }
    }
}

pub struct FileDescriptor {
    pub proto: descriptor_pb::FileDescriptorProto,
    pub messages: Vec<Id<MessageDescriptor>>,
    pub enums: Vec<Id<EnumDescriptor>>,
    pub comments: HashMap<Vec<i32>, descriptor_pb::SourceCodeInfoNestedLocation>,
    pub proto3: bool,
}

impl FileDescriptor {
    pub fn new(db: &mut Database, proto: descriptor_pb::FileDescriptorProto) -> Id<FileDescriptor> {
        let mut comments: HashMap<Vec<i32>, descriptor_pb::SourceCodeInfoNestedLocation> =
            HashMap::default();
        if proto.has_source_code_info() {
            for loc in proto.source_code_info().location() {
                if loc.leading_comments().is_empty() {
                    continue;
                }
                let path = loc.path().to_vec();
                comments.insert(path, loc.clone());
            }
        }
        let mut messages = Vec::with_capacity(proto.message_type().len() + 10);
        let mut enums = Vec::with_capacity(proto.enum_type().len() + 10);
        let fd = db.files.alloc(FileDescriptor {
            proto3: proto.syntax() == "proto3",
            proto,
            messages: Vec::new(),
            enums: Vec::new(),
            comments,
        });
        MessageDescriptor::parse_file(db, &mut messages, fd);
        EnumDescriptor::parse_file(db, &mut enums, fd, &messages);
        let mut f = db.files.get_mut(fd).unwrap();
        f.messages = messages;
        f.enums = enums;
        fd
    }
}

pub struct MessageDescriptor {
    pub file: Id<FileDescriptor>,
    pub proto: descriptor_pb::DescriptorProto,
    pub parent: Option<Id<MessageDescriptor>>,
    pub nested: Vec<Id<MessageDescriptor>>,
    pub enums: Vec<Id<EnumDescriptor>>,
    pub type_name: Vec<String>,
    pub index: usize,
    pub group: bool,
}

impl MessageDescriptor {
    pub fn new(
        db: &mut Database,
        proto: descriptor_pb::DescriptorProto,
        parent: Option<Id<MessageDescriptor>>,
        file: Id<FileDescriptor>,
        index: usize,
    ) -> Id<MessageDescriptor> {
        let mut type_name = Vec::new();
        type_name.push(proto.name().to_owned());
        let mut group = false;
        if let Some(id) = parent {
            let mut p = parent;
            while let Some(id) = p {
                let node = db.messages.get(id).unwrap();
                type_name.push(node.proto.name().to_owned());
                p = node.parent;
            }
            type_name.reverse();
            let full_name = type_name.iter().fold(String::new(), |mut s, part| {
                s.push('.');
                s.push_str(part);
                s
            });
            let node = db.messages.get(id).unwrap();
            for f in node.proto.field() {
                if f.r#type() == descriptor_pb::FieldDescriptorProtoNestedType::TypeGroup
                    && f.type_name() == full_name
                {
                    group = true;
                    break;
                }
            }
        }
        db.messages.alloc(MessageDescriptor {
            file,
            proto,
            parent,
            index,
            nested: vec![],
            enums: vec![],
            type_name,
            group,
        })
    }

    pub fn parse(
        db: &mut Database,
        collector: &mut Vec<Id<MessageDescriptor>>,
        proto: descriptor_pb::DescriptorProto,
        parent: Option<Id<MessageDescriptor>>,
        file: Id<FileDescriptor>,
        index: usize,
    ) -> Id<MessageDescriptor> {
        let desc = MessageDescriptor::new(db, proto, parent, file, index);
        collector.push(desc);
        let nested_types = mem::replace(
            db.messages.get_mut(desc).unwrap().proto.nested_type_mut(),
            vec![],
        );
        let mut nested_ids = Vec::with_capacity(nested_types.len());
        for (i, nested) in nested_types.into_iter().enumerate() {
            let t = MessageDescriptor::parse(db, collector, nested, Some(desc), file, i);
            nested_ids.push(t);
        }
        db.messages.get_mut(desc).unwrap().nested = nested_ids;
        desc
    }

    pub fn parse_file(
        db: &mut Database,
        collector: &mut Vec<Id<MessageDescriptor>>,
        file: Id<FileDescriptor>,
    ) {
        let message_type = mem::replace(
            db.files.get_mut(file).unwrap().proto.message_type_mut(),
            Vec::new(),
        );
        for (i, m) in message_type.into_iter().enumerate() {
            MessageDescriptor::parse(db, collector, m, None, file, i);
        }
    }
}

pub struct EnumDescriptor {
    pub file: Id<FileDescriptor>,
    pub proto: descriptor_pb::EnumDescriptorProto,
    pub parent: Option<Id<MessageDescriptor>>,
    pub type_name: Vec<String>,
    pub index: usize,
}

impl EnumDescriptor {
    pub fn new(
        db: &mut Database,
        proto: descriptor_pb::EnumDescriptorProto,
        parent: Option<Id<MessageDescriptor>>,
        file: Id<FileDescriptor>,
        index: usize,
    ) -> Id<EnumDescriptor> {
        let mut type_name = Vec::new();
        type_name.push(proto.name().to_owned());
        if parent.is_some() {
            let mut p = parent;
            while let Some(id) = p {
                let node = db.messages.get(id).unwrap();
                type_name.push(node.proto.name().to_owned());
                p = node.parent;
            }
            type_name.reverse();
        }
        db.enums.alloc(EnumDescriptor {
            file,
            proto,
            parent,
            type_name,
            index,
        })
    }

    pub fn parse_file(
        db: &mut Database,
        collector: &mut Vec<Id<EnumDescriptor>>,
        file: Id<FileDescriptor>,
        parents: &[Id<MessageDescriptor>],
    ) {
        let enum_type = mem::replace(
            db.files.get_mut(file).unwrap().proto.enum_type_mut(),
            vec![],
        );
        for (i, proto) in enum_type.into_iter().enumerate() {
            collector.push(EnumDescriptor::new(db, proto, None, file, i))
        }
        for nested in parents {
            let enum_type = mem::replace(
                db.messages.get_mut(*nested).unwrap().proto.enum_type_mut(),
                vec![],
            );
            let mut enum_ids = Vec::with_capacity(enum_type.len());
            for (i, proto) in enum_type.into_iter().enumerate() {
                let id = EnumDescriptor::new(db, proto, Some(*nested), file, i);
                collector.push(id);
                enum_ids.push(id);
            }
            db.messages.get_mut(*nested).unwrap().enums = enum_ids;
        }
    }
}
