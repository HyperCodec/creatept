use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) { 
        app
            .add_event::<LevelLoadRequest>()
            .add_event::<LevelLoaded>()
            .add_systems(Startup, init_levels)
            .add_systems(Update, load_level);
    }
}

#[derive(Resource)]
pub struct LevelManager {
    pub current_level: usize,
    pub levels: Vec<Level>,
}

pub struct Level {
    pub name: String,
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

#[derive(Component)]
pub struct Persistent;

fn load_level(
    entities: Query<Entity, Without<Persistent>>,
    mut commands: Commands,
    mut level_manager: ResMut<LevelManager>,
    mut level_load_request: EventReader<LevelLoadRequest>,
    mut level_loaded: EventWriter<LevelLoaded>,
) {
    for ev in level_load_request.read() {
        level_manager.current_level = ev.level;
        if let Some(level) = level_manager.levels.get(ev.level) {
            // unload previous level
            for entity in entities.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // spawn new level
            commands.spawn((
                SceneBundle {
                    scene: level.scene.clone(),
                    ..default()
                },
                AsyncSceneCollider::default(),
                RigidBody::Fixed,
            ));
            level_loaded.send(LevelLoaded { level: ev.level });
        }
    }
}

pub fn init_levels(
    _asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // TODO load the actual scenes
    commands.insert_resource(LevelManager {
        current_level: 0,
        levels: vec![
            Level {
                name: "level1".to_string(),
                scene: Handle::default(),
            },
            Level {
                name: "level2".to_string(),
                scene: Handle::default(),
            },
        ],
    });
}