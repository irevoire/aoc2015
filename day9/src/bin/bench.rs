#![feature(test)]

extern crate test;
use itertools::Itertools;

fn hash(input: &str) {
    let mut map = std::collections::HashMap::new();
    let mut all_cities = std::collections::HashSet::new();

    for line in input.lines() {
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

    println!("the shortest path is: {}", distance);
}

#[derive(Clone, Debug)]
pub struct Node {
    //              node  index, distance
    pub connections: Vec<(usize, usize)>,
    pub used: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            used: false,
        }
    }
}

fn graph(input: &str) {
    let mut map = std::collections::HashMap::new();
    let mut graph = Vec::new();
    let mut last_index: usize = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split("=").map(str::trim).collect();
        let (cities, distance) = (line[0], line[1]);
        let distance: usize = distance.parse().unwrap();
        let cities: Vec<&str> = cities.split("to").map(str::trim).collect();
        let (left, right) = (cities[0], cities[1]);

        let left_index = *map.entry(left).or_insert_with(|| {
            let current = last_index;
            last_index += 1;
            graph.push(Node::new());
            current
        });
        let right_index = *map.entry(right).or_insert_with(|| {
            let current = last_index;
            last_index += 1;
            graph.push(Node::new());
            current
        });

        graph[left_index].connections.push((right_index, distance));
        graph[right_index].connections.push((left_index, distance));
    }

    let distance: usize = map
        .values()
        .map(|start_index| distance(*start_index, graph.clone()))
        .min()
        .unwrap();

    println!("the shortest path is: {}", distance);
}

fn distance(current: usize, mut graph: Vec<Node>) -> usize {
    graph[current].used = true;
    graph[current]
        .connections
        .iter()
        .filter_map(|(city, dist)| {
            if graph[*city].used {
                return None;
            }
            Some(dist + distance(*city, graph.clone()))
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        let input = String::from_utf8(std::fs::read("input").unwrap()).unwrap();

        b.iter(move || hash(&input));
    }

    #[bench]
    fn bench_graph(b: &mut Bencher) {
        let input = String::from_utf8(std::fs::read("input").unwrap()).unwrap();

        b.iter(move || graph(&input));
    }
}

fn main() {
    let input = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    graph(&input);
}
