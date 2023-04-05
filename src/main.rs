#![allow(
    clippy::redundant_closure_call,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

use bevy::prelude::*;

mod assets;
mod constants;
mod state;
mod utils;

fn main() {
    App::new()
        .add_plugin(assets::Assets)
        .add_plugin(state::State)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tranquil General".to_string(), // ToDo
                resolution: (1280., 920.).into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .run();
}
