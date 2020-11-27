use aoc::*;
use std::collections::HashSet;

fn main() {
    let (santa, robot): (Vec<_>, Vec<_>) = parser::chars_as_strings_from_args(1)
        .filter_map(|c| c.parse::<Direction>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .scan(
            ((Coord::<isize>::default()), Coord::<isize>::default()),
            |(sa, ba), dir| {
                *sa += dir[0];
                *ba += dir[1];
                Some((*sa, *ba))
            },
        )
        .unzip();

    let total = santa.iter().chain(&robot).collect::<HashSet<_>>().len();

    println!("total: {}", total);
}
