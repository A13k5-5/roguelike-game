use tcod::Color;
use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::game_map;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALL: bool = true;
const TORCH_RADIUS: i32 = 10;

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

pub fn populate_fov_map(fov_map: &mut FovMap, map: &game_map::Map) {
    for y in 0..game_map::MAP_HEIGHT {
        for x in 0..game_map::MAP_WIDTH {
            fov_map.set(
                x,
                y,
                !map[x as usize][y as usize].blocks_sight(),
                !map[x as usize][y as usize].blocks(),
            )
        }
    }
}
