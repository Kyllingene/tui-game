use std::fmt::Display;

use crate::player::Player;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;

#[derive(Debug, Default, Clone, Copy)]
pub struct Map {
    tiles: [[Tile; WIDTH]; HEIGHT],

    player: Player,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(map: &str) -> Self {
        let mut out = Self::new();

        let mut x = 0;
        let mut y = 0;
        for ch in map.chars() {
            match ch {
                '~' => out.tiles[y][x] = Tile { kind: TileKind::Water },
                '"' => out.tiles[y][x] = Tile { kind: TileKind::Grass },
                '!' => out.tiles[y][x] = Tile { kind: TileKind::Forest },
                'n' => out.tiles[y][x] = Tile { kind: TileKind::Hill },
                'A' => out.tiles[y][x] = Tile { kind: TileKind::Mountain },
                '\n' => {
                    x = 0;
                    y += 1;

                    if y >= HEIGHT { break; }
                    continue;
                }
                _ => continue,
            }

            x += 1;
            if x >= WIDTH {
                x = 0;
                y += 1;

                if y >= HEIGHT { break; }
            }
        }

        out
    }

    pub fn print(&self) {
        for row in self.tiles {
            for tile in row {
                print!("{tile}{tile}");
            }

            println!();
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TileKind {
    #[default]
    Water = b'~',
    Grass = b'"',
    Forest = b'!',
    Hill = b'n',
    Mountain = b'A',
}

impl TileKind {
    pub fn color(&self) {
        cod::color::fg(match self {
            Self::Water => 6,
            Self::Grass => 10,
            Self::Forest => 2,
            Self::Hill => 70,
            Self::Mountain => 7,
        });

        cod::style::de();
        match self {
            Self::Water
                | Self::Grass => cod::style::italic(),
            _ => cod::style::bold(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    kind: TileKind,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.kind.color();
        write!(f, "{}", self.kind as u8 as char)
    }
}

