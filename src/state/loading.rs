use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::assets::*;
use crate::state::AppState;

mod assets_loading;
use crate::constants::*;
use assets_loading::AssetsLoading;

pub(super) struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_system(spawn_camera.in_schedule(OnEnter(AppState::Loading)))
            .add_system(show_ui.in_schedule(OnEnter(AppState::Loading)))
            .add_system(load_sprites.in_schedule(OnEnter(AppState::Loading)))
            .add_system(load_fonts.in_schedule(OnEnter(AppState::Loading)))
            .add_system(load_sound.in_schedule(OnEnter(AppState::Loading)))
            .add_system(create_color_materials.in_schedule(OnEnter(AppState::Loading)))
            .add_system(check_loading.in_set(OnUpdate(AppState::Loading)))
            .add_system(clean_up.in_schedule(OnExit(AppState::Loading)));
    }
}

#[derive(Component)]
struct LoadingScreen;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn show_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::all(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                gap: Size::all(Val::Percent(1.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            ..default()
        })
        .insert(LoadingScreen)
        .with_children(|screen| {
            screen.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("Kenney Future.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ));
        });
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

fn load_sprites(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut sprites: ResMut<Sprites>,
) {
    let mut load_sprite = |path| load_asset(&asset_server, &mut assets_loading, path);
    sprites.background = load_sprite("background.png");
    sprites.tower = load_sprite("doctor.png");
    sprites.bullet = load_sprite("pill.png");
    sprites.enemy = load_sprite("infected.png");
    sprites.defeated_enemy = load_sprite("girl.png");
}

fn load_fonts(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut fonts: ResMut<Fonts>,
) {
    let mut load_font = |path| load_asset(&asset_server, &mut assets_loading, path);
    fonts.default_font = load_font("Kenney Future.ttf");
}

fn load_sound(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut music: ResMut<Music>,
) {
    let mut load_sound = |path| load_asset(&asset_server, &mut assets_loading, path);
    music.main_theme = load_sound("tranquil.ogg");
    music.game_over = load_sound("game_over.ogg");
}

fn create_color_materials(
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut materials: ResMut<Materials>,
) {
    materials.goal = color_materials.add(ColorMaterial::from(GOAL_COLOR));
    materials.tower_range = color_materials.add(ColorMaterial::from(SHOOT_RADIUS_COLOR));
    materials.tower_invalid = color_materials.add(ColorMaterial::from(TOWER_INVALID_COLOR));
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

fn clean_up(mut commands: Commands, entities: Query<Entity, With<LoadingScreen>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
