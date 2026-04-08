#[derive(Clone, Copy, Debug)]
pub struct Tile {
    blocked: bool,
    block_sight: bool,
    explored: bool
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
        }
    }

    pub fn blocks(&self) -> bool {
        self.blocked
    }

    pub fn blocks_sight(&self) -> bool {
        self.block_sight
    }

    pub fn is_explored(&self) -> bool {
        self.explored
    }

    pub fn explore(&mut self) {
        self.explored = true;
    }
}
