fn main() {
    let original = day7::parse();
    let mut assignations = original.clone();

    let a = assignations.get("a").unwrap().clone();
    let a = a.compute(&mut assignations);

    let mut assignations = original.clone();
    *assignations.get_mut("b").unwrap() = a;

    // restart everything
    let a = assignations.get("a").unwrap().clone();

    aoc::answer!("{}", a.compute(&mut assignations).unwrap());
}
