use anyhow::Context;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod constants;
mod enemy;
mod errors;
mod player;

use crate::enemy::plugins::EnemyPlugin;
use crate::errors::NO_WINDOW_ERROR;
use crate::player::plugins::PlayerPlugin;

fn exit_game(keyboard: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
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

fn main() {
    App::new()
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin::default())
        .add_plugin(EnemyPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_system(exit_game)
        .run();
}
