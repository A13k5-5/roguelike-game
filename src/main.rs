mod controls;
mod fov_map;
mod game_map;
mod game_object;
mod gui;
mod game;
mod item;

use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;
use controls::PlayerAction;
use game_object::Object;
use crate::gui::status_bar;
use crate::gui::message_log;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const PLAYER: usize = 0;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
    panel: Offscreen,
    fov: FovMap,
}

fn render_all(tcod: &mut Tcod, game: &game::Game, objects: &[Object]) {
    // draw all visible objects
    let mut to_draw: Vec<_> = objects.iter()
        .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
        .collect();
    // sort so that non-blocking objects come first
    to_draw.sort_by(|o1, o2| { o1.blocks().cmp(&o2.blocks()) });
    // draw all visible objects from the list
    for object in &to_draw {
        if tcod.fov.is_in_fov(object.x, object.y) {
            gui::object::draw(object, &mut tcod.con);
        }
    }

    gui::map::draw_map(&game.map, tcod);

    blit(
        &tcod.con,
        (0, 0),
        (game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();
    let hp = objects[PLAYER].fighter.map_or(0, |f| f.get_hp());
    let max_hp = objects[PLAYER].fighter.map_or(0, |f| f.get_max_hp());

    // show player's stats
    status_bar::render_bar(
        &mut tcod.panel,
        1, 1, BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARK_RED
    );

    message_log::draw_game_messages(&game.messages, &mut tcod.panel);

    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
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
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
    };

    // create object representing the player
    let player = Object::new_player();

    let mut objects = vec![player];

    let mut game = game::Game {
        map: game_map::make_map(&mut objects),
        messages: message_log::Messages::new(),
        inventory: vec![]
    };

    game.messages.add(
        "Welcome stranger! Prepare to perish in the Tombs of the Acient Kings.",
        RED
    );

    fov_map::populate_fov_map(&mut tcod.fov, &game.map);

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        tcod.con.clear();

        let player = &objects[PLAYER];
        // recompute field of view if position changed
        if previous_player_position != player.pos() {
            fov_map::explore_map(&mut tcod.fov, &mut game.map, player.x, player.y);
        }

        render_all(&mut tcod, &mut game, &objects);
        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();
        let player_action = controls::handle_keys(&mut tcod, &mut objects, &mut game);

        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].is_alive() && player_action == PlayerAction::TookTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    game_object::ai::ai_take_turn(id, &tcod, &mut game, &mut objects);
                }
            }
        }
    }
}
