use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;

mod player;
mod environment;
mod debug;
mod ui;
pub mod common_assets;

#[derive(Resource, Default, PartialEq)]
pub enum GameState {
    #[default]
    Menu,

    Playing,
}

impl GameState {
    pub fn is_playing(&self) -> bool {
        matches!(self, GameState::Playing)
    }

    pub fn is_menu(&self) -> bool {
        matches!(self, GameState::Menu)
    }
}

pub fn apply_game_plugins(mut app: App) -> App {
    app
        .init_resource::<GameState>()
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