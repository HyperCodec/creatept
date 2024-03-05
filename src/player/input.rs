use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_xpbd_3d::prelude::*;

use super::Player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorGrabbed>()
            .add_systems(Update, (
                handle_w.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyW)),
                handle_s.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyS)),
                handle_a.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyA)),
                handle_d.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyD)),
                handle_space.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::Space)),
                mouse_camera_movement.run_if(|grabbed: Res<CursorGrabbed>| grabbed.0),
                cursor_grab.run_if(|mouse_button: Res<ButtonInput<MouseButton>>| mouse_button.pressed(MouseButton::Left)),
                cursor_ungrab.run_if(|inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(KeyCode::Escape)),
            ));
    }
}


fn handle_w(
    mut query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    let (transform, mut velocity) = query.single_mut();

    let direction = transform.rotation * Vec3::Z;
    velocity.0 += direction * 10.0;
}

fn handle_s(
    mut query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    let (transform, mut velocity) = query.single_mut();

    let direction = transform.rotation * Vec3::Z;
    velocity.0 -= direction * 10.0;
}

fn handle_a(
    mut query: Query<(&Transform, &mut AngularVelocity), With<Player>>,
) {
    let (transform, mut velocity) = query.single_mut();

    let direction = transform.rotation * Vec3::Y;
    velocity.0 += direction * 10.0;
}

fn handle_d(
    mut query: Query<(&Transform, &mut AngularVelocity), With<Player>>,
) {
    let (transform, mut velocity) = query.single_mut();

    let direction = transform.rotation * Vec3::Y;
    velocity.0 -= direction * 10.0;
}

fn handle_space(
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    let mut velocity = query.single_mut();
    velocity.0 += Vec3::Y * 10.0;
}

fn mouse_camera_movement(
    mut query: Query<(&mut Transform, &mut Rotation), With<Player>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for (mut transform, mut rotation) in query.iter_mut() {
        for event in mouse_motion.read() {
            let delta = Vec2::new(event.delta.x, event.delta.y);
            transform.rotate(Quat::from_rotation_y(-delta.x.to_radians() * 0.1));
            rotation.0 *= Quat::from_rotation_x(-delta.y.to_radians() * 0.1);
        }
    }
}

#[derive(Resource, Default)]
pub struct CursorGrabbed(pub bool);

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut grabbed: ResMut<CursorGrabbed>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;

    grabbed.0 = true;
}

fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut grabbed: ResMut<CursorGrabbed>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;

    grabbed.0 = false;
}