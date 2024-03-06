mod timer;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(timer::TimerUIPlugin)
    }
}