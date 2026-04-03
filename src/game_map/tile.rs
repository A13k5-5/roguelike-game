#[derive(Clone, Copy, Debug)]
pub struct Tile {
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

    pub fn blocks(&self) -> bool {
        self.blocked
    }

    pub fn blocks_sight(&self) -> bool {
        self.block_sight
    }
}
