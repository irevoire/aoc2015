use itertools::Itertools;

fn main() {
    let weights: Vec<usize> = aoc::parser::lines_from_args(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let total: usize = weights.iter().sum();
    let tier = total / 3;

    let mut solutions = Vec::new();
    let mut minimum = std::usize::MAX;

    for len in 0..weights.len() {
        println!("{} / {}", len, weights.len());
        for permutation in weights.iter().copied().combinations(len) {
            if permutation.iter().sum::<usize>() == tier {
                if permutation.len() < minimum {
                    solutions = vec![permutation.clone()];
                    minimum = permutation.len();
                } else if permutation.len() == minimum {
                    solutions.push(permutation.clone());
                }
            }
        }
    }

    let best: usize = solutions.iter().map(|s| s.iter().product()).min().unwrap();

    println!(
        "The best solution for santa has a quantum entanglement of {}",
        best
    );
}
