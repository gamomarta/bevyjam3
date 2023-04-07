use bevy::prelude::*;

use crate::constants::*;
use crate::state::game::side_effect::*;

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
        background_color: BackgroundColor::from(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.97,
        }),
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
pub fn side_effect_text(side_effect: &SideEffect, font: &Handle<Font>) -> TextBundle {
    let color = match side_effect.get_type() {
        SideEffectType::Good => Color::Rgba {
            red: 0.0,
            green: 0.8,
            blue: 0.0,
            alpha: 1.0,
        },
        SideEffectType::Bad => Color::Rgba {
            red: 0.8,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        },
    };
    TextBundle::from_section(
        side_effect.get_description(),
        TextStyle {
            font: font.clone(),
            font_size: 15.0,
            color,
        },
    )
}
