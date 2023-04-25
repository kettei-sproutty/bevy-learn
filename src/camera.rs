use crate::errors::NO_WINDOW_ERROR;
use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    };

    commands.spawn(camera);
}
