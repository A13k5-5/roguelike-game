use tcod::{BackgroundFlag, Color, Console};
use crate::Tcod;
use super::tile::Tile;

// colours of the map
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150
};

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map
}

impl Game {
    pub fn draw_map(&self, tcod: &mut Tcod) {
        // go through all the tiles and set their background colour
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if self.map[x as usize][y as usize].blocks_sight() {
                    tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
                } else {
                    tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
                }
            }
        }
    }
}
