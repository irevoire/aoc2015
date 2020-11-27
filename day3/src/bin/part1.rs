use aoc::*;
use std::collections::HashSet;

fn main() {
    let total = parser::chars_as_strings_from_args(1)
        .filter_map(|c| c.parse::<Direction>().ok())
        .scan(Coord::<isize>::default(), |acc, dir| {
            *acc += dir;
            Some(*acc)
        })
        .collect::<HashSet<Coord<_>>>()
        .len();

    println!("total: {}", total);
}
