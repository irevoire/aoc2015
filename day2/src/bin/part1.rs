use day2::Cuboid;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let paper: usize = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split('x');
            Cuboid(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .map(|cube| cube.paper_needed())
        .sum();

    println!("Paper needed: {}", paper);
}
