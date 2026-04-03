pub mod tile;
pub mod map;
pub mod room;
mod dungeon_generator;

// reexport to hide the game_map implementation
pub use map::*;
pub use dungeon_generator::make_map;