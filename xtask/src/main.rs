use std::{env::args, fs, process::Command};

fn exec(cmd: &mut Command) {
    let status = cmd.status().unwrap();
    assert!(status.success(), "{:?}", status);
}

const PROTO_INCLUDED: &[&str] = &["proto"];

const REFLECTION_PROTOS: &[&str] = &["proto/google/protobuf/descriptor.proto"];

const COMPILER_PROTOS: &[&str] = &[
    "proto/google/protobuf/compiler/plugin.proto",
    "proto/pecan/options.proto",
];

const WELL_KNOWN_PROTOS: &[&str] = &[
    "proto/google/protobuf/any.proto",
    "proto/google/protobuf/api.proto",
    "proto/google/protobuf/duration.proto",
    "proto/google/protobuf/empty.proto",
    "proto/google/protobuf/field_mask.proto",
    "proto/google/protobuf/source_context.proto",
    "proto/google/protobuf/struct.proto",
    "proto/google/protobuf/timestamp.proto",
    "proto/google/protobuf/type.proto",
    "proto/google/protobuf/wrappers.proto",
];

const CONFORMANCE_PROTOS: &[&str] = &[
    "proto/conformance/conformance.proto",
    "proto/google/protobuf/test_messages_proto2.proto",
    "proto/google/protobuf/test_messages_proto3.proto",
];

const BENCHMARK_PROTOS: &[&str] = &[
    "proto/datasets//google_message1/proto2/benchmark_message1_proto2.proto",
    "proto/datasets//google_message1/proto3/benchmark_message1_proto3.proto",
    "proto/datasets//google_message4/benchmark_message4.proto",
    "proto/datasets//google_message4/benchmark_message4_2.proto",
    "proto/datasets//google_message4/benchmark_message4_1.proto",
    "proto/datasets//google_message4/benchmark_message4_3.proto",
    "proto/datasets//google_message3/benchmark_message3_6.proto",
    "proto/datasets//google_message3/benchmark_message3_4.proto",
    "proto/datasets//google_message3/benchmark_message3.proto",
    "proto/datasets//google_message3/benchmark_message3_2.proto",
    "proto/datasets//google_message3/benchmark_message3_5.proto",
    "proto/datasets//google_message3/benchmark_message3_7.proto",
    "proto/datasets//google_message3/benchmark_message3_3.proto",
    "proto/datasets//google_message3/benchmark_message3_1.proto",
    "proto/datasets//google_message3/benchmark_message3_8.proto",
    "proto/datasets//google_message2/benchmark_message2.proto",
    "proto/benchmarks.proto",
    "proto/google_size.proto",
];

fn prepare_protoc() -> Command {
    let mut cmd = Command::new("protoc");
    cmd.arg("--plugin=protoc-gen-rust=./target/debug/plugin");
    for i in PROTO_INCLUDED {
        cmd.args(&["-I", *i]);
    }
    cmd
}

fn compile_well_known_types() {
    let mut cmd = prepare_protoc();
    for p in WELL_KNOWN_PROTOS {
        cmd.arg(*p);
    }
    fs::create_dir_all("compiler/src/proto").unwrap();
    exec(cmd.args(&[
        "--rust_out=types/src/",
        "--rust_opt=skip-builtin-well-known=true",
    ]));
}

fn compile_compiler() {
    let mut cmd = prepare_protoc();
    for p in COMPILER_PROTOS {
        cmd.arg(*p);
    }
    fs::create_dir_all("compiler/src/proto").unwrap();
    exec(cmd.args(&[
        "--rust_out=compiler/src/proto",
        "--rust_opt=skip-builtin-compiler=true",
    ]));
    fs::rename(
        "compiler/src/proto/google/protobuf/compiler/plugin_pb.rs",
        "compiler/src/plugin_pb.rs",
    )
    .unwrap();
    fs::rename(
        "compiler/src/proto/pecan/options_pb.rs",
        "compiler/src/options_pb.rs",
    )
    .unwrap();
    fs::remove_dir_all("compiler/src/proto").unwrap();
}

fn compile_descriptor() {
    let mut cmd = prepare_protoc();
    for p in REFLECTION_PROTOS {
        cmd.arg(*p);
    }
    exec(cmd.args(&[
        "--rust_out=pecan/src/reflection",
        "--rust_opt=skip-builtin-reflection=true",
    ]));
    let mut content =
        fs::read_to_string("pecan/src/reflection/google/protobuf/descriptor_pb.rs").unwrap();
    content = content.replace("pecan::", "crate::");
    fs::write("pecan/src/reflection/descriptor_pb.rs", content.as_bytes()).unwrap();
    fs::remove_dir_all("pecan/src/reflection/google/").unwrap();
}

fn compile_conformance() {
    let mut cmd = prepare_protoc();
    for p in CONFORMANCE_PROTOS {
        cmd.arg(*p);
    }
    exec(cmd.arg("--rust_out=conformance/src/"));
}

fn compile_benchmark() {
    let mut cmd = prepare_protoc();
    for p in BENCHMARK_PROTOS {
        cmd.arg(*p);
    }
    exec(cmd.arg("--rust_out=benchmark/src/"));
}

fn codegen() {
    exec(Command::new("cargo").args(&["build", "-p", "pecan-compiler"]));
    compile_descriptor();
    compile_compiler();
    compile_well_known_types();
    compile_conformance();
    compile_benchmark();
}

fn main() {
    let mut args = args();
    args.next();
    let cmd = match args.next() {
        Some(c) => c,
        None => panic!("expected at lease one command"),
    };
    match &*cmd {
        "codegen" => codegen(),
        cmd => panic!("unsupported command {}", cmd),
    }
}
