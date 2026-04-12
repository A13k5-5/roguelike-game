use rand::RngExt;
use tcod::colors::RED;
use crate::{game_map, Tcod, PLAYER};
use crate::game::Game;
use crate::game_object::Object;
use crate::game_object::player_movement::move_by;
use super::player_movement;
use super::util;
pub use super::components::Ai;

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &game_map::Map, objects: &mut [Object]) {
    // vector from this object to the target distance
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize to length 1, then round it and convert to integer, so movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;

    // println!("The movement vector of {} is ({}, {})", objects[id].name, dx, dy);
    player_movement::move_by(id, dx, dy, map, objects);
}

/// this method assumes that the object at monster_id is a monster, i.e. it has the ai attribute
pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    assert!(objects[monster_id].ai.is_some());
    let new_ai = match objects[monster_id].ai.take().unwrap() {
        Ai::Basic => ai_basic(monster_id, tcod, game, objects),
        Ai::Confused { previous_ai, num_turns } => {
            ai_confused(monster_id, tcod, game, objects, previous_ai, num_turns)
        }
    };
    objects[monster_id].ai = Some(new_ai);
}

pub fn ai_basic(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) ->  Ai {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.get_hp() > 0) {
            // close enough, attack!
            let (player, monster) = util::mut_two(PLAYER, monster_id, objects);
            monster.attack(player, &mut game.messages);
        }
    }
    Ai::Basic
}

pub fn ai_confused(monster_id: usize, _tcod: &Tcod, game: &mut Game, objects: &mut [Object], previous_ai: Box<Ai>, num_turns: i32) -> Ai {
    if num_turns >= 0 {
        // still confused
        // move in the random direction and decrease the counter
        move_by(
            monster_id,
            rand::rng().random_range(-1..=1),
            rand::rng().random_range(-1..=1),
            &game.map,
            objects
        );
        Ai::Confused {
            previous_ai,
            num_turns: num_turns - 1
        }
    } else {
        // restore the previous AI
        game.messages.add(
            format!("The {} is not longer confused!", objects[monster_id].name()), RED
        );
        *previous_ai
    }
}
