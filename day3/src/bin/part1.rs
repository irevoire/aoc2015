use aoc::*;
use std::collections::HashSet;

fn main() {
    let total = parser::chars_from_args_as::<Direction>(1)
        .scan(Coord::<isize>::default(), |acc, dir| {
            *acc += dir;
            Some(*acc)
        })
        .collect::<HashSet<Coord<_>>>()
        .len();

    println!("total: {}", total);
}
