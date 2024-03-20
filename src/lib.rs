use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;

mod player;
mod environment;
mod debug;
mod ui;
pub mod common_assets;

pub fn apply_game_plugins(mut app: App) -> App {
    app
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            HanabiPlugin,

            common_assets::AssetsLoaderPlugin,

            player::PlayerPlugins,
            environment::EnvironmentPlugins,

            ui::UIPlugins,
        ));

    app
}