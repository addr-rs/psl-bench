extern crate psl;
#[macro_use]
extern crate criterion;

use std::process::Command;

use psl::{Psl, List};
use criterion::Criterion;

fn pypy_cmd() -> Command {
    let mut command = Command::new("pypy");
    command.arg("benches/bench.py");
    command
}

fn bench(c: &mut Criterion) {
    let domains = vec!["example.gb.com"];

    let list = List::new();

    c.bench_function_over_inputs("Rust", move |b, &domain| {
        b.iter(|| { list.suffix(domain).unwrap(); } )
    }, domains.clone());

    c.bench_program_over_inputs("PyPy", pypy_cmd, domains);
}

criterion_group!(benches, bench);
criterion_main!(benches);
