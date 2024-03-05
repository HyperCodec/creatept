use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_egui::EguiContext;
use bevy_inspector_egui::prelude::*;

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add(DebugPlugin)
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugEnabled>()
            .add_systems(Update, 
                toggle_debug
                .run_if(|inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(KeyCode::F7)));
    }
}

#[derive(Resource, Default)]
pub struct DebugEnabled(pub bool);

fn toggle_debug(
    mut debug_mode: ResMut<DebugEnabled>,
) {
    debug_mode.0 = !debug_mode.0;
}

fn inspector_ui(
    debug_mode: Res<DebugEnabled>,
    world: &mut World,
) {
    if !debug_mode.0 {
        return;
    }

    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Inspector").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}