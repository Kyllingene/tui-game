use std::fmt::Display;

use rand::{thread_rng, Rng};

use crate::entity::{Entity, EntityKind};
use crate::input::{self, TurnResult};
use crate::map::{Direction, Map, TileKind, HEIGHT, WIDTH};
use crate::player::{constants::*, Player};
use crate::world_map::{sectors, START};

#[derive(Debug)]
pub struct World {
    pub map: Map,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub turn: u32,
}

impl World {
    pub fn new(player_x: u32, player_y: u32) -> Self {
        let (entities, map) = Map::new(sectors(), START);
        Self {
            // map: Map::parse(map),
            map,
            player: Player {
                x: player_x,
                y: player_y,
                hunger: 0,
                thirst: 0,
                health: 10,
                damage: 1,
            },
            entities,
            turn: 0,
        }
    }

    pub fn max_entities(&self) -> u32 {
        4 + self.turn / 10
    }

    pub fn spawn_chance_coeff(&self) -> f32 {
        1.0 + self.turn as f32 / 15.0
    }

    pub fn draw_message(&self, msg: impl Display, color: u8) {
        cod::goto::pos(0, HEIGHT as u32);
        //cod::clear::line();
        cod::color::de();
        print!("{}+  ", "-".repeat(WIDTH * 2));

        cod::goto::pos(1, HEIGHT as u32);
        cod::color::fg(color);
        print!("{msg}");
        cod::goto::bot();
        cod::flush();
    }

    pub fn draw_result(&self, res: TurnResult) {
        self.draw_key();
        match res {
            TurnResult::NoKey => self.draw_message("Please press a key", 1),
            TurnResult::InvalidMove(_) => self.draw_message("You can't move there", 1),
            TurnResult::WaterMove => self.draw_message("You drank your fill", 2),
            TurnResult::InvalidKey(_) => self.draw_message("That's not a valid key", 1),
            TurnResult::Fight(dmg, hp) => self.draw_message(
                format!(
                    "You dealt {}, they dealt {dmg} and are at {hp}",
                    self.player.damage
                ),
                3,
            ),
            TurnResult::WonFight(upgrade) => {
                if upgrade {
                    self.draw_message("You won and got an upgrade!", 2)
                } else {
                    self.draw_message("You killed the enemy!", 2)
                }
            }
            TurnResult::DefeatedBoss(_id) => {
                self.draw_message("You killed the boss!", 2);
            }
            TurnResult::Ate(food) => {
                self.draw_message(format!("You ate {food} food and healed 2"), 2)
            }
            _ => {}
        }
    }

    pub fn update(&mut self) -> TurnResult {
        let mut res = input::handle(self)?;
        while res != TurnResult::Ok {
            self.draw(0, 0);
            self.draw_key();
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
            self.draw_result(res);

            if entity.alive {
                self.entities[i].x = entity.x;
                self.entities[i].y = entity.y;
            } else {
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

    pub fn interact(&mut self) -> TurnResult {
        let mut kill = None;
        let mut res = TurnResult::Ok;
        if let Some((i, entity)) = &mut self
            .entities
            .iter_mut()
            .enumerate()
            .find(|(_, e)| e.x == self.player.x && e.y == self.player.y)
        {
            res = entity.interact(&mut self.player, &mut self.map)?;

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

    pub fn draw(&self, x: u32, y: u32) {
        self.map.draw(x, y);
        self.draw_key();

        for entity in &self.entities {
            entity.draw(x, y);
        }

        cod::color::fg(140);
        cod::color::de_bg();
        cod::pixel('&', self.player.x * 2 + x, self.player.y + y);
        cod::color::de_fg();
    }

    fn draw_key(&self) {
        cod::goto::pos(0, HEIGHT as u32 + 1);
        for kind in [
            TileKind::Water,
            TileKind::Grass,
            TileKind::Forest,
            TileKind::Hill,
            TileKind::Mountain,
        ] {
            let (r, g, b) = kind.color();
            cod::color::tc_fg(r, g, b);
            let (r, g, b) = kind.dark_faded_color();
            cod::color::tc_bg(r, g, b);
            print!("{kind:?}: {}  ", kind as u8 as char);
        }

        cod::color::de();

        cod::color::fg(108);
        print!("\nFood: +  ");

        cod::color::fg(210);
        print!("Enemy: +  ");

        cod::color::fg(136);
        cod::color::bg(9);
        print!("Boss: #");
        cod::color::de();
        print!("  ");

        cod::color::fg(140);
        print!("\nPlayer: &  ");

        cod::color::fg(1);
        print!("Health: {:2}  ", self.player.health);

        cod::color::fg(7);
        print!("Damage: {:2}  ", self.player.damage);

        cod::color::fg(223);
        print!("Hunger: {:2}  ", HUNGER_CAP - self.player.hunger);

        cod::color::fg(12);
        print!("Thirst: {:2}", THIRST_CAP - self.player.thirst);
    }

    #[allow(dead_code)]
    pub fn spawn(&mut self, x: u32, y: u32, entity: EntityKind) {
        if x < WIDTH as u32 && y < WIDTH as u32 {
            self.entities.push(Entity {
                kind: entity,
                x,
                y,
                alive: true,
                persist: false,
            });
        }
    }

    pub fn go(&mut self, direction: Direction) -> TurnResult {
        let (diff_x, diff_y) = direction.diff();

        let x = self.player.x.saturating_add_signed(diff_x);
        let y = self.player.y.saturating_add_signed(diff_y);

        if (x == self.player.x && y == self.player.y) || x as usize >= WIDTH || y as usize >= HEIGHT
        {
            let neighbor = self.map.sector().neighbor(direction);
            if let Some(new_sector) = neighbor {
                let entities = std::mem::take(&mut self.entities);
                self.map.save_entities(self.map.current_sector.id, entities
                    .into_iter()
                    .filter(|e| e.persist)
                    .collect());
                self.entities = self.map.load(new_sector);

                match direction {
                    Direction::Up => self.player.y = HEIGHT as u32 - 1,
                    Direction::Down => self.player.y = 0,
                    Direction::Left => self.player.x = WIDTH as u32 - 1,
                    Direction::Right => self.player.x = 0,
                }

                return TurnResult::Ok;
            } else {
                return TurnResult::InvalidMove(direction);
            }
        }

        match self.map.get(x, y).unwrap().kind {
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
                        res = entity.interact(&mut self.player, &mut self.map)?;

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
