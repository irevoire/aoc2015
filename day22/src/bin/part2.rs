use day22::*;
use Effect::*;

fn main() {
    let mut boss = Entity::default();

    for line in aoc::parser::lines::<String>() {
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
    [MagicMissile, Drain, Shield, Poison, Recharge]
        .iter()
        .filter_map(|effect| {
            if player.mana < effect.cost() {
                return None;
            } else if current + effect.cost() >= current_best {
                return None;
            } else if player.effects.iter().find(|e| e.1 == *effect) != None {
                return None;
            }

            let mut player = player.clone();
            let mut boss = boss.clone();

            if player_turn(&mut player, &mut boss, *effect) {
                return None;
            }
            if boss.hp == 0 {
                return Some(effect.cost());
            }

            if boss_turn(&mut player, &mut boss) {
                return Some(effect.cost());
            }
            if player.hp == 0 {
                return None;
            }

            let total = effect.cost() + lala(player, boss, current + effect.cost(), current_best)?;
            if total < current_best {
                current_best = total;
            }
            Some(total)
        })
        .min()
}

fn player_turn(player: &mut Entity, boss: &mut Entity, effect: Effect) -> bool {
    player.hp = player.hp.saturating_sub(1);
    if player.hp == 0 {
        true;
    }
    player.apply_effects();
    boss.apply_effects();
    if player.hp == 0 {
        true;
    }
    effect.cast(player, boss);
    false
}

fn boss_turn(player: &mut Entity, boss: &mut Entity) -> bool {
    player.apply_effects();
    boss.apply_effects();
    if boss.hp == 0 {
        return true;
    }
    boss.attack(player);
    false
}
