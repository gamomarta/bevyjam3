use bevy::prelude::*;

use crate::constants::*;
use crate::state::game::hud::disappear_timer::DisappearTimer;
use crate::state::*;

pub fn display_popup<Text: Into<String>>(
    text: Text,
    position: &Vec3,
    commands: &mut Commands,
    font: Handle<Font>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                position: UiRect {
                    left: Val::Px(WINDOW_WIDTH / 2.0 + position.x),
                    top: Val::Px(WINDOW_HEIGHT / 2.0 - position.y),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(DisappearTimer::from_seconds(0.5))
        .insert(GameEntity)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::RED,
                    },
                )
                .with_text_alignment(TextAlignment::Left),
            );
        });
}
