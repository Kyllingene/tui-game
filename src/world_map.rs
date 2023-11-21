use std::collections::HashMap;

use crate::difficulty::Difficulty;
use crate::entity::{Entity, EntityKind};
use crate::item::{Buff, Item};
use crate::map::{Direction, Tile, TileKind};
use crate::sector::Sector;

macro_rules! sector {
    ( $sectors:expr, $id:expr => $neighbors:expr, $entities:expr $(,)? ) => {
        $sectors.insert(
            $id,
            Sector::new(
                include_str!(concat!("../map/", $id, ".txt")),
                $id,
                $entities,
                $neighbors,
            ),
        )
    };

    ( $sectors:expr, $id:expr => $neighbors:expr, $entities:expr, $difficulty:expr $(,)? ) => {
        $sectors.insert(
            $id,
            Sector::new(
                include_str!(concat!("../map/", $id, ".txt")),
                $id,
                $entities,
                $neighbors,
            )
            .with_difficulty($difficulty),
        )
    };
}

pub fn sectors() -> HashMap<&'static str, Sector> {
    let mut entity_id_counter = 0;
    let mut entity_id = move || {
        entity_id_counter += 1;
        entity_id_counter
    };

    let mut sectors = HashMap::new();

    sector!(
        sectors, "start" =>
        [None, None, Some("plains1"), None],
        vec![
            Entity::new(
                7,
                7,
                EntityKind::Boss {
                    health: 15,
                    damage: 3,
                    damage_gain: 2,
                    id: entity_id(),
                    block: (
                        Direction::Up,
                        Tile {
                            kind: TileKind::Grass,
                        },
                    ),
                },
                true,
            ),
            Entity::new(
                16,
                12,
                EntityKind::Boss {
                    health: 30,
                    damage: 5,
                    damage_gain: 2,
                    id: entity_id(),
                    block: (
                        Direction::Right,
                        Tile {
                            kind: TileKind::Forest,
                        },
                    ),
                },
                true,
            ),
        ],
        Difficulty::easy(),
    );

    sector!(sectors, "plains1" =>
        [Some("plains4"), Some("plains3"), Some("plains2"), Some("start")],
        vec![],
    );

    sector!(sectors, "plains2" =>
        [None, None, Some("mountains1"), Some("plains1")],
        vec![],
    );

    sector!(sectors, "plains3" =>
        [Some("plains1"), Some("peninsula1"), None, None],
        vec![
            Entity::new(4, 11,
                EntityKind::Item(Item::basic("Sword", entity_id(), Buff::Damage(2))),
                true,
            ),
        ],
    );

    sector!(sectors, "plains4" =>
        [None, Some("plains1"), None, None],
        vec![
            Entity::new(20, 11,
                EntityKind::Item(Item::basic("Chestplate", entity_id(), Buff::MaxHealth(3))),
                true,
            ),
        ],
    );

    sectors.get_mut("plains4").unwrap().entrance(17, 11, "village1");

    sector!(sectors, "mountains1" =>
        [None, None, None, Some("plains2")],
        vec![
            Entity::new(3, 6,
                EntityKind::Item(Item::buffs("Vial of Fortitude", entity_id(), vec![
                        Buff::MaxHealth(3),
                        Buff::HungerCap(2),
                        Buff::ThirstCap(2),
                ])),
                true,
            ),
            Entity::new(20, 7,
                EntityKind::Npc {
                    dialogue: &[
                        "You'd better be careful,\nthese wilds are dangerous.",
                        "They say there's valuable\ntreasure past the river.",
                        "Well, good luck, traveler!"
                    ],
                    dialogue_idx: Some(0),
                    items: vec![],
                    id: entity_id(),
                },
                true,
            ),
        ],
        Difficulty {
            food_mul: 0.5,
            food_food_mul: 1.5,

            enemy_mul: 2.0,
            enemy_health_mul: 2.0,
            enemy_damage_mul: 2.0,
        },
    );

    sector!(sectors, "peninsula1" =>
        [Some("plains3"), None, None, None],
        vec![
            Entity::new(17, 14,
                EntityKind::Item(Item::full("Battleaxe", entity_id(), vec![
                    Buff::Damage(3),
                    Buff::MaxHealth(2),
                ], vec![
                    Buff::HungerCap(2),
                    Buff::ThirstCap(2),
                ])),
                true,
            ),
        ],
        Difficulty::hard()
    );

    sectors.insert("village1", Sector::town(
        include_str!("../map/village1.txt"),
        "village1",
        vec![
            Entity::new(3, 2,
                EntityKind::Npc {
                    dialogue: &[
                        "Welcome, traveler!\nStay as long as you like.",
                        "We're a quiet town, so don't\nexpect many attractions.",
                        "I've got to get back to work."
                    ],
                    dialogue_idx: Some(0),
                    items: vec![(Item::basic("Pouch", entity_id(), Buff::HungerCap(2)), 2)],
                    id: entity_id(),
                },
                true,
            ),
        ],
        "plains4",
        17, 11,
    ));

    sectors
}
