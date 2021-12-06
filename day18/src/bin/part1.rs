use aoc::{Coord, Grid};

fn main() {
    let grid: Vec<Vec<bool>> = aoc::parser::lines::<String>()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    c => panic!("unknown char {}", c),
                })
                .collect::<Vec<bool>>()
        })
        .collect();

    let mut grid = aoc::Grid::from(grid);

    for _ in 0..100 {
        grid = update_grid(&grid);
    }

    let total: usize = grid.iter().map(|b| *b as usize).sum();

    aoc::answer!("There is a total of {} light on", total);
}

fn update_grid(grid: &Grid<bool>) -> Grid<bool> {
    let mut res = grid.clone();

    for coord in Coord::at(0, 0).to(Coord::at(99, 99)).unwrap() {
        let n = day18::neighbours(coord, &grid);
        match (grid[coord], n) {
            (true, 2) | (true, 3) | (false, 3) => res[coord] = true,
            _ => res[coord] = false,
        }
    }

    res
}
