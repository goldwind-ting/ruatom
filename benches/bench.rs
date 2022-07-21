use criterion::{criterion_group, criterion_main, Criterion};
use ruatom::parser::Parser;

fn fibonacci() {
    let p = Parser::new("NC[C@@H]1O[C@H](O[C@@H]2[C@@H](CSCCNC(=S)NCCCCN3C(=O)c4ccc5c6ccc7c8c(ccc(c9ccc(c4c59)C3=O)c86)C(=O)N(CCCCNC(=S)NCCSC[C@H]3O[C@@H](O[C@@H]4[C@@H](O)[C@H](N)C[C@H](N)[C@H]4O[C@H]4O[C@H](CN)[C@@H](O)[C@H](O)[C@H]4N)[C@H](O)[C@@H]3O[C@H]3O[C@@H](CN)[C@@H](O)[C@H](O)[C@H]3N)C7=O)O[C@@H](O[C@@H]3[C@@H](O)[C@H](N)C[C@H](N)[C@H]3O[C@H]3O[C@H](CN)[C@@H](O)[C@H](O)[C@H]3N)[C@@H]2O)[C@H](N)[C@@H](O)[C@@H]1O");
    let mut m = p.parse().unwrap();
    m.to_smiles().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tosmiles", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
