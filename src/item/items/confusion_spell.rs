use crate::game::Game;
use crate::game_object::components::Ai;
use crate::game_object::Object;
use crate::item::items::utils;
use crate::item::use_result::UseResult;
use crate::Tcod;

const CONFUSE_RANGE: i32 = 8;
const CONFUSE_DURATION: i32 = 10; // turns

pub fn cast_confusion(_inventory_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    let monster_id = utils::closest_monster(objects, CONFUSE_RANGE, tcod);
    match monster_id {
        Some(monster_id) => {
            let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
            objects[monster_id].ai = Some(Ai::Confused {
                previous_ai: Box::new(old_ai),
                num_turns: CONFUSE_DURATION
            });
            UseResult::UsedUp
        },
        None => {
            UseResult::Cancelled
        }
    }
}
