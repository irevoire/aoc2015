use aoc::*;

fn main() {
    let nice: usize = parser::lines::<String>().filter(|line| nice(line)).count();

    answer!("There is {} nice string", nice);
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
        .any(|arr| arr[0] == arr[2])
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
