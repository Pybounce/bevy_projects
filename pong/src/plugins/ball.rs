use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{AppState, AppStateLifetime, GameState};

use super::paddles::*;

const BALL_SPEED: f32 = 500.0;
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_COUNT: i16 = 1;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_ball)
        .add_systems(Update, (clamp_velocity, check_paddle_collision).run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
    }
}

#[derive(Component)]
pub struct Ball;


fn spawn_ball(mut commands: Commands) {
    for i in 0..BALL_COUNT {
        let y = (i * 40) - ((BALL_COUNT - 1) * 20);
        commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, y.into(), 0.0),
                scale: BALL_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.0))
        .insert(Velocity::linear(Vec2::new(3.0, 0.0)))
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Ball)
        .insert(AppStateLifetime::Game);
    }

}

fn clamp_velocity(mut query: Query<&mut Velocity, With<Ball>>) {
    for mut velocity in &mut query {
        velocity.linvel = (velocity.linvel).normalize() * BALL_SPEED;
    }
}

fn check_paddle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    paddles_config: Res<PaddlesConfig>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddle_query: Query<(&Paddle, &Transform)>,
) {
    for collision_event in collision_events.read() {
        let (entity1, entity2) = match collision_event {
            CollisionEvent::Stopped(e1, e2, _) => { (*e1, *e2) },
            CollisionEvent::Started(_, _, _) => { continue; },
        };

         let (ball_entity, paddle_entity) = if ball_query.get(entity1).is_ok() && paddle_query.get(entity2).is_ok() {
            (entity1, entity2)
        } else if ball_query.get(entity2).is_ok() && paddle_query.get(entity1).is_ok() {
            (entity2, entity1)
        } else {
            continue;
        };

        let (paddle, paddle_transform) = paddle_query.get(paddle_entity).unwrap();
        let paddle_size_y = match paddle {
            Paddle::LeftPaddle => paddles_config.l_paddle.size.y,
            Paddle::RightPaddle => paddles_config.r_paddle.size.y
        };
        let (mut ball_velocity, ball_transform) = ball_query.get_mut(ball_entity).unwrap();
        let y_diff = ball_transform.translation.y - paddle_transform.translation.y;
        let y_diff_normalised = y_diff / paddle_size_y * 2.0;
        let y = (y_diff_normalised * 0.5).min(0.5).max(-0.5);
        let x = (1.0 - y.abs()) * ball_velocity.linvel.x.signum();

        ball_velocity.linvel = (Vec2::new(x, y)) * BALL_SPEED;

    }
}

