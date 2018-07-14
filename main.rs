extern crate psl;

use std::io::{self, BufRead, Write};
use psl::{Psl, List};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let list = List::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let domain_str = line.unwrap();
        if let Some(suffix) = list.suffix(&domain_str) {
            handle.write(suffix.as_bytes()).unwrap();
            handle.write(b"\n").unwrap();
        }
    }
}
