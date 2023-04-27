use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use rand::random;

use crate::enemy::{Enemy, EnemyDifficultyEnum};
use crate::globals::{AUDIO_IMPACT_DIR, INITIAL_ENEMY_NUMBER, NO_WINDOW_ERROR, SPRITE_BALL_DIR};

pub fn get_enemy_texture(difficulty: EnemyDifficultyEnum) -> String {
    match difficulty {
        EnemyDifficultyEnum::Easy => format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"),
        EnemyDifficultyEnum::Medium => format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"),
        EnemyDifficultyEnum::Hard => format!("{}/{}", SPRITE_BALL_DIR, "ball_red_large.png"),
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
        let enemy = Enemy(250.);

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let sprite_size: f32 = 128.;

        let sprite = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            transform: Transform::from_xyz(random_x, random_y, 0.),
            texture: asset_server.load(get_enemy_texture(EnemyDifficultyEnum::Easy)),
            ..default()
        };

        let velocity = Velocity::zero();

        let rigid_body = RigidBody::Dynamic;
        let collider = Collider::ball(sprite_size / 2.0);

        commands.spawn((sprite, enemy, velocity, rigid_body, collider));
    });
}
