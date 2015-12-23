extern crate rand;

use rand::{thread_rng, Rng};

use std::cmp::max;

#[derive(Debug, Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum StatusEffect {
    Shield(i32),
    Recharge(i32),
    Poison(i32),
}

impl StatusEffect {
    fn same_kind(&self, other: &StatusEffect) -> bool {
        match (*self, *other) {
            (StatusEffect::Shield(_), StatusEffect::Shield(_)) => true,
            (StatusEffect::Recharge(_), StatusEffect::Recharge(_)) => true,
            (StatusEffect::Poison(_), StatusEffect::Poison(_)) => true,
            _ => false,
        }
    }
}

struct Player {
    effects: Vec<(StatusEffect, i32)>,
    hitpoints: i32,
    mana: i32,
    mana_spent: i32,
}

impl Player {
    fn new(hitpoints: i32, mana: i32) -> Player {
        Player {
            effects: Vec::new(),
            hitpoints: hitpoints,
            mana: mana,
            mana_spent: 0,
        }
    }

    fn cast_spell(&mut self, spell: Spell, target: &mut Entity) -> Option<i32> {
        if self.mana < spell.cost() {
            return None;
        }

        if !match spell {
            Spell::MagicMissile => {
                target.deal_damage(4);

                true
            }
            Spell::Drain => {
                target.deal_damage(2);
                self.heal(2);

                true
            }
            Spell::Shield => self.apply_status(StatusEffect::Shield(7), 6),
            Spell::Poison => self.apply_status(StatusEffect::Poison(3), 6),
            Spell::Recharge => self.apply_status(StatusEffect::Recharge(101), 5),
        } {
            return None;
        }

        self.mana -= spell.cost();

        Some(spell.cost())
    }

    fn apply_status(&mut self, effect: StatusEffect, duration: i32) -> bool {
        if !self.effects.iter().any(|&(ref e, _)| e.same_kind(&effect)) {
            self.effects.push((effect, duration));

            true
        } else {
            false
        }
    }

    fn tick_status(&mut self, foe: &mut Entity) {
        let mut mana = 0;

        for &(ref e, _) in &self.effects {
            match *e {
                StatusEffect::Shield(_) => {}
                StatusEffect::Recharge(n) => {
                    mana += n;
                }
                StatusEffect::Poison(n) => {
                    foe.deal_damage(n);
                }
            }
        }

        self.mana += mana;

    }

    fn decay_status(&mut self) {
        self.effects = self.effects
                           .iter()
                           .filter_map(|&(ref e, ref n)| {
                               if *n > 1 {
                                   Some((*e, *n - 1))
                               } else {
                                   None
                               }
                           })
                           .collect();
    }

}

struct Boss {
    hitpoints: i32,
    damage: i32,
}

impl Boss {
    fn new() -> Boss {
        Boss {
            hitpoints: 55,
            damage: 8,
        }
    }
}

trait Entity {
    fn hitpoints(&self) -> i32;
    fn armor(&self) -> i32;
    fn damage(&self) -> i32;

    fn deal_damage(&mut self, amount: i32) -> i32;
    fn heal(&mut self, amount: i32) -> i32;
}

impl Entity for Player {
    fn hitpoints(&self) -> i32 {
        self.hitpoints
    }

    fn armor(&self) -> i32 {
        self.effects.iter().fold(0, |acc, &(ref eff, _)| {
            match eff {
                &StatusEffect::Shield(n) => acc + n,
                _ => acc,
            }
        })
    }

    fn damage(&self) -> i32 {
        0
    }

    fn deal_damage(&mut self, damage: i32) -> i32 {
        self.hitpoints -= max(1, damage - self.armor());

        max(1, damage - self.armor())
    }

    fn heal(&mut self, amount: i32) -> i32 {
        self.hitpoints += amount;

        amount
    }
}

impl Entity for Boss {
    fn hitpoints(&self) -> i32 {
        self.hitpoints
    }

    fn armor(&self) -> i32 {
        0
    }

    fn damage(&self) -> i32 {
        self.damage
    }

    fn deal_damage(&mut self, damage: i32) -> i32 {
        self.hitpoints -= damage;

        damage
    }

    fn heal(&mut self, amount: i32) -> i32 {
        self.hitpoints += amount;

        self.hitpoints
    }
}

enum TurnResult {
    PlayerRip,
    BossRip,
    Boring,
}

fn player_turn(player: &mut Player, boss: &mut Boss, spells: &mut [Spell]) -> TurnResult {
    //
    // Hard mode
    player.deal_damage(1);

    // Apply all active effects
    player.tick_status(boss);
    player.decay_status();

    // Poisonned to death? (not implemented, but possible)
    if player.hitpoints() <= 0 {
        return TurnResult::PlayerRip;
    }

    let choices = spells.iter().filter(|s| s.cost() <= player.mana).collect::<Vec<_>>();

    if choices.len() == 0 {
        // Out of mana, duh
        return TurnResult::PlayerRip;
    }

    for &spell in &choices {
        if let Some(n) = player.cast_spell(*spell, boss) {
            player.mana_spent += n;

            if boss.hitpoints() <= 0 {
                return TurnResult::BossRip;
            }

            return TurnResult::Boring;
        }
    }

    unreachable!();
}

fn boss_turn(player: &mut Player, boss: &mut Boss) -> TurnResult {
    // Boss turn
    player.tick_status(boss);
    player.decay_status();

    // Poisonned to death?
    if boss.hitpoints() <= 0 {
        return TurnResult::BossRip;
    }

    player.deal_damage(boss.damage());

    if player.hitpoints() <= 0 {
        TurnResult::PlayerRip
    } else {
        TurnResult::Boring
    }
}

fn turn(player: &mut Player, boss: &mut Boss, spells: &mut [Spell]) -> TurnResult {
    match player_turn(player, boss, spells) {
        TurnResult::Boring => boss_turn(player, boss),
        result => result,
    }
}

fn main() {
    let mut spells = vec![Spell::MagicMissile,
                          Spell::Drain,
                          Spell::Shield,
                          Spell::Poison,
                          Spell::Recharge];

    let mut rng = thread_rng();

    let mut min_mana = i32::max_value();
    let mut win_player = 0;
    let mut win_boss = 0;

    loop {
        let mut p = Player::new(50, 500);
        let mut b = Boss::new();

        loop {
            rng.shuffle(&mut spells);

            match turn(&mut p, &mut b, &mut spells) {
                TurnResult::PlayerRip => {
                    win_boss += 1;

                    // println!("Round {}: RIP player [{}, {}/{}]",
                    // win_player + win_boss,
                    // min_mana,
                    // win_player,
                    // win_boss);
                    break;
                }
                TurnResult::BossRip => {
                    win_player += 1;

                    if p.mana_spent < min_mana {
                        min_mana = p.mana_spent;

                        println!("Round {}: RIP boss [{}, {}/{}]",
                                 win_player + win_boss,
                                 p.mana_spent,
                                 win_player,
                                 win_boss);
                    }

                    break;
                }
                _ => {}
            }
        }
    }
}
