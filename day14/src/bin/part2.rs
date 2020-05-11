use day14::Reindeer;

fn main() {
    let time = 2503;

    let reindeers = aoc::parser::lines_from_args(1)
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
        .collect::<Vec<_>>();

    let mut scores = vec![0; reindeers.len()];

    for t in 0..time {
        let winner = reindeers
            .iter()
            .map(|reindeer| reindeer.distance_for(t))
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index);
        scores[winner.unwrap()] += 1;
    }

    println!("The winner scored {}", scores.iter().max().unwrap());
}
