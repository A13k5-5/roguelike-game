mod controls;
mod fov_map;
mod game_map;
mod game_object;

use crate::fov_map::{FOV_ALGO, FOV_LIGHT_WALL};
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;
use controls::PlayerAction;
use game_object::Object;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const PLAYER: usize = 0;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

fn render_all(tcod: &mut Tcod, game: &mut game_map::Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[PLAYER];
        tcod.fov.compute_fov(
            player.x,
            player.y,
            fov_map::TORCH_RADIUS,
            FOV_LIGHT_WALL,
            FOV_ALGO,
        );
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
        1.0,
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
    let mut player = Object::new(0, 0, '@', WHITE, "player", true);
    player.alive = true;

    let mut objects = vec![player];

    let mut game = game_map::Game {
        map: game_map::make_map(&mut objects),
    };

    fov_map::populate_fov_map(&mut tcod.fov, &game.map);

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        tcod.con.clear();
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &mut game, &objects, fov_recompute);
        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();
        let player_action = controls::handle_keys(&mut tcod, &mut objects, &game.map);

        if player_action == PlayerAction::Exit {
            break;
        }

        // if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
        //     for object in &objects {
        //         // all objects but the player themself
        //         if (object as *const _) != (&objects[PLAYER] as *const _) {
        //             println!("The {} growls", object.name())
        //         }
        //     }
        // }
    }
}
