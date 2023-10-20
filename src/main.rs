use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ball;
mod paddle;

use ball::BallPlugin;
use paddle::PaddlePlugin;

pub const VIEW_WIDTH: f32 = 256.0;
pub const VIEW_HEIGHT: f32 = 144.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Pong Clone".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins((PaddlePlugin, BallPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: VIEW_WIDTH,
        min_height: VIEW_HEIGHT,
    };

    commands.spawn(camera);
}
