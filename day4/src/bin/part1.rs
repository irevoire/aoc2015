fn main() {
    let base = std::env::args().skip(1).next().expect("give me your inupt");
    for id in 0.. {
        let new = format!("{}{}", base, id);
        let digest = md5::compute(new);
        if &digest[0..2] == &[0; 2] && digest[2] < 0xf {
            println!("finished {}", id);
            break;
        }
    }
}
