use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::player::Player;

use super::EnvironmentTime;

pub struct SpawnCyclePlugin;

impl Plugin for SpawnCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndLevelEvent>()
            .add_systems(Update, 
                end_level.run_if(|ev: EventReader<EndLevelEvent>| !ev.is_empty()));
    }
}

#[derive(Component)]
pub struct Spawnpoint;

#[derive(Component)]
pub struct Goal;

fn handle_collision_goal(
    goal_q: Query<Entity, With<Goal>>,
    colliding_q: Query<&CollidingEntities, With<Player>>,
    mut ev: EventWriter<EndLevelEvent>,
    mut commands: Commands,
) {
    for coll_ents in &colliding_q {
        for goal in &goal_q {
            if coll_ents.contains(&goal) {
                ev.send(EndLevelEvent);
                break;
            }
        }
    }
}

#[derive(Event)]
pub struct EndLevelEvent;

fn end_level(
    mut etime: ResMut<EnvironmentTime>,
) {
    etime.is_ticking = false;

    // TODO other level end stuff
}