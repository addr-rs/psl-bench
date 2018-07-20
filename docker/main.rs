extern crate psl;

use std::io::{self, BufRead, BufWriter, Write};
use psl::{Psl, List};

fn main() {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let list = List::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let domain_str = line.unwrap();
        match list.domain(&domain_str) {
            Some(domain) => handle.write(domain.as_bytes()).unwrap(),
            None => handle.write(b"(None)").unwrap(),
        };
        handle.write(b"\n").unwrap();
    }
}
