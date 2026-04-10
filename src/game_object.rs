use crate::{game_map, PLAYER};
use tcod::{BackgroundFlag, Color, Console};

// any game object
#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    name: String,
    blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object {
            x,
            y,
            char,
            color,
            name: name.into(),
            blocks,
            alive: false,
        }
    }

    // set the colour and then draw the char of this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn set_pos(&mut self, point: (i32, i32)) {
        self.x = point.0;
        self.y = point.1;
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn blocks(&self) -> bool {
        self.blocks
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
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

pub fn player_more_or_attack(dx: i32, dy: i32, map: &game_map::Map, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object
    let target_id = objects.iter().position(|object| object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            println!(
                "The {} laughs at your puny efforts to attack him!",
                objects[target_id].name()
            )
        }
        None => {
            move_by(PLAYER, dx, dy, &map, objects);
        }
    }
}
