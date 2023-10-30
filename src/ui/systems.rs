use crate::ui::components::{GameOverMessage, WelcomeMessage};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_welcome(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/message.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        WelcomeMessage,
    ));
}

pub fn toggle_welcome_message(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    welcome_message_query: Query<Entity, With<WelcomeMessage>>,
) {
    if game_state.get().ne(&(GameState::GameOver)) && game_state.get().ne(&(GameState::Paused)) {
        if let Ok(wm) = welcome_message_query.get_single() {
            commands.entity(wm).despawn();
        }
    }
}

// TODO: Change this so that we only run the system on game over event from the plugin, not within the system
pub fn spawn_game_over(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // if game_state.get().eq(&(GameState::GameOver)) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/gameover.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
            ..default()
        },
        GameOverMessage,
    ));
    // }
}
