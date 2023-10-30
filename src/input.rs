use crate::map::Map;
use crate::Direction;

use cod::Key;

pub fn handle(map: &mut Map) -> bool {
    if let Some(key) = cod::read::key() {
        !match key {
            Key::ArrowUp => map.go(Direction::Up),
            Key::ArrowDown => map.go(Direction::Down),
            Key::ArrowLeft => map.go(Direction::Left),
            Key::ArrowRight => map.go(Direction::Right),
            _ => false,
        }
    } else {
        false
    }
}
