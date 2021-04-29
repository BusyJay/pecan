mod google_size_pb;
mod benchmarks_pb;
mod datasets;

use std::{env, fs, path::Path};

use benchmarks_pb::BenchmarkDataset;
use criterion::{BenchmarkGroup, Criterion, measurement::WallTime};
use bytes::{Bytes, BytesMut};
use pecan::prelude::*;

use crate::datasets::{google_message1::{proto2::benchmark_message1_proto2_pb, proto3::benchmark_message1_proto3_pb}, google_message2::benchmark_message2_pb::GoogleMessage2, google_message3::benchmark_message3_pb::GoogleMessage3, google_message4::benchmark_message4_pb::GoogleMessage4};

fn bench_read_from<M: Default + BufMessage<Bytes, BytesMut>>(g: &mut BenchmarkGroup<WallTime>, data_set: &BenchmarkDataset) {
    let mut ds = data_set.payload.iter().cycle();
    g.bench_function("read_from", |b| {
        b.iter(|| {
            let mut m = M::default();
            m.merge_from_buf(&mut ds.next().unwrap().clone()).unwrap();
            criterion::black_box(m);
        })
    });
}

fn bench_merge_from<M: Default + BufMessage<Bytes, BytesMut>>(g: &mut BenchmarkGroup<WallTime>, data_set: &BenchmarkDataset) {
    let mut ds = data_set.payload.iter().cycle();
    g.bench_function("merge_from", |b| {
        let mut m = M::default();
        b.iter(|| {
            m.merge_from_buf(&mut ds.next().unwrap().clone()).unwrap();
        })
    });
}

fn bench_write_to<M: Default + BufMessage<Bytes, BytesMut>>(g: &mut BenchmarkGroup<WallTime>, data_set: &BenchmarkDataset) {
    let msgs: Vec<M> = data_set.payload.iter().map(|b| {
        let mut m = M::default();
        m.merge_from_buf(&mut b.clone()).unwrap();
        m
    }).collect();
    let mut msgs_iter = msgs.iter().cycle();
    let mut buffer = BytesMut::new();
    g.bench_function("write_to", |b| {
        b.iter(|| {
            msgs_iter.next().unwrap().write_to_buf(&mut buffer).unwrap();
            buffer.clear();
        })
    });
}

fn build_group<M: Default + BufMessage<Bytes, BytesMut>>(g: &mut BenchmarkGroup<WallTime>, data_set: &BenchmarkDataset) {
    bench_read_from::<M>(g, data_set);
    bench_merge_from::<M>(g, data_set);
    bench_write_to::<M>(g, data_set);
}

fn run_one_test(c: &mut Criterion, f: impl AsRef<Path>) {
    let data = fs::read(f).unwrap();
    let mut data_set = BenchmarkDataset::new();
    data_set.merge_from_buf(&mut data.as_ref()).unwrap();
    let mut group = c.benchmark_group(&data_set.name);
    match &*data_set.message_name {
        "benchmarks.proto3.GoogleMessage1" => build_group::<benchmark_message1_proto3_pb::GoogleMessage1>(&mut group, &data_set),
        "benchmarks.proto2.GoogleMessage1" => build_group::<benchmark_message1_proto2_pb::GoogleMessage1>(&mut group, &data_set),
        "benchmarks.proto2.GoogleMessage2" => build_group::<GoogleMessage2>(&mut group, &data_set),
        "benchmarks.google_message3.GoogleMessage3" => build_group::<GoogleMessage3>(&mut group, &data_set),
        "benchmarks.google_message4.GoogleMessage4" => build_group::<GoogleMessage4>(&mut group, &data_set),
        name => panic!("unknown message: {}", name),
    }
    group.finish();
}

fn main() {
    let mut args = env::args();
    args.next();
    let mut c = Criterion::default();
    for f in args {
        run_one_test(&mut c, f);
    }
    c.final_summary();
}
