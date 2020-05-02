use crate::descriptor::{Database, EnumDescriptor, FileDescriptor, MessageDescriptor};
use crate::pecan_descriptor::google::protobuf::compiler::plugin_pb;
use id_arena::Id;
use pecan::Message;
use pecan_utils::naming;
use std::collections::HashMap;
use std::fmt::{self, Write};

pub struct Printer<'a> {
    output: &'a mut Vec<u8>,
    indent: Vec<u8>,
    at_line_start: bool,
}

impl Printer<'_> {
    fn new(output: &mut Vec<u8>) -> Printer {
        Printer {
            output,
            indent: Vec::new(),
            at_line_start: true,
        }
    }

    pub fn write_raw(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        if bytes[0] != b'\n' && self.at_line_start {
            self.output.extend_from_slice(&self.indent);
            self.at_line_start = false;
        }

        self.output.extend_from_slice(bytes);
    }

    pub fn indent(&mut self) {
        self.indent.extend_from_slice(b"    ");
    }

    pub fn outdent(&mut self) {
        let new_len = self.indent.len() - 4;
        self.indent.truncate(new_len)
    }
}

impl Write for Printer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.indent.is_empty() {
            self.output.extend_from_slice(s.as_bytes());
            return Ok(());
        }

        let mut start = 0;
        for (i, b) in s.bytes().enumerate() {
            if b == b'\n' {
                self.write_raw(&s.as_bytes()[start..=i]);
                self.at_line_start = true;
                start = i + 1;
            }
        }
        self.write_raw(&s.as_bytes()[start..]);
        Ok(())
    }
}

pub struct Context {
    pub db: Database,
    pub files: HashMap<String, Id<FileDescriptor>>,
    pub message_address: HashMap<String, Id<MessageDescriptor>>,
    pub enum_address: HashMap<String, Id<EnumDescriptor>>,
    pub to_generate: Vec<Id<FileDescriptor>>,
}

impl Context {
    pub fn new(mut request: plugin_pb::CodeGeneratorRequest) -> Context {
        let mut db = Database::with_capacity(request.proto_file().len());
        let mut files = HashMap::with_capacity(request.proto_file().len());
        let mut message_address = HashMap::with_capacity(request.proto_file().len());
        let mut enum_address = HashMap::with_capacity(request.proto_file().len());
        for file in request.proto_file_mut().drain(..) {
            let name = file.name().to_owned();
            let fd = FileDescriptor::new(&mut db, file);
            files.insert(name, fd);
            let f = db.files.get(fd).unwrap();
            let pkg_name = f.proto.package();
            for msg_id in &f.messages {
                let msg = db.messages.get(*msg_id).unwrap();
                let full_name = naming::full_name(pkg_name, &msg.type_name);
                eprintln!("register {}", full_name);
                message_address.insert(full_name, *msg_id);
            }
            for enum_id in &f.enums {
                let e = db.enums.get(*enum_id).unwrap();
                let full_name = naming::full_name(pkg_name, &e.type_name);
                eprintln!("register {}", full_name);
                enum_address.insert(full_name, *enum_id);
            }
        }
        let mut to_generate = Vec::with_capacity(request.file_to_generate().len());
        for f in request.file_to_generate() {
            eprintln!("mark {} to generate", f);
            to_generate.push(files.get(f).unwrap().clone());
        }
        Context {
            db,
            files,
            to_generate,
            message_address,
            enum_address,
        }
    }
}

#[derive(Default)]
pub struct Output {
    response: plugin_pb::CodeGeneratorResponse,
}

impl Output {
    pub fn open(&mut self, file_name: &str) -> Printer {
        let mut f = plugin_pb::CodeGeneratorResponseNestedFile::default();
        f.set_name(file_name.to_owned());
        self.response.file_mut().push(f);
        Printer::new(unsafe {
            self.response
                .file_mut()
                .last_mut()
                .unwrap()
                .content_mut()
                .as_mut_vec()
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let buffer = vec![];
        let mut s = pecan::CodedOutputStream::new(buffer);
        self.response.write_to(&mut s).unwrap();
        s.into_inner().unwrap()
    }
}
