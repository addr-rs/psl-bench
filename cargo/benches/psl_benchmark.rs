extern crate psl;
extern crate publicsuffix;
extern crate addr;
#[macro_use]
extern crate criterion;

use std::process::Command;
use criterion::Criterion;

fn pypy_cmd() -> Command {
    let mut command = Command::new("pypy");
    command.arg("benches/bench.py");
    command
}

fn bench(c: &mut Criterion) {
    let domains = vec!["example.gb.com"];
    let list = publicsuffix::List::fetch().unwrap();

    c.bench_function_over_inputs("Rust - addr", move |b, &domain| {
        use std::str::FromStr;
        use addr::DomainName;
        b.iter(|| { DomainName::from_str(domain).unwrap(); } )
    }, domains.clone());

    c.bench_function_over_inputs("Rust - publicsuffix", move |b, &domain| {
        b.iter(|| { list.parse_domain(domain).unwrap(); } )
    }, domains.clone());

    c.bench_function_over_inputs("Rust - psl", move |b, &domain| {
        use psl::{Psl, List};
        b.iter(|| { List.domain(domain).unwrap(); } )
    }, domains.clone());

    c.bench_program_over_inputs("PyPy", pypy_cmd, domains);
}

criterion_group!(benches, bench);
criterion_main!(benches);
