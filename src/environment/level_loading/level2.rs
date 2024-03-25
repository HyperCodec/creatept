use bevy::prelude::*;

use crate::{common_assets::CommonAssets, environment::spawn_cycle::{Goal, GoalBundle, SpawnpointBundle}, player::core::setup_player};

use super::LevelLoaded;

pub(super) fn load_level_2(
    common_assets: Res<CommonAssets>,
    mut events: EventReader<LevelLoaded>,
    mut commands: Commands,
) {
    for e in events.read() {
        if e.level == 2 {
            setup_player(&common_assets, &mut commands, Transform::default());

            // dynamic scene format is still in the works so i have to do this manually
            commands.spawn(SpawnpointBundle {
                transform: Transform::from_xyz(0., 0.1, 0.).into(),
                ..default()
            });

            commands.spawn(GoalBundle {
                goal: Goal {
                    size: 4.5,
                },
                transform: Transform::from_xyz(0., 0., -45.).into(),
                ..default()
            });

            // TODO tutorial text for bomb spawning
        }
    }
}