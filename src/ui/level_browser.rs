use bevy::prelude::*;

use crate::{common_assets::{self, CommonAssets}, environment::level_loading::{LevelCleanup, LevelLoadRequest, LevelManager}, handle_empty_event, GameState};

use self::level_end::handle_return_click;

use super::*;

pub struct LevelBrowserPlugin;

impl Plugin for LevelBrowserPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReturnToMenuEvent>()
            .add_systems(Startup, setup
                .after(common_assets::init_assets))
            .add_systems(Update, (
                handle_load_click
                    .run_if(|state: Res<GameState>| state.is_menu()),
                handle_empty_event!(setup
                    .before(handle_return_click), ReturnToMenuEvent),
            ));
    }
}

#[derive(Event)]
pub struct ReturnToMenuEvent;

#[derive(Component)]
struct LoadLevelButton(usize);

fn setup(
    common_assets: Res<CommonAssets>,
    levels: Res<LevelManager>,
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
            // level loading buttons
            for (i, level) in levels.levels.iter().enumerate() {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.),
                            height: Val::Px(50.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    LoadLevelButton(i),
                ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(level.name.clone(), text_style.clone()));
                    });
            }
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