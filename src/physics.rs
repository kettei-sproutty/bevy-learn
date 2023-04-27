use crate::globals::NO_WINDOW_ERROR;
use anyhow::Context;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query
        .get_single()
        .with_context(|| NO_WINDOW_ERROR)
        .unwrap();

    let x_axis = window.width();
    let y_axis = window.height();

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
        Collider::cuboid(x_axis, 1.),
        RigidBody::Fixed,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., y_axis, 0.0)),
        Collider::cuboid(x_axis, 1.),
        RigidBody::Fixed,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
        Collider::cuboid(1., y_axis),
        RigidBody::Fixed,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(x_axis, 0., 0.0)),
        Collider::cuboid(1., y_axis),
        RigidBody::Fixed,
    ));
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physics);
    }
}
