use crate::difficulty::Difficulty;
use crate::entity::Entity;
use crate::map::{Direction, Tile, TileKind};

pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 16;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Sector {
    pub id: &'static str,
    tiles: [[Tile; WIDTH]; HEIGHT],
    entities: Vec<Entity>,
    neighbors: [Option<&'static str>; 4],
    changed: Vec<(u32, u32)>,
    pub difficulty: Difficulty,
    pub do_survival: bool,
    entrances: Vec<(u32, u32, &'static str)>,
    pub return_tile: Option<(u32, u32)>,
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
                '=' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Road,
                    }
                }
                '%' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Village,
                    }
                }
                '^' => {
                    tiles[y][x] = Tile {
                        kind: TileKind::Building,
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
            changed: Vec::new(),
            difficulty: Difficulty::normal(),
            do_survival: true,
            entrances: Vec::new(),
            return_tile: None,
        }
    }

    pub fn entrance(&mut self, x: u32, y: u32, id: &'static str) {
        self.entrances.push((x, y, id));
    }

    pub fn get_entrance(&self, x: u32, y: u32) -> Option<&'static str> {
        self.entrances.iter().find(|(tx, ty, _)| (*tx, *ty) == (x, y)).map(|(_, _, id)| *id)
    }

    pub fn town(
        map: &str,
        id: &'static str,
        entities: Vec<Entity>,
        outside: &'static str,
        return_x: u32,
        return_y: u32,
    ) -> Self {
        let mut s = Self::new(map, id, entities, [Some(outside); 4]);
        
        s.do_survival = false;
        s.difficulty = Difficulty::none();
        s.return_tile = Some((return_x, return_y));

        s
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

    pub fn despawn(&mut self, i: usize) {
        self.entities.remove(i);
    }

    pub fn despawn_id(&mut self, id: u32) {
        if let Some((i, _)) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, e)| e.id() == Some(id))
        {
            self.despawn(i);
        }
    }

    pub fn changed(&self) -> &[(u32, u32)] {
        &self.changed
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
        let ix = x as usize;
        let iy = y as usize;

        if ix >= WIDTH || iy >= HEIGHT {
            panic!("({x}, {y}) out of bounds ({WIDTH}, {HEIGHT})")
        }

        self.tiles[iy][ix] = tile;
        self.changed.push((x, y));
    }

    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }
}
