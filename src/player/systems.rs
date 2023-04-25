use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::{Collider, RapierConfiguration, RigidBody, Velocity};

use crate::globals::{NO_WINDOW_ERROR, SPRITE_BALL_DIR};

use crate::player::Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    let center_x = window.width() / 2.;
    let center_y = window.height() / 2.;

    let player = Player(500.);

    let sprite_size: f32 = 128.;

    let sprite = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..default()
        },
        transform: Transform::from_xyz(center_x, center_y, 0.),
        texture: asset_server.load(format!("{}/{}", SPRITE_BALL_DIR, "ball_blue_large_alt.png")),
        ..default()
    };

    let velocity = Velocity::zero();

    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::ball(sprite_size / 2.0);

    commands.spawn((sprite, player, velocity, rigid_body, collider));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Velocity)>,
) {
    if let Ok((player, mut velocity)) = player_query.get_single_mut() {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) as i8;
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) as i8;
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) as i8;
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) as i8;

        let x_axis: i8 = -left + right;
        let y_axis: i8 = -down + up;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);

        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        velocity.linvel = move_delta * player.0;
    }
}
