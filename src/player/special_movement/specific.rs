use bevy::prelude::*;
use bevy_hanabi::ParticleEffectBundle;
use bevy_rapier3d::prelude::*;

use crate::player::PlayerCamera;

use super::{ExplodeOnTouch, ExplosiveProjectileBundle};

pub struct SpecialMovementSpecificsPlugin;

impl Plugin for SpecialMovementSpecificsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                spawn_rocket.run_if(|inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(KeyCode::KeyR)), // temp keybind
            ));
    }
}

fn spawn_rocket(
    cam_q: Query<&Transform, With<PlayerCamera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cam_transform = cam_q.single();

    // force & location
    let direction = cam_transform.forward().normalize();
    let force = direction * 10.;
    let rocket_transform = Transform::from_translation(cam_transform.translation + direction);

    commands.spawn((
        ExplosiveProjectileBundle {
            rigidbody: RigidBody::KinematicVelocityBased,
            collider: Collider::ball(0.1),
            velocity: Velocity {
                linvel: force,
                angvel: Vec3::ZERO,
            },
            explosion: ExplodeOnTouch {
                radius: 5.,
                force: 100.,
                damage: 10.,
            },
            active_events: ActiveEvents::COLLISION_EVENTS,
        },
        GravityScale(0.),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere {
                radius: 0.1,
                ..default()
            })), // TODO use rocket model
            material: materials.add(StandardMaterial::from(Color::rgb(1., 0., 0.))),
            transform: rocket_transform,
            ..default()
        },
    ));
}