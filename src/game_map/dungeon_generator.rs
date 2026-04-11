use super::map;
use super::room::Rect;
use super::tile::Tile;
use crate::game_map::{MAP_HEIGHT, MAP_WIDTH};
use crate::{Object, PLAYER, game_map};
use rand::RngExt;

// parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;
const MAX_ROOM_ITEMS: i32 = 2;

pub fn make_map(objects: &mut Vec<Object>) -> game_map::Map {
    // fill map with wall tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let rooms = generate_rooms();

    let player = &mut objects[PLAYER];
    // set player's initial position to centre of first room
    player.set_pos(rooms.first().unwrap().centre());

    // mark all the rooms in the map
    for r in &rooms[..] {
        map::create_room(r, &mut map);
    }

    // place monsters in each room
    for r in &rooms[..] {
        place_monsters(r, &map, objects);
    }

    // draw tunnels between rooms
    for i in 1..rooms.len() {
        let (cur_x, cur_y) = rooms.get(i).unwrap().centre();
        let (prev_x, prev_y) = rooms.get(i - 1).unwrap().centre();

        // horizontal and then vertical or the other way around
        if rand::random() {
            map::create_h_tunnel(cur_x, prev_x, cur_y, &mut map);
            map::create_v_tunnel(cur_y, prev_y, prev_x, &mut map);
        } else {
            map::create_v_tunnel(cur_y, prev_y, cur_x, &mut map);
            map::create_h_tunnel(cur_x, prev_x, prev_y, &mut map);
        }
    }

    map
}

fn generate_rooms() -> Vec<Rect> {
    let mut rooms: Vec<Rect> = Vec::new();

    for _ in 0..MAX_ROOMS {
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
            continue;
        }

        rooms.push(new_room);
    }

    rooms
}

fn place_monsters(room: &Rect, map: &game_map::Map, objects: &mut Vec<Object>) {
    place_objects(room, map, objects, MAX_ROOM_MONSTERS, &generate_monster);
}

/// General implementation function that randomly generates objects in a room.
/// The objects it generates are generated using the generation_function argument.
fn place_objects(room: &Rect, map: &game_map::Map, objects: &mut Vec<Object>, max_num_items: i32, generation_function: &dyn Fn(i32, i32) -> Object) {
    let num_objects = rand::rng().random_range(0..=max_num_items);

    for _ in 0..num_objects {
        let x = rand::rng().random_range(room.x1 + 1..room.x2);
        let y = rand::rng().random_range(room.y1 + 1..room.y2);

        if game_map::is_blocked(map, x, y, objects) {
            continue;
        }

        let object = generation_function(x, y);

        objects.push(object);
    }
}

fn generate_monster(x: i32, y: i32) -> Object {
    //80% change of generating an orc
    match rand::random::<f32>() < 0.8 {
        true => Object::new_orc(x, y),
        false => Object::new_troll(x, y),
    }
}

fn generate_item(x: i32, y: i32) -> Object {
    Object::new_healing_potion(x, y)
}

