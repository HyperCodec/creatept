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
            let strength = (1. - distance / explosion.radius) * explosion.force;

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

// most of this is copied from the example in the bevy_hanabi repo, but modified to make it look more like an explosion
pub fn generate_explosion_particles(
    effects: &mut ResMut<Assets<EffectAsset>>,
    radius: f32,
) -> Handle<EffectAsset> {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));
    size_gradient1.add_key(0.3, Vec2::splat(0.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -8.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(5.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(radius).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(20.) + writer.lit(60.)).expr(),
    };

    let effect = EffectAsset::new(
        32768,
        Spawner::burst(2500.0.into(), 2.0.into()),
        writer.finish(),
    )
        .with_name("explosion")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
        });

    effects.add(effect)
}