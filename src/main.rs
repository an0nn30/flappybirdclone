use crate::game::GamePlugin;
use crate::player::PlayerPlugin;
use crate::world::World;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

mod game;
mod player;
mod systems;
mod world;

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (288., 512.).into(),
                        title: "Flappy Bird Clone".into(),
                        decorations: true,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())),
        )
        .add_plugins(World)
        .add_plugins(GamePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
