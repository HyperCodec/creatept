use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_xpbd_3d::prelude::*;

use super::{Player, PlayerCamera};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorGrabbed>()
            .init_resource::<MouseMoveState>()
            .add_systems(Update, (
                handle_w.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyW)),
                handle_s.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyS)),
                handle_a.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyA)),
                handle_d.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::KeyD)),
                handle_space.run_if(|input: Res<ButtonInput<KeyCode>>| input.pressed(KeyCode::Space)),
                player_look.run_if(|grabbed: Res<CursorGrabbed>| grabbed.0),
                toggle_grab_cursor.run_if(|input: Res<ButtonInput<KeyCode>>| input.just_pressed(KeyCode::Escape)),
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

#[derive(Resource, Default)]
struct MouseMoveState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// credit to https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs (there's still no crate with this functionality so have to skid it)
fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<MouseMoveState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    // not adding settings bc not distributing this
    const SENS: f32 = 0.0001;

    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (SENS * ev.delta.y * window_scale).to_radians();
                        yaw -= (SENS * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

#[derive(Resource, Default)]
pub struct CursorGrabbed(pub bool);

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}