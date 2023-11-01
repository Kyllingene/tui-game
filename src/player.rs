use serde::{Serialize, Deserialize};

use crate::item::Item;

pub mod constants {
    pub const HUNGER_INTERVAL: u32 = 8;
    pub const THIRST_INTERVAL: u32 = 6;

    pub const INITIAL_HUNGER_CAP: u32 = 10;
    pub const INITIAL_THIRST_CAP: u32 = 10;

    pub const UPGRADE_CHANCE: f32 = 0.4;
    pub const CHARACTER: char = 'G';
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub x: u32,
    pub y: u32,

    pub hunger: u32,
    pub thirst: u32,

    pub hunger_cap: u32,
    pub thirst_cap: u32,

    pub health: u32,
    pub max_health: u32,
    pub damage: u32,

    pub inventory: Vec<Item>,
}
