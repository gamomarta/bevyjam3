use crate::state::game::damage::Damage;
use bevy::prelude::*;
use std::fmt::Formatter;
use std::ops::SubAssign;

#[derive(Clone, Component, Debug)]
pub struct Health(f32);

impl Health {
    pub fn new(value: f32) -> Self {
        Health(value)
    }
    pub fn die(&mut self) {
        self.0 = 0.0;
    }
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.0
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubAssign<&Damage> for Health {
    fn sub_assign(&mut self, damage: &Damage) {
        self.0 -= damage.value()
    }
}
