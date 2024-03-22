mod setup;
pub mod spawn_cycle;
pub mod fx;
pub mod level_loading;
pub mod jump_pad;

use bevy::{app::PluginGroupBuilder, prelude::*, time::Stopwatch};

use level_loading::LevelLoaded;

pub struct EnvironmentPlugins;

impl PluginGroup for EnvironmentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::EnvironmentSetupPlugin)
            .add(EnvironmentBasePlugin)
            .add(fx::FxPlugin)
            .add(level_loading::LevelLoadingPlugin)
            .add(jump_pad::JumpPadPlugin)
    }
}

struct EnvironmentBasePlugin;

impl Plugin for EnvironmentBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnvironmentTime>()
            .add_systems(Update, (
                tick_time.run_if(|etime: Res<EnvironmentTime>| etime.is_ticking),
                //start_timer.run_if(|events: EventReader<LevelLoaded>| !events.is_empty()),

                // testing timer
                start_timer.run_if(|inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(KeyCode::KeyT)),
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