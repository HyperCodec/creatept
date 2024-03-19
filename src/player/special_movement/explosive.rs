use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_hanabi::prelude::*;

use super::PlayerEnactForceEvent;

#[derive(Component)]
pub struct Explosion {
    pub radius: f32,
    pub force: f32,
}

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub transform: TransformBundle,
}

pub(super) fn handle_explosions(
    explosions_q: Query<(Entity, &Explosion, &Transform)>,
    player_q: Query<&Transform, With<LogicalPlayer>>,
    mut evw: EventWriter<PlayerEnactForceEvent>,
    mut commands: Commands,
) {
    let player_transform = player_q.single();

    for (entity, explosion, transform) in explosions_q.iter() {
        let distance = transform.translation.distance(player_transform.translation).max(1.);
        if distance < explosion.radius {
            let strength = explosion.force / distance;

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

pub fn generate_explosion_particles(
    effects: &mut ResMut<Assets<EffectAsset>>,
    radius: f32,
) -> Handle<EffectAsset> {
    // white, yellow, orange, red - small fading effect
    let mut color_grad = Gradient::new();
    color_grad.add_key(0., Vec4::new(1., 1., 1., 1.));
    color_grad.add_key(0.25, Vec4::new(1., 1., 0., 1.));
    color_grad.add_key(0.5, Vec4::new(1., 0.5, 0., 0.95));
    color_grad.add_key(1., Vec4::new(1., 0., 0., 0.85));

    let mut module = Module::default();

    // randomly initialize particles in a sphere
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(radius),
        dimension: ShapeDimension::Volume,
    };

    // particles go outwards
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };

    let lifetime = module.lit(10.);
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(
        1000,
        Spawner::rate(100.0.into()),
        module,
    )
        .with_name("Explosion")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient: color_grad });

    effects.add(effect)
}