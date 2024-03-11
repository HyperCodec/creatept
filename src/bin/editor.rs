use bevy::{diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, prelude::*};
use bevy_editor_pls::prelude::*;

fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "CreatePT Editor".to_string(),
                    ..default()
                }),
                ..default()
            }),
            
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            EditorPlugin::default(),
    ));

    app = creatept::apply_game_plugins(app);

    app.run();
}