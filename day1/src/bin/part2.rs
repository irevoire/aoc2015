fn main() {
    let input = std::fs::read("input").unwrap();
    let mut acc = 0;
    let res = input
        .iter()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .position(|i| {
            acc += i;
            acc == -1
        })
        .unwrap();

    println!("position: {}", res + 1);
}
