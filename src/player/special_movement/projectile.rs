use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::explosive::*;

#[derive(Bundle)]
pub struct ExplosiveProjectileBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub explosion: ExplodeOnTouch,
    pub active_events: ActiveEvents,
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
    explosive_query: Query<(Entity, &ExplodeOnTouch, &Transform)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        for (entity, explode, transform) in explosive_query.iter() {
            info!("Explosive collision");
    
            if let CollisionEvent::Started(e1, e2, _) = event {
                if *e1 != entity && *e2 != entity {
                    continue;
                }

                commands.spawn(ExplosionBundle {
                    explosion: explode.into(),
                    transform: Transform::from_translation(transform.translation).into(),
                });
        
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}