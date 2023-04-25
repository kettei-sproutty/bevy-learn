use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub is_invincible: bool,
    pub movement_speed: f32,
    pub sprite_size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_invincible: true,
            movement_speed: 500.,
            sprite_size: 128.,
        }
    }
}
