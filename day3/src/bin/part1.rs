use aoc::*;
use std::collections::HashSet;

fn main() {
    let total = parser::chars::<Direction>()
        .scan(Coord::<isize>::default(), |acc, dir| {
            *acc += dir;
            Some(*acc)
        })
        .collect::<HashSet<Coord<_>>>()
        .len();

    answer!("{} houses will receive at least one present.", total);
}
