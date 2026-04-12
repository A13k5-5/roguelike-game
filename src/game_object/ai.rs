use crate::{game_map, Tcod, PLAYER};
use crate::game::Game;
use crate::game_object::Object;
use super::player_movement;
use super::util;

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

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
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
}

