pub fn in_memory(string: &str) -> usize {
    if string.len() == 2 {
        return 0;
    }
    let string = string[1..].replace("\\\\", "\\\x12");
    let mut string = string.split('\\');
    string.next().unwrap().len()
        + string
            .map(|slice| {
                if slice == "" {
                    0
                } else {
                    match slice.split_at(1) {
                        ("x", s) => s.len() - 1,
                        // this is a backslash now
                        ("\x12", s) => s.len() + 1,
                        ("\"", s) => s.len() + 1,
                        (_c, s) => 1 + s.len(),
                    }
                }
            })
            .sum::<usize>()
        - 1
}

pub fn encode_size(string: &str) -> usize {
    string
        .chars()
        .map(|c| match c {
            '\"' => 2,
            '\\' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory() {
        assert_eq!(in_memory("\"\""), 0);
        assert_eq!(in_memory("\"abc\""), 3);
        assert_eq!(in_memory("\"aaa\\\"aaa\""), 7);
        assert_eq!(in_memory("\"\\x27\""), 1);
        assert_eq!(in_memory("\"\\\\cv\\\"\""), 4);
    }

    #[test]
    fn test_encode_size() {
        assert_eq!(encode_size("\"\""), 6);
    }
}
