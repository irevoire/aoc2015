use aoc::*;

fn main() {
    let mut grid = Grid::from(vec![vec![0_usize; 1000]; 1000]);

    parser::lines::<String>().for_each(|line| {
        let mut vec: Vec<&str> = line.rsplitn(4, ' ').collect();
        vec.reverse();
        let base: Coord<usize> = vec[1].parse().unwrap();
        let end: Coord<usize> = vec[3].parse().unwrap();

        let iter = grid.through_mut(base, end).unwrap();
        match vec[0] {
            "turn on" => iter.for_each(|el| *el += 1),
            "turn off" => iter.for_each(|el| *el = el.saturating_sub(1)),
            "toggle" => iter.for_each(|el| *el += 2),
            s => panic!("wtf is this shit: {}", s),
        }
    });

   aoc::answer!("There is {} light lit", grid.iter().sum::<usize>());
}
