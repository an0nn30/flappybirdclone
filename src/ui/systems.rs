use crate::score::resources::Score;
use crate::ui::components::{GameOverMessage, ScoreText, WelcomeMessage};
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

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_over_message_query: Query<Entity, With<GameOverMessage>>,
) {
    let window = window_query.get_single().unwrap();
    // Prevent spamming new game over entities while in GameOver state since this system is called on Update
    if !game_over_message_query.is_empty() {
        return;
    }
    debug!("Game over, spawning message");
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/gameover.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
            ..default()
        },
        GameOverMessage,
    ));
}

// TODO: This is not despawning out game over thing...
pub fn reset_game_over(
    mut commands: Commands,
    game_over_msg_query: Query<Entity, With<GameOverMessage>>,
) {
    for message in game_over_msg_query.iter() {
        debug!("Despawning game over messsage");
        commands.entity(message).despawn_recursive();
    }
}
