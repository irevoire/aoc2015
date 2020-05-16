use std::collections::{HashMap, HashSet};
fn main() {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let mut end = false;
    let mut input = String::new();
    for line in aoc::parser::lines_from_args(1) {
        if end {
            input = line.into();
        } else if line.is_empty() {
            end = true;
        } else {
            let line = line.split("=>").map(str::trim).collect::<Vec<&str>>();
            let (key, value) = (line[0], line[1]);

            let entry = replacements.entry(key.into()).or_insert(Vec::new());
            entry.push(value.into());
        }
    }

    let nb = permutations(&input, &replacements);

    println!("There is {} possible permutations", nb);
}

fn permutations(input: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
    let mut set = HashSet::new();

    for i in 0..input.len() {
        for (key, value) in replacements.iter() {
            if input[i..].starts_with(key) {
                for v in value {
                    set.insert(format!("{}{}{}", &input[..i], v, &input[i + key.len()..]));
                }
            }
        }
    }

    set.len()
}
