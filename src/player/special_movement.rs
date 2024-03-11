use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SpecialMovementPlugin;

impl Plugin for SpecialMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (

        ));
    }
}