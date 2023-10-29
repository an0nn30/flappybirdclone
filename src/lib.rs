use crate::game::GamePlugin;
use crate::player::PlayerPlugin;
use crate::world::World;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

mod game;
mod player;
mod systems;
mod world;

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins
                .set(setup_window())
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

pub fn run_game() {
    main();
}

// #[cfg(target_os = "ios")]
pub fn ios_entry_point() {
    // iOS-specific setup or modifications
    // ...
    run_game();
}

#[no_mangle]
pub extern "C" fn ios_start_game() {
    ios_entry_point();
}

fn setup_window() -> WindowPlugin {
    #[cfg(target_os = "ios")]
    let window = Window {
        mode: WindowMode::BorderlessFullscreen,
        resizable: false,
        ..default()
    };

    #[cfg(not(target_os = "ios"))]
    let window = Window {
        resolution: (288., 512.).into(),
        title: "Flappy Bird Clone".into(),
        decorations: true,
        resizable: false,
        ..default()
    };

    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}