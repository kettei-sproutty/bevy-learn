use crate::states::AppState;
use bevy::prelude::*;

pub fn toggle_app_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Old State: {:?}", app_state.0);
        match app_state.0 {
            AppState::Paused => next_state.set(AppState::Game),
            AppState::Game => next_state.set(AppState::Paused),
            AppState::Menu => {}
            AppState::GameOver => {}
        }

        println!("New State: {:?}", app_state.0);
    }
}
