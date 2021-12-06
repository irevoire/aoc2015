fn main() {
    let input: usize = aoc::parser::get_args(1).unwrap().parse().unwrap();

    let mut houses = vec![10; 1_000_000];
    let mut max_index = houses.len();

    for i in 2..houses.len() {
        if i > max_index {
            break;
        }
        for current in (i..max_index).step_by(i) {
            houses[current] += i * 10;
            if houses[current] >= input {
                max_index = current;
                break;
            }
        }
    }
    println!(
        "The house {} will be the first to get {} gifts or more",
        max_index, input
    );
}
