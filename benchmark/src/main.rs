use std::env;
use std::fs::File;
use std::io::prelude::*;
use benchmark::benchmarks::proto3::benchmark_message1_proto3_pb::GoogleMessage1;
use benchmark::benchmarks::benchmarks_pb::BenchmarkDataset;
use pecan::*;
use criterion::{Criterion, Throughput};

pub struct DataSet {
    name: String,
    bytes: Vec<Vec<u8>>,
    msgs: Vec<GoogleMessage1>,
}

fn bench_all(config: &mut Criterion, datasets: &[DataSet]) {
    for ds in datasets {
        let mut group = config.benchmark_group(&ds.name);
        group.throughput(Throughput::Bytes(ds.bytes.iter().map(|b| b.len() as u64).sum()));
        group.bench_function("merge_from", |b| {
            b.iter(|| {
                for (j, payload) in ds.bytes.iter().enumerate() {
                    let mut msg = GoogleMessage1::default();
                    match msg.merge_from_bytes(payload) {
                        Ok(_) => (),
                        Err(e) => panic!("failed to parse {}: {}", j, e),
                    }
                    drop(criterion::black_box(msg));
                }
            })
        });
        group.throughput(Throughput::Elements(ds.msgs.len() as u64));
        group.bench_function("write to", |b| {
            b.iter(|| {
                for (j, msg) in ds.msgs.iter().enumerate() {
                    match msg.write_as_bytes() {
                        Ok(b) => drop(criterion::black_box(b)),
                        Err(e) => panic!("failed to serialize {}: {}", j, e),
                    }
                }
            })
        }).bench_function("len", |b| {
            b.iter(|| {
                for msg in &ds.msgs {
                    let l = msg.len();
                    criterion::black_box(l);
                }
            })
        });
    }
}

fn main() {
    let datasets: Vec<_> = env::args().into_iter().skip(1).map(|path| {
        let mut f = File::open(path).unwrap();
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).unwrap();
        let mut dataset_pb = BenchmarkDataset::new();
        dataset_pb.merge_from_bytes(&bytes).unwrap();
        if dataset_pb.message_name() != "benchmarks.proto3.GoogleMessage1" {
            panic!("Unsupport message: {}", dataset_pb.message_name());
        }
        let mut msgs = Vec::with_capacity(dataset_pb.payload().len());
        let mut bytes = Vec::with_capacity(dataset_pb.payload().len());
        for (j, payload) in dataset_pb.payload_mut().drain(..).enumerate() {
            let mut msg = GoogleMessage1::new();
            msg.merge_from_bytes(&payload).unwrap();
            let b = msg.write_as_bytes().unwrap();
            let mut check = GoogleMessage1::new();
            check.merge_from_bytes(&b).unwrap();
            assert_eq!(check, msg, "{}: {:?}", j, msg);
            msgs.push(msg);
            bytes.push(payload);
        }
        DataSet {
            name: dataset_pb.name().to_owned(),
            bytes,
            msgs,
        }
    }).collect();

    let mut config = Criterion::default();
    bench_all(&mut config, &datasets);
}
