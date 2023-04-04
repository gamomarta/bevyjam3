#![allow(
    clippy::redundant_closure_call,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
#![feature(core_intrinsics)]

use bevy::prelude::*;

mod assets;
mod constants;
mod state;
mod utils;
mod sound;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(assets::Assets)
        .add_plugin(state::State)
        .run();
}
