use criterion::{criterion_group, criterion_main, Criterion};

use num_bigint::BigInt;

use mimc_rs::Mimc7;

fn criterion_benchmark(c: &mut Criterion) {
    let b1: BigInt = BigInt::parse_bytes(
        b"12242166908188651009877250812424843524687801523336557272219921456462821518061",
        10,
    )
    .unwrap();
    let b2: BigInt = BigInt::parse_bytes(
        b"12242166908188651009877250812424843524687801523336557272219921456462821518061",
        10,
    )
    .unwrap();
    let mut big_arr: Vec<BigInt> = Vec::new();
    big_arr.push(b1.clone());
    big_arr.push(b2.clone());
    let mimc7 = Mimc7::new();

    c.bench_function("hash", |b| b.iter(|| mimc7.hash(big_arr.clone()).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
