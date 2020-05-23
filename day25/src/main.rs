use day25::*;

fn main() {
    let row: usize = aoc::parser::get_args(1)
        .expect("Which line do you want")
        .parse()
        .unwrap();
    let col: usize = aoc::parser::get_args(2)
        .expect("Which column do you want")
        .parse()
        .unwrap();

    if row == 0 || col == 0 {
        panic!("The row and column must be > 0");
    }

    let mut gen = GridGenerator::default();

    let (_, _, value) = gen
        .find(|(r, c, _)| *r == (row - 1) && *c == (col - 1))
        .unwrap();

    println!("The password at position {} {} is {}", row, col, value);
}
