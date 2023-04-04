use crate::assets::Sprites;
use crate::constants::*;
use crate::state::AppState;
use bevy::prelude::*;

pub(super) struct TowerChoice;

impl Plugin for TowerChoice {
    fn build(&self, app: &mut App) {
        app.add_system(generate_towers.in_schedule(OnEnter(AppState::TowerChoice)))
            .add_system(click.in_set(OnUpdate(AppState::TowerChoice)))
            .add_system(cleanup.in_schedule(OnExit(AppState::TowerChoice)));
    }
}

#[derive(Component)]
pub struct TowerButton;

#[derive(Component)]
pub struct TowerSelectionScreen;

fn generate_towers(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                size: Size::all(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(1.0)),
                gap: Size::all(Val::Percent(1.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..default()
            },
            ..default()
        })
        .insert(TowerSelectionScreen)
        .with_children(|parent| {
            for _ in 0..NUMBER_OF_TOWERS_TO_GENERATE {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor::from(BUTTON_COLOR),
                        image: UiImage::from(sprites.tower.clone()),
                        ..default()
                    })
                    .insert(TowerButton);
            }
        });
}

fn click(
    mut next_state: ResMut<NextState<AppState>>,
    mut tower_buttons: Query<(&Interaction, &mut BackgroundColor), With<TowerButton>>,
) {
    for (button_interaction, mut button_color) in tower_buttons.iter_mut() {
        button_color.0 = match button_interaction {
            Interaction::Clicked => {
                next_state.set(AppState::TowerPlacing);
                CLICKED_COLOR
            }
            Interaction::Hovered => HOVERED_COLOR,
            Interaction::None => BUTTON_COLOR,
        };
    }
}

fn cleanup(mut commands: Commands, screen_elements: Query<Entity, With<TowerSelectionScreen>>) {
    for element in screen_elements.iter() {
        commands.entity(element).despawn_recursive();
    }
}
