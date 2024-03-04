use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .with_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle::default());
}