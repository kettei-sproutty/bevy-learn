use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use crate::player::systems::*;

pub struct PlayerPlugin {}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement);
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
