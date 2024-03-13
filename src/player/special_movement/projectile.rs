use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::explosive::*;

#[derive(Bundle)]
pub struct ExplosiveProjectileBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub explosion: ExplodeOnTouch,
}

#[derive(Component)]
pub struct ExplodeOnTouch {
    pub radius: f32,
    pub force: f32,
    pub damage: f32,
}

impl Into<Explosion> for &ExplodeOnTouch {
    fn into(self) -> Explosion {
        Explosion {
            radius: self.radius,
            force: self.force,
            damage: self.damage,
        }
    }
}

pub(super) fn handle_explosive_collision(
    query: Query<(Entity, &ExplodeOnTouch, &Transform, &CollidingEntities)>,
    mut commands: Commands,
) {
    for (entity, explode, transform, collisions) in query.iter() {
        if collisions.is_empty() {
            continue;
        }
        
        commands.spawn(ExplosionBundle {
            explosion: explode.into(),
            transform: Transform::from_translation(transform.translation).into(),
        });

        commands.entity(entity).despawn();
    }
}