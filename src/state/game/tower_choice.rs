use bevy::prelude::*;

use crate::assets::*;
use crate::constants::*;
use crate::state::AppState;

mod style;
use style::*;

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

fn generate_towers(mut commands: Commands, sprites: Res<Sprites>, fonts: Res<Fonts>) {
    commands
        .spawn(tower_selection_screen())
        .insert(TowerSelectionScreen)
        .with_children(|screen| {
            for _ in 0..NUMBER_OF_TOWERS_TO_GENERATE {
                screen
                    .spawn(tower_selection_panel())
                    .with_children(|parent| {
                        parent
                            .spawn(tower_selection_button(&sprites.tower))
                            .insert(TowerButton);
                        parent.spawn(good_side_effect_text(
                            "Good side effect text",
                            &fonts.default_font,
                        ));
                        parent.spawn(bad_side_effect_text(
                            "Bad side effect text",
                            &fonts.default_font,
                        ));
                    });
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
