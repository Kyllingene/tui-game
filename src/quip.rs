use rand::{thread_rng, Rng};

use crate::input::BadResult;

const HUNGER_QUIPS: &str = include_str!("../deaths/hunger.txt");
const THIRST_QUIPS: &str = include_str!("../deaths/thirst.txt");
const VIOLENT_QUIPS: &str = include_str!("../deaths/violent.txt");
const QUIT_QUIPS: &str = include_str!("../deaths/quit.txt");

pub fn random(res: BadResult) -> &'static str {
    let quips = match res {
        BadResult::HungerDeath => HUNGER_QUIPS,
        BadResult::ThirstDeath => THIRST_QUIPS,
        BadResult::ViolentDeath => VIOLENT_QUIPS,
        BadResult::Quit => QUIT_QUIPS,
    }
    .lines()
    .collect::<Vec<_>>();

    let i = thread_rng().gen_range(0..quips.len());
    quips[i]
}
