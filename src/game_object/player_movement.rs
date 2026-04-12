use crate::game::Game;
use crate::{game_map, PLAYER};
use crate::game_object::{util, Object};

// move by the given amount
pub fn move_by(id: usize, dx: i32, dy: i32, map: &game_map::Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();

    // if blocked wall, not possible to get there
    if game_map::is_blocked(map, x + dx, y + dy, objects) {
        return;
    }

    objects[id].set_pos((x + dx, y + dy));
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object
    let target_id = objects.iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = util::mut_two(PLAYER, target_id, objects);
            player.attack(target, &mut game.messages);
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}

