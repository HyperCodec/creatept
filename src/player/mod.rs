pub mod core;
mod special_movement;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_fps_controller::controller::FpsControllerPlugin;

pub struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::PlayerCorePlugin)
            .add(FpsControllerPlugin)
            .add(special_movement::SpecialMovementPlugin)
    }
}

#[derive(Component)]
pub struct PlayerCamera;