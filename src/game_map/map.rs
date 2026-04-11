use super::room::Rect;
use super::tile::Tile;
use crate::game_object::Object;
use std::cmp;
use crate::gui::message_log;

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
    pub messages: message_log::Messages,
}

pub fn is_blocked(map: &Map, x: i32, y: i32, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocks() {
        return true;
    }

    objects
        .iter()
        .any(|object| object.blocks() && object.pos() == (x, y))
}

pub fn create_room(room: &Rect, map: &mut Map) {
    // go through the tiles in the rectangle, and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel
    for x in cmp::min(x1, x2)..cmp::max(x1, x2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // horizontal tunnel
    for y in cmp::min(y1, y2)..cmp::max(y1, y2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
