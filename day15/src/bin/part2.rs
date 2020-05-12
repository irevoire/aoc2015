use day15::*;
use std::convert::TryInto;

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

    let sprinkles = &all_ingredients[0];
    let butterscotch = &all_ingredients[1];
    let chocolate = &all_ingredients[2];
    let candy = &all_ingredients[3];

    let mut best = 0;
    for nb_sprinkles in 0..100 {
        for nb_butterscotch in 0..(100 - nb_sprinkles) {
            for nb_chocolate in 0..(100 - (nb_sprinkles + nb_butterscotch)) {
                let nb_candy = 100 - (nb_sprinkles + nb_butterscotch + nb_chocolate);
                let capacity = sprinkles.capacity * nb_sprinkles
                    + butterscotch.capacity * nb_butterscotch
                    + chocolate.capacity * nb_chocolate
                    + candy.capacity * nb_candy;
                let durability = sprinkles.durability * nb_sprinkles
                    + butterscotch.durability * nb_butterscotch
                    + chocolate.durability * nb_chocolate
                    + candy.durability * nb_candy;
                let flavor = sprinkles.flavor * nb_sprinkles
                    + butterscotch.flavor * nb_butterscotch
                    + chocolate.flavor * nb_chocolate
                    + candy.flavor * nb_candy;
                let texture = sprinkles.texture * nb_sprinkles
                    + butterscotch.texture * nb_butterscotch
                    + chocolate.texture * nb_chocolate
                    + candy.texture * nb_candy;
                let calories = sprinkles.calories * nb_sprinkles
                    + butterscotch.calories * nb_butterscotch
                    + chocolate.calories * nb_chocolate
                    + candy.calories * nb_candy;

                let capacity: usize = capacity.try_into().unwrap_or(0);
                let durability: usize = durability.try_into().unwrap_or(0);
                let flavor: usize = flavor.try_into().unwrap_or(0);
                let texture: usize = texture.try_into().unwrap_or(0);
                let calories: usize = calories.try_into().unwrap_or(0);

                if calories != 500 {
                    continue;
                }

                let score = capacity * durability * flavor * texture;

                if score > best {
                    best = score;
                }
            }
        }
    }

    println!("greedy best score is {}", best);
}
