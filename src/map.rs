use crate::player::Player;
use crate::Direction;

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

    pub fn parse(map: &str, x: u32, y: u32) -> Self {
        let mut out = Self::new();
        out.player = Player { x, y };

        let mut x = 0;
        let mut y = 0;
        for ch in map.trim().chars() {
            match ch {
                '~' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Water,
                    }
                }
                '"' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Grass,
                    }
                }
                '$' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Forest,
                    }
                }
                'n' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Hill,
                    }
                }
                'A' => {
                    out.tiles[y][x] = Tile {
                        kind: TileKind::Mountain,
                    }
                }
                '\n' => {
                    if y >= HEIGHT {
                        break;
                    }
                    y += 1;
                    x = 0;

                    continue;
                }
                ' ' | '\t' => continue,
                _ => panic!("invalid tile: `{ch}`"),
            }

            if x >= WIDTH {
                if y >= HEIGHT {
                    break;
                }
                y += 1;
                x = 0;
            }
            x += 1;
        }

        out
    }

    pub fn print(&self, mut x: u32, mut y: u32) {
        let ox = x;
        let oy = y;

        let mut dark = false;
        for row in self.tiles {
            for tile in row {
                cod::color::bg(if dark { 236 } else { 238 });
                dark = !dark;
                tile.print(x, y);
                cod::color::de_bg();
                x += 2;
            }

            if WIDTH % 2 == 0 {
                dark = !dark
            };
            x = ox;
            y += 1;
        }
        println!();

        cod::color::fg(140);
        cod::pixel('&', self.player.x * 2 + ox, self.player.y + oy);
        cod::color::de();
    }

    pub fn go(&mut self, direction: Direction) -> bool {
        let (diff_x, diff_y) = direction.diff();

        let x = self.player.x.saturating_add_signed(diff_x);
        let y = self.player.y.saturating_add_signed(diff_y);

        match self.tiles[y as usize][x as usize].kind {
            TileKind::Water | TileKind::Mountain => false,
            _ => {
                self.player.x = x;
                self.player.y = y;
                true
            }
        }
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
    pub fn color(&self) -> u8 {
        match self {
            Self::Water => 6,
            Self::Grass => 11,
            Self::Forest => 2,
            Self::Hill => 70,
            Self::Mountain => 7,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub kind: TileKind,
}

impl Tile {
    fn color(&self) -> u8 {
        self.kind.color()
    }

    fn print(&self, x: u32, y: u32) {
        cod::color::fg(self.kind.color());
        cod::blit(format!("{0}{0}", self.kind as u8 as char), x, y);
    }
}
