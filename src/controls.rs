use crate::game_object::Object;
use crate::{game_map, Tcod, PLAYER, game_object};

pub fn handle_keys(tcod: &mut Tcod, objects: &mut [Object], game: &game_map::Game) -> bool {
    use tcod::input::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {
            code: KeyCode::Enter,
            alt: true,
            ..
        } => {
            // alt+enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(fullscreen);
        }
        Key { code: KeyCode::Escape, .. } => return true, // exit the game

        // movement controls
        Key { code: KeyCode::Up, .. } => game_object::move_by(PLAYER, 0, -1, game, objects),
        Key { code: KeyCode::Down, .. } => game_object::move_by(PLAYER, 0, 1, game, objects),
        Key { code: KeyCode::Left, .. } => game_object::move_by(PLAYER, -1, 0, game, objects),
        Key { code: KeyCode::Right, .. } => game_object::move_by(PLAYER, 1, 0, game, objects),

        _ => {}
    }

    false
}
