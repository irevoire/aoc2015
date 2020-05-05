use day2::Cuboid;

fn main() {
    let reader = aoc::parser::lines_from_args(1);

    let paper: usize = reader
        .map(|line| {
            let mut split = line.split('x');
            Cuboid(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .map(|cube| cube.paper_needed())
        .sum();

    println!("Paper needed: {}", paper);
}
