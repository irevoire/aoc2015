use aoc::*;

fn main() {
    let res: usize = parser::chars::<char>()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .scan(0, |acc, el| {
            *acc += el;
            Some(*acc)
        })
        .position(|level| level == -1)
        .unwrap();

   aoc::answer!(
        "The position of the character that causes Santa to first enter the basement is: {}.",
        res
    );
}
