use day21::*;

fn main() {
    let mut boss = Entity::new();
    let mut player = Entity::new();
    let shop = Shop::new();

    for line in aoc::parser::lines::<String>() {
        let line: Vec<&str> = line.split(':').map(str::trim).collect();

        match line[0] {
            "Hit Points" => boss.hp = line[1].parse().unwrap(),
            "Damage" => boss.damage = line[1].parse().unwrap(),
            "Armor" => boss.armor = line[1].parse().unwrap(),
            _ => panic!("ntm"),
        }
    }

    let mut best_cost = std::usize::MAX;

    loop {
        let (won, cost) = player.fight(&boss, &shop);
        if won && cost < best_cost {
            best_cost = cost;
        }
        player.increment(&shop);
        if player.weapon_used == shop.weapons.len() {
            break;
        }
    }

    aoc::answer!(
        "The best cost possible to win against the boss is {}",
        best_cost
    );
}
