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

    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        self.tiles
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
            .map(|t| *t = tile);
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

            cod::color::de_fg();
            print!("|");
        }
        println!("\n{}+", "-".repeat(WIDTH * 2));
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
                '_' => {
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
    Grass = b'_',
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
            Self::Mountain => (230, 255, 242),
        }
    }

    pub fn faded_color(&self) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = self.color();

        let r_dim = r as f32 / 255.0;
        let g_dim = g as f32 / 255.0;
        let b_dim = b as f32 / 255.0;

        r = (r as f32 * r_dim * 0.5) as u8;
        g = (g as f32 * g_dim * 0.5) as u8;
        b = (b as f32 * b_dim * 0.5) as u8;

        (r, g, b)
    }

    pub fn dark_faded_color(&self) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = self.color();

        let r_dim = r as f32 / (255.0 * 0.8);
        let g_dim = g as f32 / (255.0 * 0.8);
        let b_dim = b as f32 / (255.0 * 0.8);

        r = (r as f32 * r_dim * 0.5) as u8;
        g = (g as f32 * g_dim * 0.5) as u8;
        b = (b as f32 * b_dim * 0.5) as u8;

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
