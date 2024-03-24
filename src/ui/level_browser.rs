use bevy::prelude::*;

use crate::{common_assets::{self, CommonAssets}, environment::level_loading::{LevelCleanup, LevelLoadRequest}, GameState};

use super::*;

pub struct LevelBrowserPlugin;

impl Plugin for LevelBrowserPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup
                .after(common_assets::init_assets))
            .add_systems(Update, (
                handle_load_click
                    .run_if(|state: Res<State<GameState>>| state.is_menu()),
            ));
    }
}

#[derive(Event)]
pub struct ReturnToMenuEvent;

#[derive(Component)]
struct LoadLevelButton(usize);

fn setup(
    common_assets: Res<CommonAssets>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera,
        LevelCleanup,
    ));

    // root node
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        MenuContent,
        LevelCleanup,
    ))
        .with_children(|parent| {
            // button to load test level, centered
            // TODO iterate through levels and make buttons for each
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(10.),
                        height: Val::Percent(10.),
                        ..default()
                    },
                    ..default()
                },
                LoadLevelButton(0),
            ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Load Test Level",
                            TextStyle {
                                font: common_assets.times_new_roman.clone(),
                                font_size: 20.,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn handle_load_click(
    button_q: Query<(&LoadLevelButton, &Interaction), Changed<Interaction>>,
    mut requester: EventWriter<LevelLoadRequest>,
) {
    for (LoadLevelButton(level_index), interaction) in button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                requester.send(LevelLoadRequest {
                    level: *level_index,
                });
            }
            _ => {}
        }
    }
}