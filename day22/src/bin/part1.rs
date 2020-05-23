use day22::*;
use Effect::*;

fn main() {
    let mut boss = Entity::default();

    for line in aoc::parser::lines_from_args(1) {
        let mut line = line.split(':').map(str::trim);
        match line.next().unwrap() {
            "Hit Points" => boss.hp = line.next().unwrap().parse().unwrap(),
            "Damage" => boss.damage = line.next().unwrap().parse().unwrap(),
            s => panic!("Unexpected stat: {}", s),
        }
    }
    let mut player = Entity::default();
    player.hp = 50;
    player.mana = 500;

    println!("{}", lala(player, boss, 0, std::usize::MAX).unwrap());
}

fn lala(player: Entity, boss: Entity, current: usize, mut current_best: usize) -> Option<usize> {
    if boss.hp == 0 {
        return Some(0);
    }
    if player.hp == 0 {
        return None;
    }

    [MagicMissile, Drain, Shield, Poison, Recharge]
        .iter()
        .filter_map(|effect| {
            if player.mana < effect.cost() {
                return None;
            } else if current + effect.cost() >= current_best {
                return None;
            }

            let mut player = player.clone();
            let mut boss = boss.clone();

            effect.cast(&mut player, &mut boss);
            boss.apply_effects();
            player.apply_effects();
            boss.attack(&mut player);

            let total = effect.cost() + lala(player, boss, current + effect.cost(), current_best)?;
            if total < current_best {
                current_best = total;
            }
            Some(total)
        })
        .min()
}
