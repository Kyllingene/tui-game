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
        for ch in map.trim().chars() {
            match ch {
                '~' => out.tiles[y][x] = Tile { kind: TileKind::Water },
                '"' => out.tiles[y][x] = Tile { kind: TileKind::Grass },
                '$' => out.tiles[y][x] = Tile { kind: TileKind::Forest },
                'n' => out.tiles[y][x] = Tile { kind: TileKind::Hill },
                'A' => out.tiles[y][x] = Tile { kind: TileKind::Mountain },
                '\n' => {
                    if y >= HEIGHT { break; }
                    y += 1;
                    x = 0;

                    continue;
                }
                ' ' | '\t' => continue,
                _ => panic!("invalid tile: `{ch}`"),
            }

            if x >= WIDTH {
                if y >= HEIGHT { break; }
                y += 1;
                x = 0;
            }
            x += 1;
        }

        out
    }

    pub fn print(&self, mut x: u32, mut y: u32) {
        let ox = x;
        let mut dark = false;
        for row in self.tiles {
            for tile in row {
                cod::color::bg(if dark { 236 } else { 238 });
                dark = !dark;
                tile.print(x, y);
                cod::color::de_bg();
                x += 2;
            }

            if WIDTH % 2 == 0 { dark = !dark };
            x = ox;
            y += 1;
        }

        println!();
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TileKind {
    #[default]
    Water = b'~',
    Grass = b'"',
    Forest = b'$',
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

        // cod::style::de();
        // match self {
        //     Self::Water
        //         | Self::Grass => cod::style::italic(),
        //     _ => cod::style::bold(),
        // }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    kind: TileKind,
}

impl Tile {
    fn print(&self, x: u32, y: u32) {
        self.kind.color();
        cod::blit(format!("{0}{0}", self.kind as u8 as char), x, y);
    }
}

