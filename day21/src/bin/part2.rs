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

    let mut worst_cost = 0;

    loop {
        let (won, cost) = player.fight(&boss, &shop);
        if !won && cost > worst_cost {
            worst_cost = cost;
        }
        player.increment(&shop);
        if player.weapon_used == shop.weapons.len() {
            break;
        }
    }

    aoc::answer!(
        "The most gold you can spend and still lose against the boss is {}",
        worst_cost
    );
}
