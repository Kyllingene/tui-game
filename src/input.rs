use cod::{BoxChars, Key};

use crate::difficulty::Difficulty;
use crate::map::Direction;
use crate::save;
use crate::sector::HEIGHT;
use crate::world::World;

#[macro_export]
macro_rules! good {
    () => { $crate::input::GoodResult::Ok.into() };
    ( $kind:ident ) => { $crate::input::GoodResult::$kind.into() };
    ( $kind:ident, $( $arg:expr ),* ) => { $crate::input::GoodResult::$kind($($arg,)+).into() };
}

#[macro_export]
macro_rules! bad {
    ( $kind:ident ) => { $crate::input::BadResult::$kind.into() };
    ( $kind:ident, $( $arg:expr ),* ) => { $crate::input::BadResult::$kind($($arg,)+).into() };
}

pub fn handle(world: &mut World) -> TurnResult {
    if let Some(key) = cod::read::key() {
        match key {
            Key::ArrowUp => world.go(Direction::Up),
            Key::ArrowDown => world.go(Direction::Down),
            Key::ArrowLeft => world.go(Direction::Left),
            Key::ArrowRight => world.go(Direction::Right),
            Key::Char('q') | Key::Char('\x04') => bad!(Quit),
            Key::Char(' ') => world.interact(),
            Key::Char('s') => {
                world.draw_message("Saving game", 3);
                if save::save(world) {
                    good!(Saved)
                } else {
                    good!()
                }
            }
            Key::Char('l') => {
                world.draw_message("Loading game", 3);
                if save::load(world) {
                    good!(Loaded)
                } else {
                    good!()
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
                            cod::rect_lines(
                                BoxChars {
                                    horizontal: '-',
                                    vertical: '|',
                                    corner: '+',
                                },
                                1,
                                1,
                                width + 1,
                                4,
                            )
                            .unwrap();

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

                good!(Menued)
            }
            Key::Char('d') => {
                cod::goto::pos(0, HEIGHT as u32);
                cod::clear::line();
                print!("Difficulty (easy, normal hard): ");
                cod::flush();
                let diff_str = cod::read::line();
                let difficulty = diff_str.as_ref().and_then(|d| {
                    Some(match d.to_lowercase().as_str() {
                        "easy" => Difficulty::easy(),
                        "normal" => Difficulty::normal(),
                        "hard" => Difficulty::hard(),
                        _ => None?,
                    })
                });

                if let Some(difficulty) = difficulty {
                    world.difficulty = difficulty;
                    world.draw_message(format!("Set difficulty to {}", diff_str.unwrap()), 2);
                    cod::read::key();
                } else {
                    world.draw_message("Invalid difficulty", 1);
                }

                good!(Menued)
            }
            _ => good!(InvalidKey, key)
        }
    } else {
        good!(NoKey)
    }
}

pub type TurnResult = Result<GoodResult, BadResult>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GoodResult {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BadResult {
    HungerDeath,
    ThirstDeath,
    ViolentDeath,
    Quit,
}

impl From<GoodResult> for TurnResult {
    fn from(res: GoodResult) -> Self {
        Ok(res)
    }
}

impl From<BadResult> for TurnResult {
    fn from(res: BadResult) -> Self {
        Err(res)
    }
}

