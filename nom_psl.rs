use nom_psl::List;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let list = List::parse_source_file("public_suffix_list.dat", 10_000_000).unwrap();

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_str = line.unwrap();
        handle.write(input_str.as_bytes()).unwrap();
        handle.write(b": ").unwrap();
        match list.parse_domain(&input_str) {
            Some(domain) => handle.write(domain.as_bytes()).unwrap(),
            None => handle.write(b"(null)").unwrap(),
        };
        handle.write(b"\n").unwrap();
    }
    handle.flush().unwrap();
}
