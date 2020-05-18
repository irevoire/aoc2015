#[derive(Debug, Default)]
pub struct Entity {
    pub hp: usize,
    pub damage: usize,
    pub armor: usize,

    pub weapon_used: usize,
    pub armor_used: Option<usize>,
    pub left_ring: Option<usize>,
    pub right_ring: Option<usize>,
}

impl Entity {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fight(&self, other: &Self, shop: &Shop) -> (bool, usize) {
        let mut me = Entity::new();
        let mut cost = 0;
        me.hp = self.hp;
        me.damage += shop.weapons[self.weapon_used].damage;
        me.armor += shop.weapons[self.weapon_used].armor;
        cost += shop.weapons[self.weapon_used].cost;

        if self.armor_used.is_some() {
            me.damage += shop.armor[self.armor_used.unwrap()].damage;
            me.armor += shop.armor[self.armor_used.unwrap()].armor;
            cost += shop.armor[self.armor_used.unwrap()].cost;
        }
        if self.left_ring.is_some() {
            me.damage += shop.rings[self.left_ring.unwrap()].damage;
            me.armor += shop.rings[self.left_ring.unwrap()].armor;
            cost += shop.rings[self.left_ring.unwrap()].cost;
        }
        if self.right_ring.is_some() {
            me.damage += shop.rings[self.right_ring.unwrap()].damage;
            me.armor += shop.rings[self.right_ring.unwrap()].armor;
            cost += shop.rings[self.right_ring.unwrap()].cost;
        }

        let mut my_damage = me.damage.saturating_sub(other.armor);
        if my_damage == 0 {
            my_damage += 1
        }
        let mut other_damage = other.damage.saturating_sub(me.armor);
        if other_damage == 0 {
            other_damage += 1
        }

        (my_damage >= other_damage, cost)
    }

    pub fn increment(&mut self, shop: &Shop) {
        if self.right_ring.is_none() && self.left_ring != Some(0) {
            self.right_ring = Some(0);
        } else if self.right_ring.is_none() && self.left_ring == Some(0) {
            self.right_ring = Some(1);
        } else if let Some(mut r) = self.right_ring {
            if self.left_ring == Some(r + 1) {
                r += 1;
            }
            r += 1;
            if r == shop.rings.len() {
                self.right_ring = None;
                self.increment_left_ring(shop);
            } else {
                self.right_ring = Some(r);
            }
        }
    }

    pub fn increment_left_ring(&mut self, shop: &Shop) {
        if self.left_ring.is_none() {
            self.left_ring = Some(0);
        } else if let Some(mut l) = self.left_ring {
            l += 1;
            if l == shop.rings.len() {
                self.left_ring = None;
                self.increment_armor(shop);
            } else {
                self.left_ring = Some(l);
            }
        }
    }

    pub fn increment_armor(&mut self, shop: &Shop) {
        if self.armor_used.is_none() {
            self.armor_used = Some(0);
        } else if let Some(mut a) = self.armor_used {
            a += 1;
            if a == shop.armor.len() {
                self.armor_used = None;
                self.increment_weapon(shop);
            } else {
                self.armor_used = Some(a);
            }
        }
    }

    pub fn increment_weapon(&mut self, _shop: &Shop) {
        self.weapon_used += 1;
    }
}

#[derive(Debug, Default)]
pub struct Shop<'a> {
    pub weapons: Vec<Item<'a>>,
    pub armor: Vec<Item<'a>>,
    pub rings: Vec<Item<'a>>,
}

impl Shop<'_> {
    pub fn new() -> Self {
        let mut shop = Self::default();
        shop.weapons.push(Item::new("Dagger", 8, 4, 0));
        shop.weapons.push(Item::new("Shortsword", 10, 5, 0));
        shop.weapons.push(Item::new("Warhammer", 25, 6, 0));
        shop.weapons.push(Item::new("Longsword", 40, 7, 0));
        shop.weapons.push(Item::new("Greataxe", 74, 8, 0));

        shop.armor.push(Item::new("Leather", 13, 0, 1));
        shop.armor.push(Item::new("Chainmail", 31, 0, 2));
        shop.armor.push(Item::new("Splintmail", 53, 0, 3));
        shop.armor.push(Item::new("Bandedmail", 75, 0, 4));
        shop.armor.push(Item::new("Platemail", 102, 0, 5));

        shop.rings.push(Item::new("Damage +1", 25, 1, 0));
        shop.rings.push(Item::new("Damage +2", 50, 2, 0));
        shop.rings.push(Item::new("Damage +3", 100, 3, 0));
        shop.rings.push(Item::new("Defense +1", 20, 0, 1));
        shop.rings.push(Item::new("Defense +2", 40, 0, 2));
        shop.rings.push(Item::new("Defense +3", 80, 0, 3));

        shop
    }
}

#[derive(Debug, Clone)]
pub struct Item<'a> {
    pub name: &'a str,
    pub cost: usize,
    pub damage: usize,
    pub armor: usize,
}

impl<'a> Item<'a> {
    pub fn new(name: &'a str, cost: usize, damage: usize, armor: usize) -> Self {
        Self {
            name,
            cost,
            damage,
            armor,
        }
    }
}
