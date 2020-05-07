use aoc::iterator::Until;

pub fn expand(s: &str) -> String {
    let mut res = String::new();
    let iter = &mut s.chars().peekable();

    while let Some(base) = iter.next() {
        let nb = iter.until(|c| *c != base) + 1;
        res.push_str(&nb.to_string());
        res.push_str(&base.to_string());
    }
    res
}
