use rand::RngExt;
use tcod::colors;
use crate::game_map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::{Object};
use super::tile::Tile;
use super::room::{Rect};
use super::map;

// parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;

pub fn make_map(objects: &mut Vec<Object>) -> Map {
    // fill map with wall tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let rooms = generate_rooms();

    let player = &mut objects[0];
    // set player's initial position to centre of first room
    let (new_x, new_y) = rooms.first().unwrap().centre();
    player.x = new_x;
    player.y = new_y;

    // mark all the rooms in the map
    for r in &rooms[..] {
        map::create_room(r, &mut map);
    }

    for r in &rooms[..] {
        place_objects(r, objects);
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

fn place_objects(room: &Rect, objects: &mut Vec<Object>) {
    let num_monsters = rand::rng().random_range(0..=MAX_ROOM_MONSTERS);

    for _ in 0..num_monsters {
        let x = rand::rng().random_range(room.x1 + 1..room.x2);
        let y = rand::rng().random_range(room.y1 + 1..room.y2);

        //80% change of generating an orc
        let monster = match rand::random::<f32>() < 0.8 {
            true => {
                Object::new(x, y, 'o', colors::DESATURATED_GREEN)
            },
            false => {
                Object::new(x, y, 'T', colors::DARKER_GREEN)
            }
        };

        objects.push(monster);
    }
}
