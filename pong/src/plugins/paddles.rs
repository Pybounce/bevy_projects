use bevy::prelude::*;
use crate::components::velocity::*;
use crate::systems::paddle_movement::move_paddle;
use crate::systems::movement::apply_velocity;

pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_paddles_config)
        .add_systems(Startup, spawn_paddles)
        .add_systems(Update, (move_paddle, apply_velocity).chain());
}
}

#[derive(Component)]
pub enum Paddle {
    RightPaddle,
    LeftPaddle
}

#[derive(Resource, Default)]
struct PaddlesConfig
{
    l_paddle: PaddleConfig,
    r_paddle: PaddleConfig
}

struct PaddleConfig {
    colour: Vec4,
    size: Vec2,
    position: Vec2
}

impl Default for PaddleConfig {
    fn default() -> Self {
        Self {
            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
            size: Vec2::new(15.0, 100.0),
            position: Vec2::new(0.0, 0.0)
        }
    }
}

fn spawn_paddles(mut commands: Commands, game_config: Res<PaddlesConfig>) {
    spawn_paddle(&mut commands, &game_config.l_paddle, Paddle::LeftPaddle);
    spawn_paddle(&mut commands, &game_config.r_paddle, Paddle::RightPaddle);
}

fn spawn_paddle(commands: &mut Commands, paddle_config: &PaddleConfig, paddle_component: Paddle) {
    commands.spawn((paddle_component, Velocity {current: default()}, SpriteBundle {
        transform: Transform {
            translation: paddle_config.position.extend(0.0),
            scale: paddle_config.size.extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: paddle_config.colour.into(),
            ..default()
        },
        ..default()
    }));
}

fn setup_paddles_config(mut commands: Commands) {
    let mut paddles_config = PaddlesConfig::default();
    paddles_config.l_paddle.position.x = -500.0;
    paddles_config.l_paddle.colour = Vec4::new(0.7, 0.0, 0.0, 1.0);
    paddles_config.r_paddle.position.x = 500.0;
    paddles_config.r_paddle.colour = Vec4::new(0.0, 0.0, 0.7, 1.0);

    commands.insert_resource(paddles_config);
}

