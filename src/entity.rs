use cod::BoxChars;
use rand::{thread_rng, Rng};

use crate::{good, bad};
use crate::difficulty::DifficultyMul;
use crate::input::TurnResult;
use crate::item::Item;
use crate::map::{Direction, Map, Tile, TileKind, HEIGHT, WIDTH};
use crate::player::Player;
use crate::world::World;

const FOOD_MOVE_CHANCE: f32 = 0.55;
const ENEMY_MOVE_CHANCE: f32 = 0.60;

const SPAWN_CHANCE: f32 = 0.5;
const FOOD_SPAWN_CHANCE: f32 = 0.6;
const ENEMY_SPAWN_CHANCE: f32 = 0.4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let difficulty = world.difficulty * world.map.sector().difficulty;

        if world.entities.len() >= world.max_entities() as usize {
            return None;
        }

        let mut rng = thread_rng();
        let r = rng.gen::<f32>();
        if r <= SPAWN_CHANCE * world.spawn_chance_coeff() {
            let fsc = FOOD_SPAWN_CHANCE * difficulty.food_mul;

            let r = rng.gen::<f32>() * difficulty.food_mul * difficulty.enemy_mul;
            let kind = if r <= fsc {
                EntityKind::Food {
                    food: rng.gen_range(2..8).apply(difficulty.food_food_mul),
                }
            } else if r - fsc <= ENEMY_SPAWN_CHANCE * difficulty.enemy_mul {
                EntityKind::Enemy {
                    health: rng
                        .gen_range(2..(world.player.damage / 4).max(3))
                        .apply(difficulty.enemy_health_mul),
                    damage: rng
                        .gen_range(1..(world.player.health / 3).max(2))
                        .apply(difficulty.enemy_damage_mul),
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
        // TODO: fix this filthy hack
        let mut iterations = 0;
        'outer: loop {
            for y in (0..HEIGHT as u32).rev() {
                for x in (0..WIDTH as u32).rev() {
                    if world.player.x == x && world.player.y == y {
                        continue;
                    }
                    let tile = world.map.get(x, y).unwrap();

                    let mut chance = kind.spawn_percentage(&tile) / (WIDTH * HEIGHT) as f32;
                    if chance == 0.0 {
                        continue;
                    }

                    chance += iterations as f32 / (WIDTH * HEIGHT / 2) as f32;

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
                player.health = (player.health + 2).min(player.max_health);
                self.alive = false;

                good!(Ate, *food)
            }

            EntityKind::Enemy { health, damage } => {
                if player.damage >= *health {
                    self.alive = false;
                    good!(WonFight, false)
                } else if *damage >= player.health {
                    player.health = 0;
                    bad!(ViolentDeath)
                } else {
                    *health -= player.damage;
                    player.health -= *damage;

                    good!(Fight, *damage, *health)
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

                    // wrapping to allow across-the-map setups
                    let x = self.x.wrapping_add_signed(diff.0);
                    let y = self.y.wrapping_add_signed(diff.1);
                    map.set(x, y, *tile);
                    good!(DefeatedBoss, *id)
                } else if *damage >= player.health {
                    player.health = 0;
                    bad!(ViolentDeath)
                } else {
                    *health -= player.damage;
                    player.health -= *damage;

                    good!(Fight, *damage, *health)
                }
            }

            EntityKind::Item(item) => {
                self.alive = false;
                item.apply(player);
                let name = item.name.clone();
                player.inventory.push(std::mem::take(item));

                good!(PickedUpItem, name)
            }

            EntityKind::Npc {
                dialogue,
                dialogue_idx,
                items,
                ..
            } => {
                if let Some(idx) = dialogue_idx {
                    let speech = &dialogue[*idx];

                    if !items.is_empty() {
                        let (item, item_idx) = &items[0];

                        // prevent farming items via saves
                        if item_idx == idx && !player.inventory.iter().any(|i| i.name == item.name) {
                            let (item, _) = items.remove(0);
                            item.apply(player);
                            player.inventory.push(item);
                        }
                    }

                    let idx = *idx + 1;
                    if idx < dialogue.len() {
                        *dialogue_idx = Some(idx);
                    } else {
                        *dialogue_idx = None;
                    }

                    let lines = speech.lines();
                    let (width, height) = lines.fold((0, 0), |(w, h), l| (w.max(l.len()), h + 1));

                    cod::color::de();
                    cod::clear::rect(0, 0, width as u32 + 1, height + 1).unwrap();
                    cod::rect_lines(
                        BoxChars {
                            horizontal: '-',
                            vertical: '|',
                            corner: '+',
                        },
                        0,
                        0,
                        width as u32 + 1,
                        height + 1,
                    ).unwrap();

                    cod::blit(speech, 1, 1);
                    cod::flush();
                    cod::read::key();
                }

                good!(Menued)
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
            }
            EntityKind::Enemy { health, damage } => {
                let health_coeff = (*health as f32).tanh() / 2.0 + 0.5;
                let damage_coeff = (*damage as f32).tanh() / 2.0 + 0.5;

                let move_chance =
                    rng.gen::<f32>() * health_coeff * damage_coeff * ENEMY_MOVE_CHANCE;
                if move_chance <= ENEMY_MOVE_CHANCE {
                    let (x, y) = self.random_move(true, world, &mut rng);
                    if world.player.x == x && world.player.y == y {
                        let mut player = world.player.clone();
                        let mut map = world.map.clone();
                        let res = self.interact(&mut player, &mut map);
                        world.player = player;
                        world.map = map;
                        return res;
                    }
                }
            }
            EntityKind::Boss { .. } | EntityKind::Item(_) | EntityKind::Npc { .. } => {}
        }

        good!()
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
                TileKind::Water | TileKind::Mountain | TileKind::Village | TileKind::Building => continue,
                _ => break,
            }
        }

        self.x = x;
        self.y = y;

        (x, y)
    }

    pub fn id(&self) -> Option<u32> {
        Some(match &self.kind {
            EntityKind::Boss { id, .. }
            | EntityKind::Npc { id, .. } => *id,
            EntityKind::Item(item) => item.id,
            _ => None?,
        })
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Item(Item),
    Npc {
        dialogue: &'static [&'static str],
        dialogue_idx: Option<usize>,
        items: Vec<(Item, usize)>,
        id: u32,
    },
}

impl EntityKind {
    pub fn spawn_percentage(&self, tile: &Tile) -> f32 {
        match tile.kind {
            TileKind::Water | TileKind::Mountain | TileKind::Road | TileKind::Village | TileKind::Building => 0.0,
            TileKind::Grass => match self {
                EntityKind::Food { .. } => 0.75,
                EntityKind::Enemy { .. } => 0.15,
                EntityKind::Boss { .. } | EntityKind::Item(_) | EntityKind::Npc { .. } => 0.0,
            },
            TileKind::Forest => match self {
                EntityKind::Food { .. } => 0.60,
                EntityKind::Enemy { .. } => 0.25,
                EntityKind::Boss { .. } | EntityKind::Item(_) | EntityKind::Npc { .. } => 0.0,
            },
            TileKind::Hill => match self {
                EntityKind::Food { .. } => 0.10,
                EntityKind::Enemy { .. } => 0.75,
                EntityKind::Boss { .. } | EntityKind::Item(_) | EntityKind::Npc { .. } => 0.0,
            },
        }
    }

    pub fn color(&self) -> u8 {
        match self {
            Self::Food { .. } => 108,
            Self::Enemy { .. } => 210,
            Self::Boss { .. } => 136,
            Self::Item(_) => 56,
            Self::Npc { .. } => 79,
        }
    }

    pub fn sprite(&self) -> char {
        match self {
            Self::Food { .. } => '+',
            Self::Enemy { .. } => '!',
            Self::Boss { .. } => '#',
            Self::Item(_) => '?',
            Self::Npc { .. } => '&',
        }
    }
}
