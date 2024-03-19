use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EnvironmentSetupPlugin;

impl Plugin for EnvironmentSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // basic plane for testing
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {
                half_size: Vec3::new(100., 0.01, 100.),
            })),
            material: materials.add(StandardMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(0., -1., 0.),
            ..default()
        },
        RigidBody::Fixed,
        AsyncCollider(ComputedColliderShape::TriMesh),
        Friction {
            coefficient: 1.0,
            ..default()
        }
    ));

    // point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.,
            ..default()
        },
        transform: Transform::from_xyz(0., 10., 0.),
        ..default()
    });
}