use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_xpbd_3d::prelude::*;

use super::Player;

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(
    mut commands: Commands,
) {
    // player camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
        Player,
        RigidBody::Dynamic,
        Collider::capsule(1.0, 0.5),
    ));
}