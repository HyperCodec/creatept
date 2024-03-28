use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod common_assets;
mod debug;
mod environment;
mod player;
mod ui;

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
    app.init_resource::<GameState>().add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        HanabiPlugin,
        common_assets::AssetsLoaderPlugin,
        player::PlayerPlugins,
        environment::EnvironmentPlugins,
        ui::UIPlugins,
    ));

    app
}

#[macro_export]
macro_rules! handle_empty_event {
    ($S: expr, $T: ty) => {
        $S.run_if(|mut ev: EventReader<$T>| {
            let b = !ev.is_empty();

            if b {
                ev.clear();
            }

            b
        })
    };
}
