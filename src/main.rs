mod player;
mod environment;
mod debug;
mod ui;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
            RapierPhysicsPlugin::<NoUserData>::default(),

            player::PlayerPlugins,
            environment::EnvironmentPlugins,

            bevy_egui::EguiPlugin,
            ui::UIPlugins,
            debug::DebugPlugins,
        ))
        .run();
}