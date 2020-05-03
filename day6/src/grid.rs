use crate::Coord;

pub struct Grid {
    data: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            data: vec![vec![false; 1000]; 1000],
        }
    }

    pub fn toggle(&mut self, coord: Coord) {
        self[&coord] = !self[&coord];
    }

    pub fn turn_on(&mut self, coord: Coord) {
        self[&coord] = true;
    }

    pub fn turn_off(&mut self, coord: Coord) {
        self[&coord] = false;
    }

    pub fn through(&mut self, from: Coord, to: Coord, apply: impl Fn(&mut Self, Coord)) {
        for coord in from.to(to) {
            apply(self, coord);
        }
    }

    pub fn lit(&self) -> usize {
        Coord::new(0, 0)
            .to(Coord::new(999, 999))
            .fold(0_usize, |acc, coord| acc + self[&coord] as usize)
    }
}

impl std::ops::Index<&Coord> for Grid {
    type Output = bool;

    fn index(&self, index: &Coord) -> &Self::Output {
        assert!(index.x < 1000 && index.y < 1000);
        &self.data[index.y][index.x]
    }
}

impl std::ops::IndexMut<&Coord> for Grid {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        assert!(index.x < 1000 && index.y < 1000);
        &mut self.data[index.y][index.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_through() {
        let mut grid = Grid::new();

        grid.through(Coord::new(0, 0), Coord::new(2, 2), Grid::turn_on);
        assert_eq!(grid.lit(), 9);
    }
}
