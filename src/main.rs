#![feature(try_trait_v2)]

mod input;
mod map;
mod player;

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
            ~~~""""""""""~~~
            ~~~~~~~~~~~~~~~~
        "#,
        12,
        14,
    );

    loop {
        cod::clear::all();
        map.print(0, 0);

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
                TurnResult::InvalidMove(_) => println!("You can't move there"),
                _ => panic!("Unhandled bad outcome: {res:#?}"),
            }
            cod::flush();

            break;
        }
    }
}

fn draw_key(map: &map::Map) {
        cod::goto::pos(0, WIDTH as u32 + 1);
        cod::color::fg(0);
        cod::color::bg(2);
        print!("Key:");
        cod::color::de_bg();
        print!(" ");

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

        cod::color::fg(140);
        print!("\nPlayer: &  ");

        cod::color::fg(223);
        print!("Hunger: {:2}  ", player::constants::HUNGER_CAP - map.player.hunger);

        cod::color::fg(12);
        print!("Thirst: {:2}", player::constants::THIRST_CAP - map.player.thirst);
}

