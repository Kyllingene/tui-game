use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::world::World;
use crate::map::Tile;
use crate::player::Player;

#[derive(Serialize, Deserialize)]
struct SaveData {
    player: Player,
    current_sector: String,
    tile_changes: HashMap<String, Vec<(u32, u32, Tile)>>,
    despawned: Vec<(String, u32)>,
}

impl SaveData {
    pub fn from(world: &World) -> Self {
        let mut tile_changes = HashMap::new();
        let mut tiles = Vec::new();
        for (id, sector) in &world.map.sectors {
            tiles.clear();
            for (x, y) in sector.changed() {
                let x = *x;
                let y = *y;

                let tile = world.map.get(x, y).unwrap();
                tiles.push((x, y, tile));
            }

            tile_changes.insert(id.to_string(), std::mem::take(&mut tiles));
        }

        let despawned = world.despawned.iter()
            .map(|(s, i)| (s.to_string(), *i))
            .collect();

        Self {
            player: world.player.clone(),
            current_sector: world.map.current_sector.id.to_string(),
            tile_changes,
            despawned,
        }
    }

    pub fn apply(self, world: &mut World) {
        world.map.load(&self.current_sector);
        world.entities.clear();
        world.player = self.player;

        for (sector, changes) in self.tile_changes {
            world.map.get_sector_mut(&sector).map(|sector| {
                for (x, y, tile) in changes {
                    sector.set(x, y, tile);
                }
            });
        }

        for (sector, id) in self.despawned {
            world.map.get_sector_mut(&sector).map(|sector| sector.despawn(id));
        }
    }
}

pub fn save_to<S: AsRef<Path>>(file: S, world: &World) {
    let data = SaveData::from(world);
    
    let json = serde_json::to_string(&data).expect("Failed to serialize save data");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)
        .expect("Failed to open save file");

    file.write_all(json.as_bytes()).expect("Failed to write to save file");
    file.flush().expect("Failed to write to save file");
}

// TODO: use a better save location
// TODO: use zstd
pub fn save(world: &World) {
    save_to("frob-save.json", world);
}

pub fn load_from<S: AsRef<Path>>(file: S, world: &mut World) {
    let file = BufReader::new(File::open(file).expect("Failed to open save file"));
    let data: SaveData = serde_json::from_reader(file).expect("Failed to parse save file");
    data.apply(world);
}

pub fn load(world: &mut World) {
    load_from("frob-save.json", world);
}

