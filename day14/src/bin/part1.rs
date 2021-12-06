use day14::Reindeer;

fn main() {
    let time = 2503;

    let distance = aoc::parser::lines()
        .filter_map(|line| {
            let line: Vec<&str> = line.split(' ').collect();
            let (name, speed, duration, rest) = (
                line[0].into(),
                line[3].parse::<usize>().ok()?,
                line[6].parse::<usize>().ok()?,
                line[13].parse::<usize>().ok()?,
            );

            Some(Reindeer::new(name, speed, duration, rest))
        })
        .map(|reindeer| reindeer.distance_for(time))
        .max()
        .unwrap();

    answer!("The winner traveled for {}", distance);
}
