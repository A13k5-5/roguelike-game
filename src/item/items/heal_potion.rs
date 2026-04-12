use tcod::colors::{LIGHT_VIOLET, RED};
use crate::game::Game;
use crate::game_object::Object;
use crate::item::use_result::UseResult;
use crate::PLAYER;

const HEAL_AMOUNT: i32 = 4;

pub fn cast_heal(_inventory_id: usize, game: &mut Game, objects: &mut [Object]) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.has_full_hp() {
            game.messages.add("You are already at full health.", RED);
            return UseResult::Cancelled;
        }
        game.messages.add("Your wounds start to feel better!", LIGHT_VIOLET);
        objects[PLAYER].heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}
