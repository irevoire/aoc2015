pub struct Grid {
    data: Vec<Vec<bool>>,
}

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            data: vec![vec![false; 1000]; 1000],
        }
    }

    pub fn toogle(&mut self, coord: Coord) {
        assert!(coord.x < 1000 && coord.y < 1000);

        self.data[coord.y][coord.x] = !self.data[coord.y][coord.x];
    }

    pub fn turn_on(&mut self, coord: Coord) {
        assert!(coord.x < 1000 && coord.y < 1000);

        self.data[coord.y][coord.x] = true;
    }

    pub fn turn_off(&mut self, coord: Coord) {
        assert!(coord.x < 1000 && coord.y < 1000);

        self.data[coord.y][coord.x] = false;
    }

    pub fn through(&mut self, from: Coord, to: Coord, apply: impl Fn(&mut Self, Coord)) {
        apply(self, from);
    }
}
