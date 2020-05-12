use day15::*;
use rand::seq::SliceRandom;

fn main() {
    let all_ingredients: Vec<Ingredient> = aoc::parser::lines_from_args(1)
        .map(|line| {
            let line: Vec<&str> = line.split(':').collect();
            let name = line[0].into();
            let line: Vec<isize> = line[1]
                .split(',')
                .map(str::trim)
                .map(|attribute| attribute.split(' ').collect::<Vec<&str>>())
                .map(|attr| attr[1].parse::<isize>().unwrap())
                .collect();
            let (capacity, durability, flavor, texture, calories) =
                (line[0], line[1], line[2], line[3], line[4]);
            Ingredient::new(name, capacity, durability, flavor, texture, calories)
        })
        .collect();

    let mut rng = rand::thread_rng();
    let mut iter = 0;
    let mut best = 0;
    // iter while we have not get 1000 iteration without any improvement
    while iter < 100000 {
        let mut base: Vec<&Ingredient> =
            std::iter::repeat_with(|| all_ingredients.choose(&mut rng).unwrap())
                .take(100)
                .collect();
        greed(&all_ingredients, &mut base);
        let score = score(&base);
        if score > best {
            best = score;
            iter = 0;
        }
        iter += 1;
    }

    println!("greedy best score is {}", best);
}

fn greed<'a>(base: &'a Vec<Ingredient>, current: &mut Vec<&'a Ingredient>) {
    let mut base_score = score(current);
    if base_score == 0 {
        return; // it’s shit and I don’t want to lose any time on this
    }
    for idx in 0..base.len() {
        let base_ingr = current[idx];
        for ingr in base {
            if ingr == base_ingr {
                return;
            }
            current[idx] = &ingr;
            let score = score(current);
            if score > base_score {
                base_score = score;
            } else {
                current[idx] = base_ingr;
            }
        }
    }
}
