use tcod::colors::{GREEN, RED, WHITE};
use crate::game::Game;
use crate::game_object::Object;
use crate::item::items;
use crate::item::use_result::UseResult;
use crate::Tcod;

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

pub fn use_item(inventory_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Item::Heal => items::heal_potion::cast_heal,
            Item::LightningSpell => items::lightning_spell::cast_lightning
        };

        match on_use(inventory_id, tcod, game, objects) {
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
