use crate::game_object::Object;
use crate::{game_map, Tcod};

pub fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &game_map::Game) -> bool {
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
        Key { code: KeyCode::Up, .. } => player.move_by(0, -1, game),
        Key { code: KeyCode::Down, .. } => player.move_by(0, 1, game),
        Key { code: KeyCode::Left, .. } => player.move_by(-1, 0, game),
        Key { code: KeyCode::Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }

    false
}
