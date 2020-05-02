use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let nice: usize = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| nice(line))
        .count();

    println!("There is {} nice string", nice);
}

fn nice(s: &str) -> bool {
    three_vowels(s) && twice_in_a_row(s) && no_banned_string(s)
}

fn three_vowels(s: &str) -> bool {
    s.chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        })
        .sum::<u8>()
        >= 3
}

fn twice_in_a_row(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|arr| arr[0] == arr[1])
}

fn no_banned_string(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .all(|arr| match (arr[0], arr[1]) {
            ('a', 'b') => false,
            ('c', 'd') => false,
            ('p', 'q') => false,
            ('x', 'y') => false,
            _ => true,
        })
}
