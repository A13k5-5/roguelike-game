use super::death_callback::DeathCallback;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32,
    pub on_death: DeathCallback
}

impl Fighter {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32, on_death: DeathCallback) -> Self {
        Fighter {
            max_hp,
            hp,
            defense,
            power,
            on_death
        }
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }

    pub fn get_max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn take_damage(&mut self, damage: i32) {
        if damage > 0 {
            self.hp -= damage;
        }
    }

    pub fn get_power(&self) -> i32 {
        self.power
    }

    pub fn get_defense(&self) -> i32 {
        self.defense
    }
}