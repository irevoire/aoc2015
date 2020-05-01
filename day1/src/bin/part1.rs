fn main() {
    let input = std::fs::read("input").unwrap();
    let res: i32 = input
        .iter()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum();

    println!("res: {}", res);
}
