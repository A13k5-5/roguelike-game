use tcod::colors::{LIGHT_BLUE, RED};
use crate::game::Game;
use crate::game_object::Object;
use crate::item::use_result::UseResult;
use crate::{Tcod, PLAYER};

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

fn closest_monster(objects: &[Object], max_range: i32, tcod: &Tcod) -> Option<usize> {
    let mut closest_monster: Option<usize> = None;
    let mut closest_dist = (max_range + 1) as f32;

    for (id, object) in objects.iter().enumerate() {
        if id != PLAYER && object.fighter.is_some() && object.ai.is_some() && tcod.fov.is_in_fov(object.x, object.y) {
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                closest_monster = Some(id);
                closest_dist = dist;
            }
        }
    }

    closest_monster
}
