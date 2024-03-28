use std::f32::consts::TAU;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;

use crate::{
    common_assets::CommonAssets,
    environment::{
        level_loading::{LevelCleanup, LevelLoaded},
        spawn_cycle::EndLevelEvent,
    },
    handle_empty_event, GameState,
};

use super::PlayerCamera;

pub struct PlayerCorePlugin;

impl Plugin for PlayerCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                manage_cursor.run_if(|state: Res<GameState>| state.is_playing()),
                handle_empty_event!(initial_grab_cursor, LevelLoaded),
                handle_empty_event!(ungrab_cursor, EndLevelEvent),
            ),
        );

        #[cfg(target_arch = "wasm32")]
        app.insert_resource(Msaa::Off);
    }
}

pub fn setup_player(common_assets: &Res<CommonAssets>, commands: &mut Commands, pos: Transform) {
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::ZERO, Vec3::Y, 0.25),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(pos),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController::default(),
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .insert(LevelCleanup)
        .id();

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
        RenderPlayer { logical_entity },
        PlayerCamera,
        LevelCleanup,
    ));

    // crosshair (TODO probably move to ui module)
    let text_style = TextStyle {
        font: common_assets.times_new_roman.clone(),
        font_size: 25.,
        color: Color::Rgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 0.5,
        },
    };

    commands
        .spawn(TextBundle::from_section("+", text_style).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.),
            left: Val::Percent(50.),
            ..default()
        }))
        .insert(LevelCleanup);
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

fn initial_grab_cursor(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn ungrab_cursor(
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();

    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
    for mut controller in &mut controller_query {
        controller.enable_input = false;
    }
}
