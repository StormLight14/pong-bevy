use crate::paddle::Paddle;
use bevy::prelude::*;

const BORDER_HEIGHT_FROM_MIDDLE: f32 = 65.0;

pub struct BallPlugin;

const BALL_SPEED: f32 = 30.0;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, move_balls);
    }
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_texture = asset_server.load("ball.png");

    commands.spawn((
        SpriteBundle {
            texture: ball_texture,
            ..default()
        },
        Ball {
            velocity: Vec3::new(1.0, 1.0, 0.0),
        },
    ));
}

fn move_balls(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    paddle_query: Query<(&Transform, &Paddle), Without<Ball>>, // USE
) {
    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        ball_transform.translation +=
            ball.velocity * Vec3::new(BALL_SPEED, BALL_SPEED, 0.0) * time.delta_seconds();

        if ball_transform.translation.y < -BORDER_HEIGHT_FROM_MIDDLE {
            ball_transform.translation.y = -BORDER_HEIGHT_FROM_MIDDLE;
            ball.velocity.y *= -1.0;
        }

        if ball_transform.translation.y > BORDER_HEIGHT_FROM_MIDDLE {
            ball_transform.translation.y = BORDER_HEIGHT_FROM_MIDDLE;
            ball.velocity.y *= -1.0;
        }

        for (paddle_transform, paddle) in paddle_query.iter() {
            let paddle_translation = paddle_transform.translation;
        }
    }
}
