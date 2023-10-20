use crate::Score;
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

#[derive(Component)]
struct ScoreText;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::left(Val::Percent(50.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("{} | {}", score.0, score.1),
                        TextStyle {
                            font_size: 30.0,
                            font: asset_server.load("minecraftia.ttf"),
                            ..default()
                        },
                    ),
                    ..default()
                },
                ScoreText,
            ));
        });
}
