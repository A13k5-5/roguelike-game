use crate::game_map::Map;
use crate::game_object::Object;
use crate::gui::message_log;

pub struct Game {
    pub map: Map,
    pub messages: message_log::Messages,
    pub inventory: Vec<Object>,
}
