use bevy::prelude::*;
use crate::{common_assets::CommonAssets, environment::spawn_cycle::SpawnpointBundle, player::core::setup_player};

use super::LevelLoaded;

pub(super) fn load_test_level(
    common_assets: Res<CommonAssets>,
    mut events: EventReader<LevelLoaded>,
    mut commands: Commands,
) {
    for e in events.read() {
        if e.level == 0 {
            setup_player(&common_assets, &mut commands, Transform::default());

            commands.spawn(SpawnpointBundle {
                transform: Transform::from_xyz(0., 0.1, 0.).into(),
                ..default()
            });
        }
    }
}