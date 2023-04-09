use bevy::prelude::*;

use crate::assets::Fonts;
use crate::state::*;

pub(super) struct Win;

impl Plugin for Win {
    fn build(&self, app: &mut App) {
        app.add_system(show_ui.in_schedule(OnEnter(AppState::Win)));
    }
}

fn show_ui(mut commands: Commands, fonts: Res<Fonts>) {
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
            background_color: BackgroundColor::from(Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            ..default()
        })
        .with_children(|screen| {
            screen.spawn(TextBundle::from_section(
                "You win",
                TextStyle {
                    font: fonts.default_font.clone(),
                    font_size: 100.0,
                    color: Color::RED,
                },
            ));
            screen.spawn(TextBundle::from_section(
                "Your doctors healed everyone and stopped the disease",
                TextStyle {
                    font: fonts.default_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}
