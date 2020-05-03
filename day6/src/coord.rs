use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to(self, end: Coord) -> Range {
        Range {
            start: self.clone(),
            current: self,
            finished: false,
            end,
        }
    }
}

impl cmp::PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

#[derive(Debug)]
pub struct Range {
    pub start: Coord,
    current: Coord,
    finished: bool,
    pub end: Coord,
}

impl Iterator for Range {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current.clone();
        if self.finished {
            return None;
        }
        if self.current.x == self.end.x && self.current.y == self.end.y {
            self.finished = true;
        } else if self.current.x == self.end.x {
            self.current.x = self.start.x;
            self.current.y += 1;
        } else {
            self.current.x += 1;
        }
        Some(res)
    }
}

impl std::str::FromStr for Coord {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x = coords[0].parse::<usize>()?;
        let y = coords[1].parse::<usize>()?;

        Ok(Coord { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_through() {
        let mut iter = Coord::new(0, 0).to(Coord::new(2, 2));

        assert_eq!(iter.next(), Some(Coord::new(0, 0)));
        assert_eq!(iter.next(), Some(Coord::new(1, 0)));
        assert_eq!(iter.next(), Some(Coord::new(2, 0)));
        assert_eq!(iter.next(), Some(Coord::new(0, 1)));
    }
}
