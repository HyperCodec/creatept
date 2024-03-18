use bevy::prelude::*;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_assets);
    }
}

#[derive(Resource)]
pub struct CommonAssets {
    pub times_new_roman: Handle<Font>,
}

fn init_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(CommonAssets {
        times_new_roman: asset_server.load("font/OPTITimes-Roman.otf")
    });
}