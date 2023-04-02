use bevy::prelude::*;

use crate::assets::Fonts;
use crate::state::game::money::Money;
use crate::state::game::player::Player;
use crate::state::AppState;

pub(super) struct Hud;

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_system(display_hud.in_schedule(OnEnter(AppState::Game)))
            .add_system(update_money.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct BuyButton;

#[derive(Component)]
struct MoneyDisplay;

fn display_hud(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::width(Val::Percent(10.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                margin: UiRect::all(Val::Percent(1.0)),
                gap: Size::all(Val::Percent(1.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Money:",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::Left),
                )
                .insert(MoneyDisplay);

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::GRAY),
                    ..default()
                })
                .insert(BuyButton);
        });
}

fn update_money(
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    money: Query<&Money, With<Player>>,
) {
    let mut money_display = money_display.single_mut();
    let Some(mut money_text) = money_display.sections.first_mut() else { return; };
    let money = money.single();

    money_text.value = format!("Money: {}", money);
}
