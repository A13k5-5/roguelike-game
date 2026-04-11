use tcod::colors::DARK_RED;
use crate::game_object::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object) {
        use DeathCallback::*;
        let callback: fn(&mut Object) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object);
    }

}

fn player_death(player: &mut Object) {
    println!("You died!");
    player.set_char('%');
    player.set_color(DARK_RED);
}

fn monster_death(monster: &mut Object) {
    println!("{} is dead!", monster.name());
    monster.set_char('%');
    monster.set_color(DARK_RED);
    monster.set_blocks(false);
    monster.set_name(format!("remains of {}", monster.name()));
    monster.fighter = None;
    monster.ai = None;
}
