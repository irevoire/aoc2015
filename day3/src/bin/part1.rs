use aoc::{Coord, Direction};
use std::collections::HashSet;

fn main() {
    let mut coord = Coord::<i32>::new();
    let mut set = HashSet::new();

    let input = std::fs::read("input").unwrap();

    set.insert(coord.clone());
    for c in input.iter().map(|b| *b as char) {
        if let Ok(dir) = c.to_string().parse::<Direction>() {
            coord += dir;
            set.insert(coord.clone());
        }
    }

    println!("total: {}", set.len());
}
