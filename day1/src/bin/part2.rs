fn main() {
    let res: usize = aoc::parser::chars_from_args(1)
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

    println!("position: {}", res + 1);
}
