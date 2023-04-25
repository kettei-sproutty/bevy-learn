use crate::enemy::resources::EnemyTimer;
use crate::enemy::systems::*;
use bevy::prelude::*;

pub struct EnemyPlugin {}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies)
            .init_resource::<EnemyTimer>()
            .add_system(enemy_movement)
            .add_system(confine_enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(spawn_enemy_timer)
            .add_system(spawn_enemy_over_time);
    }
}

impl Default for EnemyPlugin {
    fn default() -> Self {
        Self {}
    }
}
