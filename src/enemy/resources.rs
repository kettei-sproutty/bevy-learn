use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyTimer {
    pub timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(10., TimerMode::Repeating);

        Self { timer }
    }
}
