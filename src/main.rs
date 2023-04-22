use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::prelude::*;

use rand::prelude::*;

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

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

        let random_x_movement = random::<f32>();
        let random_y_movement = random::<f32>();

        let direction = Vec2::new(random_x_movement, random_y_movement).normalize();

        let enemy = Enemy { direction };

        commands.spawn((sprite, enemy));
    })
}

const ENEMY_MOVEMENT_SPEED: f32 = 250.;
const ENEMY_SPRITE_SIZE: f32 = 128.;

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_MOVEMENT_SPEED * time.delta_seconds();
    }
}

fn change_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot get window")
        .unwrap();

    let half_enemy_size: f32 = ENEMY_SPRITE_SIZE / 2.;
    let minimum = 0. + half_enemy_size;
    let x_maximum = window.width() - half_enemy_size;
    let y_maximum = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < minimum || translation.x > x_maximum {
            enemy.direction.x *= -1.;
        }
        if translation.y < minimum || translation.y > y_maximum {
            enemy.direction.y *= -1.;
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let half_enemy_size = ENEMY_SPRITE_SIZE / 2.;
        let minimum = 0. + half_enemy_size;
        let x_maximum = window.width() - half_enemy_size;
        let y_maximum = window.height() - half_enemy_size;

        if transform.translation.x < minimum {
            transform.translation.x = minimum
        }

        if transform.translation.x > x_maximum {
            transform.translation.x = x_maximum
        }

        if transform.translation.y < minimum {
            transform.translation.y = minimum
        }

        if transform.translation.y > y_maximum {
            transform.translation.y = y_maximum
        }
    }
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
        .add_plugin(EditorPlugin::default())
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(change_enemy_movement)
        .add_system(confine_enemy_movement)
        .run();
}
