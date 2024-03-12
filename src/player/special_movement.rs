use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

pub struct SpecialMovementPlugin;

impl Plugin for SpecialMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerEnactForceEvent>()
            .add_systems(Update, (
                handle_player_enact_force_event.after(handle_explosions),
                handle_explosions,
            ));
    }
}

#[derive(Component)]
pub struct Explosive {
    pub radius: f32,
    pub force: f32,
    pub damage: f32,
}

#[derive(Event)]
pub struct PlayerEnactForceEvent {
    pub force: Velocity,
}

fn handle_player_enact_force_event(
    mut events: EventReader<PlayerEnactForceEvent>,
    mut player_q: Query<&mut Velocity, With<LogicalPlayer>>,
) {
    let mut player_velocity = player_q.single_mut();

    for event in events.read() {
        player_velocity.angvel += event.force.angvel;
        player_velocity.linvel += event.force.linvel;
    }
}

fn handle_explosions(
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