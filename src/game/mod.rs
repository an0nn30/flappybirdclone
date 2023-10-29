use crate::game::systems::global_input;
use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, global_input);
    }
}
