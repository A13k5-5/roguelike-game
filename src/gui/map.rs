use tcod::{BackgroundFlag, Color, Console};
use crate::game_map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::Tcod;


// colours of the map
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
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


pub fn draw_map(map: &mut Map, tcod: &mut Tcod) {
    // go through all the tiles and set their background colour
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = &mut map[x as usize][y as usize];
            let is_visible = tcod.fov.is_in_fov(x, y);
            if is_visible {
                tile.explore();
            }
            if !tile.is_explored() {
                continue;
            }
            let is_wall = tile.blocks_sight();
            let color = get_color(is_visible, is_wall);
            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
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
