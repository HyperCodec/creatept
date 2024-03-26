use bevy::{audio::Volume, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::prelude::*;

use crate::{environment::fx::{DespawnAfterTime, Sfx}, player::PlayerCamera};

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
    let force = direction * 5.;

    let bomb_linvel = player_velocity.linvel + force;
    let bomb_pos = cam_transform.translation + direction * Vec3::new(1., 0., 1.) * 0.5;

    commands.spawn((
        Bomb {
            timer: Timer::from_seconds(2., TimerMode::Once),
        },
        RigidBody::Dynamic,
        Collider::ball(0.25),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.25 })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.5, 0.5, 0.5))),
            transform: Transform::from_translation(bomb_pos),
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
        },
    ));
}

pub(super) fn tick_bombs(
    time: Res<Time>,
    sfx: Res<Sfx>,
    mut commands: Commands,
    mut bombs_q: Query<(Entity, &Transform, &mut Bomb)>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let dt = time.delta();

    for (entity, transform, mut bomb) in bombs_q.iter_mut() {
        bomb.timer.tick(dt);

        if bomb.timer.finished() {
            commands.entity(entity).despawn_recursive();

            let effect = generate_explosion_particles(&mut effects, 2.);

            commands.spawn(ExplosionBundle {
                explosion: Explosion {
                    radius: 15.,
                    force: 40.,
                },
                transform: Transform::from_translation(transform.translation - Vec3::Y * 0.5).into(),
                ..default()
            });

            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                AudioBundle {
                    source: sfx.explosion.clone(),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.5),
                        ..default()
                    }
                },
                DespawnAfterTime {
                    timer: Timer::from_seconds(1., TimerMode::Once),
                },
            ));
        }
    }
}

