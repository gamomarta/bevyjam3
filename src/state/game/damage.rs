use bevy::prelude::*;

#[derive(Component)]
pub struct Damage(f32);

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage(value)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
}
