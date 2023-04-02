use bevy::prelude::*;

pub struct Assets;

impl Plugin for Assets {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>();
    }
}

#[derive(Default, Resource)]
pub struct Sprites {
    pub bevy_logo: Handle<Image>,
    pub enemy: Handle<Image>,
}
