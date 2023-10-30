use std::ops::ControlFlow;

use cod::Key;

use crate::map::Map;
use crate::Direction;

pub fn handle(map: &mut Map) -> TurnResult {
    if let Some(key) = cod::read::key() {
        match key {
            Key::ArrowUp => map.go(Direction::Up),
            Key::ArrowDown => map.go(Direction::Down),
            Key::ArrowLeft => map.go(Direction::Left),
            Key::ArrowRight => map.go(Direction::Right),
            Key::Char('q') => TurnResult::Quit,
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
    InvalidMove(Direction),
    WaterMove,
    HungerDeath,
    ThirstDeath,
    Quit,
}

impl TurnResult {
    pub fn good(&self) -> bool {
        !self.bad()
    }

    pub fn bad(&self) -> bool {
        match self {
            Self::Ok |
            Self::NoKey |
            Self::InvalidKey(_) |
            Self::InvalidMove(_) |
            Self::WaterMove => false,
            Self::Quit |
            Self::HungerDeath |
            Self::ThirstDeath => true,
        }
    }
}

impl std::ops::FromResidual for TurnResult {
    fn from_residual(residual: Self) -> Self { residual }
}

impl std::ops::Try for TurnResult {
    type Output = Self;
    type Residual = Self;

    fn from_output(output: Self) -> Self { output }
    fn branch(self) -> ControlFlow<Self, Self> {
        if self.good() {
            ControlFlow::Continue(self)
        } else {
            ControlFlow::Break(self)
        }
    }
}

