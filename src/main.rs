mod input;
mod map;
mod player;

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
            ~~~~~~~~
            ~""""""~
            ~"$$$$"~
            ~"$An$"~
            ~"$nA$"~
            ~"$$$$"~
            ~""""""~
            ~~~~~~~~
        "#,
        1,
        1,
    );

    loop {
        cod::clear::all();
        map.print(0, 0);

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
        print!("\nPlayer: &");

        cod::color::de();
        cod::goto::bot();
        cod::flush();

        if input::handle(&mut map) {
            cod::goto::bot();
            cod::color::de();
            cod::color::fg(1);
            cod::goto::up(1);
            println!("Invalid move");
            cod::flush();

            break;
        }
    }
}
