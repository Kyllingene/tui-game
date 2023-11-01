use std::ops::ControlFlow;

use cod::Key;

use crate::map::Direction;
use crate::world::World;
pub fn handle(world: &mut World) -> TurnResult {
    if let Some(key) = cod::read::key() {
        match key {
            Key::ArrowUp => world.go(Direction::Up),
            Key::ArrowDown => world.go(Direction::Down),
            Key::ArrowLeft => world.go(Direction::Left),
            Key::ArrowRight => world.go(Direction::Right),
            Key::Char('q') | Key::Char('\x04') => TurnResult::Quit,
            Key::Char(' ') => world.interact(),
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
    PickedUpItem(&'static str),
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
