use crate::state::AppState;
use bevy::prelude::*;

pub(super) struct TowerChoice;

impl Plugin for TowerChoice {
    fn build(&self, app: &mut App) {
        app.add_system(generate_towers.in_schedule(OnEnter(AppState::TowerChoice)))
            .add_system(
                show_ui
                    .in_schedule(OnEnter(AppState::TowerChoice))
                    .after(generate_towers),
            )
            .add_system(click.in_set(OnUpdate(AppState::TowerChoice)));
    }
}

fn generate_towers() {
    dbg!();
}

fn show_ui() {
    dbg!();
}

fn click(mut next_state: ResMut<NextState<AppState>>) {
    dbg!();
    next_state.set(AppState::TowerPlacing);
}
