use bevy::prelude::*;

use crate::constants::*;
use crate::state::game::damage::Damage;
use crate::state::game::hud::disappear_timer::DisappearTimer;

pub fn display_damage(
    value: &Damage,
    position: &Vec3,
    commands: &mut Commands,
    font: Handle<Font>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Px(WINDOW_WIDTH / 2.0 + position.x),
                    top: Val::Px(WINDOW_HEIGHT / 2.0 - position.y),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(DisappearTimer::from_seconds(3.0))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("{value}"),
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
