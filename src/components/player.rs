use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::errors::NO_WINDOW_ERROR;

#[derive(Component)]
pub struct Player {
    pub is_invincible: bool,
    pub movement_speed: f32,
    pub sprite_size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_invincible: true,
            movement_speed: 500.,
            sprite_size: 128.,
        }
    }
}

fn spawn_player(
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
        texture: asset_server.load("sprites/ball/ball_blue_large_alt.png"),
        ..default()
    };

    commands.spawn((sprite, player));
}

fn player_movement(
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

fn confine_player_movement(
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

fn handle_player_collision() {
    ()
}

#[derive(Resource)]
struct PlayerTimer {
    timer: Timer,
}

impl Default for PlayerTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(2.5, TimerMode::Once);

        Self { timer }
    }
}

// TODO: Move to event
fn toggle_player_invincibility(
    mut _player_query: Query<&mut Player>,
    _player_timer: Res<PlayerTimer>,
) {
    // if let Ok(mut player) = player_query.get_single_mut() {
    //     if player_timer.timer.finished() {
    //         // TODO: Add visual effect
    //         player.is_invincible = false;
    //     }
    // }
}

fn start_player_timer_tick(mut player_time: ResMut<PlayerTimer>, time: Res<Time>) {
    player_time.timer.tick(time.delta());
}

pub struct PlayerPlugin {}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .init_resource::<PlayerTimer>()
            .add_system(player_movement)
            .add_system(start_player_timer_tick)
            .add_system(confine_player_movement)
            .add_system(toggle_player_invincibility)
            .add_system(handle_player_collision);
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
