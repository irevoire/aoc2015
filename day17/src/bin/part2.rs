fn main() {
    let containers: Vec<usize> = aoc::parser::lines_from_args(1)
        .map(|line| line.parse().unwrap())
        .collect();
    let mut used = vec![false; containers.len()];
    let mut total = 0;
    let mut min = std::usize::MAX;

    loop {
        increment(&mut used);
        let c = total_containers(&used);
        if c > min {
            continue;
        }
        let t = total_capacity(&containers, &used);
        if t == 150 && c < min {
            total = 1;
            min = c;
        } else if t == 150 && c == min {
            total += 1;
        } else if t == 0 && c == 0 {
            break;
        }
    }

    println!(
        "There is {} way to fill 150 litters with a minimum of {} containers",
        total, min
    );
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

fn total_containers(used: &[bool]) -> usize {
    used.iter().filter(|el| **el).count()
}

fn total_capacity(containers: &[usize], used: &[bool]) -> usize {
    used.iter()
        .zip(containers)
        .filter(|(used, _)| **used)
        .map(|(_, capacity)| capacity)
        .sum()
}
