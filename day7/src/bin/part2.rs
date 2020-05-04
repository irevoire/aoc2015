use std::fs::File;
use std::io::BufReader;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let original = day7::parse(reader);
    let mut assignations = original.clone();

    let a = assignations.get("a").unwrap().clone();
    let a = a.compute(&mut assignations);

    let mut assignations = original.clone();
    *assignations.get_mut("b").unwrap() = a;

    // restart everything
    let a = assignations.get("a").unwrap().clone();

    println!("{}", a.compute(&mut assignations).unwrap());
}
