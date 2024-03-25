mod player;
mod environment;
mod debug;
mod ui;
mod common_assets;

pub use creatept::GameState;
pub use creatept::handle_empty_event;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
        
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CreatePT Game".to_string(),
                ..default()
            }),
            ..default()
        }),
        bevy_egui::EguiPlugin,
        debug::DebugPlugins,
    ));

    app = creatept::apply_game_plugins(app);
    
    app.run();
}