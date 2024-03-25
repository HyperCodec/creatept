use bevy::prelude::*;
use crate::{common_assets::CommonAssets, environment::{level_loading::{cleanup, LevelCleanup}, spawn_cycle::{end_level, EndLevelEvent}, EnvironmentTime}, handle_empty_event, GameState};

use self::{level_browser::ReturnToMenuEvent, timer::timer_string};

use super::*;

pub struct LevelEndUIPlugin;

impl Plugin for LevelEndUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_empty_event!(setup_level_end_ui
                    .after(end_level), EndLevelEvent),
                handle_return_click
                    .run_if(|state: Res<GameState>| state.is_menu()),
            ));
    }
}

#[derive(Component)]
pub(super) struct ReturnButton;

fn setup_level_end_ui(
    common_assets: Res<CommonAssets>,
    etime: Res<EnvironmentTime>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera,
        LevelCleanup,
    ));

    let text_style = TextStyle {
        font: common_assets.times_new_roman.clone(),
        font_size: 25.,
        color: Color::Rgba {
            red: 0., 
            green: 0., 
            blue: 0.,
            alpha: 1.,
        },
    };

    // root node
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        MenuContent,
        LevelCleanup,
    ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Level Complete!\n{}", timer_string(etime.time.elapsed())),
                text_style.clone(),
            ));

            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                ..default()
            })
                .insert(ReturnButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Level Menu",
                        text_style.clone(),
                    ));
            });
        });
}

pub fn handle_return_click(
    button_q: Query<&Interaction, (Changed<Interaction>, With<ReturnButton>)>,
    entities: Query<Entity, With<LevelCleanup>>,
    mut open_browser: EventWriter<ReturnToMenuEvent>,
    mut commands: Commands,
) {
    for interaction in button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                cleanup(&entities, &mut commands);
                open_browser.send(ReturnToMenuEvent);
            }
            _ => {}
        }
    }
}