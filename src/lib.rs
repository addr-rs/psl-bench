#![feature(test)]
#![cfg(test)]

extern crate test;

use test::Bencher;

const DOMAIN_NAME: &[u8] = b"www.example.com";

#[bench]
fn psl(b: &mut Bencher) {
    b.iter(|| {
        psl::domain(DOMAIN_NAME).unwrap();
    });
}

#[bench]
fn publicsuffix(b: &mut Bencher) {
    use publicsuffix::{List, Psl};
    let list: List = include_str!("../public_suffix_list.dat").parse().unwrap();
    b.iter(|| {
        list.domain(DOMAIN_NAME).unwrap();
    });
}
