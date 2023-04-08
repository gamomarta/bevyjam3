use bevy::prelude::*;

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>();
    }
}

pub struct GameOverEvent {
    pub reason: String,
    //     enemies_killed: TODO
}
