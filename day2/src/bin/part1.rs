use aoc::*;
use day2::Cuboid;

fn main() {
    let paper: usize = parser::lines::<Cuboid>()
        .map(|cube| cube.paper_needed())
        .sum();
    answer!("They should order {} square feet of wrapping paper.", paper);
}
