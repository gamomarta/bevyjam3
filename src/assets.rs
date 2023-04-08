use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>()
            .init_resource::<Materials>()
            .init_resource::<Fonts>()
            .init_resource::<Music>();
    }
}

#[derive(Default, Resource)]
pub struct Sprites {
    pub background: Handle<Image>,
    pub tower: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy: Handle<Image>,
    pub defeated_enemy: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct Materials {
    pub goal: Handle<ColorMaterial>,
    pub tower_range: Handle<ColorMaterial>,
    pub tower_invalid: Handle<ColorMaterial>,
}

#[derive(Default, Resource)]
pub struct Fonts {
    pub default_font: Handle<Font>,
}

#[derive(Default, Resource)]
pub struct Music {
    pub main_theme: Handle<AudioSource>,
}
