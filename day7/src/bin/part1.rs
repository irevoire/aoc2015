use aoc::*;

fn main() {
    let mut assignations = day7::parse();

    let a = assignations.get("a").unwrap().clone();
    answer!(
        "The final signal provided to the wire a is {}.",
        a.compute(&mut assignations).unwrap()
    );
}
