use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::environment::EnvironmentTime;

pub struct TimerUIPlugin;

impl Plugin for TimerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_timer);
    }
}

fn display_timer(
    etime: Res<EnvironmentTime>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Timer").show(contexts.ctx_mut(), |ui| {
        // TODO actual time formatting
        ui.label(format!("Time: {:#?}", etime.time.elapsed()));
    });
}