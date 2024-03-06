use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_xpbd_3d::prelude::*;

use super::{Player, PlayerCamera};

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, move_camera_with_player.after(setup_player));
    }
}

fn setup_player(
    mut commands: Commands,
) {
    // main player
    commands.spawn((
        TransformBundle::default(),
        Player,
        RigidBody::Dynamic,
        Collider::capsule(1.0, 0.5),
    )).with_children(|player| {
        // player camera
        player.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                ..default()
            },
            BloomSettings::NATURAL,
            PlayerCamera,
        ));
    });
}

fn move_camera_with_player(
    player_q: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut cam_q: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let mut cam_trans = cam_q.single_mut();
    let player_trans = player_q.single();

    cam_trans.translation = player_trans.translation;
}