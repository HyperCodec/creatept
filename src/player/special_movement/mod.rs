pub mod explosive;

use explosive::*;

use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

pub struct SpecialMovementPlugin;

impl Plugin for SpecialMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerEnactForceEvent>()
            .add_systems(Update, (
                handle_player_enact_force_event.after(handle_explosions),
                handle_explosions,
            ));
    }
}

#[derive(Event)]
pub struct PlayerEnactForceEvent {
    pub force: Velocity,
}

fn handle_player_enact_force_event(
    mut events: EventReader<PlayerEnactForceEvent>,
    mut player_q: Query<&mut Velocity, With<LogicalPlayer>>,
) {
    let mut player_velocity = player_q.single_mut();

    for event in events.read() {
        player_velocity.angvel += event.force.angvel;
        player_velocity.linvel += event.force.linvel;
    }
}