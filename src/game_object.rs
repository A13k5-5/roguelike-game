use tcod::{BackgroundFlag, Color, Console};
use crate::game_map;

// any game object
#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    name: String,
    blocks: bool,
    alive: bool
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object{
            x,
            y,
            char,
            color,
            name: name.into(),
            blocks,
            alive: false
        }
    }

    // move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &game_map::Game) {

        // if blocked wall, not possible to get there
        if game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocks() {
            return;
        }

        self.x += dx;
        self.y += dy;
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
}

