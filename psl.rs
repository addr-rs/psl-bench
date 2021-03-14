use psl::{Psl, List};
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_str = line.unwrap();
        let input = input_str.as_bytes();
        handle.write(input).unwrap();
        handle.write(b": ").unwrap();
        match List.domain(input) {
            Some(domain) => handle.write(domain.as_bytes()).unwrap(),
            None => handle.write(b"(null)").unwrap(),
        };
        handle.write(b"\n").unwrap();
    }
    handle.flush().unwrap();
}
