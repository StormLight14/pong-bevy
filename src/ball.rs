use crate::paddle::{Paddle, BORDER_HEIGHT_FROM_MIDDLE, HALF_PADDLE_HEIGHT, HALF_PADDLE_WIDTH};
use crate::{VIEW_HEIGHT, VIEW_WIDTH};
use bevy::prelude::*;

const BALL_SPEED: f32 = 50.0;
const HALF_BALL_WIDTH: f32 = 3.0 / 2.0;
const HALF_BALL_HEIGHT: f32 = 3.0 / 2.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, move_balls)
            .add_systems(Update, reset);
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
            velocity: Vec3::new(BALL_SPEED, BALL_SPEED, 0.0),
        },
        Name::from("Ball"),
    ));
}

fn move_balls(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    paddle_query: Query<(&Transform, &Paddle), Without<Ball>>, // USE
) {
    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        ball_transform.translation += ball.velocity * time.delta_seconds();

        if ball_transform.translation.y - HALF_BALL_HEIGHT < -BORDER_HEIGHT_FROM_MIDDLE {
            ball.velocity.y *= -1.0;
            ball_transform.translation.y += 1.0;
        }

        if ball_transform.translation.y + HALF_BALL_HEIGHT > BORDER_HEIGHT_FROM_MIDDLE {
            ball.velocity.y *= -1.0;
            ball_transform.translation.y -= 1.0;
        }

        for (paddle_transform, paddle) in paddle_query.iter() {
            // ball y position == middle y of ball
            // paddle y position == middle y of paddle
            if (ball_transform.translation.y + HALF_BALL_HEIGHT
                >= paddle_transform.translation.y - HALF_PADDLE_HEIGHT
                && ball_transform.translation.y + HALF_BALL_HEIGHT
                    <= paddle_transform.translation.y + HALF_PADDLE_HEIGHT)
                || (ball_transform.translation.y - HALF_BALL_HEIGHT
                    >= paddle_transform.translation.y - HALF_PADDLE_HEIGHT
                    && ball_transform.translation.y - HALF_BALL_HEIGHT
                        <= paddle_transform.translation.y + HALF_PADDLE_HEIGHT)
            {
                if paddle.player == 1 {
                    if ball_transform.translation.x - HALF_BALL_WIDTH
                        <= paddle_transform.translation.x + HALF_PADDLE_WIDTH
                    {
                        ball.velocity.x *= -1.0;
                    }
                }

                if paddle.player == 2 {
                    if ball_transform.translation.x + HALF_BALL_WIDTH
                        >= paddle_transform.translation.x - HALF_PADDLE_WIDTH
                    {
                        ball.velocity.x *= -1.0;
                    }
                }
            }
        }
    }
}

fn reset(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Ball>>,
    asset_server: Res<AssetServer>,
) {
    for (ball_entity, ball_transform) in query.iter() {
        if ball_transform.translation.x < -VIEW_WIDTH / 2.0
            || ball_transform.translation.x > VIEW_WIDTH / 2.0
        {
            commands.entity(ball_entity).despawn();
            let ball_texture = asset_server.load("ball.png");

            commands.spawn((
                SpriteBundle {
                    texture: ball_texture,
                    ..default()
                },
                Ball {
                    velocity: Vec3::new(BALL_SPEED, BALL_SPEED, 0.0),
                },
                Name::from("Ball"),
            ));
        }
    }
}
