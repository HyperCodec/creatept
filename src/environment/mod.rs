pub mod spawn_cycle;
pub mod fx;
pub mod level_loading;
pub mod jump_pad;

use bevy::{app::PluginGroupBuilder, prelude::*, time::Stopwatch};

use level_loading::LevelLoaded;

use crate::handle_empty_event;

pub struct EnvironmentPlugins;

impl PluginGroup for EnvironmentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EnvironmentBasePlugin)
            .add(fx::FxPlugin)
            .add(level_loading::LevelLoadingPlugin)
            .add(jump_pad::JumpPadPlugin)
            .add(spawn_cycle::SpawnCyclePlugin)
    }
}

struct EnvironmentBasePlugin;

impl Plugin for EnvironmentBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnvironmentTime>()
            .add_systems(Update, (
                tick_time.run_if(|etime: Res<EnvironmentTime>| etime.is_ticking),
                handle_empty_event!(start_timer, LevelLoaded),
            ));
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

fn start_timer(
    mut timer: ResMut<EnvironmentTime>,
) {
    timer.time.reset();
    timer.is_ticking = true;
}