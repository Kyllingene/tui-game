use std::collections::HashMap;

use crate::entity::{Entity, EntityKind};
use crate::item::{Buff, Item};
use crate::map::{Direction, Tile, TileKind};
use crate::sector::Sector;
use crate::difficulty::Difficulty;

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
            ).with_difficulty($difficulty),
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
                    id: entity_id(),
                },
                true,
            ),
        ],
        Difficulty::hard(),
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

    sectors
}
