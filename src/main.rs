use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    let sprite = SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        texture: asset_server.load("sprites/ball/ball_blue_large_alt.png"),
        ..default()
    };

    let player = Player {};

    commands.spawn((sprite, player));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    };

    commands.spawn(camera);
}

const NUMBER_ENEMIES: i32 = 4;

fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    (0..NUMBER_ENEMIES).for_each(|_| {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let z: f32 = 0.;

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, z),
            texture: asset_server.load("sprites/ball/ball_red_large.png"),
            ..default()
        };

        let enemy = Enemy {};

        commands.spawn((sprite, enemy));
    })
}

const PLAYER_MOVEMENT_SPEED: f32 = 500.;
const PLAYER_SPRITE_SIZE: f32 = 128.;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction += Vec3::new(0., 0., 1.);
        }

        if direction.length() > 0. {
            direction += direction.normalize();
        }

        println!("{:?}", direction);
        transform.translation += direction * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query
            .get_single()
            .with_context(|| "Cannot find window")
            .unwrap();
        let half_player_size: f32 = PLAYER_SPRITE_SIZE / 2.;
        let minimum = 0. + half_player_size;
        let x_maximum = window.width() - half_player_size;
        let y_maximum = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < minimum {
            translation.x = minimum;
        }
        if translation.y < minimum {
            translation.y = minimum
        }
        if translation.x > x_maximum {
            translation.x = x_maximum
        }
        if translation.y > y_maximum {
            translation.y = y_maximum
        }

        player_transform.translation = translation;
    }
}

fn main() {
    App::new()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();
}
