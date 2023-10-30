use std::fmt::Display;

use crate::player::{constants::*, Player};
use crate::input::{self, TurnResult};
use crate::entity::{Entity, EntityKind};
use crate::Direction;

use rand::{Rng, thread_rng};

pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 16;

#[derive(Debug, Default, Clone)]
pub struct Map {
    tiles: [[Tile; WIDTH]; HEIGHT],

    pub player: Player,
    pub turn: u32,

    pub entities: Vec<Entity>,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_entities(&self) -> u32 {
        4 + self.turn / 3
    }

    pub fn spawn_chance_coeff(&self) -> f32 {
        1.0 + self.turn as f32 / 4.0
    }

    pub fn draw_message(&self, msg: impl Display) {
        cod::goto::bot();
        cod::color::de();
        cod::color::fg(1);
        cod::goto::up(1);
        cod::clear::line();
        println!("{msg}");
    }

    pub fn draw_result(&self, res: TurnResult) {
        crate::draw_key(self);
        match res {
            TurnResult::NoKey => self.draw_message("Please press a key"),
            TurnResult::InvalidMove(_) => self.draw_message("You can't move there"),
            TurnResult::WaterMove => self.draw_message("You drank your fill"),
            TurnResult::InvalidKey(_) => self.draw_message("That's not a valid key"),
            TurnResult::Fight(dmg, hp) => self.draw_message(format!("You dealt {}, they dealt {dmg} and are at {hp}", self.player.damage)),
            TurnResult::WonFight(upgrade) => if upgrade { self.draw_message("You won and got an upgrade!") } else { self.draw_message("You killed the enemy!") },
            TurnResult::Ate(food) => self.draw_message(format!("You regained {food} food")),
            _ => {}
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<Tile> {
        self.tiles.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }

    pub fn update(&mut self) -> TurnResult {
        let mut res = input::handle(self)?;
        while res != TurnResult::Ok {
            self.draw(0, 0);
            crate::draw_key(self);
            self.draw_result(res);
            res = input::handle(self)?;
        }

        self.turn += 1;

        if self.turn % HUNGER_INTERVAL == 0 {
            self.player.hunger += 1;
        }

        if self.turn % THIRST_INTERVAL == 0 {
            self.player.thirst += 1;
        }

        if self.player.thirst > THIRST_CAP {
            return TurnResult::ThirstDeath;
        } else if self.player.hunger > HUNGER_CAP {
            return TurnResult::HungerDeath;
        }

        let mut kill = Vec::new();
        let mut entities = self.entities.clone();
        for (i, entity) in entities.iter_mut().enumerate() {
            res = entity.ai(self)?;
            self.draw(0, 0);
            crate::draw_key(self);
            self.draw_result(res);

            if !entity.alive {
                kill.push(i);
            }
        }

        for i in kill {
            entities.remove(i);
        }

        if let Some(e) = Entity::spawn_random(self) {
            entities.push(e);
        }

        self.entities = entities;

        TurnResult::Ok
    }

    pub fn parse(map: &str, x: u32, y: u32) -> Self {
        let mut out = Self::new();
        out.player = Player { x, y, hunger: 0, thirst: 0, health: 10, damage: 1 };

        let mut x = 0;
        let mut y = 0;
        for ch in map.trim().chars() {
            match ch {
                '~' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Water,
                    }
                }
                '"' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Grass,
                    }
                }
                '$' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Forest,
                    }
                }
                'n' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Hill,
                    }
                }
                'A' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Mountain,
                    }
                }
                '\n' => {
                    if x != 0 {
                        y += 1;
                        if y >= HEIGHT {
                            break;
                        }
                        x = 0;
                    }

                    continue;
                }
                ' ' | '\t' => continue,
                _ => panic!("invalid tile: `{ch}`"),
            }

            x += 1;
            if x >= WIDTH {
                y += 1;
                if y >= HEIGHT {
                    break;
                }
                x = 0;
            }
        }

        out
    }

    pub fn interact(&mut self) -> TurnResult {
        let mut kill = None;
        let mut res = TurnResult::Ok;
        if let Some((i, entity)) = &mut self.entities.iter_mut().enumerate().find(|(_, e)| e.x == self.player.x && e.y == self.player.y) {
            res = entity.interact(&mut self.player)?;

            if !entity.alive {
                kill = Some(*i);
            }

            if let Some(i) = kill {
                self.entities.remove(i);
            }
        } else {
            self.player.health = (self.player.health + self.turn % 2).min(10);
        }

        if matches!(res, TurnResult::WonFight(_)) {
            let mut rng = thread_rng();
            if rng.gen::<f32>() <= UPGRADE_CHANCE {
                self.player.damage += 1;
                res = TurnResult::WonFight(true);
            }
        }

        res
    }

    pub fn draw(&self, mut x: u32, mut y: u32) {
        let ox = x;
        let oy = y;

        let mut dark = false;
        for row in self.tiles {
            for tile in row {
                cod::color::bg(if dark { 236 } else { 238 });
                dark = !dark;
                tile.print(x, y);
                cod::color::de_bg();
                x += 2;
            }

            if WIDTH % 2 == 0 {
                dark = !dark
            };
            x = ox;
            y += 1;
        }
        println!();

        for entity in &self.entities {
            entity.draw(ox, oy);
        }

        cod::color::fg(140);
        cod::pixel('&', self.player.x * 2 + ox, self.player.y + oy);
        cod::color::de();
    }

    pub fn spawn(&mut self, entity: EntityKind, x: u32, y: u32) {
        if x < WIDTH as u32 && y < WIDTH as u32 {
            self.entities.push(Entity { kind: entity, x, y, alive: true });
        }
    }

    pub fn go(&mut self, direction: Direction) -> TurnResult {
        let (diff_x, diff_y) = direction.diff();

        let x = self.player.x.saturating_add_signed(diff_x);
        let y = self.player.y.saturating_add_signed(diff_y);

        match self.tiles[y as usize][x as usize].kind {
            TileKind::Mountain => TurnResult::InvalidMove(direction),
            TileKind::Water => {
                self.player.thirst = 0;
                TurnResult::WaterMove
            }

            _ => {
                self.player.x = x;
                self.player.y = y;

                let mut res = TurnResult::Ok;

                let mut kill = Vec::new();
                for (i, entity) in self.entities.iter_mut().enumerate() {
                    if entity.x == x && entity.y == y {
                        res = entity.interact(&mut self.player)?;

                        if !entity.alive {
                            kill.push(i);
                        }
                    }
                }

                for (i, e) in kill.iter().enumerate() {
                    self.entities.remove(e - i);
                }

                res
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TileKind {
    #[default]
    Water = b'~',
    Grass = b'"',
    Forest = b'$',
    Hill = b'n',
    Mountain = b'A',
}

impl TileKind {
    pub fn color(&self) -> u8 {
        match self {
            Self::Water => 6,
            Self::Grass => 11,
            Self::Forest => 2,
            Self::Hill => 70,
            Self::Mountain => 7,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub kind: TileKind,
}

impl Tile {
    fn color(&self) -> u8 {
        self.kind.color()
    }

    fn print(&self, x: u32, y: u32) {
        cod::color::fg(self.kind.color());
        cod::blit(format!("{0}{0}", self.kind as u8 as char), x, y);
    }
}
