use day6::{Coord, Grid};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mut grid = Grid::new();

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let mut vec: Vec<&str> = line.rsplitn(4, ' ').collect();
            vec.reverse();
            let base: Coord = vec[1].parse().unwrap();
            let end: Coord = vec[3].parse().unwrap();

            match vec[0] {
                "turn on" => grid.through(base, end, Grid::update(1)),
                "turn off" => grid.through(base, end, Grid::update(-1)),
                "toggle" => grid.through(base, end, Grid::update(2)),
                s => panic!("wtf is this shit: {}", s),
            }
        });

    println!("There is {} light lit", grid.lit());
}
