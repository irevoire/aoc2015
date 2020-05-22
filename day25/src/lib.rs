#[derive(Debug)]
pub struct Generator {
    current: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self { current: 20151125 }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current;
        self.current *= 252533;
        self.current %= 33554393;

        Some(res)
    }
}

#[derive(Default, Debug)]
pub struct GridGenerator {
    row: usize,
    max_row: usize,
    col: usize,
    generator: Generator,
}

impl Iterator for GridGenerator {
    ///         (row,   col,   value)
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let crow = self.row;
        let ccol = self.col;
        let cvalue = self.generator.next()?;

        if crow == 0 {
            self.max_row += 1;
            self.row = self.max_row;
            self.col = 0;
        } else {
            self.col += 1;
            self.row -= 1;
        }

        Some((crow, ccol, cvalue))
    }
}
