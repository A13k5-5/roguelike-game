mod game_map;
mod game_object;
mod controls;
mod fov_map;

use game_object::Object;

use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;
use crate::fov_map::{FOV_ALGO, FOV_LIGHT_WALL};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

fn render_all(tcod: &mut Tcod, game: &mut game_map::Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[0];
        tcod.fov.compute_fov(player.x, player.y, fov_map::TORCH_RADIUS, FOV_LIGHT_WALL, FOV_ALGO);
    }

    // draw all visible objects from the list
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    game.draw_map(tcod);

    blit(
        &tcod.con,
        (0, 0),
        (game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0
    );
}


fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
        fov: FovMap::new(game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
    };

    // create object representing the player
    let mut player = Object::new(0, 0, '@', WHITE);

    let mut game = game_map::Game {
        map: game_map::make_map(&mut player)
    };

    fov_map::populate_fov_map(&mut tcod.fov, &game.map);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 -5, SCREEN_HEIGHT / 2 - 5, '@', GREEN);

    let mut objects = [player, npc];

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        tcod.con.clear();
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &mut game, &objects, fov_recompute);
        tcod.root.flush();

        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = controls::handle_keys(&mut tcod, player, &game);
        if exit {
            break;
        }
    }
}
