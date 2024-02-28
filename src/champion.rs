use crate::stats::Stats;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Champion {
    pub name: String,
    pub melee: bool,
    pub stats: Stats,
    pub effects: Vec<String>,
}

pub fn init(vec: &mut Vec<Rc<Champion>>) {
    vec.push(Rc::new(Champion {
        name: String::from("udyr"),
        melee: true,
        stats: Stats {
            attack_damage: 130.0,
            base_attack_speed: 0.65,
            attack_speed: 0.51,
            crit_damage: 1.75,
            health: 2228.0,
            armor: 110.9,
            magic_resistance: 66.85,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(Champion {
        name: String::from("yasuo"),
        melee: true,
        stats: Stats {
            attack_damage: 111.0,
            base_attack_speed: 0.697,
            attack_speed: 0.595,
            crit_damage: 1.75,
            health: 2460.0,
            armor: 198.2,
            magic_resistance: 66.85,
            ..Default::default()
        },
        effects: vec![String::from("yasuo passive")],
        ..Default::default()
    }));

    vec.push(Rc::new(Champion {
        name: String::from("vayne"),
        melee: false,
        stats: Stats {
            attack_damage: 99.5,
            base_attack_speed: 0.658,
            attack_speed: 0.561,
            crit_damage: 1.75,
            health: 2301.0,
            armor: 101.2,
            magic_resistance: 52.1,
            ..Default::default()
        },
        effects: vec![String::from("vayne w")],
        ..Default::default()
    }));

    vec.push(Rc::new(Champion {
        name: String::from("Dummy"),
        stats: Stats {
            health: 2500.0,
            armor: 0.0,
            ..Default::default()
        },
        ..Default::default()
    }));
}
