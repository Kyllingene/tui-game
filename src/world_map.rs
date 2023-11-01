use std::collections::HashMap;

use map_macro::hash_map;

use crate::sector::Sector;
use crate::entity::{Entity, EntityKind};
use crate::map::{Tile, TileKind, Direction};

pub const START: &'static str = "start";

pub fn sectors() -> HashMap<&'static str, Sector> {
    hash_map! {
        START => Sector::new(r#"
                ~~~~~~~~~~~~~~~~~~~~~~~~
                ~~________~~~~~~~$nn$~~~
                ~____$$_$_____~~$$$nn$~~
                ~__$$$$$n$$$___~$$$An$~~
                ~~~~~$$$$$n$$_~~$$nn$_~~
                ~~~~~~~$$$$$$__~~nA$$_~~
                ~__~~~~A$$$$$__~~nn$$_~~
                __$_____~~~____~~$n$_~~~
                __$_____~~~~~~~~$nn$$_~~
                ~__nA$_________~$nA$$_~~
                ~____nnnnA_____~nn$$$$~~
                ~~____$$nn$__n__A$$$n$~~
                ~~~_____________$A$n$$~~
                ~~~___________~~~~~$$ ~~
                ~~~~~________~~~~~~~~~~~
                ~~~~~~~~~~~~~~~~~~~~~~~~
            "#,
            START,
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
            [None, None, Some("left"), None]
        ),

        "left" => Sector::new(
            r#"
                ~~~~~~~~~~~~~~~~~~~~~~~~
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~~~~~~~~~~~~~~~~~~~~~~A_
                ~~~~~~~~~~~~~~~~~~~~~~A_
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~                      ~
                ~~~~~~~~~~~~~~~~~~~~~~~~
            "#,
            "left",
            vec![],
            [None, None, None, Some(START)]
        ),
    }
}
