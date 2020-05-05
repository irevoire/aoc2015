use day8::encode_size;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| (line.len(), encode_size(&line)))
        .fold((0, 0), |(accl, accr), (l, r)| (accl + l, accr + r));

    println!("res: {}", res.1 - res.0);
}
