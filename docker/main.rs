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
        if let Some(domain) = list.domain(&domain_str) {
            handle.write(domain.as_bytes()).unwrap();
            handle.write(b"\n").unwrap();
        }
    }
}
