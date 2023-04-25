use bevy::prelude::*;

mod systems;

use crate::player::systems::*;

#[derive(Component)]
pub struct Player(f32);

pub struct PlayerPlugin {}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
