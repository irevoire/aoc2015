fn main() {
    let base = std::env::args().skip(1).next().expect("give me your inupt");
    for id in 0.. {
        let new = format!("{}{}", base, id);
        let digest = md5::compute(new);
        if &digest[0..3] == &[0; 3] {
            println!("finished {}", id);
            break;
        }
    }
}
