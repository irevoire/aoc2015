use aoc::{Coord, Direction};
use std::collections::HashSet;

fn main() {
    let mut santa = Coord::<i32>::new();
    let mut bot = Coord::new();
    let mut set = HashSet::new();

    let input = std::fs::read("input").unwrap();

    set.insert(santa.clone());
    let mut iter = input.iter().map(|b| *b as char);

    loop {
        let s = iter.next().unwrap();
        if let Ok(dir) = s.to_string().parse::<Direction>() {
            santa += dir;
            set.insert(santa.clone());
        } else {
            break;
        }
        let b = iter.next().unwrap();
        if let Ok(dir) = b.to_string().parse::<Direction>() {
            bot += dir;
            set.insert(bot.clone());
        } else {
            break;
        }
    }

    println!("total: {}", set.len());
}
