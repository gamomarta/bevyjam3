use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::Materials;
use crate::components::Player;
use crate::constants::layers::*;
use crate::constants::*;
use crate::events::GameOverEvent;
use crate::state::game::health::Health;
use crate::state::game::money::*;
use crate::state::game::tower::Tower;
use crate::state::*;

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_goal.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(lose.in_set(OnUpdate(AppState::Game)))
            .add_system(also_lose.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Goal;

fn spawn_goal(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, materials: Res<Materials>) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(GOAL_WIDTH, WINDOW_HEIGHT)) //TODO
                        .into(),
                )
                .into(),
            material: materials.goal.clone(),
            transform: Transform::from_translation(Vec3::new(
                GOAL_POSITION + GOAL_WIDTH / 2.0,
                0.0,
                GOAL_LAYER,
            )),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(Health::new(GOAL_HEALTH))
        .insert(Goal)
        .insert(GameEntity);
}

fn lose(
    goals: Query<With<Goal>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    if goals.is_empty() {
        game_over_event.send(GameOverEvent {
            reason: "Everyone in the hospital was infected.".to_string(),
        });
        next_state.set(AppState::GameOver);
    }
}

fn also_lose(
    players: Query<(&Money, &TowerCost), With<Player>>,
    towers: Query<With<Tower>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    if towers.is_empty() {
        let can_buy_tower = players.iter().all(|(money, price)| money.can_buy(price));
        if !can_buy_tower {
            game_over_event.send(GameOverEvent {
                reason: "No doctors left and no money to hire.".to_string(),
            });
            next_state.set(AppState::GameOver);
        }
    }
}
