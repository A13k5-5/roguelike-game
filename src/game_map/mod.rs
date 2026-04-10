mod dungeon_generator;
mod map;
mod room;
mod tile;

// reexport to hide the game_map implementation
pub use dungeon_generator::make_map;
pub use map::*;
