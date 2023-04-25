use bevy::prelude::*;
use rand::random;

pub enum EnemyDifficultyEnum {
    Easy,
    Medium,
    Hard,
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub difficulty: EnemyDifficultyEnum,
    pub movement_speed: f32,
    pub size: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        let random_x = random::<f32>();
        let random_y = random::<f32>();

        let direction = Vec2::new(random_x, random_y).normalize();

        Self {
            direction,
            difficulty: EnemyDifficultyEnum::Easy,
            movement_speed: 200.0,
            size: 128.,
        }
    }
}
