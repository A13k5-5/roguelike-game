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
