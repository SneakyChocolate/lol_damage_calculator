use crate::champion::Champion;
use crate::lol_item::LolItem;
use crate::stats::{Damage, Stats};
use std::rc::Rc;

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub champion: Rc<Champion>,
    pub items: Rc<LolItem>,
    pub rune: Rc<Stats>,
}

impl Entity {
    pub fn sum(&self) -> Stats {
        self.champion.stats + self.items.stats + *self.rune
    }

    pub fn get_as(&self) -> f32 {
        let self_sum = self.sum();
        let mut attackspeed = self_sum.base_attack_speed * (1.0 + self_sum.attack_speed * self_sum.attack_speed_ratio);
        if attackspeed > 2.5 {
            attackspeed = 2.5;
        }
        return attackspeed;
    }
    
    pub fn fight(&self, enemy: &Self) -> (f32, f32) {
        let mut enemy_sum: Stats = enemy.sum();
        let mut self_sum: Stats = self.sum();

        // self sum on hit
        let sso = &mut self_sum.on_hit_damage;

        let mut attack_speed = self.get_as();
        let mut enemy_attack_speed = enemy.get_as();
        let bonus_damage = self.items.stats.attack_damage + self.rune.attack_damage;
        let mut attack_damage = self_sum.attack_damage;
        let mut life_steal = self_sum.life_steal;
        let mut damage_taken: Damage = Default::default();

        // calculate raw on hit damage
        let mut raw_on_hit: Damage = sso.flat
            + sso.enemy_hp.powf(3.0) * (enemy_sum.health / 3.0)
            + sso.bonus_ad * bonus_damage
            + sso.base_ad * self.champion.stats.attack_damage
            + sso.max_hp * self_sum.health;

        let mut crit_chance = if self_sum.crit_chance > 1.0 {
            1.0
        } else {
            self_sum.crit_chance
        };
        let mut crit_damage_multiplier = (self_sum.crit_damage + 1.0) * crit_chance;

        // handle item effects
        let mut c = String::from("seething strike");
        if self.items.effects.contains(&c) {
            raw_on_hit *= 4.0 / 3.0;
        }
        c = String::from("grievous wounds");
        if enemy.items.effects.contains(&c) {
            life_steal *= 0.6;
        }
        c = String::from("thorns");
        if enemy.items.effects.contains(&c) {
            damage_taken.magic += 10.0 + enemy_sum.armor * 0.25 * attack_speed;
        }
        c = String::from("voidborn resilience");
        if enemy.items.effects.contains(&c) {
            // increase bonus armor and magic reduction by 30%
            self_sum.armor += (self_sum.armor - self.champion.stats.armor) * 1.3;
            self_sum.magic_resistance +=
                (self_sum.magic_resistance - self.champion.stats.magic_resistance) * 1.3;
        }

        // handle champion effects (abilities and passives)
        c = String::from("yasuo passive");
        if self.champion.effects.contains(&c) {
            let rcc = self_sum.crit_chance * 2.0;
            crit_chance *= if rcc > 1.0 {
                attack_damage += 0.4 * (rcc - 1.0) * 100.0;
                1.0
            } else {
                rcc
            };
            crit_damage_multiplier = (self_sum.crit_damage * 0.9 + 1.0) * crit_chance;
        }

        c = String::from("vayne w");
        if self.champion.effects.contains(&c) {
            // sso.enemy_max_hp.trued += 0.1 / 3.0;
            raw_on_hit.trued += 0.1 / 3.0 * enemy_sum.health;
        }

        // raw damage per hit
        let rdph = Damage {
            normal: crit_damage_multiplier * attack_damage + raw_on_hit.normal,
            magic: raw_on_hit.magic,
            trued: raw_on_hit.trued,
            ..Default::default()
        };

        // actual damage per hit
        let adph = rdph.reduce(&self_sum, &enemy_sum);

        // damage per second
        let dps = attack_speed * adph.sum();

        return (
            dps,
            dps * life_steal - damage_taken.reduce(&self_sum, &enemy_sum).sum(),
        );
    }

    pub fn evaluate(&self, enemy: &Self) -> f32 {
        let (self_dps, self_healing) = self.fight(&enemy);
        let (enemy_dps, enemy_healing) = enemy.fight(&self);
        // self survival time
        let sst = self.sum().health / (enemy_dps - self_healing);
        // enemy survival time
        let est = enemy.sum().health / (self_dps - enemy_healing);
        // sst / est
        // self_dps
        // self_healing
        self_dps + self_healing
    }
}
