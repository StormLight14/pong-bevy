use crate::paddle::{Paddle, BORDER_HEIGHT_FROM_MIDDLE, HALF_PADDLE_HEIGHT, HALF_PADDLE_WIDTH};
use crate::{Player, Score, VIEW_HEIGHT, VIEW_WIDTH};
use bevy::prelude::*;
use rand::seq::SliceRandom;

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

    let possible_nums = vec![-1.0, 1.0];

    let (rand_one, rand_two) = (
        possible_nums.choose(&mut rand::thread_rng()).unwrap(),
        possible_nums.choose(&mut rand::thread_rng()).unwrap(),
    );

    info!("{:?}", rand_one);

    commands.spawn((
        SpriteBundle {
            texture: ball_texture,
            ..default()
        },
        Ball {
            velocity: Vec3::new(rand_one * BALL_SPEED, rand_two * BALL_SPEED, 0.0),
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
                if paddle.player == Player::One {
                    if ball_transform.translation.x - HALF_BALL_WIDTH
                        <= paddle_transform.translation.x + HALF_PADDLE_WIDTH
                        && ball_transform.translation.x - HALF_BALL_WIDTH
                            >= paddle_transform.translation.x - HALF_PADDLE_WIDTH
                    {
                        ball.velocity.x *= -1.0;
                        ball_transform.translation.x += 1.0;
                    }
                }

                if paddle.player == Player::Two {
                    if ball_transform.translation.x + HALF_BALL_WIDTH
                        >= paddle_transform.translation.x - HALF_PADDLE_WIDTH
                    {
                        ball.velocity.x *= -1.0;
                        ball_transform.translation.x -= 1.0;
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
    mut score: ResMut<Score>,
) {
    for (ball_entity, ball_transform) in query.iter() {
        if ball_transform.translation.x < -VIEW_WIDTH / 2.0
            || ball_transform.translation.x > VIEW_WIDTH / 2.0
        {
            let possible_nums = vec![-1.0, 1.0];

            let (rand_one, rand_two) = (
                possible_nums.choose(&mut rand::thread_rng()).unwrap(),
                possible_nums.choose(&mut rand::thread_rng()).unwrap(),
            );

            let ball_texture = asset_server.load("ball.png");

            commands.entity(ball_entity).despawn();

            commands.spawn((
                SpriteBundle {
                    texture: ball_texture,
                    ..default()
                },
                Ball {
                    velocity: Vec3::new(rand_one * BALL_SPEED, rand_two * BALL_SPEED, 0.0),
                },
                Name::from("Ball"),
            ));

            if ball_transform.translation.x > VIEW_WIDTH / 2.0 {
                score.0 += 1;
            }
            if ball_transform.translation.x < -VIEW_WIDTH / 2.0 {
                score.1 += 1;
            }
        }
    }
}
