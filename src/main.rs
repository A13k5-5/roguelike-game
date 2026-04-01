use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GOUND: Color = Color {
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

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

// any game object
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
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // set the colour and then draw the char of this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
    use tcod::input::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key { code: KeyCode::Up, .. } => player.move_by(0, -1),
        Key { code: KeyCode::Down, .. } => player.move_by(0, 1),
        Key { code: KeyCode::Left, .. } => player.move_by(-1, 0),
        Key { code: KeyCode::Right, .. } => player.move_by(1, 0),
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

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 -5, SCREEN_HEIGHT / 2 - 5, '@', GREEN);

    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );

        tcod.root.flush();


        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}
