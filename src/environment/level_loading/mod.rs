mod test_level;
mod level1;
mod level2;
mod level3;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) { 
        app
            .add_event::<LevelLoadRequest>()
            .add_event::<LevelLoaded>()
            .add_systems(Startup, init_levels)
            .add_systems(Update, (
                load_level,
                change_playing_state
                    .run_if(|events: EventReader<LevelLoaded>| !events.is_empty()),

                (
                    test_level::load_test_level,
                    level1::load_level_1,
                    level2::load_level_2,
                    level3::load_level_3,
                )
                    // could make all of this stuff into a chain statement but this allows me to put the bulky stuff last in the code
                    .after(load_level)
                    .before(change_playing_state),
            ));
    }
}

#[derive(Resource)]
pub struct LevelManager {
    pub current_level: usize,
    pub levels: Vec<Level>,
}

pub struct Level {
    pub name: String,

    // technically bad practice to have these floating around in memory, but
    // these models are small so its fine
    pub scene: Handle<Scene>,
}

#[derive(Event)]
pub struct LevelLoadRequest {
    pub level: usize,
}

#[derive(Event)]
pub struct LevelLoaded {
    pub level: usize,
}

#[derive(Component, Default)]
pub struct LevelCleanup;

fn load_level(
    entities: Query<Entity, With<LevelCleanup>>,
    mut commands: Commands,
    mut level_manager: ResMut<LevelManager>,
    mut level_load_request: EventReader<LevelLoadRequest>,
    mut level_loaded: EventWriter<LevelLoaded>,
) {
    for ev in level_load_request.read() {
        level_manager.current_level = ev.level;
        if let Some(level) = level_manager.levels.get(ev.level) {
            // unload previous level
            cleanup(&entities, &mut commands);

            // spawn new level
            commands.spawn((
                SceneBundle {
                    scene: level.scene.clone(),
                    ..default()
                },
                AsyncSceneCollider {
                    shape: Some(ComputedColliderShape::ConvexHull), // have to do convex hull bc bug https://github.com/dimforge/bevy_rapier/issues/386
                    ..default()
                },
                RigidBody::Fixed,
                LevelCleanup,
            ));
            level_loaded.send(LevelLoaded { level: ev.level });
        }
    }
}

pub fn init_levels(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(LevelManager {
        current_level: 0,
        levels: vec![
            Level {
                name: "test".to_string(),
                scene: asset_server.load("scene/test.gltf#Scene0"),
            },
            Level {
                name: "Simple Jumps".to_string(),
                scene: asset_server.load("scene/level1.gltf#Scene0"),
            },
            Level {
                name: "Bomb Jump".to_string(),
                scene: asset_server.load("scene/level2.gltf#Scene0"),
            },
            Level {
                name: "Multibomb Jump".to_string(),
                scene: asset_server.load("scene/level3.gltf#Scene0"),
            },
        ],
    });
}

fn change_playing_state(
    mut state: ResMut<GameState>,
) {
    *state = GameState::Playing;
}

pub fn cleanup(
    entities: &Query<Entity, With<LevelCleanup>>,
    commands: &mut Commands,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}