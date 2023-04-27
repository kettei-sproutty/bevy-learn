use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    Game,
    #[default]
    Paused,
    Menu,
    GameOver,
}
