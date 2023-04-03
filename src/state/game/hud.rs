use bevy::prelude::*;

use crate::assets::{Fonts, Sprites};
use crate::state::game::money::{Money, TowerCost};
use crate::state::game::player::Player;
use crate::state::AppState;

pub(super) struct Hud;

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_system(display_hud.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(update_money.in_set(OnUpdate(AppState::Game)))
            .add_system(update_price.in_set(OnUpdate(AppState::Game)))
            .add_system(buy.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct BuyButton;

const BUTTON_COLOR: Color = Color::DARK_GRAY;
const HOVERED_COLOR: Color = Color::GREEN;
const CLICKED_COLOR: Color = Color::LIME_GREEN;
const DEACTIVATED_COLOR: Color = Color::ORANGE_RED;

#[derive(Component)]
struct BuyText;

#[derive(Component)]
struct MoneyDisplay;

fn display_hud(mut commands: Commands, fonts: Res<Fonts>, sprites: Res<Sprites>) {
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
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::Left),
                )
                .insert(MoneyDisplay);

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(BUTTON_COLOR),
                    image: UiImage::from(sprites.tower.clone()),
                    ..default()
                })
                .insert(BuyButton)
                .with_children(|button| {
                    button
                        .spawn(
                            TextBundle::from_section(
                                "Buy:",
                                TextStyle {
                                    font: fonts.default_font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_alignment(TextAlignment::Center),
                        )
                        .insert(BuyText);
                });
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

fn update_price(
    mut buy_text: Query<&mut Text, With<BuyText>>,
    price: Query<&TowerCost, With<Player>>,
) {
    let mut buy_text = buy_text.single_mut();
    let Some(mut buy_text) = buy_text.sections.first_mut() else { return; };
    let price = price.single();

    buy_text.value = format!("Buy: {}", price);
}

fn buy(
    mut next_state: ResMut<NextState<AppState>>,
    mut button: Query<(&Interaction, &mut BackgroundColor), With<BuyButton>>,
    mut player: Query<(&TowerCost, &mut Money), With<Player>>,
) {
    let (button_interaction, mut button_color) = button.single_mut();
    let (price, mut money) = player.single_mut();

    button_color.0 = if money.can_buy(price) {
        match button_interaction {
            Interaction::Clicked => {
                next_state.set(AppState::TowerPlacing);
                money.buy(price);
                CLICKED_COLOR
            }
            Interaction::Hovered => HOVERED_COLOR,
            Interaction::None => BUTTON_COLOR,
        }
    } else {
        DEACTIVATED_COLOR
    }
}
