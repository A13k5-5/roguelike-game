use crate::{game_map, Tcod, PLAYER};
use tcod::{colors, Color};
use tcod::colors::{VIOLET, WHITE};
use crate::game::Game;
use crate::gui::message_log::Messages;
use super::components;

// any game object
#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    name: String,
    blocks: bool,
    alive: bool,
    pub fighter: Option<components::Fighter>,
    pub ai: Option<components::Ai>
}

impl Object {
    fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object {
            x,
            y,
            char,
            color,
            name: name.into(),
            blocks,
            alive: false,
            fighter: None,
            ai: None
        }
    }

    pub fn new_player() -> Self {
        let mut player = Self::new(0, 0, '@', WHITE, "player", true);
        player.alive = true;
        player.fighter = Some(components::Fighter::new(
            30, 30, 2, 5, components::DeathCallback::Player
        ));
        player
    }

    pub fn new_troll(x: i32, y: i32) -> Self {
        let mut troll = Object::new(x, y, 'T', colors::DESATURATED_GREEN, "troll", true);
        troll.alive = true;
        troll.fighter = Some(components::Fighter::new(
            16, 16, 1, 4, components::DeathCallback::Monster
        ));
        troll.ai = Some(components::Ai::Basic);
        troll
    }

    pub fn new_orc(x: i32, y: i32) -> Self {
        let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "orc", true);
        orc.alive = true;
        orc.fighter = Some(components::Fighter::new(
            10, 10, 0, 3, components::DeathCallback::Monster
        ));
        orc.ai = Some(components::Ai::Basic);
        orc
    }

    pub fn new_healing_potion(x: i32, y: i32) -> Self {
        Object::new(x, y, '!', VIOLET, "healing potion", false)
    }

    pub fn set_pos(&mut self, point: (i32, i32)) {
        self.x = point.0;
        self.y = point.1;
    }

    pub fn set_char(&mut self, new_char: char) {
        self.char = new_char;
    }

    pub fn get_char(&self) -> char {
        self.char
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn blocks(&self) -> bool {
        self.blocks
    }

    pub fn set_blocks(&mut self, blocks: bool) {
        self.blocks = blocks;
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn die(&mut self) {
        self.alive = false;
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, messages: &mut Messages) {
        if let Some(fighter) = self.fighter.as_mut() {
            fighter.take_damage(damage);
        }

        // check for death, call the death function
        if let Some(fighter) = self.fighter {
            if fighter.get_hp() <= 0 {
                self.die();
                fighter.on_death.callback(self, messages);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, messages: &mut Messages) {
        let damage = self.fighter.map_or(0, |f| f.get_power()) - target.fighter.map_or(0, |f| f.get_defense());

        if damage > 0 {
            messages.add(format!("{} attacks {} for {} hit points.", self.name(), target.name(), damage),
                         WHITE
            );
        } else {
            messages.add(format!("{} attacks {}, but it has no effect!", self.name(), target.name()),
                         WHITE
            );
        }
        target.take_damage(damage, messages);
    }
}

// move by the given amount
pub fn move_by(id: usize, dx: i32, dy: i32, map: &game_map::Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();

    // if blocked wall, not possible to get there
    if game_map::is_blocked(map, x + dx, y + dy, objects) {
        return;
    }

    objects[id].set_pos((x + dx, y + dy));
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object
    let target_id = objects.iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, &mut game.messages);
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &game_map::Map, objects: &mut [Object]) {
    // vector from this object to the target distance
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize to length 1, then round it and convert to integer, so movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;

    // println!("The movement vector of {} is ({}, {})", objects[id].name, dx, dy);
    move_by(id, dx, dy, map, objects);
}

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.get_hp() > 0) {
            // close enough, attack!
            let (player, monster) = mut_two(PLAYER, monster_id, objects);
            monster.attack(player, &mut game.messages);
        }
    }
}

fn mut_two<T> (first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index < second_index);
    let (first_slice, second_slice) = items.split_at_mut(second_index);
    (&mut first_slice[first_index], &mut second_slice[0])
}
