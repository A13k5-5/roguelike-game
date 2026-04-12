use tcod::colors::{LIGHT_BLUE, RED};
use crate::game::Game;
use crate::game_object::Object;
use crate::item::use_result::UseResult;
use crate::Tcod;
use super::utils::closest_monster;

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;

pub fn cast_lightning(_inventory_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    let monster_id = closest_monster(objects, LIGHTNING_RANGE, tcod);
    match monster_id {
        Some(monster_id) => {
            let monster_to_hit = &mut objects[monster_id];
            game.messages.add(
                format!(
                    "A lightning bolt strikes the {} with a loud thunder! \
                    The damage is {} hit points", monster_to_hit.name(), LIGHTNING_DAMAGE
                ), LIGHT_BLUE
            );
            monster_to_hit.take_damage(LIGHTNING_DAMAGE, &mut game.messages);
            UseResult::UsedUp
        },
        None => {
            // no monster found
            game.messages.add("No enemy is close enough to strike.", RED);
            UseResult::Cancelled
        }
    }
}

