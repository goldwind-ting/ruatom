use criterion::{criterion_group, criterion_main, Criterion};
use ruatom::parser::Parser;

fn long_smiles() {
    let p = Parser::new(
        r#"C1C[C@TH1H](O[C@TH1H]2C[C@TH1H]3O[C@TH1]4(C)CC[C@TH1H]5O[C@TH1]6(C)[C@TH1H](O[C@TH1]7(C)C[C@TH2H](O)[C@TH1H](O[C@TH1H]7[C@TH2H]6O)[C@TH2H]6O[C@TH1H]7[C@TH2H](C[C@TH1H]6O)O[C@TH1H](C[C@TH1H](O)C[C@TH2H](O)[C@TH2H]6O[C@TH1H]8[C@TH2H](O[C@TH1H]([C@TH2H]9O[C@TH2H]%10[C@TH2H](O[C@TH1H]%11C[C@TH1H]%12O[C@TH1H]%13[C@TH1H](C[C@TH1H]%12O[C@TH1H]%11[C@TH1H]%10O)O[C@TH1H](C[C@TH1H](O)[C@TH2H](O)[C@TH1H]%10O[C@TH1H]%11[C@TH1H](C[C@TH1H]%10O)O[C@TH1]%10(C)C[C@TH1H]%12O[C@TH1]%14(C)C[C@TH2H](O)[C@TH1H]%15O[C@TH1H]([C@TH2H](O)[C@TH2H](O)[C@TH1H]%15O[C@TH1H]%14C[C@TH1H]%12O[C@TH1H]%10[C@TH1H]%11O)[C@TH2H](C)[C@TH1H](O)[C@TH1H](C)CC[C@TH2H](OS([O-])(=O)=O)[C@TH2H](O)[C@TH1H](C)C[C@TH1H](O)C(=C)/C(/C)=C/CO)[C@TH2H](OS([O-])(=O)=O)[C@TH2H]%13O)[C@TH2H](O)[C@TH1H]9O)[C@TH2H](O)[C@TH1H]8O)C[C@TH1H]6O)[C@TH2H](O)[C@TH1H]7O)C[C@TH1]5(C)O[C@TH1H]4C[C@TH1]3(C)O[C@TH1H]12)[C@TH1]1(C)O[C@TH1]2(C)C[C@TH1H]3O[C@TH1H]4C[C@TH1H]5O[C@TH1H]6C/C=C\[C@TH1H]7O[C@TH1]8(C)C[C@TH1]9(C)O[C@TH1]%10(CC[C@TH1H](O[C@TH1H]%10C[C@TH1H]9O[C@TH1H]8C[C@TH1H]7O[C@TH1H]6C[C@TH1]5(C)O[C@TH1]4(C)CC[C@TH1]3(C)O[C@TH1H]2C[C@TH2H]1O)[C@TH1H](O)[C@TH2H](O)C[C@TH2H](C)[C@TH2H](C)CC=C)C"#,
    );
    let mut m = p.parse().unwrap();
    m.to_smiles().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("long_smiles", |b| b.iter(|| long_smiles()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
