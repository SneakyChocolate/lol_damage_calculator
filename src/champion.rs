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
            crit_damage: 0.75,
            health: 2228.0,
            armor: 110.9,
            magic_resistance: 66.85,
            attack_speed_ratio: 0.65,
            ..Default::default()
        },
        ..Default::default()
    }));
    vec.push(Rc::new(Champion {
        name: String::from("kayle"),
        melee: false,
        stats: Stats {
            attack_damage: 92.5,
            base_attack_speed: 0.625,
            attack_speed_ratio: 0.667,
            crit_damage: 0.75,
            health: 2234.0,
            armor: 97.4,
            magic_resistance: 44.1,
            ..Default::default()
        },
        effects: vec!["kayle e".into(), "kayle passive".into()],
        ..Default::default()
    }));

    vec.push(Rc::new(Champion {
        name: String::from("yasuo"),
        melee: true,
        stats: Stats {
            attack_damage: 111.0,
            base_attack_speed: 0.697,
            crit_damage: 0.75,
            attack_speed_ratio: 0.670,
            health: 2460.0,
            armor: 110.2,
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
            crit_damage: 0.75,
            health: 2301.0,
            armor: 101.2,
            magic_resistance: 52.1,
            attack_speed_ratio: 1.0,
            ..Default::default()
        },
        effects: vec![String::from("vayne w")],
        ..Default::default()
    }));
    vec.push(Rc::new(Champion {
        name: String::from("sona"),
        melee: false,
        stats: Stats {
            attack_damage: 100.0,
            base_attack_speed: 0.644,
            attack_speed: 0.391,
            crit_damage: 0.75,
            health: 2097.0,
            armor: 97.4,
            magic_resistance: 52.1,
            attack_speed_ratio: 0.644,
            ..Default::default()
        },
        ..Default::default()
    }));
    vec.push(Rc::new(Champion {
        name: String::from("senna"),
        melee: false,
        stats: Stats {
            attack_damage: 50.0,
            base_attack_speed: 0.625,
            attack_speed: 0.68,
            attack_speed_ratio: 0.4,
            crit_damage: 0.75,
            health: 2043.0,
            armor: 107.9,
            magic_resistance: 52.1,
            ..Default::default()
        },
        ..Default::default()
    }));

    vec.push(Rc::new(Champion {
        name: String::from("dummy"),
        stats: Stats {
            health: 2500.0,
            armor: 500.0,
            magic_resistance: 500.0,
            attack_speed_ratio: 1.0,
            ..Default::default()
        },
        ..Default::default()
    }));
}
