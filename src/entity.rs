use std::ops::Range;

use rand::{thread_rng, Rng};

use crate::input::TurnResult;
use crate::map::{Direction, Map, Tile, TileKind, HEIGHT, WIDTH};
use crate::player::Player;
use crate::world::World;

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
    pub persist: bool,
}

impl Entity {
    pub const fn new(x: u32, y: u32, kind: EntityKind, persist: bool) -> Self {
        Self {
            x,
            y,
            kind,
            alive: true,
            persist,
        }
    }

    pub fn spawn_random(world: &World) -> Option<Entity> {
        if world.entities.len() >= world.max_entities() as usize {
            return None;
        }

        let mut rng = thread_rng();
        let r = rng.gen::<f32>();
        if r <= SPAWN_CHANCE * world.spawn_chance_coeff() {
            let r = rng.gen();
            let kind = if FOOD_SPAWN_AREA.contains(&r) {
                EntityKind::Food {
                    food: rng.gen_range(2..8),
                }
            } else if ENEMY_SPAWN_AREA.contains(&r) {
                EntityKind::Enemy {
                    health: rng.gen_range(2..(world.player.damage / 4).max(3)),
                    damage: rng.gen_range(1..(world.player.health / 3).max(2)),
                }
            } else {
                return None;
            };

            let (x, y) = Self::pick_spawn_tile(&kind, world);

            Some(Entity {
                x,
                y,
                kind,
                alive: true,
                persist: false,
            })
        } else {
            None
        }
    }

    pub fn pick_spawn_tile(kind: &EntityKind, world: &World) -> (u32, u32) {
        let mut rng = thread_rng();
        let mut iterations = 0;
        'outer: loop {
            // TODO: fix this filthy hack
            for y in (0..HEIGHT as u32).rev() {
                for x in (0..WIDTH as u32).rev() {
                    if world.player.x == x && world.player.y == y {
                        continue;
                    }
                    let tile = world.map.get(x, y).unwrap();

                    let chance = match tile.kind {
                        TileKind::Water | TileKind::Mountain => continue,
                        TileKind::Grass => match kind {
                            EntityKind::Food { .. } => 0.75,
                            EntityKind::Enemy { .. } => 0.25,
                            EntityKind::Boss { .. } => {
                                unreachable!("Bosses cannot be spawned randomly")
                            }
                        },
                        TileKind::Forest => match kind {
                            EntityKind::Food { .. } => 0.60,
                            EntityKind::Enemy { .. } => 0.25,
                            EntityKind::Boss { .. } => {
                                unreachable!("Bosses cannot be spawned randomly")
                            }
                        },
                        TileKind::Hill => match kind {
                            EntityKind::Food { .. } => 0.10,
                            EntityKind::Enemy { .. } => 0.75,
                            EntityKind::Boss { .. } => {
                                unreachable!("Bosses cannot be spawned randomly")
                            }
                        },
                    } / (WIDTH * HEIGHT) as f32
                        + (iterations as f32 * 0.1);

                    let r: f32 = rng.gen();
                    if r <= chance {
                        break 'outer (x, y);
                    }
                }
            }

            iterations += 1;
        }
    }

    pub fn interact(&mut self, player: &mut Player, map: &mut Map) -> TurnResult {
        match &mut self.kind {
            EntityKind::Food { food } => {
                player.hunger = player.hunger.saturating_sub(*food);
                player.health = (player.health + 2).min(10);
                self.alive = false;

                TurnResult::Ate(*food)
            }

            EntityKind::Enemy { health, damage } => {
                if player.damage >= *health {
                    self.alive = false;
                    TurnResult::WonFight(false)
                } else if *damage >= player.health {
                    player.health = 0;
                    TurnResult::ViolentDeath
                } else {
                    *health -= player.damage;
                    player.health -= *damage;

                    TurnResult::Fight(*damage, *health)
                }
            }

            EntityKind::Boss {
                health,
                damage,
                damage_gain,
                block,
                id,
            } => {
                if player.damage >= *health {
                    self.alive = false;
                    player.damage += *damage_gain;

                    let (dir, tile) = block;
                    let diff = dir.diff();

                    let x = self.x.saturating_add_signed(diff.0);
                    let y = self.y.saturating_add_signed(diff.1);
                    map.set(x, y, *tile);
                    TurnResult::DefeatedBoss(*id)
                } else if *damage >= player.health {
                    player.health = 0;
                    TurnResult::ViolentDeath
                } else {
                    *health -= player.damage;
                    player.health -= *damage;

                    TurnResult::Fight(*damage, *health)
                }
            }
        }
    }

    pub fn ai(&mut self, world: &mut World) -> TurnResult {
        let mut rng = thread_rng();
        match &mut self.kind {
            EntityKind::Food { .. } => {
                let move_chance: f32 = rng.gen();
                if move_chance <= FOOD_MOVE_CHANCE {
                    self.random_move(false, world, &mut rng);
                }

                TurnResult::Ok
            }
            EntityKind::Enemy { health, damage } => {
                let health_coeff = (*health as f32).tanh() / 2.0 + 0.5;
                let damage_coeff = (*damage as f32).tanh() / 2.0 + 0.5;

                let move_chance =
                    rng.gen::<f32>() * health_coeff * damage_coeff * ENEMY_MOVE_CHANCE;
                if move_chance <= ENEMY_MOVE_CHANCE {
                    let (x, y) = self.random_move(true, world, &mut rng);
                    if world.player.x == x && world.player.y == y {
                        return self.interact(&mut world.player, &mut world.map);
                    }
                }

                TurnResult::Ok
            }
            EntityKind::Boss { .. } => TurnResult::Ok,
        }
    }

    pub fn random_move(
        &mut self,
        into_player: bool,
        world: &World,
        rng: &mut impl Rng,
    ) -> (u32, u32) {
        let mut iterations = 0;
        let mut x;
        let mut y;
        loop {
            if iterations >= 8 {
                return (self.x, self.y);
            }
            iterations += 1;

            let diff = rng.gen::<Direction>().diff();
            x = self.x.saturating_add_signed(diff.0);
            y = self.y.saturating_add_signed(diff.1);

            if !into_player && world.player.x == x && world.player.y == y {
                continue;
            };
            if world.entities.iter().any(|e| e.x == x && e.y == y) {
                continue;
            };

            let Some(tile) = world.map.get(x, y) else {
                continue;
            };

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

        match self.kind {
            EntityKind::Boss { .. } => cod::color::bg(9),
            _ => cod::color::de_bg(),
        }

        cod::color::fg(self.kind.color());
        cod::pixel(self.kind.sprite(), x, y + self.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Food {
        food: u32,
    },
    Enemy {
        health: u32,
        damage: u32,
    },
    Boss {
        health: u32,
        damage: u32,
        id: u32,
        damage_gain: u32,
        block: (Direction, Tile),
    },
}

impl EntityKind {
    pub fn color(&self) -> u8 {
        match self {
            Self::Food { .. } => 108,
            Self::Enemy { .. } => 210,
            Self::Boss { .. } => 136,
        }
    }

    pub fn sprite(&self) -> char {
        match self {
            Self::Food { .. } => '+',
            Self::Enemy { .. } => '!',
            Self::Boss { .. } => '#',
        }
    }
}
