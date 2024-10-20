mod champion;
mod combine;
mod entity;
mod lol_item;
mod passives;
mod stats;

use std::rc::Rc;

use crate::champion::Champion;
use crate::entity::Entity;
use lol_item::LolItem;
use stats::*;

fn main() {
    let mut lol_items: Vec<Rc<LolItem>> = Vec::new();
    let mut champions: Vec<Rc<Champion>> = Vec::new();

    lol_item::init(&mut lol_items);
    champion::init(&mut champions);

    let build1 = Rc::new(LolItem::build(
        vec![
            "infinity edge",
            "bloodthirster",
            "phantom dancer",
            "the collector",
            "yun tal wildarrows",
            "dominiks",
        ],
        &lol_items,
    ));

    let cq_heal = Rc::new(Stats {
        attack_damage: 29.0,
        attack_speed: 0.3,
        life_steal: 0.0525 + 0.08,
        ..Default::default()
    });

    let combinations = combine::select(&lol_items, 6);

    let enemy = Entity {
        name: String::from(""),
        champion: match champions.iter().find(|&a| a.name == String::from("yasuo")) {
            Some(c) => Rc::clone(&c),
            None => panic!("champion not found"),
        },
        items: Rc::clone(&build1),
        rune: Rc::clone(&cq_heal),
    };

    let mut best_value_option: Option<f32> = None;
    let mut best_entity: Option<Rc<Entity>> = None;
    let mut best_combination: Option<Vec<&Rc<LolItem>>> = None;
    for c in combinations {
        // check if combination is valid (no duplicated categories)
        let count1 = c.iter().filter(|e| e.categories == 1).count();
        if count1 > 1 {
            continue;
        }

        // make item build
        let buildx = Rc::new(LolItem::make(&c));

        // update entity for evaluation
        let my_entity = Rc::new(Entity {
            name: String::from(""),
            champion: match champions.iter().find(|&a| a.name == String::from("sona")) {
                Some(c) => Rc::clone(&c),
                None => panic!("champion not found"),
            },
            items: Rc::clone(&buildx),
            rune: Rc::clone(&cq_heal),
        });

        // my_champ.items = &buildx;

        // check if evaluation is better
        let new_value = my_entity.evaluate(&enemy);
        match best_value_option {
            None => {
                best_value_option = Some(new_value);
            }
            Some(best_value) => {
                if new_value > best_value {
                    best_value_option = Some(new_value);
                    best_entity = Some(Rc::clone(&my_entity));
                    best_combination = Some(c.clone());
                    // println!("new best value is {value} \n({:#?})\n\n", &c);
                }
            }
        }
    }

    println!("lt_heal: {:#?}", enemy.evaluate(&enemy));
    println!(
        "best combination: {:#?}\nwith the value of: {:?}",
        &best_combination, &best_value_option
    );
}
