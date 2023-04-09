use crate::constants::ENEMY_DAMAGE;
use bevy::prelude::*;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign};

#[derive(Clone, Component)]
pub struct Damage(f32);

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage(value)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
    pub fn is_increased(&self) -> bool {
        self.0 > ENEMY_DAMAGE
    }
}

impl std::fmt::Display for Damage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", -self.0)
    }
}

impl Add<&Damage> for &Damage {
    type Output = Damage;

    fn add(self, rhs: &Damage) -> Self::Output {
        Damage(self.0 + rhs.0)
    }
}

impl AddAssign<&Damage> for Damage {
    fn add_assign(&mut self, rhs: &Damage) {
        self.0 += rhs.0;
    }
}
