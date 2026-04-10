use super::actions::PlayerAction;
use crate::game_object::Object;
use crate::{PLAYER, Tcod, game_map, game_object};

pub fn handle_keys(tcod: &mut Tcod, objects: &mut [Object], map: &game_map::Map) -> PlayerAction {
    use tcod::input::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;

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
        ) => game_object::move_by(PLAYER, 0, -1, map, objects),
        (
            Key {
                code: KeyCode::Down,
                ..
            },
            _,
            true,
        ) => game_object::move_by(PLAYER, 0, 1, map, objects),
        (
            Key {
                code: KeyCode::Left,
                ..
            },
            _,
            true,
        ) => game_object::move_by(PLAYER, -1, 0, map, objects),
        (
            Key {
                code: KeyCode::Right,
                ..
            },
            _,
            true,
        ) => game_object::move_by(PLAYER, 1, 0, map, objects),

        _ => return PlayerAction::DidntTakeTurn,
    }

    // get here only on Up, Down, Left, Right keys
    PlayerAction::TookTurn
}
