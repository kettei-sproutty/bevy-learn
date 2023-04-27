use crate::states::AppState;
use bevy::prelude::*;

mod systems;

#[derive(Component)]
pub struct Player(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(systems::player_movement)
            .add_system(systems::display_intersection_info);
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
