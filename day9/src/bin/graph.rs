fn main() {
    println!("graph G {{");
    for line in aoc::parser::lines_from_args(1) {
        let line: Vec<&str> = line.split("=").map(str::trim).collect();
        let (cities, distance) = (line[0], line[1]);
        let distance: usize = distance.parse().unwrap();
        let cities: Vec<&str> = cities.split("to").map(str::trim).collect();
        let (left, right) = (cities[0], cities[1]);

        println!(r#"    {} -- {} [label="{}"];"#, left, right, distance);
    }
    println!("}}");
}
