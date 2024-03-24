pub mod explosive;
pub mod bomb;

use explosive::*;

use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

use crate::{environment::spawn_cycle::end_level, GameState};

pub struct SpecialMovementPlugin;

// TODO turn into plugin group
impl Plugin for SpecialMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerEnactForceEvent>()
            .add_systems(Update, (
                bomb::spawn_bomb.run_if(|inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(KeyCode::KeyR)), // temp keybind
                bomb::tick_bombs.after(crate::environment::fx::init_sfx),

                (
                    handle_explosions
                        .after(end_level),
                    handle_player_enact_force_event,
                )
                    .chain()
                    .run_if(|state: Res<GameState>| state.is_playing()),
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