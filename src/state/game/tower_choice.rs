use bevy::prelude::*;

use crate::assets::*;
use crate::constants::*;
use crate::state::game::side_effect::*;
use crate::state::AppState;

mod style;
use style::*;

pub(super) struct TowerChoice;

impl Plugin for TowerChoice {
    fn build(&self, app: &mut App) {
        app.add_system(generate_towers.in_schedule(OnEnter(AppState::TowerChoice)))
            .add_system(click.in_set(OnUpdate(AppState::TowerChoice)))
            .add_system(cleanup.in_schedule(OnExit(AppState::TowerChoice)))
            .add_event::<TowerCreationEvent>();
    }
}

#[derive(Component)]
pub struct TowerButton;

#[derive(Component)]
pub struct TowerSelectionScreen;

#[derive(Clone, Component)]
pub struct TowerCreationEvent {
    pub side_effects: Vec<SideEffect>,
}

impl TowerCreationEvent {
    fn random() -> Self {
        TowerCreationEvent {
            side_effects: (0..2).map(|_| SideEffect::random()).collect(),
        }
    }
}

fn generate_towers(mut commands: Commands, sprites: Res<Sprites>, fonts: Res<Fonts>) {
    commands
        .spawn(tower_selection_screen())
        .insert(TowerSelectionScreen)
        .with_children(|screen| {
            for _ in 0..NUMBER_OF_TOWERS_TO_GENERATE {
                let tower_creation_event = TowerCreationEvent::random();
                screen
                    .spawn(tower_selection_panel())
                    .with_children(|panel| {
                        panel
                            .spawn(tower_selection_button(&sprites.tower))
                            .insert(TowerButton)
                            .insert(tower_creation_event.clone());
                        // for side_effect in &tower_creation_event.side_effects {
                        //     side_effect.insert_into(&mut button);
                        // }
                        for side_effect in tower_creation_event.side_effects {
                            panel.spawn(side_effect_text(side_effect, &fonts.default_font));
                        }
                    });
            }
        });
}

fn click(
    mut next_state: ResMut<NextState<AppState>>,
    mut tower_buttons: Query<
        (&Interaction, &mut BackgroundColor, &TowerCreationEvent),
        With<TowerButton>,
    >,
    mut tower_creation_event_writer: EventWriter<TowerCreationEvent>,
) {
    for (button_interaction, mut button_color, tower_creation_event) in tower_buttons.iter_mut() {
        button_color.0 = match button_interaction {
            Interaction::Clicked => {
                tower_creation_event_writer.send(tower_creation_event.clone());
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
