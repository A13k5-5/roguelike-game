use crate::{game_map};
use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::game_map::{MAP_HEIGHT, MAP_WIDTH};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALL: bool = true;
const TORCH_RADIUS: i32 = 10;

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

fn recompute_fov(fov_map: &mut FovMap, player_x: i32, player_y: i32) {
    fov_map.compute_fov(
        player_x,
        player_y,
        TORCH_RADIUS,
        FOV_LIGHT_WALL,
        FOV_ALGO,
    );
}

pub fn explore_map(fov_map: &mut FovMap, map: &mut game_map::Map, player_x: i32, player_y: i32) {
    recompute_fov(fov_map, player_x, player_y);

    // mark all visible tiles as explored
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = &mut map[x as usize][y as usize];
            let is_visible = fov_map.is_in_fov(x, y);
            if is_visible {
                tile.explore();
            }
        }
    }
}
