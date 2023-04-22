use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    let sprite = SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        texture: asset_server.load("sprites/ball/ball_red_large.png"),
        ..default()
    };

    let player = Player {};

    commands.spawn((sprite, player));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query
        .get_single()
        .with_context(|| "Cannot find window")
        .unwrap();

    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    };

    commands.spawn(camera);
}

fn main() {
    App::new()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}
