use day3::Coord;
use std::collections::HashSet;

fn main() {
    let mut coord = Coord::new();
    let mut set = HashSet::new();

    let input = std::fs::read("input").unwrap();

    set.insert(coord.clone());
    for c in input.iter().map(|b| *b as char) {
        if c == '\n' {
            continue;
        }
        coord += c;
        set.insert(coord.clone());
    }

    println!("total: {}", set.len());
}
