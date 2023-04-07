use bevy::prelude::*;
use std::ops::AddAssign;

#[derive(Clone, Component)]
pub struct Damage(f32);

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage(value)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl AddAssign<&Damage> for Damage {
    fn add_assign(&mut self, rhs: &Damage) {
        self.0 += rhs.0;
    }
}
