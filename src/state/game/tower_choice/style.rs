
use bevy::prelude::*;

use crate::constants::*;

pub fn tower_selection_screen() -> NodeBundle {
    NodeBundle {
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
    }
}
pub fn tower_selection_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}
pub fn tower_selection_button(sprite: &Handle<Image>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(100.0), Val::Px(100.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor::from(BUTTON_COLOR),
        image: UiImage::from(sprite.clone()),
        ..default()
    }
}
fn side_effect_text(text: &str, font: &Handle<Font>, color: Color) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 15.0,
            color,
        },
    )
}
pub fn good_side_effect_text(text: &str, font: &Handle<Font>) -> TextBundle {
    side_effect_text(text, font, Color::GREEN)
}
pub fn bad_side_effect_text(text: &str, font: &Handle<Font>) -> TextBundle {
    side_effect_text(text, font, Color::RED)
}
