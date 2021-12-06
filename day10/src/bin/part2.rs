use day10::expand;

fn main() {
    let input = aoc::parser::get_args(1).expect("Provide your input");
    let mut output = input;
    for _ in 0..50 {
        output = expand(&output);
    }
    aoc::anwser!("{}", output.len());
}
