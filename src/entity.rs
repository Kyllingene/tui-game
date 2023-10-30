use std::ops::Range;

use rand::{Rng, thread_rng};

use crate::map::{Map, TileKind, WIDTH, HEIGHT};
use crate::player::Player;
use crate::input::TurnResult;
use crate::Direction;

const FOOD_MOVE_CHANCE: f32 = 0.65;
const ENEMY_MOVE_CHANCE: f32 = 0.60;

const SPAWN_CHANCE: f32 = 0.5;
const FOOD_SPAWN_AREA: Range<f32> = 0.3..1.0;
const ENEMY_SPAWN_AREA: Range<f32> = 0.0..0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub kind: EntityKind,
    pub alive: bool,
}

impl Entity {
    pub fn spawn_random(map: &Map) -> Option<Entity> {
        if map.entities.len() >= map.max_entities() as usize { return None; }

        let mut rng = thread_rng();
        let r = rng.gen::<f32>();
        if r <= SPAWN_CHANCE * map.spawn_chance_coeff() {
            let r = rng.gen();
            let kind = if FOOD_SPAWN_AREA.contains(&r) {
                EntityKind::Food { food: rng.gen_range(2..8) }
            } else if ENEMY_SPAWN_AREA.contains(&r) {
                EntityKind::Enemy { health: rng.gen_range(2..(map.player.damage / 4).max(3)), damage: rng.gen_range(1..(map.player.health / 3).max(2)) }
            } else {
                return None;
            };

            let (x, y) = Self::pick_spawn_tile(&kind, map);

            Some(Entity { x, y, kind, alive: true })
        } else {
            None
        }
    }

    pub fn pick_spawn_tile(kind: &EntityKind, map: &Map) -> (u32, u32) {
        let mut rng = thread_rng();
        let mut iterations = 0;
        'outer: loop {
            for y in 0..HEIGHT as u32 {
                for x in 0..WIDTH as u32 {
                    if map.player.x == x && map.player.y == y { continue; }
                    let tile = map.get_tile(x, y).unwrap();

                    let chance = match tile.kind {
                        TileKind::Water | TileKind::Mountain => continue,
                        TileKind::Grass => match kind {
                            EntityKind::Food { .. } => 0.75,
                            EntityKind::Enemy { .. } => 0.25,
                        },
                        TileKind::Forest => match kind {
                            EntityKind::Food { .. } => 0.60,
                            EntityKind::Enemy { .. } => 0.40,
                        },
                        TileKind::Hill => match kind {
                            EntityKind::Food { .. } => 0.25,
                            EntityKind::Enemy { .. } => 0.75,
                        },
                    } / ( WIDTH * HEIGHT ) as f32 + ( iterations as f32 * 0.1 );

                    let r: f32 = rng.gen();
                    if r <= chance {
                        break 'outer (x, y);
                    }
                }
            }

            iterations += 1;
        }
    }

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
                    TurnResult::WonFight(false)
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
                    self.random_move(false, map, &mut rng);
                }

                TurnResult::Ok
            }
            EntityKind::Enemy { health, damage } => {
                let health_coeff = (*health as f32).tanh() / 2.0 + 0.5;
                let damage_coeff = (*damage as f32).tanh() / 2.0 + 0.5;

                let move_chance = rng.gen::<f32>() * health_coeff * damage_coeff * ENEMY_MOVE_CHANCE;
                if move_chance <= ENEMY_MOVE_CHANCE {
                    let (x, y) = self.random_move(true, map, &mut rng);
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

