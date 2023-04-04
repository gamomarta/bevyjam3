use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::assets::*;
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

//TODO: could be a method of AssetsLoading -> changed to AssetLoader
fn load_asset<T: Asset + TypeUuid>(
    asset_server: &AssetServer,
    assets_loading: &mut AssetsLoading,
    path: &str,
) -> Handle<T> {
    let handle = asset_server.load(path);
    assets_loading.push(handle.clone_untyped());
    handle
}

fn load(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut sprites: ResMut<Sprites>,
    mut fonts: ResMut<Fonts>,
) {
    let mut load_sprite = |path| load_asset(&asset_server, &mut assets_loading, path);
    sprites.bevy_logo = load_sprite("icon.png");
    sprites.tower = load_sprite("icon.png");
    sprites.bullet = load_sprite("icon.png");
    sprites.enemy = load_sprite("icon.png");
    sprites.goal = load_sprite("icon.png");

    let mut load_font = |path| load_asset(&asset_server, &mut assets_loading, path);
    fonts.default_font = load_font("Kenney Future.ttf");
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
            next_state.set(AppState::PreGame);
        }
        LoadState::NotLoaded | LoadState::Loading | LoadState::Unloaded => {}
        LoadState::Failed => {
            unimplemented!()
        }
    }
}
