use itertools::Itertools;

fn main() {
    let mut map = std::collections::HashMap::new();
    let mut all_cities = std::collections::HashSet::new();

    for line in aoc::parser::lines::<String>() {
        let line: Vec<&str> = line.split("=").map(str::trim).collect();
        let (cities, distance) = (line[0], line[1]);
        let distance: usize = distance.parse().unwrap();
        let cities: Vec<&str> = cities.split("to").map(str::trim).collect();
        let (left, right) = (cities[0], cities[1]);

        map.insert((left.to_string(), right.to_string()), distance);
        map.insert((right.to_string(), left.to_string()), distance);

        // could be better
        all_cities.insert(left.to_string());
        all_cities.insert(right.to_string());
    }

    let distance: usize = all_cities
        .iter()
        .permutations(all_cities.len())
        .map(|all_cities| {
            all_cities
                .windows(2)
                .map(|arr| map.get(&(arr[0].clone(), arr[1].clone())).unwrap())
                .sum::<usize>()
        })
        .min()
        .unwrap();

    aoc::answer!("the shortest path is: {}", distance);
}
