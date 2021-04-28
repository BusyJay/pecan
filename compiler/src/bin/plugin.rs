use bytes::{Bytes, BytesMut};
use pecan::prelude::*;
use pecan::Message;
use pecan_compiler::plugin_pb::*;
use pecan_compiler::Database;
use std::io::{self, Read, Write};

fn main() {
    let mut content = Vec::new();
    io::stdin().read_to_end(&mut content).unwrap();
    let mut bytes: Bytes = content.into();
    let mut req = CodeGeneratorRequest::new();
    let mut input = CodedInputStream::new(&mut bytes);
    req.merge_from(&mut input).unwrap();

    let mut db = Database::default();
    if !req.parameter().contains("skip-builtin-reflection=true") {
        db.load_reflection_descriptor();
    }
    if !req.parameter().contains("skip-builtin-compiler=true") {
        db.load_complier_descriptor();
    }
    if !req.parameter().contains("skip-builtin-well-known=true") {
        db.load_well_known_descriptor();
    }
    for r in req.proto_file {
        db.load(r);
    }
    let mut resp = CodeGeneratorResponse::default();
    for f in &req.file_to_generate {
        let g = match db.generator_for(f) {
            Some(g) => g,
            None => panic!("{} not found in proto", f),
        };
        let mut f = CodeGeneratorResponse_File::default();
        f.set_name(g.file().target_path().to_string());
        f.set_content(g.generate());
        resp.file.push(f);
    }
    let mut buffer = BytesMut::new();
    let mut output = CodedOutputStream::new(&mut buffer);
    resp.write_to(&mut output).unwrap();
    drop(output);
    io::stdout().write_all(&buffer).unwrap();
}
