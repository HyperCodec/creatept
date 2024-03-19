use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::prelude::*;

use crate::player::PlayerCamera;

use super::{generate_explosion_particles, Explosion, ExplosionBundle};

#[derive(Component)]
pub struct Bomb {
    timer: Timer,
}

pub(super) fn spawn_bomb(
    player_q: Query<&Velocity, With<LogicalPlayer>>,
    cam_q: Query<&Transform, With<PlayerCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let player_velocity = player_q.single();
    let cam_transform = cam_q.single();

    let direction = cam_transform.forward().normalize();
    let force = direction * 10.;

    let bomb_linvel = player_velocity.linvel + force;

    commands.spawn((
        Bomb {
            timer: Timer::from_seconds(3., TimerMode::Once),
        },
        RigidBody::Dynamic,
        Collider::ball(0.25),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.25 })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.5, 0.5, 0.5))),
            transform: *cam_transform,
            ..default()
        },
        Velocity {
            linvel: bomb_linvel,
            angvel: Vec3::ZERO,
        },

        // TODO stop it from rolling so much
        Friction {
            coefficient: 10.,
            combine_rule: CoefficientCombineRule::Multiply,
        }
    ));
}

pub(super) fn tick_bombs(
    time: Res<Time>,
    mut commands: Commands,
    mut bombs_q: Query<(Entity, &Transform, &mut Bomb)>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let dt = time.delta();

    for (entity, transform, mut bomb) in bombs_q.iter_mut() {
        bomb.timer.tick(dt);

        if bomb.timer.finished() {
            commands.entity(entity).despawn_recursive();

            let effect = generate_explosion_particles(&mut effects, 5.);

            commands.spawn((
                ExplosionBundle {
                    explosion: Explosion {
                        radius: 5.,
                        force: 25.,
                    },
                    transform: TransformBundle::from_transform(Transform::from_translation(transform.translation - Vec3::Y * 0.5)),
                },
                ParticleEffect::new(effect),
            ));
        }
    }
}

