use tcod::colors::{GREEN, LIGHT_VIOLET, RED, WHITE};
use crate::game::Game;
use crate::game_object::Object;
use crate::item::use_result::UseResult;
use crate::PLAYER;

const HEAL_AMOUNT: i32 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    LightningSpell
}

pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name()
            ), RED
        );
        return;
    }

    let item = objects.swap_remove(object_id);
    game.messages.add(
        format!("You picked up a {}!", item.name()), GREEN
    );
    game.inventory.push(item);
}

pub fn use_item(inventory_id: usize, game: &mut Game, objects: &mut [Object]) {
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Item::Heal => cast_heal,
            Item::LightningSpell => cast_lightning
        };

        match on_use(inventory_id, game, objects) {
            UseResult::UsedUp => {
                // destroy after use
                game.inventory.remove(inventory_id);
            },
            UseResult::Cancelled => {
                game.messages.add("Cancelled", WHITE);
            }
        }
    } else {
        game.messages.add(
            format!("The {} cannot be used", game.inventory[inventory_id].name()),
            WHITE
        );
    }
}

fn cast_heal(_inventory_id: usize, game: &mut Game, objects: &mut [Object]) -> UseResult {
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

fn cast_lightning(_inventory_id: usize, game: &mut Game, objects: &mut [Object]) -> UseResult {
    UseResult::Cancelled
}
