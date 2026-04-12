use super::actions::PlayerAction;
use crate::game_object::Object;
use crate::game_object::player_movement;
use crate::{PLAYER, Tcod};
use crate::game::Game;
use crate::gui::inventory;
use crate::item::pick_item_up;

pub fn handle_keys(tcod: &mut Tcod, objects: &mut Vec<Object>, game: &mut Game) -> PlayerAction {
    use tcod::input::{Key, KeyCode};

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].is_alive();

    match (key, key.printable, player_alive) {
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
        ) => player_movement::player_move_or_attack(0, -1, game, objects),
        (
            Key {
                code: KeyCode::Down,
                ..
            },
            _,
            true,
        ) => player_movement::player_move_or_attack(0, 1, game, objects),
        (
            Key {
                code: KeyCode::Left,
                ..
            },
            _,
            true,
        ) => player_movement::player_move_or_attack(-1, 0, game, objects),
        (
            Key {
                code: KeyCode::Right,
                ..
            },
            _,
            true,
        ) => player_movement::player_move_or_attack(1, 0, game, objects),

        // pick item
        (Key { code: KeyCode::Char, .. }, 'g', true) => {
            let item_id = objects.iter()
                .position(|object| object.item.is_some() && object.pos() == objects[PLAYER].pos());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            return PlayerAction::DidntTakeTurn;
        },

        // inventory
        (Key { code: KeyCode::Char, .. }, 'e', true) => {
            inventory::inventory_menu(
                &game.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root
            );
            return PlayerAction::DidntTakeTurn;
        }

        _ => return PlayerAction::DidntTakeTurn,
    }

    // get here only on Up, Down, Left, Right keys
    PlayerAction::TookTurn
}
