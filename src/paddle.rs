use bevy::prelude::*;

const PADDLE_SPEED: f32 = 400.0;
const PADDLE_DISTANCE_FROM_MIDDLE: f32 = 120.0;
const HALF_PADDLE_WIDTH: f32 = 3.0 / 2.0;
const HALF_PADDLE_HEIGHT: f32 = 12.0 / 2.0;

const BORDER_HEIGHT_FROM_MIDDLE: f32 = 65.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles)
            .add_systems(Update, paddle_movement);
    }
}

#[derive(Component, Default)]
pub struct Paddle {
    speed: f32,
    pub player: u8,
}

fn spawn_paddles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let paddle_texture = asset_server.load("paddle.png");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-PADDLE_DISTANCE_FROM_MIDDLE, 0.0, 0.0),
                ..default()
            },
            texture: paddle_texture.clone(),
            ..default()
        },
        Paddle {
            speed: PADDLE_SPEED,
            player: 1,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(PADDLE_DISTANCE_FROM_MIDDLE, 0.0, 0.0),
                ..default()
            },
            texture: paddle_texture,
            ..default()
        },
        Paddle {
            speed: PADDLE_SPEED,
            player: 2,
        },
    ));
}

fn paddle_movement(
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
    input: Res<Input<KeyCode>>,
    time: ResMut<Time>,
) {
    for (mut paddle_transform, paddle) in paddle_query.iter_mut() {
        if paddle.player == 1 {
            //info!("Paddle 1 y position: {:?}", paddle_transform.translation.y);
            if input.pressed(KeyCode::W) {
                paddle_transform.translation.y += paddle.speed * time.delta_seconds();
            }
            if input.pressed(KeyCode::S) {
                paddle_transform.translation.y -= paddle.speed * time.delta_seconds();
            }
        }

        if paddle.player == 2 {
            if input.pressed(KeyCode::O) {
                paddle_transform.translation.y += paddle.speed * time.delta_seconds();
            }
            if input.pressed(KeyCode::L) {
                paddle_transform.translation.y -= paddle.speed * time.delta_seconds();
            }
        }

        if paddle_transform.translation.y - HALF_PADDLE_HEIGHT < -BORDER_HEIGHT_FROM_MIDDLE {
            paddle_transform.translation.y = -BORDER_HEIGHT_FROM_MIDDLE + HALF_PADDLE_HEIGHT;
        }

        if paddle_transform.translation.y + HALF_PADDLE_HEIGHT > BORDER_HEIGHT_FROM_MIDDLE {
            paddle_transform.translation.y = BORDER_HEIGHT_FROM_MIDDLE - HALF_PADDLE_HEIGHT;
        }
    }
}
