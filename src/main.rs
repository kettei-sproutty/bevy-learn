use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod enemy;
mod globals;
mod player;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<states::AppState>()
        .add_startup_system(camera::setup_camera)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
