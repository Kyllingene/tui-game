use std::collections::HashMap;

use crate::entity::{Entity, EntityKind};
use crate::map::{Direction, Tile, TileKind};
use crate::sector::Sector;
use crate::item::{Item, Buff};

macro_rules! sector {
    ( $sectors:expr, $id:expr => $neighbors:expr, $entities:expr $(,)? ) => {
        $sectors.insert($id, Sector::new(
            include_str!(concat!("../map/", $id, ".txt")),
            $id,
            $entities,
            $neighbors,
        ))
    }
}

pub fn sectors() -> HashMap<&'static str, Sector> {
    let mut item_id_counter = 0;
    let mut item_id = move || { item_id_counter += 1; item_id_counter };

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
                    id: 0,
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
                    id: 1,
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
    );

    sector!(sectors, "plains1" =>
        [Some("plains4"), Some("plains3"), Some("plains2"), Some("start")],
        vec![],
    );

    sector!(sectors, "plains2" =>
        [None, None, None, Some("plains1")],
        vec![],
    );

    sector!(sectors, "plains3" =>
        [Some("plains1"), None, None, None],
        vec![
            Entity::new(4, 11,
                EntityKind::Item(Item::basic("Sword", item_id(), Buff::Damage(2))),
                true,
            ),
        ],
    );

    sector!(sectors, "plains4" =>
        [None, Some("plains1"), None, None],
        vec![],
    );

    sectors
}
