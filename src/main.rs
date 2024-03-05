mod player;
mod environment;
mod debug;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "CreatePT Game".to_string(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            player::PlayerPlugins,
            environment::EnvironmentPlugins,
            bevy_egui::EguiPlugin,
            debug::DebugPlugins,
        ))
        .run();
}