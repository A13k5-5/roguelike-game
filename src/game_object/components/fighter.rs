#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32
}

impl Fighter {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32) -> Self {
        Fighter {
            max_hp,
            hp,
            defense,
            power
        }
    }
}