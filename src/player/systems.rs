use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::errors::NO_WINDOW_ERROR;
use crate::globals::SPRITE_BALL_DIR;

use crate::player::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    let center_x = window.width() / 2.;
    let center_y = window.height() / 2.;

    let player = Player::default();

    let sprite = SpriteBundle {
        transform: Transform::from_xyz(center_x, center_y, 0.),
        texture: asset_server.load(format!("{}/{}", SPRITE_BALL_DIR, "ball_blue_large_alt.png")),
        ..default()
    };

    commands.spawn((sprite, player));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Movement Up Handler
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }

        // Movement Left Handler
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }

        // Movement Right Handler
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }

        // Movement Down Handler
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0. {
            direction += direction.normalize();
        }

        player_transform.translation += direction * player.movement_speed * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let window = window_query
            .get_single()
            .with_context(|| NO_WINDOW_ERROR)
            .unwrap();

        let half_player_size = player.sprite_size / 2.;
        let x_minimum = 0. + half_player_size;
        let x_maximum = window.width() - half_player_size;
        let y_minimum = 0. + half_player_size;
        let y_maximum = window.height() - half_player_size;

        let mut player_translation = player_transform.translation;

        if player_translation.x < x_minimum {
            player_translation.x = x_minimum
        }

        if player_translation.x > x_maximum {
            player_translation.x = x_maximum
        }

        if player_translation.y < y_minimum {
            player_translation.y = y_minimum
        }

        if player_translation.y > y_maximum {
            player_translation.y = y_maximum
        }

        player_transform.translation = player_translation;
    }
}
