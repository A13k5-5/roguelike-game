use tcod::{BackgroundFlag, Console};
use tcod::console::Offscreen;
use crate::game_object::Object;

pub fn draw(object: &Object, con: &mut Offscreen) {
    con.set_default_foreground(object.get_color());
    con.put_char(object.x, object.y, object.get_char(), BackgroundFlag::None);
}