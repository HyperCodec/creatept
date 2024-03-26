use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

use crate::{environment::level_loading::cleanup, handle_empty_event, GameState};

use super::{fx::{DespawnAfterTime, Sfx}, level_loading::LevelCleanup, EnvironmentTime};

pub struct SpawnCyclePlugin;

impl Plugin for SpawnCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RespawnEvent>()
            .add_event::<EndLevelEvent>()
            .add_systems(Update, (
                handle_empty_event!(end_level, EndLevelEvent),
                handle_empty_event!(respawn, RespawnEvent),

                (
                    handle_collision_goal.before(end_level),
                    fall_off,
                )
                    .run_if(|state: Res<GameState>| state.is_playing()),
            ));
    }
}

#[derive(Component, Default)]
pub struct Spawnpoint;

#[derive(Bundle)]
pub struct SpawnpointBundle {
    pub spawnpoint: Spawnpoint,
    pub transform: TransformBundle,
    pub level_cleanup: LevelCleanup,
}

impl Default for SpawnpointBundle {
    fn default() -> Self {
        Self {
            spawnpoint: Spawnpoint,
            level_cleanup: LevelCleanup,
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct Goal{ 
    pub size: f32,
}

#[derive(Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
    pub transform: TransformBundle,
    pub level_cleanup: LevelCleanup,
}

impl Default for GoalBundle {
    fn default() -> Self {
        Self {
            goal: Goal::default(),
            level_cleanup: LevelCleanup,
            transform: TransformBundle::default(),
        }
    }

}

fn handle_collision_goal(
    goal_q: Query<(&Transform, &Goal)>,
    player_q: Query<&Transform, With<LogicalPlayer>>,
    mut ev: EventWriter<EndLevelEvent>,
    mut state: ResMut<GameState>,
) {
    let player_transform = player_q.single();
    for (goal_transform, goal) in goal_q.iter() {
        if player_transform.translation.distance_squared(goal_transform.translation) < goal.size.powi(2) {
            ev.send(EndLevelEvent);
            *state = GameState::Menu;
        }
    }
}

#[derive(Event)]
pub struct EndLevelEvent;

pub fn end_level(
    entities: Query<Entity, With<LevelCleanup>>,
    mut state: ResMut<GameState>,
    mut etime: ResMut<EnvironmentTime>,
    mut commands: Commands,
) {
    info!("Ending level");
    etime.is_ticking = false;

    *state = GameState::Menu;

    cleanup(&entities, &mut commands);
}

#[derive(Event)]
pub struct RespawnEvent;

fn respawn(
    sfx: Res<Sfx>,
    spawn_q: Query<&Transform, (With<Spawnpoint>, Without<LogicalPlayer>)>,
    mut player_q: Query<(Entity, &mut Transform, &mut Velocity), (With<LogicalPlayer>, Without<Spawnpoint>)>,
    mut commands: Commands,
) {
    let spawn = spawn_q.single();
    let (entity, mut player_trans, mut player_vel) = player_q.single_mut();

    *player_trans = spawn.clone();
    *player_vel = Velocity::zero();

    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            AudioBundle {
                source: sfx.fail.clone(),
                ..default()
            },
            DespawnAfterTime {
                timer: Timer::from_seconds(1., TimerMode::Once),
            },
        ));
    });
}

fn fall_off(
    player_q: Query<&Transform, With<LogicalPlayer>>,
    mut ev: EventWriter<RespawnEvent>,
) {
    for player_transform in player_q.iter() {
        if player_transform.translation.y <= -50. {
            ev.send(RespawnEvent);
        }
    }
}