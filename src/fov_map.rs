use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::game_map;

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGHT_WALL: bool = true;
pub const TORCH_RADIUS: i32 = 10;

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
