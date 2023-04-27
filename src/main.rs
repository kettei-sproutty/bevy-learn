use bevy::app::StartupSet::Startup;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

mod camera;
mod enemy;
mod globals;
mod physics;
mod player;
mod states;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .add_state::<states::AppState>()
        .add_system(camera::setup_camera.on_startup())
        .add_system(systems::toggle_app_state)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .run();
}
