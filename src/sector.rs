use crate::entity::Entity;
use crate::map::{Direction, Tile, TileKind};

pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sector {
    pub id: &'static str,
    tiles: [[Tile; WIDTH]; HEIGHT],
    entities: Vec<Entity>,
    neighbors: [Option<&'static str>; 4],
}

impl Sector {
    pub fn new(
        map: &str,
        id: &'static str,
        entities: Vec<Entity>,
        neighbors: [Option<&'static str>; 4],
    ) -> Self {
        let mut tiles = [[Tile::default(); WIDTH]; HEIGHT];
        let mut x = 0;
        let mut y = 0;
        for ch in map.trim().chars() {
            match ch {
                '~' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Water,
                    }
                }
                '_' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Grass,
                    }
                }
                '$' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Forest,
                    }
                }
                'n' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Hill,
                    }
                }
                'A' => {
                    tiles[y][x] = Tile {
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

        Sector {
            id,
            tiles,
            entities,
            neighbors,
        }
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    #[allow(dead_code)]
    pub fn entities_mut(&mut self) -> &mut [Entity] {
        &mut self.entities
    }

    pub fn save_entities(&mut self, entities: Vec<Entity>) {
        self.entities = entities;
    }

    #[allow(dead_code)]
    pub fn add_neighbor(&mut self, direction: Direction, neighbor: &'static str) {
        self.neighbors[direction as usize] = Some(neighbor);
    }

    pub fn neighbor(&self, direction: Direction) -> Option<&'static str> {
        self.neighbors[direction as usize]
    }

    pub fn tiles(&self) -> &[[Tile; WIDTH]; HEIGHT] {
        &self.tiles
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Tile> {
        self.tiles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }

    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let x = x as usize;
        let y = y as usize;

        if x >= WIDTH || y >= HEIGHT {
            panic!("({x}, {y}) out of bounds ({WIDTH}, {HEIGHT})")
        }
        self.tiles[y][x] = tile;
    }
}
