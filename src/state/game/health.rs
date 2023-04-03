use crate::state::game::damage::Damage;
use bevy::prelude::*;
use std::ops::SubAssign;

#[derive(Component)]
pub struct Health(f32);

impl Health {
    pub fn new(value: f32) -> Self {
        Health(value)
    }
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.0
    }
}

impl SubAssign<&Damage> for Health {
    fn sub_assign(&mut self, damage: &Damage) {
        self.0 -= damage.value()
    }
}
