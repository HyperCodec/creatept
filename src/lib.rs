use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod environment;
mod debug;
mod ui;
mod common_assets;

pub fn apply_game_plugins(mut app: App) -> App {
    app
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),

            common_assets::AssetsLoaderPlugin,

            player::PlayerPlugins,
            environment::EnvironmentPlugins,

            ui::UIPlugins,
        ));

    app
}