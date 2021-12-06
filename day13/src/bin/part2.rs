use itertools::Itertools;

fn main() {
    let mut map = std::collections::HashMap::new();
    let mut peoples = std::collections::HashSet::new();
    peoples.insert("me".to_string());

    for line in aoc::parser::lines::<String>() {
        let line: Vec<&str> = line[..line.len() - 1].split(" ").map(str::trim).collect();
        let (left, right, gain, value) = (line[0], line[10], line[2], line[3]);
        let value = match gain {
            "gain" => value.parse::<isize>().unwrap(),
            "lose" => -value.parse::<isize>().unwrap(),
            gain => panic!("Unexpected value {}", gain),
        };

        map.insert((left.to_string(), right.to_string()), value);
        map.insert(("me".to_string(), right.to_string()), 0);
        map.insert((left.to_string(), "me".to_string()), 0);
        peoples.insert(left.to_string());
    }

    let happiness: isize = peoples
        .iter()
        .permutations(peoples.len())
        .map(|relations| {
            relations
                .windows(2)
                .map(|arr| {
                    map.get(&(arr[0].clone(), arr[1].clone())).unwrap()
                        + map.get(&(arr[1].clone(), arr[0].clone())).unwrap()
                })
                .sum::<isize>()
                + map
                    .get(&(relations[relations.len() - 1].clone(), relations[0].clone()))
                    .unwrap()
                + map
                    .get(&(relations[0].clone(), relations[relations.len() - 1].clone()))
                    .unwrap()
        })
        .max()
        .unwrap();

   aoc::answer!(
        "the best arrangement generate {} total happiness",
        happiness
    );
}
