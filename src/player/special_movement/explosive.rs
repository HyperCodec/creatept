use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;

use super::PlayerEnactForceEvent;

#[derive(Component)]
pub struct Explosive {
    pub radius: f32,
    pub force: f32,
    pub damage: f32,
}

#[derive(Bundle)]
pub struct ExplosiveBundle {
    pub explosive: Explosive,
    pub transform: TransformBundle,
}

pub(super) fn handle_explosions(
    explosions_q: Query<(Entity, &Explosive, &Transform)>,
    player_q: Query<&Transform, With<LogicalPlayer>>,
    mut evw: EventWriter<PlayerEnactForceEvent>,
    mut commands: Commands,
) {
    let player_transform = player_q.single();

    for (entity, explosive, transform) in explosions_q.iter() {
        let distance = transform.translation.distance(player_transform.translation).max(0.1);
        if distance < explosive.radius {
            let strength = explosive.force.powi(2) / distance;

            let direction = (player_transform.translation - transform.translation).normalize();
            let force = direction * strength;
            let event = PlayerEnactForceEvent {
                force: Velocity {
                    linvel: force,
                    angvel: Vec3::ZERO,
                },
            };
            evw.send(event);
        }

        commands.entity(entity).despawn();
    }
}