use crate::game::GamePlugin;
use crate::pipe::PipePlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::*;

mod game;
mod player;
pub mod score;
mod systems;
mod world;

mod pipe;
mod sounds;
mod ui;

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins
                .set(setup_window())
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace,winit=error".to_string(),
                })),
        )
        .add_state::<GameState>()
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(PipePlugin)
        .add_plugins(ScorePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    #[cfg(target_os = "ios")]
    let global_scale_factor = Vec3::splat(0.8);
    #[cfg(target_os = "ios")]
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            scale: global_scale_factor,
            ..Default::default()
        },
        ..default()
    });

    #[cfg(not(target_os = "ios"))]
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
        resolution: (400., 512.).into(),
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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Running,
    GameOver,
    #[default]
    Paused,
}
