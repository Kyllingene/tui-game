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
    pub fn full(name: &str, id: u32, buffs: Vec<Buff>, debuffs: Vec<Buff>) -> Self {
        Self {
            name: name.to_string(),
            buffs,
            debuffs,
            id,
        }
    }

    pub fn buffs(name: &str, id: u32, buffs: Vec<Buff>) -> Self {
        Self {
            name: name.to_string(),
            buffs,
            debuffs: Vec::new(),
            id,
        }
    }

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

    pub fn unapply(&self, player: &mut Player) {
        for buff in &self.buffs {
            buff.apply(player, true);
        }
        for debuff in &self.debuffs {
            debuff.apply(player, false);
        }
    }

    /// Returns the number of lines used by the item.
    pub fn draw(&self, x: u32, mut y: u32) -> u32 {
        let oy = y;

        cod::goto::pos(x, y);
        cod::color::fg(3);
        cod::color::de_bg();
        println!("{}", self.name);

        let x = x + 1;
        y += 1;
        for buff in &self.buffs {
            buff.draw(x, y, false);
            y += 1;
        }

        for debuff in &self.debuffs {
            debuff.draw(x, y, true);
            y += 1;
        }

        y - oy
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

    pub const fn diff(&self) -> u32 {
        match self {
            Self::MaxHealth(d)
                | Self::Damage(d)
                | Self::ThirstCap(d)
                | Self::HungerCap(d) => *d
        }
    }

    pub fn draw(&self, x: u32, y: u32, debuff: bool) {
        let (color, name) = match self {
            Self::MaxHealth(_) => (1, "health"),
            Self::Damage(_) => (7, "damage"),
            Self::ThirstCap(_) => (12, "water"),
            Self::HungerCap(_) => (223, "food"),
        };

        cod::goto::pos(x, y);
        cod::color::de_bg();
        cod::color::fg(color);
        print!("{name}: ");

        cod::color::fg(if debuff { 1 } else { 2 });
        print!("{}{}", if debuff { '-' } else { '+' }, self.diff());
    }
}
