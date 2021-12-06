use aoc::*;
use day2::Cuboid;

fn main() {
    let ribbon: usize = parser::lines::<Cuboid>()
        .map(|cube| cube.ribbon_needed())
        .sum();
   aoc::answer!("They should order {} square feet of ribbon.", ribbon);
}
