#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new() -> Self {
        Default::default()
    }
}

impl std::ops::AddAssign<char> for Coord {
    fn add_assign(&mut self, other: char) {
        match other {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!("ntm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord() {
        let mut coord = Coord::new();
        assert_eq!(coord, Coord { x: 0, y: 0 });
        coord += '>';
        assert_eq!(coord, Coord { x: 1, y: 0 });
    }
}
