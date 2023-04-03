use bevy::prelude::*;

#[derive(Component)]
pub struct Tower;

#[derive(Component, Deref, DerefMut)]
pub struct ShootRadius(pub f32);

#[derive(Component)]
pub struct ShootRadiusImage;
