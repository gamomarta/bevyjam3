use crate::constants::ENEMY_SPEED;
use bevy::prelude::*;
use std::ops::Mul;
use std::time::Duration;

#[derive(Clone, Component)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec3::new(x, y, 0.0))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn y(&self) -> f32 {
        self.0.y
    }
    pub fn is_slow(&self) -> bool {
        self.0.length() < ENEMY_SPEED
    }
    pub fn is_fast(&self) -> bool {
        self.0.length() > ENEMY_SPEED
    }
    pub fn set_speed(&mut self, new_speed: f32) {
        self.0 *= new_speed / self.0.length()
    }
}

impl Mul<Duration> for &Velocity {
    type Output = Vec3;

    fn mul(self, duration: Duration) -> Self::Output {
        self.0 * duration.as_secs_f32()
    }
}
