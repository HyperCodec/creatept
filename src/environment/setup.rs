use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

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
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y))),
            material: materials.add(StandardMaterial::from(Color::GREEN)),
            ..default()
        },
        RigidBody::Static,
        AsyncCollider(ComputedCollider::TriMesh),
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