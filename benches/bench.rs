use criterion::{criterion_group, criterion_main, Criterion};
use ruatom::parser::Parser;

fn fibonacci() {
    let p = Parser::new("CC[n+]1ccc(-c2cc[n+](Cc3cc(C[n+]4ccc(-c5cc[n+](CC)cc5)cc4)cc(C[n+]4ccc(-c5cc[n+](Cc6cc(C[n+]7ccc(-c8cc[n+](Cc9cc(C[n+]%10ccc(-c%11cc[n+](CC)cc%11)cc%10)cc(C[n+]%10ccc(-c%11cc[n+](CC)cc%11)cc%10)c9)cc8)cc7)cc(-[n+]7ccc(-c8cc[n+](-c9cc(C[n+]%10ccc(-c%11cc[n+](Cc%12cc(C[n+]%13ccc(-c%14cc[n+](CC)cc%14)cc%13)cc(C[n+]%13ccc(-c%14cc[n+](CC)cc%14)cc%13)c%12)cc%11)cc%10)cc(C[n+]%10ccc(-c%11cc[n+](Cc%12cc(C[n+]%13ccc(-c%14cc[n+](CC)cc%14)cc%13)cc(C[n+]%13ccc(-c%14cc[n+](CC)cc%14)cc%13)c%12)cc%11)cc%10)c9)cc8)cc7)c6)cc5)cc4)c3)cc2)cc1");
    let mut m = p.parse().unwrap();
    m.to_smiles().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tosmiles", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
