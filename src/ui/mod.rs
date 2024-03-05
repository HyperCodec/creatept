mod timer;

use bevy::prelude::{app::PluginGroupBuilder, *};

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(timer::TimerUIPlugin)
    }
}