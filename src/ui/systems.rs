use crate::score::resources::Score;
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

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/gameover.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
            ..default()
        },
        GameOverMessage,
    ));
}

pub fn update_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let digits = score
        .value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let parent_entity = if let Some(entity) = score.display_entity {
        // Update the existing parent entity if needed
        entity
    } else {
        // Create a new parent entity for the score display
        let entity = commands
            .spawn(SpriteBundle {
                transform: Transform::default(),
                ..default()
            })
            .id();
        score.display_entity = Some(entity);
        entity
    };

    // Despawn existing children (if any)
    // This step might vary depending on how you've set up your game.
    // For example, you could use a marker component to identify score digit entities.

    // Spawn or update children entities for each digit
    for (index, digit) in digits.iter().enumerate() {
        println!("Digit: {}", digit);
        let texture_path = format!("textures/{}.png", digit); // Assuming your textures are named like "0.png", "1.png", etc.
        let texture_handle = asset_server.load(texture_path.as_str());

        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(
                    window.width() / 2.,
                    window.height() / 2. + 200.,
                    1.,
                ),
                ..Default::default()
            }); // Optional: A marker component for easy identification of score digits
        });
    }

    // You may want to position or align the parent entity based on the number of digits
    // This positioning logic will depend on your game's UI layout
}
