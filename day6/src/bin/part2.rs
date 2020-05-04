use aoc::{Coord, Grid};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mut grid = Grid::from(vec![vec![0_usize; 1000]; 1000]);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let mut vec: Vec<&str> = line.rsplitn(4, ' ').collect();
            vec.reverse();
            let base: Coord<usize> = vec[1].parse().unwrap();
            let end: Coord<usize> = vec[3].parse().unwrap();

            let iter = grid.through_mut(base, end).unwrap();
            match vec[0] {
                "turn on" => iter.for_each(|el| *el += 1),
                "turn off" => iter.for_each(|el| *el = el.saturating_sub(1)),
                "toggle" => iter.for_each(|el| *el += 2),
                s => panic!("wtf is this shit: {}", s),
            }
        });

    println!("There is {} light lit", grid.iter().sum::<usize>());
}
