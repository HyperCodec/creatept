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
    let el = etime.time.elapsed();

    egui::Window::new("Timer").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Time: {}:{:.3}", el.as_secs() / 60, el.as_secs_f32() % 60.));
    });
}