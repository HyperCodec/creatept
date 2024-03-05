pub mod input;
mod setup;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::PlayerSpawnPlugin)
            .add(input::InputPlugin)
    }
}

#[derive(Component)]
pub struct Player;