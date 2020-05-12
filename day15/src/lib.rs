#[derive(Debug, Clone, PartialEq)]
pub struct Ingredient {
    name: String,
    pub capacity: isize,
    pub durability: isize,
    pub flavor: isize,
    pub texture: isize,
    pub calories: isize,
}

impl Ingredient {
    pub fn new(
        name: String,
        capacity: isize,
        durability: isize,
        flavor: isize,
        texture: isize,
        calories: isize,
    ) -> Self {
        Self {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

use std::convert::TryInto;
pub fn score(list: &[&Ingredient]) -> usize {
    let capacity: usize = list
        .iter()
        .map(|ing| ing.capacity)
        .sum::<isize>()
        .try_into()
        .unwrap_or(0);
    let durability: usize = list
        .iter()
        .map(|ing| ing.durability)
        .sum::<isize>()
        .try_into()
        .unwrap_or(0);
    let flavor: usize = list
        .iter()
        .map(|ing| ing.flavor)
        .sum::<isize>()
        .try_into()
        .unwrap_or(0);
    let texture: usize = list
        .iter()
        .map(|ing| ing.texture)
        .sum::<isize>()
        .try_into()
        .unwrap_or(0);

    capacity * durability * flavor * texture
}

pub fn calories(list: &[&Ingredient]) -> usize {
    let calories: usize = list
        .iter()
        .map(|ing| ing.calories)
        .sum::<isize>()
        .try_into()
        .unwrap_or(0);

    calories
}
