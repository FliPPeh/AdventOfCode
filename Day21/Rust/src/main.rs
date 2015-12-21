use std::collections::HashMap;

// In order: stock, price, damage, armor
type ItemStats = (i32, i32, i32, i32);
type ShopCategory = HashMap<&'static str, ItemStats>;

struct Shop {
    weapons: ShopCategory,
    armor: ShopCategory,
    rings: ShopCategory,
}

impl Shop {
    fn new() -> Shop {
        let mut shop = Shop {
            weapons: ShopCategory::new(),
            armor: ShopCategory::new(),
            rings: ShopCategory::new(),
        };

        shop.weapons.insert("Dagger", (1, 8, 4, 0));
        shop.weapons.insert("Shortsword", (1, 10, 5, 0));
        shop.weapons.insert("Warhammer", (1, 25, 6, 0));
        shop.weapons.insert("Longsword", (1, 40, 7, 0));
        shop.weapons.insert("Greataxe", (1, 74, 8, 0));

        shop.armor.insert("None", (1, 0, 0, 0));
        shop.armor.insert("Leather", (1, 13, 0, 1));
        shop.armor.insert("Chainmail", (1, 31, 0, 2));
        shop.armor.insert("Splitmail", (1, 53, 0, 3));
        shop.armor.insert("Bandedmail", (1, 75, 0, 4));
        shop.armor.insert("Platemail", (1, 102, 0, 5));

        shop.rings.insert("None", (1, 0, 0, 0));
        shop.rings.insert("Damage +1", (1, 25, 1, 0));
        shop.rings.insert("Damage +2", (1, 50, 2, 0));
        shop.rings.insert("Damage +3", (1, 100, 3, 0));
        shop.rings.insert("Defense +1", (1, 20, 0, 1));
        shop.rings.insert("Defense +2", (1, 40, 0, 2));
        shop.rings.insert("Defense +3", (1, 80, 0, 3));

        shop
    }

    fn restock(&mut self) {
        for (_, i) in self.weapons.iter_mut() {
            i.0 = 1;
        }
        for (_, i) in self.armor.iter_mut() {
            i.0 = 1;
        }
        for (_, i) in self.rings.iter_mut() {
            i.0 = 1;
        }
    }

    fn sorted_selection(cat: &ShopCategory) -> Vec<&'static str> {
        let mut k = cat.keys().cloned().collect::<Vec<&str>>();
        k.sort_by(|i1, i2| cat.get(i1).unwrap().1.cmp(&cat.get(i2).unwrap().1));
        k
    }

    fn weapon_selection(&self) -> Vec<&'static str> {
        Shop::sorted_selection(&self.weapons)
    }

    fn armor_selection(&self) -> Vec<&'static str> {
        Shop::sorted_selection(&self.armor)
    }

    fn ring_selection(&self) -> Vec<&'static str> {
        Shop::sorted_selection(&self.rings)
    }
}

type ItemSlot = Option<(&'static str, i32, i32)>;

struct Player {
    hitpoints: i32,
    armor: ItemSlot,
    weapon: ItemSlot,
    ring_left: ItemSlot,
    ring_right: ItemSlot,
    gold: i32,
}

impl Player {
    fn new() -> Player {
        Player {
            hitpoints: 100,
            armor: None,
            weapon: None,
            ring_left: None,
            ring_right: None,
            gold: 0,
        }
    }

    fn damage(&self) -> i32 {
        self.armor
            .iter()
            .chain(self.weapon.iter())
            .chain(self.ring_left.iter())
            .chain(self.ring_right.iter())
            .fold(0, |acc, &(_, d, _)| acc + d)
    }

    fn armor(&self) -> i32 {
        self.armor
            .iter()
            .chain(self.weapon.iter())
            .chain(self.ring_left.iter())
            .chain(self.ring_right.iter())
            .fold(0, |acc, &(_, _, a)| acc + a)
    }

    fn buy(slot: &mut Option<(&str, i32, i32)>,
           shop_category: &mut ShopCategory,
           item_name: &'static str)
           -> Option<i32> {
        if slot.is_some() {
            return None;
        }

        if let Some(item) = shop_category.get_mut(item_name) {
            if item.0 > 0 {
                item.0 -= 1;

                *slot = Some((item_name, item.2, item.3));

                return Some(item.1);
            }
        }

        None
    }

    fn buy_weapon(&mut self, shop: &mut Shop, weapon: &'static str) -> bool {
        Player::buy(&mut self.weapon, &mut shop.weapons, weapon)
            .and_then(|g| {
                self.gold -= g;

                Some(g)
            })
            .is_some()
    }

    fn buy_armor(&mut self, shop: &mut Shop, armor: &'static str) -> bool {
        Player::buy(&mut self.armor, &mut shop.armor, armor)
            .and_then(|g| {
                self.gold -= g;

                Some(g)
            })
            .is_some()
    }

    fn buy_ring(&mut self, shop: &mut Shop, ring: &'static str) -> bool {
        let gr = {
            let slot = if self.ring_left.is_none() {
                &mut self.ring_left
            } else if self.ring_right.is_none() {
                &mut self.ring_right
            } else {
                return false;
            };

            Player::buy(slot, &mut shop.rings, ring)
        };

        if let Some(g) = gr {
            self.gold -= g;
            return true;
        }

        false
    }
}

struct Foe {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl Foe {
    fn new() -> Foe {
        Foe {
            hitpoints: 100,
            damage: 8,
            armor: 2,
        }
    }
}

trait Entity {
    fn hitpoints(&self) -> i32;
    fn damage(&self) -> i32;
    fn armor(&self) -> i32;

    fn deal_damage(&mut self, dmg: i32);
}

impl Entity for Player {
    fn hitpoints(&self) -> i32 {
        self.hitpoints
    }
    fn damage(&self) -> i32 {
        self.damage()
    }
    fn armor(&self) -> i32 {
        self.armor()
    }

    fn deal_damage(&mut self, dmg: i32) {
        self.hitpoints -= dmg;
    }
}

impl Entity for Foe {
    fn hitpoints(&self) -> i32 {
        self.hitpoints
    }
    fn damage(&self) -> i32 {
        self.damage
    }
    fn armor(&self) -> i32 {
        self.armor
    }

    fn deal_damage(&mut self, dmg: i32) {
        self.hitpoints -= dmg;
    }
}

fn main() {
    let mut shop = Shop::new();

    let mut least_gold = (Vec::<Option<&'static str>>::new(), i32::max_value());
    let mut most_gold = (Vec::<Option<&'static str>>::new(), i32::min_value());

    for weapon in shop.weapon_selection() {
        for armor in [None]
                         .iter()
                         .cloned()
                         .chain(shop.armor_selection()
                                    .iter()
                                    .map(|&i| Some(i))) {
            for ring1 in [None]
                             .iter()
                             .cloned()
                             .chain(shop.ring_selection()
                                        .iter()
                                        .map(|&i| Some(i))) {
                for ring2 in [None]
                                 .iter()
                                 .cloned()
                                 .chain(shop.ring_selection()
                                            .iter()
                                            .map(|&i| Some(i))) {

                    let mut foe = Foe::new();
                    let mut player = Player::new();

                    player.buy_weapon(&mut shop, weapon);
                    armor.map(|a| player.buy_armor(&mut shop, a));
                    ring1.map(|r1| player.buy_ring(&mut shop, r1));
                    ring2.map(|r2| player.buy_ring(&mut shop, r2));

                    while foe.hitpoints() > 0 && player.hitpoints() > 0 {
                        // player turn
                        let d = player.damage() - foe.armor();

                        if d <= 0 {
                            // println!("The player punches the boss - it's not very effective.");
                        } else {
                            foe.deal_damage(d);

                            if foe.hitpoints() <= 0 {
                                // println!("Player deals {} damage - the boss dies in agony.", d);

                                if -player.gold < least_gold.1 {
                                    least_gold = (vec![Some(weapon), armor, ring1, ring2],
                                                  -player.gold)
                                }

                                break;
                            } else {
                                // println!("Player deals {} damage - boss health goes down to {}.",
                                //         d,
                                //         foe.hitpoints());
                            }
                        }

                        let d2 = foe.damage() - player.armor();
                        player.deal_damage(d2);

                        if player.hitpoints() <= 0 {
                            // println!("The boss deals {} damage - the player dies in agony.", d2);

                            if -player.gold > most_gold.1 {
                                most_gold = (vec![Some(weapon), armor, ring1, ring2], -player.gold);
                            }

                            break;
                        } else {
                            // println!("The boss deals {} damage - player health goes down to {}.",
                            //         d2,
                            //         player.hitpoints());
                        }

                        // println!("");
                    }

                    shop.restock();
                }
            }
        }
    }

    println!("Least gold spent and won: {}: {}",
             least_gold.1,
             least_gold.0.iter().filter_map(|&i| i).collect::<Vec<_>>().join(", "));
    println!("Most gold spent and lost: {}: {}",
             most_gold.1,
             most_gold.0.iter().filter_map(|&i| i).collect::<Vec<_>>().join(", "));
}
