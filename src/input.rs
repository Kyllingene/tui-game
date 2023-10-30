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
            Key::Char('q')
                | Key::Char('\x04') => TurnResult::Quit,
            Key::Char(' ') => map.interact(),
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
    WonFight,
    InvalidMove(Direction),
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
            Self::Ok |
            Self::NoKey |
            Self::InvalidKey(_) |
            Self::Fight(_, _) |
            Self::WonFight |
            Self::InvalidMove(_) |
            Self::WaterMove |
            Self::Ate(_) => false,
            Self::Quit |
            Self::HungerDeath |
            Self::ThirstDeath |
            Self::ViolentDeath => true,
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

