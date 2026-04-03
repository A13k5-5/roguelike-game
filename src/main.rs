mod game_map;
mod game_object;
mod controls;

use game_object::Object;

use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn render_all(tcod: &mut Tcod, game: &game_map::Game, objects: &[Object]) {
    // draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
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

    let con = Offscreen::new(game_map::MAP_WIDTH, game_map::MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    // create object representing the player
    let mut player = Object::new(0, 0, '@', WHITE);

    let game = game_map::Game {
        map: game_map::make_map(&mut player)
    };

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 -5, SCREEN_HEIGHT / 2 - 5, '@', GREEN);

    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        render_all(&mut tcod, &game, &objects);
        tcod.root.flush();

        let player = &mut objects[0];
        let exit = controls::handle_keys(&mut tcod, player, &game);
        if exit {
            break;
        }
    }
}
