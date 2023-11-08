use std::ops::ControlFlow;

use cod::Key;

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
