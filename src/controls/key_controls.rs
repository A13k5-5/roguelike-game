use super::actions::PlayerAction;
use crate::game_object::Object;
use crate::game_object::object;
use crate::{PLAYER, Tcod, game_map};

pub fn handle_keys(tcod: &mut Tcod, objects: &mut [Object], map: &game_map::Map) -> PlayerAction {
    use tcod::input::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].is_alive();

    match (key, key.text(), player_alive) {
        (
            Key {
                code: KeyCode::Enter,
                alt: true,
                ..
            },
            _,
            _,
        ) => {
            // alt+enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(fullscreen);
            return PlayerAction::DidntTakeTurn;
        }
        (
            Key {
                code: KeyCode::Escape,
                ..
            },
            _,
            _,
        ) => return PlayerAction::Exit, // exit the game

        // movement controls
        (
            Key {
                code: KeyCode::Up, ..
            },
            _,
            true,
        ) => object::player_more_or_attack(0, -1, map, objects),
        (
            Key {
                code: KeyCode::Down,
                ..
            },
            _,
            true,
        ) => object::player_more_or_attack(0, 1, map, objects),
        (
            Key {
                code: KeyCode::Left,
                ..
            },
            _,
            true,
        ) => object::player_more_or_attack(-1, 0, map, objects),
        (
            Key {
                code: KeyCode::Right,
                ..
            },
            _,
            true,
        ) => object::player_more_or_attack(1, 0, map, objects),

        _ => return PlayerAction::DidntTakeTurn,
    }

    // get here only on Up, Down, Left, Right keys
    PlayerAction::TookTurn
}
