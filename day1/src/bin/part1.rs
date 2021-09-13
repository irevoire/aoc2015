fn main() {
    let res: isize = aoc::parser::chars_from_args(1)
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    println!("res: {}", res);
}
