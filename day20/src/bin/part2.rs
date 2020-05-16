fn main() {
    let input: usize = aoc::parser::get_args(1).unwrap().parse().unwrap();

    let mut houses = vec![0; 1_000_000_000];
    let mut max_index = houses.len();

    for i in 1..houses.len() {
        if i > max_index {
            break;
        }
        for current in (i..houses.len()).step_by(i).take(50) {
            houses[current] += i * 11;
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
