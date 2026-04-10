use std::cmp;
use tcod::{BackgroundFlag, Color, Console};
use crate::game_object::Object;
use super::room::Rect;
use crate::Tcod;
use super::tile::Tile;

// colours of the map
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map
}

impl Game {
    pub fn draw_map(&mut self, tcod: &mut Tcod) {
        // go through all the tiles and set their background colour
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = &mut self.map[x as usize][y as usize];
                let is_visible = tcod.fov.is_in_fov(x, y);
                if is_visible {
                    tile.explore();
                }
                if !tile.is_explored() {
                    continue;
                }
                let is_wall = tile.blocks_sight();
                let color = Self::get_color(is_visible, is_wall);
                tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    fn get_color(is_visible: bool, is_wall: bool) -> Color {
        match (is_visible, is_wall) {
            // outside field-of-view
            (false, true) => COLOR_DARK_WALL,
            (false, false) => COLOR_DARK_GROUND,
            // inside field-of-view
            (true, true) => COLOR_LIGHT_WALL,
            (true, false) => COLOR_LIGHT_GROUND,
        }
    }

}

pub fn is_blocked(map: &Map, x: i32, y: i32, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocks() {
        return true;
    }

    objects.iter()
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

