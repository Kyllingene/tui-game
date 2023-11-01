pub mod constants {
    pub const HUNGER_INTERVAL: u32 = 8;
    pub const THIRST_INTERVAL: u32 = 6;

    pub const HUNGER_CAP: u32 = 10;
    pub const THIRST_CAP: u32 = 10;

    pub const UPGRADE_CHANCE: f32 = 0.4;
    pub const CHARACTER: char = 'G';
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pub x: u32,
    pub y: u32,

    pub hunger: u32,
    pub thirst: u32,

    pub health: u32,
    pub damage: u32,
}
