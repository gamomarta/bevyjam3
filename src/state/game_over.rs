use bevy::prelude::*;

use crate::assets::Fonts;
use crate::constants::*;
use crate::events::GameOverEvent;
use crate::state::*;

pub(super) struct GameOver;

impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app.add_system(clean_game_entities.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(
                show_ui
                    .in_schedule(OnEnter(AppState::GameOver))
                    .after(clean_game_entities),
            )
            .add_system(click.in_set(OnUpdate(AppState::GameOver)))
            .add_system(clean_game_over_entities.in_schedule(OnExit(AppState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverScreen;

fn clean_game_entities(mut commands: Commands, game_entities: Query<Entity, With<GameEntity>>) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_ui(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut game_over_event: EventReader<GameOverEvent>,
) {
    let game_over_event = game_over_event.iter().next().unwrap();
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
        .insert(GameOverScreen)
        .with_children(|screen| {
            screen.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: fonts.default_font.clone(),
                    font_size: 100.0,
                    color: Color::RED,
                },
            ));
            screen.spawn(TextBundle::from_section(
                &game_over_event.reason,
                TextStyle {
                    font: fonts.default_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
            screen.spawn(TextBundle::from_section(
                format!("You healed {} people.", game_over_event.enemies_killed),
                TextStyle {
                    font: fonts.default_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
            screen
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(161.8), Val::Px(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(BUTTON_COLOR),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

fn click(
    mut next_state: ResMut<NextState<AppState>>,
    mut tower_buttons: Query<(&Interaction, &mut BackgroundColor)>,
) {
    for (button_interaction, mut button_color) in tower_buttons.iter_mut() {
        button_color.0 = match button_interaction {
            Interaction::Clicked => {
                next_state.set(AppState::PreGame);
                CLICKED_COLOR
            }
            Interaction::Hovered => HOVERED_COLOR,
            Interaction::None => BUTTON_COLOR,
        };
    }
}

fn clean_game_over_entities(mut commands: Commands, screens: Query<Entity, With<GameOverScreen>>) {
    for screen in screens.iter() {
        commands.entity(screen).despawn_recursive();
    }
}
