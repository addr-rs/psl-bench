use addr::domain::Name;
use criterion::{criterion_group, criterion_main, Criterion};
use publicsuffix::List;

fn bench(c: &mut Criterion) {
    let domain = "example.gb.com";
    let list = List::fetch().unwrap();

    c.bench_function("Rust - addr", |b| {
        b.iter(|| {
            Name::parse(domain).unwrap();
        })
    });

    c.bench_function("Rust - publicsuffix", |b| {
        b.iter(|| {
            list.parse_domain(domain).unwrap();
        })
    });

    c.bench_function("Rust - psl", |b| {
        b.iter(|| {
            psl::domain(domain.as_bytes()).unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
