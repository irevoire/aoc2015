fn main() {
    let containers: Vec<usize> = aoc::parser::lines::<String>()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut used = vec![false; containers.len()];
    let mut total = 0;

    loop {
        increment(&mut used);
        let t = total_capacity(&containers, &used);
        if t == 150 {
            total += 1;
        } else if t == 0 {
            break;
        }
    }

    aoc::answer!("There is {} possibilities", total);
}

fn increment(used: &mut [bool]) {
    for el in used {
        if *el == true {
            *el = false;
        } else {
            *el = true;
            break;
        }
    }
}

fn total_capacity(containers: &[usize], used: &[bool]) -> usize {
    used.iter()
        .zip(containers)
        .filter(|(used, _)| **used)
        .map(|(_, capacity)| capacity)
        .sum()
}
