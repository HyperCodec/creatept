use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{environment::EnvironmentTime, GameState};

pub struct TimerUIPlugin;

impl Plugin for TimerUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,display_timer
                .run_if(|state: Res<GameState>| state.is_playing()));
    }
}

fn display_timer(
    etime: Res<EnvironmentTime>,
    mut contexts: EguiContexts,
) {
    let el = etime.time.elapsed();

    egui::Window::new("Timer").show(contexts.ctx_mut(), |ui| {
        ui.label(timer_string(el));
    });
}

pub fn timer_string(elapsed: Duration) -> String {
    format!("Time: {}:{:.3}", elapsed.as_secs() / 60, elapsed.as_secs_f32() % 60.)
}