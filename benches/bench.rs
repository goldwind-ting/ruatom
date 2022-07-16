use criterion::{criterion_group, criterion_main, Criterion};
use ruatom::parser::Parser;

fn fibonacci() {
    let p = Parser::new("OCC(CO)(CO)NC(=O)CCCc1ccc(cc1)C1(c2ccccc2)C23c4c5c6c7c8c9c%10c%11c%12c%13c%14c(c%15c%16c%17c%18c(c4c4c%19c5c7c5c7c%19c%19c4c%18c4c%17c%17c(c%14%16)c%14c%13c%13c%12c9c9c8c5c5c7c7c%19c4c4c%17c%14c8c%13c9c5c8c47)c2%15)c%11C13c6%10");
    let mut m = p.parse().unwrap();
    m.to_smiles().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tosmiles", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
