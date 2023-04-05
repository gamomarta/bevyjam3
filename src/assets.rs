use bevy::prelude::*;

pub struct Assets;

impl Plugin for Assets {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>()
            .init_resource::<Fonts>()
            .init_resource::<Music>();
    }
}

#[derive(Default, Resource)]
pub struct Sprites {
    pub bevy_logo: Handle<Image>,
    pub tower: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy: Handle<Image>,
    pub goal: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct Fonts {
    pub default_font: Handle<Font>,
}

#[derive(Default, Resource)]
pub struct Music {
    pub main_theme: Handle<AudioSource>,
}
