use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct DisappearTimer(Timer);

impl DisappearTimer {
    pub fn from_seconds(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Once))
    }
}

pub fn disappear(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut DisappearTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
