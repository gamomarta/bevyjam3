use bevy::prelude::*;

use crate::assets::Sprites;
use crate::state::AppState;

mod assets_loading;
use assets_loading::AssetsLoading;

pub(super) struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_system(load.in_schedule(OnEnter(AppState::Loading)))
            .add_system(check_loading.in_set(OnUpdate(AppState::Loading)));
    }
}

fn load(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut sprites: ResMut<Sprites>,
) {
    let mut load_asset = |path| {
        let handle = asset_server.load(path);
        assets_loading.push(handle.clone_untyped());
        handle
    };
    sprites.bevy_logo = load_asset("icon.png");
    sprites.tower = load_asset("icon.png");
    sprites.bullet = load_asset("icon.png");
    sprites.enemy = load_asset("icon.png");
}

fn check_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    match asset_server.get_group_load_state(loading.iter().map(|h| h.id())) {
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            next_state.set(AppState::Game);
        }
        LoadState::NotLoaded | LoadState::Loading | LoadState::Unloaded => {}
        LoadState::Failed => {
            unimplemented!()
        }
    }
}
