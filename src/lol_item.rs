use crate::stats::*;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
pub struct LolItem {
    pub name: String,
    pub stats: Stats,
    pub categories: usize,
    pub effects: Vec<String>,
}

impl fmt::Debug for LolItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LolItem")
            .field("name", &self.name)
            .field("effects", &self.effects)
            .finish()
    }
}

impl Default for LolItem {
    fn default() -> Self {
        LolItem {
            name: String::from(""),
            stats: Stats::default(),
            categories: 0,
            effects: Vec::new(),
        }
    }
}

impl Add<&LolItem> for LolItem {
    type Output = LolItem;
    fn add(self, other: &Self) -> Self::Output {
        let mut e = self.effects.clone();
        e.append(&mut other.effects.clone());
        LolItem {
            name: String::from(""),
            stats: self.stats + &other.stats,
            categories: 0,
            effects: e,
            ..Default::default()
        }
    }
}

impl<'a> Sum<&'a LolItem> for LolItem {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(LolItem::default(), |a, b| a + b)
    }
}

impl LolItem {
    pub fn build(items: Vec<&str>, list: &Vec<Rc<LolItem>>) -> LolItem {
        let mut build: LolItem = LolItem::default();
        for name in items.iter() {
            if let Some(item) = list.iter().find(|x| x.name == String::from(*name)) {
                build = build + item;
            } else {
                panic!("item {name} not found!");
            }
        }
        build
    }

    pub fn make(list: &Vec<&Rc<LolItem>>) -> LolItem {
        let mut base = LolItem::default();
        for item in list {
            base = base + item;
        }
        base
    }
}

pub fn init(vec: &mut Vec<Rc<LolItem>>) {
    vec.push(Rc::new(LolItem {
        name: String::from("immortal shieldbow"),
        stats: Stats {
            attack_damage: 55.0,
            crit_chance: 0.25,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("infinity edge"),
        stats: Stats {
            attack_damage: 70.0,
            crit_chance: 0.25,
            crit_damage: 0.40,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("kraken slayer"),
        stats: Stats {
            attack_damage: 45.0,
            attack_speed: 0.4,
            on_hit_damage: OnHitDamage::flat(Damage::normal(240.0 / 3.0)),
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("bloodthirster"),
        stats: Stats {
            attack_damage: 80.0,
            life_steal: 0.15,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("blade of the ruined king"),
        stats: Stats {
            attack_damage: 40.0,
            attack_speed: 0.25,
            life_steal: 0.1,
            on_hit_damage: OnHitDamage::enemy_hp(Damage::normal(0.6)),
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("mortal reminder"),
        stats: Stats {
            attack_damage: 35.0,
            crit_chance: 0.25,
            armor_penetration: ArmorPenetration::percentage(0.35),
            ..Default::default()
        },
        categories: 1,
        effects: vec![String::from("grievous wounds")],
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("ravenous hydra"),
        stats: Stats {
            attack_damage: 65.0,
            life_steal: 0.12,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("phantom dancer"),
        stats: Stats {
            attack_speed: 0.6,
            crit_chance: 0.25,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("wits end"),
        stats: Stats {
            attack_speed: 0.55,
            on_hit_damage: OnHitDamage::flat(Damage::magic(80.0)),
            magic_resistance: 50.0,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("berserkers"),
        stats: Stats {
            attack_speed: 0.45,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("titanic hydra"),
        stats: Stats {
            attack_damage: 50.0,
            health: 550.0,
            on_hit_damage: OnHitDamage::max_hp(Damage::normal(0.015)),
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("static shiv"),
        stats: Stats {
            attack_damage: 50.0,
            attack_speed: 0.40,
            on_hit_damage: OnHitDamage::flat(Damage::magic(90.0 / 12.0)),
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("terminus"),
        stats: Stats {
            attack_damage: 30.0,
            attack_speed: 0.35,
            on_hit_damage: OnHitDamage::flat(Damage::magic(30.0)),
            armor: 24.0,
            magic_resistance: 24.0,
            armor_penetration: ArmorPenetration::percentage(0.3),
            magic_penetration: MagicPenetration::percentage(0.3),
            ..Default::default()
        },
        categories: 1,
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("rageblade"),
        stats: Stats {
            attack_damage: 30.0,
            ability_power: 30.0,
            attack_speed: 0.25 + 0.08 * 4.0,
            on_hit_damage: OnHitDamage::flat(Damage::magic(30.0)),
            ..Default::default()
        },
        effects: vec![String::from("seething strike")],
        ..Default::default()
    }));

    vec.push(Rc::new(LolItem {
        name: String::from("nashors tooth"),
        stats: Stats {
            ability_power: 80.0,
            attack_speed: 0.5,
            on_hit_damage: OnHitDamage {
                flat: Damage::magic(15.0),
                bonus_ap: Damage::magic(0.15),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("thornmail"),
    //     stats: Stats {
    //         health: 350.0,
    //         armor: 70.0,
    //         ..Default::default()
    //     },
    //     effects: vec![String::from("thorns"), String::from("grievous wounds")],
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("jak sho"),
    //     stats: Stats {
    //         health: 300.0,
    //         armor: 50.0,
    //         magic_resistance: 50.0,
    //         ..Default::default()
    //     },
    //     effects: vec![String::from("voidborn resilience")],
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("mercurial scimitar"),
    //     stats: Stats {
    //         attack_damage: 40.0,
    //         magic_resistance: 50.0,
    //         life_steal: 0.1,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("maw of malmortius"),
    //     stats: Stats {
    //         attack_damage: 70.0,
    //         magic_resistance: 40.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("steraks gage"),
    //     stats: Stats {
    //         health: 400.0,
    //         ..Default::default()
    //     },
    //     effects: vec![String::from("the claws that catch")],
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("youmuus ghostblade"),
    //     stats: Stats {
    //         attack_damage: 60.0,
    //         armor_penetration: ArmorPenetration {
    //             flat: 18.0,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("voltaic cyclosword"),
    //     stats: Stats {
    //         attack_damage: 55.0,
    //         armor_penetration: ArmorPenetration {
    //             flat: 18.0,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("eclipse"),
    //     stats: Stats {
    //         attack_damage: 70.0,
    //         on_hit_damage: OnHitDamage {
    //             enemy_max_hp: Damage {
    //                 normal: 0.06 / 10.0,
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));

    vec.push(Rc::new(LolItem {
        name: String::from("the collector"),
        stats: Stats {
            attack_damage: 50.0,
            crit_chance: 0.25,
            armor_penetration: ArmorPenetration::flat(10.0),
            ..Default::default()
        },
        ..Default::default()
    }));

    // vec.push(Rc::new(LolItem {
    //     name: String::from("deaths dance"),
    //     stats: Stats {
    //         attack_damage: 60.0,
    //         armor: 40.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }));
    
    vec.push(Rc::new(LolItem {
        name: String::from("yun tal wildarrows"),
        stats: Stats {
            attack_damage: 50.0,
            crit_chance: 0.25,
            attack_speed: 0.20,
            ..Default::default()
        },
        ..Default::default()
    }));
    
    vec.push(Rc::new(LolItem {
        name: String::from("dominiks"),
        categories: 1,
        stats: Stats {
            attack_damage: 35.0,
            crit_chance: 0.25,
            armor_penetration: ArmorPenetration::percentage(0.35),
            ..Default::default()
        },
        ..Default::default()
    }));
}
