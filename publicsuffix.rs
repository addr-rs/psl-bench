use publicsuffix::{Psl, List};
use std::fs::File;
use std::io::{self, BufRead, Read, BufWriter, Write};

fn main() {
    let list: List = {
        let mut file = File::open("public_suffix_list.dat").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.parse().unwrap()
    };

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_str = line.unwrap();
        let input = input_str.as_bytes();
        handle.write(input).unwrap();
        handle.write(b": ").unwrap();
        match list.domain(input) {
            Some(domain) => handle.write(domain.as_bytes()).unwrap(),
            None => handle.write(b"(null)").unwrap(),
        };
        handle.write(b"\n").unwrap();
    }
    handle.flush().unwrap();
}
