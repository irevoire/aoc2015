use day16::Aunt;

fn main() {
    let aunts = day16::parse();
    let mut gift = Aunt::new("Unknown".into());
    gift.with_children(3);
    gift.with_cats(7);
    gift.with_samoyeds(2);
    gift.with_pomeranians(3);
    gift.with_akitas(0);
    gift.with_vizslas(0);
    gift.with_goldfish(5);
    gift.with_trees(3);
    gift.with_cars(2);
    gift.with_perfumes(1);

    for aunt in aunts {
        if gift == aunt {
            println!("It could be {}", aunt.name());
        }
    }
}
