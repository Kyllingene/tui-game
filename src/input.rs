use std::ops::ControlFlow;

use cod::{BoxChars, Key};

use crate::map::Direction;
use crate::save;
use crate::world::World;
use crate::sector::HEIGHT;
use crate::difficulty::Difficulty;

pub fn handle(world: &mut World) -> TurnResult {
    if let Some(key) = cod::read::key() {
        match key {
            Key::ArrowUp => world.go(Direction::Up),
            Key::ArrowDown => world.go(Direction::Down),
            Key::ArrowLeft => world.go(Direction::Left),
            Key::ArrowRight => world.go(Direction::Right),
            Key::Char('q') | Key::Char('\x04') => TurnResult::Quit,
            Key::Char(' ') => world.interact(),
            Key::Char('s') => {
                world.draw_message("Saving game", 3);
                if save::save(world) {
                    TurnResult::Saved
                } else {
                    TurnResult::Ok
                }
            }
            Key::Char('l') => {
                world.draw_message("Loading game", 3);
                if save::load(world) {
                    TurnResult::Loaded
                } else {
                    TurnResult::Ok
                }
            }
            Key::Char('i') => {
                let mut items = world.draw_inventory_full();
                let mut cap = items.len().saturating_sub(1);

                let mut item_id = 0;
                let mut last_y = None;

                if let Some(y) = items.get(item_id).copied() {
                    cod::color::de();
                    cod::pixel('-', 1, y);
                    cod::flush();

                    last_y = Some(y);
                }

                while let Some(key) = cod::read::key() {
                    match key {
                        Key::ArrowUp => item_id = item_id.saturating_sub(1),
                        Key::ArrowDown => item_id = (item_id + 1).min(cap),

                        Key::Char('d' | 'D') => {
                            let msg = "Remove (y/N)?";

                            let name = &world.player.inventory[item_id].name;
                            let width = (name.len().max(msg.len()) + 4) as u32;

                            cod::clear::rect(1, 1, width + 1, 4).unwrap();

                            cod::color::fg(1);
                            cod::rect_lines(BoxChars {
                                horizontal: '-',
                                vertical: '|',
                                corner: '+',
                            }, 1, 1, width + 1, 4).unwrap();

                            cod::color::de_fg();
                            cod::blit(msg, 3, 2);
                            cod::blit(name, 3, 3);
                            cod::goto::bot();
                            cod::flush();

                            if matches!(cod::read::key(), Some(Key::Char('y' | 'Y'))) {
                                let item = world.player.inventory.remove(item_id);
                                item.unapply(&mut world.player);
                                item_id = item_id.saturating_sub(1);
                            }

                            items = world.draw_inventory_full();
                            cap = items.len().saturating_sub(1);
                        }

                        Key::Char('q' | 'Q') | Key::Escape => break,
                        _ => continue,
                    }

                    if let Some(y) = items.get(item_id).copied() {
                        if let Some(y) = last_y {
                            cod::pixel(' ', 1, y);
                        }

                        cod::color::de();
                        cod::pixel('-', 1, y);

                        cod::flush();

                        last_y = Some(y);
                    }
                }

                TurnResult::Menued
            }
            Key::Char('d') => {
                cod::goto::pos(0, HEIGHT as u32);
                cod::clear::line();
                print!("Difficulty (easy, normal hard): ");
                cod::flush();
                let diff_str = cod::read::line();
                let difficulty = diff_str.as_ref().and_then(|d| Some(match d.to_lowercase().as_str() {
                    "easy" => Difficulty::easy(),
                    "normal" => Difficulty::normal(),
                    "hard" => Difficulty::hard(),
                    _ => None?,
                }));

                if let Some(difficulty) = difficulty {
                    world.difficulty = difficulty;
                    world.draw_message(format!("Set difficulty to {}", diff_str.unwrap()), 2);
                    cod::read::key();
                } else {
                    world.draw_message("Invalid difficulty", 1);
                }

                TurnResult::Menued
            }
            _ => TurnResult::InvalidKey(key),
        }
    } else {
        TurnResult::NoKey
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnResult {
    Ok,
    NoKey,
    InvalidKey(Key),
    Fight(u32, u32),
    WonFight(bool),
    DefeatedBoss(u32),
    InvalidMove(Direction),
    PickedUpItem(String),
    Saved,
    Loaded,
    Menued,
    WaterMove,
    Ate(u32),
    HungerDeath,
    ThirstDeath,
    ViolentDeath,
    Quit,
}

impl TurnResult {
    pub fn good(&self) -> bool {
        !self.bad()
    }

    pub fn bad(&self) -> bool {
        match self {
            Self::Ok
            | Self::NoKey
            | Self::InvalidKey(_)
            | Self::Fight(_, _)
            | Self::WonFight(_)
            | Self::DefeatedBoss(_)
            | Self::InvalidMove(_)
            | Self::PickedUpItem(_)
            | Self::WaterMove
            | Self::Saved
            | Self::Loaded
            | Self::Menued
            | Self::Ate(_) => false,
            Self::Quit | Self::HungerDeath | Self::ThirstDeath | Self::ViolentDeath => true,
        }
    }
}

impl std::ops::FromResidual for TurnResult {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}

impl std::ops::Try for TurnResult {
    type Output = Self;
    type Residual = Self;

    fn from_output(output: Self) -> Self {
        output
    }
    fn branch(self) -> ControlFlow<Self, Self> {
        if self.good() {
            ControlFlow::Continue(self)
        } else {
            ControlFlow::Break(self)
        }
    }
}
