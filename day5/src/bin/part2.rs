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
    two_pairs(s) && repeat(s)
}

fn two_pairs(s: &str) -> bool {
    let buf = s.chars().collect::<Vec<char>>();
    let mut iter = buf.windows(2);

    while let Some(arr) = iter.next() {
        let mut following = iter.clone().skip(1);
        if let Some(_el) = following.find(|arr2| &arr == arr2) {
            return true;
        }
    }
    return false;
}

fn repeat(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|arr| arr[0] != arr[1] && arr[0] == arr[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        assert_eq!(two_pairs("aaa"), false);
        assert_eq!(two_pairs("aaaa"), true);
        assert_eq!(two_pairs("xyxy"), true);
        assert_eq!(two_pairs("aabcdefgaa"), true);
    }
}
