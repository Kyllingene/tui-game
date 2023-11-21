use std::fmt::Display;

use rand::{thread_rng, Rng};

use crate::{good, bad};
use crate::difficulty::Difficulty;
use crate::entity::{Entity, EntityKind};
use crate::input::{self, TurnResult, GoodResult};
use crate::map::{Direction, Map, TileKind, HEIGHT, WIDTH};
use crate::player::{constants::*, Player};
use crate::world_map::sectors;

#[derive(Debug)]
pub struct World {
    pub map: Map,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub despawned: Vec<(&'static str, u32)>,
    pub turn: u32,
    pub difficulty: Difficulty,
}

impl World {
    pub fn new(player_x: u32, player_y: u32) -> Self {
        let (entities, map) = Map::new(sectors(), "start");
        Self {
            map,
            player: Player {
                x: player_x,
                y: player_y,
                hunger: 0,
                thirst: 0,
                hunger_cap: INITIAL_HUNGER_CAP,
                thirst_cap: INITIAL_THIRST_CAP,
                health: 10,
                max_health: 10,
                damage: 1,
                inventory: Vec::new(),
            },
            entities,
            despawned: Vec::new(),
            turn: 0,
            difficulty: Difficulty::normal(),
        }
    }

    pub fn despawn(&mut self, i: usize) {
        let entity = self.entities.remove(i);
        if let Some(id) = entity.id() {
            self.despawned.push((self.map.sector().id, id));
        }
    }

    #[allow(unused)]
    pub fn despawn_id(&mut self, id: u32) {
        if let Some((i, _)) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, e)| e.id() == Some(id))
        {
            self.despawn(i);
        }
    }

    pub fn max_entities(&self) -> u32 {
        (4 + self.turn / 20).min((WIDTH * HEIGHT) as u32 / 6)
    }

    pub fn spawn_chance_coeff(&self) -> f32 {
        1.0 + self.turn as f32 / 15.0
    }

    pub fn draw_message(&self, msg: impl Display, color: u8) {
        cod::goto::pos(0, HEIGHT as u32);
        //cod::clear::line();
        cod::color::de();
        print!("{}/  ", "-".repeat(WIDTH * 2));

        cod::goto::pos(1, HEIGHT as u32);
        cod::color::fg(color);
        print!("{msg}");
        cod::goto::bot();
        cod::flush();
    }

    pub fn draw_result(&self, res: GoodResult) {
        self.draw_key();
        match res {
            GoodResult::NoKey => self.draw_message("Please press a key", 1),
            GoodResult::InvalidMove(_) => self.draw_message("You can't move there", 1),
            GoodResult::WaterMove => self.draw_message("You drank your fill", 2),
            GoodResult::InvalidKey(_) => self.draw_message("That's not a valid key", 1),
            GoodResult::Fight(dmg, hp) => self.draw_message(
                format!(
                    "You dealt {}, they dealt {dmg} and are at {hp}",
                    self.player.damage
                ),
                3,
            ),
            GoodResult::WonFight(upgrade) => {
                if upgrade {
                    self.draw_message("You won and got an upgrade!", 2)
                } else {
                    self.draw_message("You killed the enemy!", 2)
                }
            }
            GoodResult::DefeatedBoss(_id) => {
                self.draw_message("You killed the boss!", 2);
            }
            GoodResult::Ate(food) => {
                self.draw_message(format!("You ate {food} food and healed 2"), 2)
            }
            GoodResult::Saved => {
                self.draw_message("Saved!", 2);
            }
            GoodResult::Loaded => {
                self.draw_message("Loaded!", 2);
            }
            _ => {}
        }
    }

    pub fn update(&mut self) -> TurnResult {
        let mut res = input::handle(self)?;
        while res != GoodResult::Ok {
            self.draw(0, 0);
            self.draw_key();
            self.draw_result(res);
            res = input::handle(self)?;
        }

        self.turn += 1;

        if self.turn % HUNGER_INTERVAL == 0 && self.map.sector().do_survival {
            self.player.hunger += 1;
        }

        if self.turn % THIRST_INTERVAL == 0
            && self.player.thirst <= self.player.thirst_cap
            && self.map.sector().do_survival
        {
            self.player.thirst += 1;
        }

        if self.player.thirst > self.player.thirst_cap {
            self.player.health = self.player.health.saturating_sub(1);
            if self.player.health == 0 {
                return bad!(ThirstDeath);
            }

            self.draw_message("You took 1 damage from thirst!", 1);
        } else if self.player.hunger > self.player.thirst_cap {
            return bad!(HungerDeath);
        }

        let mut kill = Vec::new();
        let mut entities = self.entities.clone();
        for (i, entity) in entities.iter_mut().enumerate() {
            let r = entity.ai(self)?;
            if r != GoodResult::Ok {
                res = r;
                self.draw_result(res);
            }
            self.draw(0, 0);

            if entity.alive {
                self.entities[i].x = entity.x;
                self.entities[i].y = entity.y;
            } else {
                kill.push(i);
            }
        }

        for (o, i) in kill.into_iter().enumerate() {
            self.despawn(i - o);
        }

        if let Some(e) = Entity::spawn_random(self) {
            entities.push(e);
        }

        self.entities = entities;

        good!()
    }

    pub fn interact(&mut self) -> TurnResult {
        let mut kill = None;
        let mut res = good!();
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
                self.despawn(i);
            }
        } else {
            self.player.health = (self.player.health + self.turn % 2).min(self.player.max_health);
        }

        if matches!(res, GoodResult::WonFight(_)) {
            let mut rng = thread_rng();
            if rng.gen::<f32>() <= UPGRADE_CHANCE {
                self.player.damage += 1;
                res = good!(WonFight, true);
            }
        }

        Ok(res)
    }

    pub fn draw(&self, x: u32, y: u32) {
        self.map.draw(x, y);
        self.draw_key();
        self.draw_inventory_side(x, y);

        for entity in &self.entities {
            entity.draw(x, y);
        }

        cod::color::fg(140);
        cod::color::de_bg();
        cod::pixel(CHARACTER, self.player.x * 2 + x, self.player.y + y);
        cod::color::de_fg();
    }

    fn draw_inventory_side(&self, x: u32, mut y: u32) {
        cod::color::de();
        let x = x + (WIDTH as u32 * 2) + 2;
        for item in &self.player.inventory {
            cod::blit(&item.name, x, y);
            y += 1;
        }
    }

    /// Draws the full inventory screen.
    ///
    /// Returns the y coordinate of each item name.
    pub fn draw_inventory_full(&self) -> Vec<u32> {
        let mut coords = Vec::with_capacity(self.player.inventory.len());
        cod::clear::all();

        let mut y = 1;
        for item in &self.player.inventory {
            coords.push(y);
            y += item.draw(3, y) + 1;
        }
        y -= 2;

        cod::color::de_fg();
        cod::pixel('+', 0, 0);
        if y != 0 {
            cod::orth_line('|', 0, 1, 0, y).unwrap();
        }
        cod::pixel('+', 0, y);
        cod::blit("- Inventory -+", 1, 0);

        cod::goto::bot();
        cod::flush();

        coords
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
            print!(" {kind:?}: {} ", kind as u8 as char);
            cod::goto::right(1);
        }

        cod::color::de();

        cod::color::fg(108);
        print!("\nFood: +  ");

        cod::color::fg(210);
        print!("Enemy: !  ");

        cod::color::fg(136);
        cod::color::bg(9);
        print!("Boss: #");
        cod::color::de();
        print!("  ");

        cod::color::fg(140);
        print!("\nPlayer: G  ");

        if self.player.health <= 4 {
            cod::color::fg(0);
            cod::color::bg(1);
        } else {
            cod::color::fg(1);
        }
        print!("Health: {:2}", self.player.health);
        cod::color::de_bg();
        cod::goto::right(2);

        cod::color::fg(7);
        print!("Damage: {:2}  ", self.player.damage);

        let food = self.player.hunger_cap.saturating_sub(self.player.hunger);
        if food <= 3 {
            cod::color::fg(1);
        } else {
            cod::color::fg(223);
        }

        print!("Hunger: {:2}  ", food);

        let water = self.player.thirst_cap.saturating_sub(self.player.thirst);
        if water <= 1 {
            cod::color::fg(1);
        } else {
            cod::color::fg(12);
        }

        print!("Thirst: {:2}", water);
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
                self.map.save_entities(
                    self.map.sector().id,
                    entities.into_iter().filter(|e| e.persist).collect(),
                );
                self.entities = self.map.load(new_sector);

                if let Some((x, y)) = self.map.sector().return_tile {
                    self.player.x = x;
                    self.player.y = y;
                } else {
                    match direction {
                        Direction::Up => self.player.y = HEIGHT as u32 - 1,
                        Direction::Down => self.player.y = 0,
                        Direction::Left => self.player.x = WIDTH as u32 - 1,
                        Direction::Right => self.player.x = 0,
                    }
                }

                return good!();
            } else {
                return good!(InvalidMove, direction);
            }
        }

        if let Some(new_sector) = self.map.sector().get_entrance(x, y) {
            let entities = std::mem::take(&mut self.entities);
            self.map.save_entities(
                self.map.current_sector.id,
                entities.into_iter().filter(|e| e.persist).collect(),
            );
            self.entities = self.map.load(new_sector);

            self.player.x = 0;
            self.player.y = 0;

            if !self.map.sector().do_survival {
                self.player.thirst = 0;
                self.player.hunger = 0;
                self.player.health = self.player.max_health;
            }

            good!()
        } else {
            match self.map.get(x, y).unwrap().kind {
                TileKind::Mountain => good!(InvalidMove, direction),
                TileKind::Water => {
                    self.player.thirst = 0;
                    good!(WaterMove)
                }

                _ => {
                    self.player.x = x;
                    self.player.y = y;

                    let mut res = good!();

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
                        self.despawn(e - i);
                    }

                    Ok(res)
                }
            }
        }
    }
}
