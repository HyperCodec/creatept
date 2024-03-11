use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

use super::EnvironmentTime;

pub struct SpawnCyclePlugin;

impl Plugin for SpawnCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndLevelEvent>()
            .add_systems(Update, (
                handle_collision_goal.before(end_level),
                end_level.run_if(|ev: EventReader<EndLevelEvent>| !ev.is_empty()),
                respawn.run_if(|ev: EventReader<RespawnEvent>| !ev.is_empty()),
            ));
    }
}

#[derive(Component)]
pub struct Spawnpoint;

#[derive(Component)]
pub struct Goal;

fn handle_collision_goal(
    goal_q: Query<Entity, With<Goal>>,
    colliding_q: Query<&CollidingEntities, With<LogicalPlayer>>,
    mut ev: EventWriter<EndLevelEvent>,
) {
    for coll_ents in &colliding_q {

        for goal in &goal_q {
            if coll_ents.contains(goal) {
                ev.send(EndLevelEvent);
                break;
            }
        }
    }
}

#[derive(Event)]
pub struct EndLevelEvent;

fn end_level(
    mut ev: EventReader<EndLevelEvent>,
    mut etime: ResMut<EnvironmentTime>,
) {
    ev.read();
    etime.is_ticking = false;

    // TODO other level end stuff
}

#[derive(Event)]
pub struct RespawnEvent;

fn respawn(
    spawn_q: Query<&Transform, (With<Spawnpoint>, Without<LogicalPlayer>)>,
    mut player_q: Query<(&mut Transform, &mut Velocity), (With<LogicalPlayer>, Without<Spawnpoint>)>,
    mut etime: ResMut<EnvironmentTime>,
) {
    let spawn = spawn_q.single();
    let (mut player_trans, mut player_vel) = player_q.single_mut();

    *player_trans = spawn.clone();
    *player_vel = Velocity::zero();

    etime.time.reset();
}