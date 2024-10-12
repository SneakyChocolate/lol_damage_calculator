use std::iter::Sum;
use std::ops::{Add, Mul, MulAssign};

#[derive(Default, Debug, Copy, Clone)]
pub struct Damage {
    pub normal: f32,
    pub magic: f32,
    pub trued: f32,
}

impl Damage {
    pub fn magic(damage: f32) -> Self {
        Damage { magic: damage, ..Default::default() }
    }
    pub fn normal(damage: f32) -> Self {
        Damage { normal: damage, ..Default::default() }
    }
    pub fn trued(damage: f32) -> Self {
        Damage { trued: damage, ..Default::default() }
    }
    pub fn reduction_multiplier(attacker_sum: &Stats, receiver_sum: &Stats) -> Self {
        let self_sum = attacker_sum;
        let enemy_sum = receiver_sum;
        Self {
            normal: 100.0
                / (100.0 + enemy_sum.armor * (1.0 - self_sum.armor_penetration.percentage)
                    - self_sum.armor_penetration.flat),
            magic: 100.0
                / (100.0
                    + enemy_sum.magic_resistance * (1.0 - self_sum.magic_penetration.percentage)
                    - self_sum.magic_penetration.flat),
            trued: 1.0,
            ..Default::default()
        }
    }

    pub fn reduce(&self, attacker: &Stats, receiver: &Stats) -> Self {
        let redu = Self::reduction_multiplier(attacker, receiver);
        *self * redu
    }

    pub fn powf(&self, n: f32) -> Self {
        Self {
            normal: self.normal.powf(n),
            magic: self.magic.powf(n),
            trued: self.trued.powf(n),
        }
    }

    pub fn sum(&self) -> f32 {
        self.normal + self.magic + self.trued
    }
}

impl MulAssign<f32> for Damage {
    fn mul_assign(&mut self, rhs: f32) {
        self.normal *= rhs;
        self.magic *= rhs;
        self.trued *= rhs;
    }
}

impl Mul for Damage {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            normal: self.normal * rhs.normal,
            magic: self.magic * rhs.magic,
            trued: self.trued * rhs.trued,
            ..Default::default()
        }
    }
}

impl Mul<f32> for Damage {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            normal: self.normal * rhs,
            magic: self.magic * rhs,
            trued: self.trued * rhs,
            ..Default::default()
        }
    }
}

impl Add<Damage> for Damage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Damage> for Damage {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            normal: self.normal + rhs.normal,
            magic: self.magic + rhs.magic,
            trued: self.trued + rhs.trued,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct OnHitDamage {
    pub flat: Damage,
    pub bonus_ad: Damage,
    pub enemy_hp: Damage,
    pub max_hp: Damage,
    pub base_ad: Damage,
    pub bonus_ap: Damage,
    pub enemy_max_hp: Damage,
}

impl OnHitDamage {
    pub fn flat(damage: Damage) -> Self {
        Self { flat: damage, ..Default::default() }
    }
    pub fn bonus_ad(damage: Damage) -> Self {
        Self { bonus_ad: damage, ..Default::default() }
    }
    pub fn enemy_hp(damage: Damage) -> Self {
        Self { enemy_hp: damage, ..Default::default() }
    }
    pub fn max_hp(damage: Damage) -> Self {
        Self { max_hp: damage, ..Default::default() }
    }
    pub fn base_ad(damage: Damage) -> Self {
        Self { base_ad: damage, ..Default::default() }
    }
    pub fn bonus_ap(damage: Damage) -> Self {
        Self { bonus_ap: damage, ..Default::default() }
    }
    pub fn enemy_max_hp(damage: Damage) -> Self {
        Self { enemy_max_hp: damage, ..Default::default() }
    }
}

impl Add<OnHitDamage> for OnHitDamage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&OnHitDamage> for OnHitDamage {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            flat: self.flat + rhs.flat,
            bonus_ad: self.bonus_ad + rhs.bonus_ad,
            enemy_hp: self.enemy_hp + rhs.enemy_hp,
            max_hp: self.max_hp + rhs.max_hp,
            base_ad: self.base_ad + rhs.base_ad,
            bonus_ap: self.bonus_ap + rhs.bonus_ap,
            enemy_max_hp: self.enemy_max_hp + rhs.enemy_max_hp,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ArmorPenetration {
    pub percentage: f32,
    pub flat: f32,
}

impl ArmorPenetration {
    pub fn percentage(penetration: f32) -> Self {
        Self {percentage: penetration, ..Default::default()}
    }
    pub fn flat(penetration: f32) -> Self {
        Self {flat: penetration, ..Default::default()}
    }
}

impl Add<ArmorPenetration> for ArmorPenetration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&ArmorPenetration> for ArmorPenetration {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            percentage: self.percentage + rhs.percentage,
            flat: self.flat + rhs.flat,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MagicPenetration {
    pub percentage: f32,
    pub flat: f32,
}

impl Add<MagicPenetration> for MagicPenetration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&MagicPenetration> for MagicPenetration {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            percentage: self.percentage + rhs.percentage,
            flat: self.flat + rhs.flat,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub attack_damage: f32,
    pub ability_power: f32,
    pub base_attack_speed: f32,
    pub attack_speed: f32,
    pub attack_speed_ratio: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub on_hit_damage: OnHitDamage,
    pub life_steal: f32,
    pub armor_penetration: ArmorPenetration,
    pub magic_penetration: MagicPenetration,
    pub health: f32,
    pub armor: f32,
    pub magic_resistance: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            attack_damage: Default::default(),
            ability_power: Default::default(),
            base_attack_speed: Default::default(),
            attack_speed: Default::default(),
            attack_speed_ratio: 1.0,
            crit_chance: Default::default(),
            crit_damage: Default::default(),
            on_hit_damage: Default::default(),
            life_steal: Default::default(),
            armor_penetration: Default::default(),
            magic_penetration: Default::default(),
            health: Default::default(),
            armor: Default::default(),
            magic_resistance: Default::default(),
        }
    }
}

impl Add<Stats> for Stats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Stats> for Stats {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            attack_damage: self.attack_damage + rhs.attack_damage,
            attack_speed: self.attack_speed + rhs.attack_speed,
            crit_chance: self.crit_chance + rhs.crit_chance,
            crit_damage: self.crit_damage + rhs.crit_damage,
            on_hit_damage: self.on_hit_damage + &rhs.on_hit_damage,
            life_steal: self.life_steal + rhs.life_steal,
            armor_penetration: self.armor_penetration + &rhs.armor_penetration,
            health: self.health + &rhs.health,
            armor: self.armor + &rhs.armor,
            ability_power: self.ability_power + rhs.ability_power,
            base_attack_speed: self.base_attack_speed + rhs.base_attack_speed,
            magic_penetration: self.magic_penetration + rhs.magic_penetration,
            magic_resistance: self.magic_resistance + rhs.magic_resistance,
            attack_speed_ratio: self.attack_speed_ratio + rhs.attack_speed_ratio,
        }
    }
}

impl<'a> Sum<&'a Stats> for Stats {
    fn sum<I: Iterator<Item = &'a Stats>>(iter: I) -> Stats {
        iter.fold(Default::default(), |a, b| a + b)
    }
}
