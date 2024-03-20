use bevy::prelude::*;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, despawn_after_time);
    }
}

#[derive(Component)]
pub struct DespawnAfterTime {
    pub timer: Timer,
}

fn despawn_after_time(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut DespawnAfterTime)>,
) {
    for (entity, mut despawn) in q.iter_mut() {
        despawn.timer.tick(time.delta());
        if despawn.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}