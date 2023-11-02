use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub buffs: Vec<Buff>,
    pub debuffs: Vec<Buff>,
    pub id: u32,
}

impl Item {
    pub fn basic(name: &str, id: u32, buff: Buff) -> Self {
        Self {
            name: name.to_string(),
            buffs: vec![buff],
            debuffs: Vec::new(),
            id,
        }
    }

    pub fn apply(&self, player: &mut Player) {
        for buff in &self.buffs {
            buff.apply(player, false);
        }
        for debuff in &self.debuffs {
            debuff.apply(player, true);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Buff {
    MaxHealth(u32),
    Damage(u32),
    ThirstCap(u32),
    HungerCap(u32),
}

impl Buff {
    pub fn apply(&self, player: &mut Player, debuff: bool) {
        let (stat, diff) = match self {
            Self::MaxHealth(i) => (&mut player.max_health, i),
            Self::Damage(i) => (&mut player.damage, i),
            Self::ThirstCap(i) => (&mut player.thirst_cap, i),
            Self::HungerCap(i) => (&mut player.hunger_cap, i),
        };

        let diff = if debuff {
            -(*diff as i32)
        } else {
            *diff as i32
        };
        *stat = stat.saturating_add_signed(diff);
    }
}
