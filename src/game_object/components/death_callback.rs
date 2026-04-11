use tcod::colors::{DARK_RED, ORANGE, RED};
use crate::game_object::Object;
use crate::gui::message_log::Messages;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object, messages: &mut Messages) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Messages) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, messages);
    }

}

fn player_death(player: &mut Object, messages: &mut Messages) {
    messages.add("You died!", RED);

    player.set_char('%');
    player.set_color(DARK_RED);
}

fn monster_death(monster: &mut Object, messages: &mut Messages) {
    messages.add(format!("{} is dead!", monster.name()), ORANGE);

    monster.set_char('%');
    monster.set_color(DARK_RED);
    monster.set_blocks(false);
    monster.set_name(format!("remains of {}", monster.name()));
    monster.fighter = None;
    monster.ai = None;
}
