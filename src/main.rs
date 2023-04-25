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

fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_loader: Res<AssetServer>,
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
        let mut is_direction_changed: bool = false;

        let translation = transform.translation;
        if translation.x < minimum || translation.x > x_maximum {
            enemy.direction.x *= -1.;
            is_direction_changed = true;
        }
        if translation.y < minimum || translation.y > y_maximum {
            enemy.direction.y *= -1.;
            is_direction_changed = true;
        }

        if is_direction_changed {
            let sound_effect = asset_loader.load("audio/impact/impactSoft_medium_000.ogg");
            audio.play(sound_effect);
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    for mut transform in enemy_query.iter_mut() {
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

fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SPRITE_SIZE / 2.;
            let enemy_radius = ENEMY_SPRITE_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                if time.elapsed_seconds() > 2.5 {
                    println!("Game over, score: {}", score.value.to_string());
                    let explosion_sound = asset_server.load("audio/sci-fi/explosionCrunch_000.ogg");
                    audio.play(explosion_sound);
                    commands.entity(player_entity).despawn();
                }
            }
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

const TOTAL_STARS: u32 = 10;

#[derive(Component)]
struct Star {}

fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find primary window")
        .unwrap();

    let star_range = 0..TOTAL_STARS;

    star_range.for_each(|_| {
        let star_x = random::<f32>() * window.height();
        let star_y = random::<f32>() * window.width();

        let asset = asset_server.load("sprites/ball/star.png");

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(star_x, star_y, 0.),
            texture: asset,
            ..default()
        };

        let star = Star {};

        commands.spawn((sprite, star));
    })
}

const STAR_SPRITE_SIZE: f32 = 30.;

fn player_hit_star(
    mut commands: Commands,
    star_entity_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_entity_query.iter() {
            let player_half_size = PLAYER_SPRITE_SIZE / 2.;
            let star_half_size = STAR_SPRITE_SIZE / 2.;
            let distance = star_transform
                .translation
                .distance(player_transform.translation);

            if distance < player_half_size + star_half_size {
                score.value += 1;
                let hit_sound = asset_server.load("audio/ui/tick_001.ogg");
                audio.play(hit_sound);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

#[derive(Resource)]
struct Score {
    value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

const STAR_SPAWN_DURATION: f32 = 1.;
#[derive(Resource)]
struct SpawnStarTimer {
    timer: Timer,
}

impl Default for SpawnStarTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(STAR_SPAWN_DURATION, TimerMode::Repeating);

        Self { timer }
    }
}

fn timer_tick(time: Res<Time>, mut star_spawn_timer: ResMut<SpawnStarTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<SpawnStarTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let star_x = random::<f32>() * window.height();
        let star_y = random::<f32>() * window.width();

        let asset = asset_server.load("sprites/ball/star.png");

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(star_x, star_y, 0.),
            texture: asset,
            ..default()
        };

        let star = Star {};

        commands.spawn((sprite, star));
    }
}

fn main() {
    App::new()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_stars)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .init_resource::<Score>()
        .init_resource::<SpawnStarTimer>()
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(timer_tick)
        .add_system(spawn_stars_over_time)
        .run();
}
