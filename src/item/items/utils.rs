use crate::game_object::Object;
use crate::{Tcod, PLAYER};

pub fn closest_monster(objects: &[Object], max_range: i32, tcod: &Tcod) -> Option<usize> {
    let mut closest_monster: Option<usize> = None;
    let mut closest_dist = max_range as f32;

    for (id, object) in objects.iter().enumerate() {
        if id != PLAYER && object.fighter.is_some() && object.ai.is_some() && tcod.fov.is_in_fov(object.x, object.y) {
            let dist = objects[PLAYER].distance_to(object);
            if dist <= closest_dist {
                closest_monster = Some(id);
                closest_dist = dist;
            }
        }
    }

    closest_monster
}
