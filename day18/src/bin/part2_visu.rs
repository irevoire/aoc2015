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
    grid[(0, 0)] = true;
    grid[(99, 0)] = true;
    grid[(0, 99)] = true;
    grid[(99, 99)] = true;

    // create 100 blank lines
    for _ in 0..100 {
        println!();
    }

    loop {
        day18::print_grid(&grid);
        grid = update_grid(&grid);
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

fn update_grid(grid: &Grid<bool>) -> Grid<bool> {
    let mut res = grid.clone();

    for coord in Coord::at(0, 0).to(Coord::at(99, 99)).unwrap() {
        if coord == (0, 0) || coord == (99, 0) || coord == (0, 99) || coord == (99, 99) {
            continue;
        }
        let n = day18::neighbours(coord, &grid);
        match (grid[coord], n) {
            (true, 2) | (true, 3) | (false, 3) => res[coord] = true,
            _ => res[coord] = false,
        }
    }

    res
}
