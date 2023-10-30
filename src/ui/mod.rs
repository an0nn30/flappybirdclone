use crate::ui::systems::{spawn_game_over, spawn_welcome, toggle_welcome_message};
use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_welcome).add_systems(
            Update,
            (
                toggle_welcome_message,
                spawn_game_over.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}
