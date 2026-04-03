use crate::Tile;

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map
}
