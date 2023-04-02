use bevy::prelude::*;
use std::ops::Mul;
use std::time::Duration;

#[derive(Component)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec3::new(x, y, 0.0))
    }
}

impl Mul<Duration> for &Velocity {
    type Output = Vec3;

    fn mul(self, duration: Duration) -> Self::Output {
        self.0 * duration.as_secs_f32()
    }
}
