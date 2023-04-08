use bevy::prelude::*;

use crate::assets::{Fonts, Sprites};
use crate::components::Player;
use crate::constants::*;
use crate::state::game::goal::Goal;
use crate::state::game::health::Health;
use crate::state::game::money::{Money, TowerCost};
use crate::state::{AppState, GameEntity};

pub mod damage;
pub mod disappear_timer;

pub(super) struct Hud;

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_system(display_hud.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(update_health.in_set(OnUpdate(AppState::Game)))
            .add_system(update_money.in_set(OnUpdate(AppState::Game)))
            .add_system(update_price.in_set(OnUpdate(AppState::Game)))
            .add_system(buy.in_set(OnUpdate(AppState::Game)))
            .add_system(disappear_timer::disappear.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct BuyButton;

#[derive(Component)]
struct BuyText;

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct MoneyDisplay;

fn display_hud(mut commands: Commands, fonts: Res<Fonts>, sprites: Res<Sprites>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::width(Val::Percent(10.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
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
        })
        .insert(GameEntity)
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Health:",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::Left),
                )
                .insert(HealthDisplay);

            parent
                .spawn(
                    TextBundle::from_section(
                        "Budget:",
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
                                "Hire:",
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

fn update_health(
    mut health_display: Query<&mut Text, With<HealthDisplay>>,
    health: Query<&Health, With<Goal>>,
) {
    let mut health_display = health_display.single_mut();
    let Some(mut health_text) = health_display.sections.first_mut() else { return; };
    for health in health.iter() {
        health_text.value = format!("Health: {health}");
    }
}

fn update_money(
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    money: Query<&Money, With<Player>>,
) {
    let mut money_display = money_display.single_mut();
    let Some(mut money_text) = money_display.sections.first_mut() else { return; };
    let money = money.single();

    money_text.value = format!("Budget: {money}");
}

fn update_price(
    mut buy_text: Query<&mut Text, With<BuyText>>,
    price: Query<&TowerCost, With<Player>>,
) {
    let mut buy_text = buy_text.single_mut();
    let Some(mut buy_text) = buy_text.sections.first_mut() else { return; };
    let price = price.single();

    buy_text.value = format!("Hire: {price}");
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
                next_state.set(AppState::TowerChoice);
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
