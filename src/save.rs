use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zstd::stream::Decoder;

use crate::map::Tile;
use crate::player::Player;
use crate::sector::HEIGHT;
use crate::world::World;

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

                let tile = sector.get(x, y).unwrap();
                tiles.push((x, y, tile));
            }

            tile_changes.insert(id.to_string(), std::mem::take(&mut tiles));
        }

        tiles.clear();
        for (x, y) in world.map.sector().changed() {
            let x = *x;
            let y = *y;

            let tile = world.map.get(x, y).unwrap();
            tiles.push((x, y, tile));
        }

        tile_changes.insert(world.map.sector().id.to_string(), tiles);

        let despawned = world
            .despawned
            .iter()
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
        world.player = self.player;

        for (sector, changes) in self.tile_changes {
            if let Some(sector) = world.map.get_sector_mut(&sector) {
                for (x, y, tile) in changes {
                    sector.set(x, y, tile);
                }
            }
        }

        for (sector, id) in self.despawned {
            if let Some(sector) = world.map.get_sector_mut(&sector) {
                sector.despawn_id(id);
                world.despawned.push((sector.id, id));
            }
        }

        world.entities = world.map.sector().entities().to_vec();
    }
}

pub fn save_to<S: AsRef<Path>>(file: S, world: &World) -> bool {
    let data = SaveData::from(world);

    let json = serde_json::to_string(&data).expect("Failed to serialize save data");

    let Ok(file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)
    else {
        world.draw_message("Save not found", 1);
        cod::read::key();
        return false;
    };
    let mut file = BufWriter::new(file);
    zstd::stream::copy_encode(json.as_bytes(), &mut file, 0).expect("Failed to write to save file");

    file.flush().expect("Failed to write to save file");
    true
}

pub fn save(world: &World) -> bool {
    if let Some(slot) = get_slot() {
        let mut dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("./"));
        dir.push("frob-adventure");
        fs::create_dir_all(&dir).expect("Failed to create save directory");
        dir.push(format!("frob-save-{slot}.json.zst"));
        save_to(dir, world)
    } else {
        false
    }
}

fn get_slot() -> Option<String> {
    cod::goto::pos(0, HEIGHT as u32);
    cod::clear::line();
    cod::color::de_bg();
    cod::color::fg(2);
    print!("Save slot: ");
    cod::color::de_fg();
    cod::flush();
    cod::read::line()
}

pub fn load_from<S: AsRef<Path>>(file: S, world: &mut World) -> bool {
    let Ok(file) = File::open(file) else {
        world.draw_message("Save not found", 1);
        cod::read::key();
        return false;
    };
    let file = BufReader::new(file);
    let stream = Decoder::new(file).expect("Failed to create decoder");
    let data: SaveData = serde_json::from_reader(stream).expect("Failed to parse save file");
    data.apply(world);
    true
}

pub fn load(world: &mut World) -> bool {
    if let Some(slot) = get_slot() {
        let mut dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("./"));
        dir.push("frob-adventure");
        fs::create_dir_all(&dir).expect("Failed to create save directory");
        dir.push(format!("frob-save-{slot}.json.zst"));
        load_from(dir, world)
    } else {
        cod::read::key();
        false
    }
}
