pub mod constants {
    pub const HUNGER_INTERVAL: u32 = 8;
    pub const THIRST_INTERVAL: u32 = 4;

    pub const HUNGER_CAP: u32 = 10;
    pub const THIRST_CAP: u32 = 6;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pub x: u32,
    pub y: u32,

    pub hunger: u32,
    pub thirst: u32,
}
