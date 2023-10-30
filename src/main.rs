#![feature(try_trait_v2)]

mod entity;
mod input;
mod map;
mod player;

use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform};

use input::TurnResult;
use map::{TileKind, WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn diff(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match Uniform::new(0, 4).sample(rng) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut map = map::Map::parse(
        r#"
            ~~~~~~~~~~~~~~~~
            ~~""""""""~~~~~~
            ~""""$$"$"""""~~
            ~""$$$$$n$$$"""~
            ~~~~~$$$$$n$$"~~
            ~~~~~~~$$$$$$""~
            ~""~~~~"$$$$$""~
            ~"$"""""~~~""""~
            ~"$"""""~~~~~~~~
            ~""nA$"""""""""~
            ~""""nnnnA"""""~
            ~~""""$$nn$""n"~
            ~~~"""""""""""~~
            ~~~""""""""""~~~
            ~~~~~""""""""~~~
            ~~~~~~~~~~~~~~~~
        "#,
        12,
        14,
    );

    // map.spawn(entity::EntityKind::Food { food: 3 }, 2, 2);
    map.spawn(entity::EntityKind::Enemy { health: 3, damage: 2 }, 2, 2);

    loop {
        cod::clear::all();
        map.draw(0, 0);

        draw_key(&map);

        cod::color::de();
        cod::goto::bot();
        cod::flush();

        let res = map.update();
        if res.bad() {
            cod::goto::bot();
            cod::color::de();
            cod::color::fg(1);
            cod::goto::up(1);
            cod::clear::line();
            match res {
                TurnResult::Quit => println!("Goodbye, wimp"),
                TurnResult::HungerDeath => println!("You died by hunger"),
                TurnResult::ThirstDeath => println!("You died by thirst"),
                _ => panic!("Unhandled bad outcome: {res:#?}"),
            }
            cod::flush();

            break;
        }
    }
}

fn draw_key(map: &map::Map) {
        cod::goto::pos(0, WIDTH as u32 + 1);
        cod::color::de_bg();
        for kind in [
            TileKind::Water,
            TileKind::Grass,
            TileKind::Forest,
            TileKind::Hill,
            TileKind::Mountain,
        ] {
            cod::color::fg(kind.color());
            print!("{kind:?}: {}  ", kind as u8 as char);
        }

        cod::color::fg(108);
        print!("\nFood: +  ");

        cod::color::fg(210);
        print!("Enemy: +  ");

        cod::color::fg(140);
        print!("\nPlayer: &  ");

        cod::color::fg(1);
        print!("Health: {:2}  ", map.player.health);

        cod::color::fg(223);
        print!("Hunger: {:2}  ", player::constants::HUNGER_CAP - map.player.hunger);

        cod::color::fg(12);
        print!("Thirst: {:2}", player::constants::THIRST_CAP - map.player.thirst);
}

