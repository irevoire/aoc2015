fn main() {
    let password = aoc::parser::get_args(1).expect("gimme yo pass");
    answer!(
        "your new password should be: {}",
        day11::next_password(&password)
    );
}
