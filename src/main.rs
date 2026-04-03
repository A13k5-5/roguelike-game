use rand::prelude::*;
use std::cmp;
use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

mod game_map {
    use rand::RngExt;
    use crate::{create_h_tunnel, create_room, create_v_tunnel, Object, Rect, Tile, MAX_ROOMS, ROOM_MAX_SIZE, ROOM_MIN_SIZE};

    // size of the map
    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;

    pub type Map = Vec<Vec<Tile>>;

    pub struct Game {
        pub map: Map
    }

    pub fn make_map(player: &mut Object) -> Map {
        // fill map with wall tiles
        let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        let mut rooms = vec![];

        for _ in 0..MAX_ROOMS {
            // for _ in 0..5 {
            // random size
            let w = rand::rng().random_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
            let h = rand::rng().random_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);

            // random position
            let x = rand::rng().random_range(0..MAP_WIDTH - w);
            let y = rand::rng().random_range(0..MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);

            let failed = rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if failed {
                continue
            }

            let (new_x, new_y) = new_room.centre();
            create_room(new_room, &mut map);

            match rooms.is_empty() {
                true => {
                    // this is the first room, where the player is spawned
                    player.x = new_x;
                    player.y = new_y;
                }
                false => {
                    // all the other rooms
                    // connect it to the previous room with a tunnel

                    let (prev_x, prev_y) = rooms.last().unwrap().centre();

                    // horizontal and then vertical or the other way around
                    if rand::random() {
                        create_h_tunnel(new_x, prev_x, new_y, &mut map);
                        create_v_tunnel(new_y, prev_y, prev_x, &mut map);
                    } else {
                        create_v_tunnel(new_y, prev_y, new_x, &mut map);
                        create_h_tunnel(new_x, prev_x, prev_y, &mut map);
                    }
                }
            }

            rooms.push(new_room);
        }

        map
    }

}

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150
};

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true
        }
    }
}

/// A recrangle on a map - used to characterise a room.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn centre(&self) -> (i32, i32) {
        let centre_x = (self.x1 + self.x2) / 2;
        let centre_y = (self.y1 + self.y2) / 2;
        (centre_x, centre_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

fn create_room(room: Rect, map: &mut game_map::Map) {
    // go through the tiles in the rectangle, and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut game_map::Map) {
    // horizontal tunnel
    for x in cmp::min(x1, x2)..cmp::max(x1, x2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut game_map::Map) {
    // horizontal tunnel
    for y in cmp::min(y1, y2)..cmp::max(y1, y2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

// any game object
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object{ x, y, char, color }
    }

    // move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &game_map::Game) {

        // if blocked wall, not possible to get there
        if game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
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
}

fn render_all(tcod: &mut Tcod, game: &game_map::Game, objects: &[Object]) {
    // draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
    }

    // go through all the tiles and set their background colour
    for y in 0..game_map::MAP_HEIGHT {
        for x in 0..game_map::MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (game_map::MAP_WIDTH, game_map::MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0
    );
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &game_map::Game) -> bool {
    use tcod::input::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {
            code: KeyCode::Enter,
            alt: true,
            ..
        } => {
            // alt+enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(fullscreen);
        }
        Key { code: KeyCode::Escape, .. } => return true, // exit the game

        // movement controls
        Key { code: KeyCode::Up, .. } => player.move_by(0, -1, game),
        Key { code: KeyCode::Down, .. } => player.move_by(0, 1, game),
        Key { code: KeyCode::Left, .. } => player.move_by(-1, 0, game),
        Key { code: KeyCode::Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(game_map::MAP_WIDTH, game_map::MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    // create object representing the player
    let mut player = Object::new(0, 0, '@', WHITE);

    let game = game_map::Game {
        map: game_map::make_map(&mut player)
    };

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 -5, SCREEN_HEIGHT / 2 - 5, '@', GREEN);

    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player, &game);
        if exit {
            break;
        }
    }
}
