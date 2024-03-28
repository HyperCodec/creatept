mod level_browser;
mod level_end;
mod timer;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct MenuContent;

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(timer::TimerUIPlugin)
            .add(level_browser::LevelBrowserPlugin)
            .add(level_end::LevelEndUIPlugin)
    }
}
