use aoc::{Coord, Direction::*, Grid};

pub fn print_grid(grid: &Grid<bool>) {
    print!("\x1B[100A");

    for line in grid.lines() {
        for el in line {
            match el {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

pub fn neighbours(coord: Coord<usize>, grid: &Grid<bool>) -> usize {
    match (coord.x, coord.y) {
        (0, 0) => {
            grid[coord + East] as usize
                + grid[coord + South] as usize
                + grid[coord + South + East] as usize
        }
        (99, 99) => {
            grid[coord + West] as usize
                + grid[coord + North] as usize
                + grid[coord + North + West] as usize
        }
        (0, 99) => {
            grid[coord + North] as usize
                + grid[coord + North + East] as usize
                + grid[coord + East] as usize
        }
        (99, 0) => {
            grid[coord + West] as usize
                + grid[coord + South] as usize
                + grid[coord + West + South] as usize
        }
        (0, _) => {
            grid.through(coord + North, coord + South + East)
                .unwrap()
                .map(|b| *b as usize)
                .sum::<usize>()
                - grid[coord] as usize
        }
        (99, _) => {
            grid.through(coord + North + West, coord + South)
                .unwrap()
                .map(|b| *b as usize)
                .sum::<usize>()
                - grid[coord] as usize
        }
        (_, 0) => {
            grid.through(coord + West, coord + South + East)
                .unwrap()
                .map(|b| *b as usize)
                .sum::<usize>()
                - grid[coord] as usize
        }
        (_, 99) => {
            grid.through(coord + North + West, coord + East)
                .unwrap()
                .map(|b| *b as usize)
                .sum::<usize>()
                - grid[coord] as usize
        }
        (_, _) => {
            grid.through(coord + North + West, coord + South + East)
                .unwrap()
                .map(|b| *b as usize)
                .sum::<usize>()
                - grid[coord] as usize
        }
    }
}
