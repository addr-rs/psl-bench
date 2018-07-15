extern crate psl;
#[macro_use]
extern crate criterion;

use std::process::Command;

use psl::{Psl, List};
use criterion::Criterion;

fn pypy() -> Command {
    let mut command = Command::new("pypy");
    command.arg("benches/bench.py");
    command
}

fn psl(c: &mut Criterion) {
    let list = List::new();

    c.bench_function("public suffix", move |b| {
        b.iter(|| { list.suffix("example.gb.com").unwrap(); } )
    });

    c.bench_function("registrable domain", move |b| {
        b.iter(|| { list.domain("example.dyndns-server.com").unwrap(); } )
    });
}

fn inter_language(c: &mut Criterion) {
    let domains = vec!["example.gb.com"];

    let list = List::new();

    c.bench_function_over_inputs("Rust", move |b, &domain| {
        b.iter(|| { list.suffix(domain).unwrap(); } )
    }, domains.clone());

    c.bench_program_over_inputs("PyPy", pypy, domains);
}

criterion_group!(benches, psl, inter_language);
criterion_main!(benches);
