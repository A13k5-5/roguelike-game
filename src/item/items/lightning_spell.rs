use crate::game::Game;
use crate::game_object::Object;
use crate::item::use_result::UseResult;

pub fn cast_lightning(_inventory_id: usize, game: &mut Game, objects: &mut [Object]) -> UseResult {
    UseResult::Cancelled
}
