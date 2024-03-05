mod setup;

use bevy::{app::PluginGroupBuilder, prelude::*, time::Stopwatch};

pub struct EnvironmentPlugins;

impl PluginGroup for EnvironmentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::EnvironmentSetupPlugin)
            .add(EnvironmentBasePlugin)
    }
}

struct EnvironmentBasePlugin;

impl Plugin for EnvironmentBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnvironmentTime>()
            .add_systems(Update, 
                tick_time.run_if(|etime: Res<EnvironmentTime>| etime.is_ticking));
    }
}

#[derive(Resource, Default)]
pub struct EnvironmentTime {
    pub time: Stopwatch,
    pub is_ticking: bool,
}

fn tick_time(
    time: Res<Time>,
    mut etime: ResMut<EnvironmentTime>,
) {
    etime.time.tick(time.delta());
}