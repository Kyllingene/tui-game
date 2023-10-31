use rand::{Rng, thread_rng};

use crate::input::TurnResult;

const HUNGER_QUIPS: &str = include_str!("deaths/hunger.txt");
const THIRST_QUIPS: &str = include_str!("deaths/thirst.txt");
const VIOLENT_QUIPS: &str = include_str!("deaths/violent.txt");
const QUIT_QUIPS: &str = include_str!("deaths/quit.txt");

pub fn random(res: TurnResult) -> &'static str {
    let quips = match res {
        TurnResult::HungerDeath => HUNGER_QUIPS,
        TurnResult::ThirstDeath => THIRST_QUIPS,
        TurnResult::ViolentDeath => VIOLENT_QUIPS,
        TurnResult::Quit => QUIT_QUIPS,
        _ => panic!("Unhandled bad result: {res:?}"),
    }.lines().collect::<Vec<_>>();

    let i = thread_rng().gen_range(0..quips.len());
    quips[i]
}

