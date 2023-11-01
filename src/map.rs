use std::collections::HashMap;

use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

use crate::entity::Entity;
use crate::sector::Sector;
pub use crate::sector::{HEIGHT, WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
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

#[derive(Debug)]
pub struct Map {
    pub sectors: HashMap<&'static str, Sector>,
    pub current_sector: Sector,
    //pub current_sector: &'static str,
}

impl Map {
    pub fn new(sectors: HashMap<&'static str, Sector>, start: &'static str) -> (Vec<Entity>, Self) {
        let current_sector = sectors
            .get(start)
            .expect("Tried to initialize map with invalid start sector")
            .clone();
        (
            current_sector.entities().to_vec(),
            Self {
                sectors,
                current_sector,
            },
        )
    }

    pub fn load(&mut self, id: &'static str) -> Vec<Entity> {
        self.current_sector = self
            .sectors
            .get(id)
            .expect("Found invalid sector identifier")
            .clone();

        self.current_sector.entities().to_vec()
    }

    pub fn save_entities(&mut self, id: &str, entities: Vec<Entity>) {
        self.sectors
            .get_mut(id)
            .expect("Found invalid sector identifier")
            .clone()
            .save_entities(entities);
    }

    pub fn sector(&self) -> &Sector {
        &self.current_sector
    }

    pub fn sector_mut(&mut self) -> &mut Sector {
        &mut self.current_sector
    }

    pub fn tiles(&self) -> &[[Tile; WIDTH]; HEIGHT] {
        self.sector().tiles()
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Tile> {
        self.sector().get(x, y)
    }

    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        self.sector_mut().set(x, y, tile)
    }

    pub fn draw(&self, mut x: u32, mut y: u32) {
        let ox = x;
        let mut dark = false;
        for row in self.tiles() {
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
        println!("\n{}/", "-".repeat(WIDTH * 2));
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

        let (r, g, b) = if dark_bg {
            self.kind.dark_faded_color()
        } else {
            self.kind.faded_color()
        };
        cod::color::tc_bg(r, g, b);

        cod::blit(format!("{0}{0}", self.kind as u8 as char), x, y);
    }
}
