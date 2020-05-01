use day3::Coord;
use std::collections::HashSet;

fn main() {
    let mut santa = Coord::new();
    let mut bot = Coord::new();
    let mut set = HashSet::new();

    let input = std::fs::read("input").unwrap();

    set.insert(santa.clone());
    let mut iter = input.iter().map(|b| *b as char);

    loop {
        let s = iter.next().unwrap();
        if s == '\n' {
            break;
        }
        let b = iter.next().unwrap();

        santa += s;
        bot += b;
        set.insert(santa.clone());
        set.insert(bot.clone());
    }

    println!("total: {}", set.len());
}
