use day10::expand;

fn main() {
    let input = aoc::parser::get_args(1).expect("Provide your input");
    let mut output = input;
    for _ in 0..40 {
        output = expand(&output);
    }
    println!("{}", output.len());
}
