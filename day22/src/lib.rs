#[derive(Default, Debug, Clone)]
pub struct Entity {
    pub hp: usize,
    pub damage: usize,
    pub armor: usize,
    pub mana: usize,
    pub effects: Vec<(usize, Effect)>,
}

impl Entity {
    pub fn apply_effects(&mut self) {
        for idx in 0..self.effects.len() {
            self.effects[idx].1.apply(self);
        }

        self.effects = self
            .effects
            .iter()
            .map(|(nb, e)| (nb - 1, *e))
            .filter(|(nb, _)| *nb > 0)
            .collect();
    }

    pub fn add_effect(&mut self, effect: (usize, Effect)) {
        self.effects.push(effect);
    }

    pub fn attack(&self, other: &mut Self) {
        let mut real_damage = self.damage.saturating_sub(other.armor);
        if real_damage == 0 {
            real_damage = 1;
        }
        other.hp = other.hp.saturating_sub(real_damage);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Effect {
    MagicMissile = 53,
    Drain = 73,
    Shield = 113,
    Poison = 173,
    Recharge = 229,
}

use Effect::*;

impl Effect {
    pub fn cost(self) -> usize {
        self as usize
    }

    pub fn cast(self, player: &mut Entity, boss: &mut Entity) {
        match self {
            MagicMissile => boss.hp = boss.hp.saturating_sub(4),
            Drain => {
                boss.hp = boss.hp.saturating_sub(2);
                player.hp += 2;
            }
            Shield => player.add_effect((6, Shield)),
            Poison => boss.add_effect((6, Poison)),
            Recharge => player.add_effect((5, Recharge)),
        }
        player.mana -= self.cost();
    }

    pub fn apply(self, entity: &mut Entity) {
        match self {
            Shield => entity.armor += 7,
            Poison => entity.hp = entity.hp.saturating_sub(3),
            Recharge => entity.mana += 101,
            e => panic!("Called with {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight_smol() {
        let mut player = Entity::default();
        player.hp = 10;
        player.mana = 250;
        let mut boss = Entity::default();
        boss.hp = 13;
        boss.damage = 8;

        // player turn
        assert_eq!(player.hp, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 250);
        assert_eq!(boss.hp, 13);
        // palyer cast poison
        Poison.cast(&mut player, &mut boss);

        // boss turn
        assert_eq!(player.hp, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hp, 13);

        boss.apply_effects();
        player.apply_effects();
        assert!(player.effects.is_empty());
        assert_eq!(boss.effects[0].0, 5);

        boss.attack(&mut player);

        // player turn
        assert_eq!(player.hp, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hp, 10);
        // palyer cast poison
        boss.apply_effects();
        player.apply_effects();
        assert_eq!(boss.effects[0].0, 4);

        // player cast a magic missile to the boss dealing 4 damage
        MagicMissile.cast(&mut player, &mut boss);

        // boss turn
        assert_eq!(player.hp, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 24);
        assert_eq!(boss.hp, 3);

        boss.apply_effects();
        player.apply_effects();
        assert_eq!(boss.effects[0].0, 3);

        // boss die
        assert_eq!(boss.hp, 0);

        // more test
        boss.apply_effects();
        assert_eq!(boss.effects[0].0, 2);

        boss.apply_effects();
        assert_eq!(boss.effects[0].0, 1);

        boss.apply_effects();
        assert!(boss.effects.is_empty());
    }
}
