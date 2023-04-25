use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::enemy::components::{Enemy, EnemyDifficultyEnum};
use crate::enemy::resources::EnemyTimer;
use crate::globals::{AUDIO_IMPACT_DIR, INITIAL_ENEMY_NUMBER, NO_WINDOW_ERROR, SPRITE_BALL_DIR};

pub fn enemy_texture_handler(enemy: &Enemy, asset_server: &Res<AssetServer>) -> Handle<Image> {
    match enemy.difficulty {
        EnemyDifficultyEnum::Easy => {
            asset_server.load(format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"))
        }
        EnemyDifficultyEnum::Medium => {
            asset_server.load(format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"))
        }
        EnemyDifficultyEnum::Hard => {
            asset_server.load(format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"))
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    let enemy_spawner_range = 0..INITIAL_ENEMY_NUMBER;

    enemy_spawner_range.for_each(|_| {
        let enemy = Enemy::default();
        let texture = enemy_texture_handler(&enemy, &asset_server);

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.),
            texture,
            ..default()
        };

        commands.spawn((sprite, enemy));
    });
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * enemy.movement_speed * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_loader: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let half_enemy_size: f32 = enemy.size / 2.;
        let minimum = 0. + half_enemy_size;
        let x_maximum = window.width() - half_enemy_size;
        let y_maximum = window.height() - half_enemy_size;

        let mut is_direction_changed: bool = false;

        let translation = transform.translation;

        if translation.x <= minimum || translation.x >= x_maximum {
            enemy.direction.x *= -1.;
            is_direction_changed = true;
        }

        if translation.y <= minimum || translation.y >= y_maximum {
            enemy.direction.y *= -1.;
            is_direction_changed = true;
        }

        if is_direction_changed {
            let change_direction_audio =
                format!("{}/{}", AUDIO_IMPACT_DIR, "impactSoft_medium_000.ogg");

            let sound_effect = asset_loader.load(change_direction_audio);
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let half_enemy_size = enemy.size / 2.;
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

pub fn spawn_enemy_timer(mut enemy_timer_spawn: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer_spawn.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    enemy_timer: Res<EnemyTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if enemy_timer.timer.finished() {
        let window = window_query
            .get_single()
            .with_context(|| NO_WINDOW_ERROR)
            .unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let enemy = Enemy::default();

        let texture = enemy_texture_handler(&enemy, &asset_server);

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.),
            texture,
            ..default()
        };

        commands.spawn((sprite, enemy));
    }
}
