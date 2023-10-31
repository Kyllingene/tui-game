use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 16;

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
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Map {
    pub tiles: [[Tile; WIDTH]; HEIGHT],
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Tile> {
        self.tiles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }

    pub fn draw(&self, mut x: u32, mut y: u32) {
        let ox = x;
        let mut dark = false;
        for row in self.tiles {
            for tile in row {
                dark = !dark;
                tile.draw(x, y, dark);
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
    }

    pub fn parse(map: &str) -> Self {
        let mut out = Self::new();

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
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Self::Water => (0, 77, 153),
            Self::Grass => (0, 153, 25),
            Self::Forest => (0, 77, 38),
            Self::Hill => (255, 238, 230),
            Self::Mountain => (0, 17, 26),
        }
    }

    pub fn faded_color(&self) -> (u8, u8, u8) {
        if *self == Self::Mountain { return (0, 7, 10); }

        let (mut r, mut g, mut b) = self.color();
        r = r.saturating_sub(125);
        g = g.saturating_sub(125);
        b = b.saturating_sub(125);
        (r, g, b)
    }

    pub fn dark_faded_color(&self) -> (u8, u8, u8) {
        if *self == Self::Mountain { return (0, 5, 8); }

        let (mut r, mut g, mut b) = self.color();
        r = r.saturating_sub(135);
        g = g.saturating_sub(135);
        b = b.saturating_sub(135);
        (r, g, b)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub kind: TileKind,
}

impl Tile {
    pub fn draw(&self, x: u32, y: u32, dark_bg: bool) {
        let (r, g, b) = self.kind.color();
        cod::color::tc_fg(r, g, b);
        let (r, g, b) = if dark_bg { self.kind.dark_faded_color() } else { self.kind.faded_color() };
        cod::color::tc_bg(r, g, b);
        cod::blit(format!("{0}{0}", self.kind as u8 as char), x, y);
    }
}
