use rand::RngExt;
use crate::game_map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::{Object};
use super::tile::Tile;
use super::room::{Rect};
use super::room;

// parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

pub fn make_map(player: &mut Object) -> Map {
    // fill map with wall tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let rooms = generate_rooms();

    // set player's initial position to centre of first room
    let (new_x, new_y) = rooms.first().unwrap().centre();
    player.x = new_x;
    player.y = new_y;

    // draw all the rooms
    for r in &rooms[..] {
        room::create_room(r, &mut map);
    }

    // draw tunnels between rooms
    for i in 1..rooms.len() {
        let (cur_x, cur_y) = rooms.get(i).unwrap().centre();
        let (prev_x, prev_y) = rooms.get(i - 1).unwrap().centre();

        // horizontal and then vertical or the other way around
        if rand::random() {
            room::create_h_tunnel(cur_x, prev_x, cur_y, &mut map);
            room::create_v_tunnel(cur_y, prev_y, prev_x, &mut map);
        } else {
            room::create_v_tunnel(cur_y, prev_y, cur_x, &mut map);
            room::create_h_tunnel(cur_x, prev_x, prev_y, &mut map);
        }
    }

    map
}

fn generate_rooms() -> Vec<Rect> {

    let mut rooms: Vec<Rect> = Vec::new();

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
            continue;
        }

        rooms.push(new_room);
    }

    rooms
}
