use day2::Cuboid;

fn main() {
    let paper: usize = aoc::parser::lines_from_args_as::<Cuboid>(1)
        .map(|cube| cube.ribbon_needed())
        .sum();
    println!("Ribbon needed: {}", paper);
}
