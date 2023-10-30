use crate::player::{constants::*, Player};
use crate::input::{self, TurnResult};
use crate::Direction;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;

#[derive(Debug, Default, Clone, Copy)]
pub struct Map {
    tiles: [[Tile; WIDTH]; HEIGHT],

    pub player: Player,
    pub turn: u32,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) -> TurnResult {
        let mut res = input::handle(self)?;
        while res != TurnResult::Ok {
            crate::draw_key(self);
            cod::goto::bot();
            cod::color::de();
            cod::color::fg(1);
            cod::goto::up(1);
            cod::clear::line();
            match res {
                TurnResult::NoKey => println!("Please press a key"),
                TurnResult::InvalidMove(_) => println!("You can't move there"),
                TurnResult::WaterMove => println!("You drank your fill"),
                TurnResult::InvalidKey(_) => println!("That's not a valid key"),
                _ => {}
            }

            cod::flush();
            res = input::handle(self)?;
        }

        self.turn += 1;

        if self.turn % HUNGER_INTERVAL == 0 {
            self.player.hunger += 1;
        }

        if self.turn % THIRST_INTERVAL == 0 {
            self.player.thirst += 1;
        }

        if self.player.thirst > THIRST_CAP {
            return TurnResult::ThirstDeath;
        } else if self.player.hunger > HUNGER_CAP {
            return TurnResult::HungerDeath;
        }

        TurnResult::Ok
    }

    pub fn parse(map: &str, x: u32, y: u32) -> Self {
        let mut out = Self::new();
        out.player = Player { x, y, hunger: 0, thirst: 0 };

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
                    if x != 0 {
                        y += 1;
                        if y >= HEIGHT {
                            break;
                        }
                        x = 0;
                    }

                    continue;
                }
                ' ' | '\t' => continue,
                _ => panic!("invalid tile: `{ch}`"),
            }

            x += 1;
            if x >= WIDTH {
                y += 1;
                if y >= HEIGHT {
                    break;
                }
                x = 0;
            }
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

    pub fn go(&mut self, direction: Direction) -> TurnResult {
        let (diff_x, diff_y) = direction.diff();

        let x = self.player.x.saturating_add_signed(diff_x);
        let y = self.player.y.saturating_add_signed(diff_y);

        match self.tiles[y as usize][x as usize].kind {
            TileKind::Mountain => TurnResult::InvalidMove(direction),
            TileKind::Water => {
                self.player.thirst = 0;
                TurnResult::WaterMove
            }

            _ => {
                self.player.x = x;
                self.player.y = y;
                TurnResult::Ok
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
