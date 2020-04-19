use pecan::{CodedInputStream, Message};
use pecan_compiler::{Context, Generator, Output};
use pecan_descriptor::google::protobuf::compiler::plugin_pb;
use std::io::{self, Read, Write};

fn main() {
    let mut sin = io::stdin();
    let mut buf = vec![];
    sin.read_to_end(&mut buf).unwrap();
    let mut s = CodedInputStream::new(buf.as_slice());
    let mut req = plugin_pb::CodeGeneratorRequest::default();
    req.merge_from(&mut s).unwrap();
    let ctx = Context::new(req);
    let mut output = Output::default();
    for id in &ctx.to_generate {
        let mut generator = Generator::new(&ctx, &mut output, *id);
        generator.generate(&ctx).unwrap();
    }
    let bytes = output.to_bytes();
    io::stdout().write_all(&bytes).unwrap();
}
