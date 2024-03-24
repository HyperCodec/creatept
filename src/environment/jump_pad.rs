use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::geometry::CollisionEventFlags};

use crate::GameState;

use super::level_loading::LevelCleanup;

pub struct JumpPadPlugin;

impl Plugin for JumpPadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                jump_pad.after(tick_jump_pad_cooldowns)
                    .run_if(|state: Res<GameState>| state.is_playing()),
                tick_jump_pad_cooldowns,
            ));
    }
}

#[derive(Component, Default)]
pub struct JumpPad {
    pub force: Velocity,
    pub cooldown: Timer,
}

#[derive(Bundle)]
pub struct JumpPadBundle {
    pub jump_pad: JumpPad,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub collision_events: ActiveEvents,
    pub level_cleanup: LevelCleanup,
}

impl Default for JumpPadBundle {
    fn default() -> Self {
        Self {
            jump_pad: JumpPad::default(),
            rigid_body: RigidBody::Fixed,
            collider: Collider::default(),
            transform: Transform::default(),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            level_cleanup: LevelCleanup,
        }
    }
}

fn jump_pad(
    mut jump_pad_q: Query<&mut JumpPad>,
    mut player_q: Query<&mut Velocity, With<LogicalPlayer>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut player_vel = player_q.single_mut();

    for ev in collision_events.read() {
        if let &CollisionEvent::Started(ent1, ent2, flags) = ev {
            if flags.contains(CollisionEventFlags::REMOVED) {
                continue;
            }

            if let Ok(mut jump_pad) = jump_pad_q.get_mut(ent1) {
                if ent1 == ent2 {
                    continue;
                }

                if jump_pad.cooldown.finished() {
                    jump_pad.cooldown.reset();
    
                    player_vel.linvel += jump_pad.force.linvel;
                }
            }
        }
    }
}

fn tick_jump_pad_cooldowns(
    time: Res<Time>,
    mut jump_pad_q: Query<&mut JumpPad>,
) {
    let dt = time.delta();

    for mut jump_pad in jump_pad_q.iter_mut() {
        jump_pad.cooldown.tick(dt);
    }
}