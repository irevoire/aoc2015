use aoc::*;

fn main() {
    let res: isize = parser::chars::<char>()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    answer!("The instructions bring Santa to the {} floor.", res);
}
