use bevy::prelude::*;

use crate::assets::Fonts;
use crate::state::game::money::Money;
use crate::state::game::player::Player;
use crate::state::AppState;

pub(super) struct Hud;

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_system(display_money.in_schedule(OnEnter(AppState::Game)))
            .add_system(update_money.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct MoneyDisplay;

fn display_money(mut commands: Commands, fonts: Res<Fonts>) {
    commands
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
