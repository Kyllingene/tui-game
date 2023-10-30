use crate::map::{Map, TileKind};
use crate::player::Player;
use crate::input::TurnResult;
use crate::Direction;
use rand::{Rng, thread_rng};

const FOOD_MOVE_CHANCE: f32 = 0.65;
const ENEMY_MOVE_CHANCE: f32 = 0.60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub kind: EntityKind,
    pub alive: bool,
}

impl Entity {
    pub fn interact(&mut self, player: &mut Player) -> TurnResult {
        match &mut self.kind {
             EntityKind::Food { food } => {
                player.hunger = player.hunger.saturating_sub(*food);
                self.alive = false;

                TurnResult::Ate(*food)
             }

             EntityKind::Enemy { health, damage } => {
                if player.damage >= *health {
                    self.alive = false;
                    TurnResult::WonFight
                } else if *damage >= player.health {
                    TurnResult::ViolentDeath
                } else {
                    *health -= player.damage;
                    player.health -= *damage;

                    TurnResult::Fight(*damage, *health)
                }
             }
        }
    }

    pub fn ai(&mut self, map: &mut Map) -> TurnResult {
        let mut rng = thread_rng();
        match &mut self.kind {
            EntityKind::Food { .. } => {
                let move_chance: f32 = rng.gen();
                if move_chance <= FOOD_MOVE_CHANCE {
                    self.random_move(false, &map, &mut rng);
                }

                TurnResult::Ok
            }
            EntityKind::Enemy { health, damage } => {
                let health_coeff = (*health as f32).tanh() / 2.0 + 0.5;
                let damage_coeff = (*damage as f32).tanh() / 2.0 + 0.5;

                let move_chance = rng.gen::<f32>() * health_coeff * damage_coeff * ENEMY_MOVE_CHANCE;
                if move_chance <= ENEMY_MOVE_CHANCE {
                    let (x, y) = self.random_move(true, &map, &mut rng);
                    if map.player.x == x && map.player.y == y {
                        return self.interact(&mut map.player);
                    }
                }

                TurnResult::Ok
            }
        }
    }

    pub fn random_move(&mut self, into_player: bool, map: &Map, rng: &mut impl Rng) -> (u32, u32) {
        let mut iterations = 0;
        let mut x;
        let mut y;
        loop {
            if iterations >= 8 { return (self.x, self.y); }
            iterations += 1;

            let diff = rng.gen::<Direction>().diff();
            x = self.x.saturating_add_signed(diff.0);
            y = self.y.saturating_add_signed(diff.1);

            if !into_player && map.player.x == x && map.player.y == y { continue; };
            if map.entities.iter().any(|e| e.x == x && e.y == y) { continue; };

            let Some(tile) = map.get_tile(x, y) else { continue; };

            match tile.kind {
                TileKind::Water | TileKind::Mountain => continue,
                _ => break,
            }
        }

        self.x = x;
        self.y = y;

        (x, y)
    }

    pub fn draw(&self, x: u32, y: u32) {
        let x = self.x * 2 + x + 1;

        cod::color::de_bg();
        cod::color::fg(self.kind.color());
        cod::pixel(self.kind.sprite(), x, y + self.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Food { food: u32 },
    Enemy { health: u32, damage: u32 },
}

impl EntityKind {
    pub fn color(&self) -> u8 {
        match self {
            Self::Food { .. } => 108,
            Self::Enemy { .. } => 210,
        }
    }

    pub fn sprite(&self) -> char {
        match self {
            Self::Food { .. } => '+',
            Self::Enemy { .. } => '!',
        }
    }
}

