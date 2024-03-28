use bevy::prelude::*;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_sfx)
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

#[derive(Resource)]
pub struct Sfx {
    pub explosion: Handle<AudioSource>,
    pub fail: Handle<AudioSource>,
}

pub fn init_sfx(asset_server: Res<AssetServer>, mut commands: Commands) {
    let explosion = asset_server.load("sfx/explosion.ogg");
    let fail = asset_server.load("sfx/fail.ogg");

    commands.insert_resource(Sfx { explosion, fail });
}
