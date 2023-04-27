use bevy::prelude::*;

mod resources;
mod systems;

#[derive(Default)]
pub enum EnemyDifficultyEnum {
    #[default]
    Easy,
    Medium,
    Hard,
}

#[derive(Component)]
pub struct Enemy(f32);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::spawn_enemies);
    }
}
